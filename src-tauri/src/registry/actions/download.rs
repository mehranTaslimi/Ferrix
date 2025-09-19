use std::{
    collections::{HashMap, VecDeque},
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, AtomicU64},
        Arc,
    },
    time::Instant,
};

use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use tokio::sync::{Mutex, RwLock};
use tokio_util::sync::CancellationToken;

use super::super::{Registry, Report};

use crate::{
    client::{AuthType, Client, ProxyType},
    dispatch,
    emitter::Emitter,
    file::File,
    models::NewDownload,
    repository::{chunk::ChunkRepository, download::DownloadRepository},
    worker::Worker,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DownloadOptions {
    file_path: Option<String>,
    chunk_count: i64,
    proxy: Option<ProxyType>,
    auth: Option<AuthType>,
    headers: Option<HashMap<String, String>>,
    cookies: Option<HashMap<String, String>>,
    speed_limit: Option<i64>,
    max_retries: Option<i64>,
    delay_secs: Option<f64>,
    backoff_factor: Option<f64>,
    timeout_secs: Option<f64>,
}

pub trait DownloadActions {
    async fn recover_downloads() -> anyhow::Result<()>;
    async fn new_download(
        opt_id: String,
        url: String,
        options: DownloadOptions,
    ) -> anyhow::Result<()>;
    async fn probe_download(
        opt_id: String,
        url: String,
        options: DownloadOptions,
    ) -> anyhow::Result<()>;
    async fn add_download_to_queue(download_id: i64) -> anyhow::Result<()>;
    async fn pause_download(download_id: i64) -> anyhow::Result<()>;
    async fn resume_download(download_id: i64) -> anyhow::Result<()>;
    async fn remove_download(download_id: i64, remove_file: bool) -> anyhow::Result<()>;
    async fn prepare_download_data(download_id: i64) -> anyhow::Result<()>;
    async fn clean_download_data(download_id: i64) -> anyhow::Result<()>;
}

impl DownloadActions for Registry {
    async fn new_download(
        opt_id: String,
        url: String,
        options: DownloadOptions,
    ) -> anyhow::Result<()> {
        let url: Vec<&str> = url.split("\n").collect();

        for u in url {
            dispatch!(
                registry,
                ProbeDownload {
                    opt_id: opt_id.clone(),
                    url: u.to_string(),
                    options: options.clone()
                }
            );
        }

        Ok(())
    }

    async fn probe_download(
        opt_id: String,
        url: String,
        options: DownloadOptions,
    ) -> anyhow::Result<()> {
        let client = Client::new(
            &url,
            &options.auth,
            &options.proxy,
            &options.headers,
            &options.cookies,
        )?;

        let response = client.inspect().await?;

        let file_path = match options.file_path {
            Some(path) => {
                let mut path_buf = PathBuf::from(path);
                path_buf.push(&response.file_name);
                path_buf.to_string_lossy().into_owned()
            }
            None => {
                let default_path = File::get_default_path(&response.file_name).await?;
                File::get_available_filename(&default_path).await?
            }
        };

        let file_name = File::get_file_name(&file_path)?;

        let chunk_count = if response.supports_range {
            options.chunk_count.clamp(1, 5) as i64
        } else {
            1
        };

        let supports_range = if response.supports_range { 1 } else { 0 };

        let new_download = NewDownload {
            auth: match &options.auth {
                Some(val) => serde_json::to_string(&val).ok(),
                None => None,
            },
            backoff_factor: options.backoff_factor,
            chunk_count,
            content_type: response.content_type,
            cookies: match &options.cookies {
                Some(val) => serde_json::to_string(val).ok(),
                None => None,
            },
            delay_secs: options.delay_secs,
            extension: response.extension,
            file_name,
            file_path,
            headers: match &options.headers {
                Some(val) => serde_json::to_string(val).ok(),
                None => None,
            },
            max_retries: options.max_retries,
            proxy: match &options.proxy {
                Some(val) => serde_json::to_string(&val).ok(),
                None => None,
            },
            speed_limit: options.speed_limit,
            status: "queued".to_string(),
            timeout_secs: options.timeout_secs,
            total_bytes: response.content_length as i64,
            url: response.url,
            supports_range,
        };

        let download_id = DownloadRepository::add(new_download).await?;

        let mut ranges = Vec::with_capacity(chunk_count as usize);

        let base_chunk_size = response.content_length / chunk_count as u64;
        let remainder = (response.content_length % chunk_count as u64) as i64;

        let mut start = 0;

        for i in 0..chunk_count {
            let extra = if i < remainder { 1 } else { 0 };
            let end = start + base_chunk_size + extra - 1;

            ranges.push((start, end));
            start = end + 1;
        }

        ChunkRepository::create_all(download_id, ranges).await?;

        dispatch!(registry, AddDownloadToQueue { download_id });

        Ok(())
    }

    async fn recover_downloads() -> anyhow::Result<()> {
        let downloads = DownloadRepository::find_all(None).await?;
        let ids = downloads
            .into_iter()
            .filter(|d| d.status == "downloading" || d.status == "queued")
            .map(|d| d.id)
            .collect::<Vec<i64>>();

        for id in ids {
            dispatch!(registry, AddDownloadToQueue { download_id: id })?
        }

        Ok(())
    }

    async fn add_download_to_queue(download_id: i64) -> anyhow::Result<()> {
        let pending_queue = Arc::clone(&Self::get_state().pending_queue);
        let mut pending_queue = pending_queue.lock().await;
        pending_queue.push_back(download_id);

        let download = DownloadRepository::find(download_id).await?;
        Emitter::emit_event("download_item", download);

        dispatch!(registry, CheckAvailablePermit)
    }

    async fn pause_download(download_id: i64) -> anyhow::Result<()> {
        dispatch!(manager, PauseDownload, (download_id))
    }

    async fn resume_download(download_id: i64) -> anyhow::Result<()> {
        dispatch!(registry, AddDownloadToQueue { download_id })
    }

    async fn remove_download(download_id: i64, remove_file: bool) -> anyhow::Result<()> {
        let file_path = DownloadRepository::delete(download_id).await?;

        if remove_file {
            File::remove_file(&file_path)?;
        }

        Ok(())
    }

    async fn prepare_download_data(download_id: i64) -> anyhow::Result<()> {
        let workers = Arc::clone(&Self::get_state().workers);
        let reports = Arc::clone(&Self::get_state().reports);

        let download = DownloadRepository::find(download_id).await?;
        let chunks = ChunkRepository::find_all(download_id).await?;

        let not_downloaded_chunks = chunks
            .into_iter()
            .filter(|chunk| chunk.downloaded_bytes < chunk.end_byte - chunk.start_byte)
            .collect::<Vec<_>>();

        let file = File::new(
            download_id,
            &download.file_path,
            download.total_bytes as u64,
        )
        .await?;

        workers.insert(
            download.id,
            Arc::new(RwLock::new(Worker {
                download: download.clone(),
                chunks: not_downloaded_chunks.clone(),
                cancel_token: Arc::new(CancellationToken::new()),
                file: Arc::new(file),
            })),
        );

        let chunks_wrote_bytes: DashMap<i64, AtomicU64> = not_downloaded_chunks
            .iter()
            .map(|f| (f.chunk_index, AtomicU64::new(f.downloaded_bytes as u64)))
            .collect();

        let buffer = File::get_chunks_bytes_from_file(download.id).await?;

        reports.insert(
            download.id,
            Arc::new(Report {
                total_downloaded_bytes: AtomicU64::new(download.downloaded_bytes as u64),
                downloaded_bytes: AtomicU64::new(0),
                total_wrote_bytes: AtomicU64::new(download.downloaded_bytes as u64),
                wrote_bytes: AtomicU64::new(0),
                download_history: Mutex::new(VecDeque::with_capacity(10)),
                wrote_history: Mutex::new(VecDeque::with_capacity(10)),
                chunks_wrote_bytes,
                total_bytes: download.total_bytes as u64,
                speed_bps: AtomicU64::new(0),
                last_update_downloaded_bytes: AtomicU64::new(download.downloaded_bytes as u64),
                stable_speed: AtomicBool::new(false),
                last_update_time: Arc::new(Mutex::new(Instant::now())),
                buffer: Arc::new(buffer),
            }),
        );

        dispatch!(manager, StartDownload, (download.id));

        Ok(())
    }

    async fn clean_download_data(download_id: i64) -> anyhow::Result<()> {
        let reports = Arc::clone(&Self::get_state().reports);
        let workers = Arc::clone(&Self::get_state().workers);

        reports.remove(&download_id);
        workers.remove(&download_id);

        Ok(())
    }
}
