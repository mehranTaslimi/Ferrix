use std::{collections::HashMap, sync::Arc};

use once_cell::sync::OnceCell;
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::{
    client::{AuthType, Client, ProxyType},
    emitter::Emitter,
    file::File,
    models::{DownloadWithChunk, NewDownload, UpdateDownload},
    repository::{chunk::ChunkRepository, download::DownloadRepository},
};

#[derive(Debug, Deserialize)]
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

impl super::Registry {
    pub async fn add_new_download(url: String, options: DownloadOptions) -> Result<(), String> {
        let client = Client::new(&url, AuthType::None, ProxyType::None)?;
        let response = client.inspect().await?;

        let file_path = match options.file_path {
            Some(path) => path,
            None => {
                let default_path = File::get_default_path(&response.file_name).await?;
                File::get_available_filename(&default_path).await?
            }
        };

        let file_name = File::get_file_name(&file_path)?;

        let new_download = NewDownload {
            auth: serde_json::to_string(&options.auth).ok(),
            backoff_factor: options.backoff_factor,
            chunk_count: options.chunk_count,
            content_type: response.content_type,
            cookies: serde_json::to_string(&options.cookies).ok(),
            delay_secs: options.delay_secs,
            extension: response.extension,
            file_name,
            file_path,
            headers: serde_json::to_string(&options.headers).ok(),
            max_retries: options.max_retries,
            proxy: serde_json::to_string(&options.proxy).ok(),
            speed_limit: options.speed_limit,
            status: "queued".to_string(),
            timeout_secs: options.timeout_secs,
            total_bytes: response.content_length as i64,
            url: response.url,
        };

        let download_id = DownloadRepository::add(new_download)
            .await
            .map_err(|e| e.to_string())?;

        let range = Self::get_chunk_ranges(response.content_length, options.chunk_count as u64)?;

        ChunkRepository::create_all(download_id, range)
            .await
            .map_err(|e| {
                e.iter()
                    .map(|f| f.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            })?;

        Self::add_download_to_pending_queue(download_id);

        Ok(())
    }

    pub fn add_download_to_pending_queue(download_id: i64) {
        let pending_queue = Arc::clone(&Self::get_state().pending_queue);
        Self::spawn("add_download_to_pending_queue", async move {
            let download = DownloadRepository::find(download_id).await.unwrap();
            Emitter::emit_event("download_item", download);

            let mut pending_queue = pending_queue.lock().await;
            pending_queue.push_back(download_id);
        });
    }

    pub fn add_download_to_downloading_queue() {
        let pending_queue = Arc::clone(&Self::get_state().pending_queue);

        Self::spawn("add_download_to_downloading_queue", async move {
            let mut pending_queue = pending_queue.lock().await;

            if let Some(pend_download) = pending_queue.pop_front() {
                Self::add_download_to_map(pend_download);
            }
        });
    }

    pub fn add_download_to_map(download_id: i64) {
        let downloading_map = Arc::clone(&Self::get_state().downloading_map);

        Self::spawn("add_download_to_map", async move {
            DownloadRepository::update(
                download_id,
                UpdateDownload {
                    status: Some("downloading".to_string()),
                    auth: None,
                    backoff_factor: None,
                    cookies: None,
                    delay_secs: None,
                    headers: None,
                    max_retries: None,
                    proxy: None,
                    speed_limit: None,
                    timeout_secs: None,
                    total_bytes: None,
                },
            )
            .await
            .unwrap();

            let download_result = DownloadRepository::find(download_id).await;

            if let Ok(ref download) = download_result {
                Emitter::emit_event("download_item", download.clone());
            }

            let chunks = ChunkRepository::find_all(download_id).await;

            match (download_result, chunks) {
                (Ok(download), Ok(chunks)) => {
                    let mut downloading_map = downloading_map.lock().await;
                    downloading_map.insert(
                        download_id,
                        Arc::new(Mutex::new(DownloadWithChunk {
                            download,
                            chunks,
                            worker_created: false,
                            cancel_token: OnceCell::new(),
                        })),
                    );
                }
                (Err(err), _) | (_, Err(err)) => {
                    Emitter::emit_error(err.to_string());
                }
            }
        });
    }
}
