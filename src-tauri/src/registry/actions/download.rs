use std::{
    collections::VecDeque,
    sync::{
        atomic::{AtomicBool, AtomicU64},
        Arc,
    },
    time::Instant,
};

use dashmap::DashMap;
use tokio::sync::{Mutex, RwLock};
use tokio_util::sync::CancellationToken;

use super::super::{Registry, Report};

use crate::{
    dispatch,
    emitter::Emitter,
    file::File,
    repository::{chunk::ChunkRepository, download::DownloadRepository},
    worker::Worker,
};

pub trait DownloadActions {
    async fn recover_downloads() -> anyhow::Result<()>;
    async fn new_download(download_id: i64) -> anyhow::Result<()>;
    async fn pause_download(download_id: i64) -> anyhow::Result<()>;
    async fn resume_download(download_id: i64) -> anyhow::Result<()>;
    async fn remove_download(download_id: i64, remove_file: bool) -> anyhow::Result<()>;
    async fn prepare_download_data(download_id: i64) -> anyhow::Result<()>;
    async fn clean_download_data(download_id: i64) -> anyhow::Result<()>;
}

impl DownloadActions for Registry {
    async fn recover_downloads() -> anyhow::Result<()> {
        let downloads = DownloadRepository::find_all(None).await?;
        let ids = downloads
            .into_iter()
            .filter(|d| d.status == "downloading" || d.status == "queued")
            .map(|d| d.id)
            .collect::<Vec<i64>>();

        for id in ids {
            dispatch!(registry, NewDownload, (id))?
        }

        Ok(())
    }

    async fn new_download(download_id: i64) -> anyhow::Result<()> {
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
        dispatch!(registry, NewDownload, (download_id))
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

        // let buffer = File::get_chunks_bytes_from_file(download.id).await?;

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
                buffer: Arc::new(DashMap::new()),
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
