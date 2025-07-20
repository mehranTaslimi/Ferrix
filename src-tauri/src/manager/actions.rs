use std::{
    collections::HashMap,
    sync::{atomic::Ordering, Arc},
};

use serde::Deserialize;

use crate::{
    client::{AuthType, Client, ProxyType},
    emitter::Emitter,
    file::File,
    manager::ManagerAction,
    models::{NewDownload, UpdateChunk, UpdateDownload},
    registry::{Registry, RegistryAction},
    repository::{chunk::ChunkRepository, download::DownloadRepository},
    worker::DownloadWorker,
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

impl super::DownloadsManager {
    pub async fn add_new_download(url: String, options: DownloadOptions) -> Result<(), String> {
        let client = Client::new(
            &url,
            options.timeout_secs.unwrap_or(30.0),
            &options.auth,
            &options.proxy,
            &options.headers,
            &options.cookies,
        )
        .map_err(|e| e.to_string())?;

        let response = client.inspect().await.map_err(|e| e.to_string())?;

        let file_path = match options.file_path {
            Some(path) => path,
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

        let download_id = DownloadRepository::add(new_download)
            .await
            .map_err(|e| e.to_string())?;

        let range = Self::get_chunk_ranges(response.content_length, chunk_count as u64);

        ChunkRepository::create_all(download_id, range)
            .await
            .map_err(|e| {
                e.iter()
                    .map(|f| f.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            })?;

        Registry::dispatch(RegistryAction::NewDownloadQueue(download_id));

        Ok(())
    }

    pub(super) async fn start_download_action(self: &Arc<Self>, download_id: i64) {
        let worker = Registry::get_state().workers.get(&download_id).unwrap();
        let worker = worker.lock().await;

        self.dispatch(ManagerAction::UpdateDownloadStatus(
            "downloading".to_string(),
            download_id,
        ));

        let worker = DownloadWorker::new(
            worker.download.clone(),
            worker.chunks.clone(),
            Arc::clone(&worker.cancel_token),
            Arc::clone(&worker.file),
            Arc::clone(self),
        );
        self.dispatch(ManagerAction::ManageWorkerResult(worker));
    }

    pub(super) async fn update_download_status_action(
        self: &Arc<Self>,
        status: String,
        download_id: i64,
    ) {
        DownloadRepository::update(
            download_id,
            UpdateDownload {
                status: Some(status),
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

        let download = DownloadRepository::find(download_id).await.unwrap();
        Emitter::emit_event("download_item", download);
    }

    pub(super) async fn manage_worker_result_action(self: &Arc<Self>, worker: Arc<DownloadWorker>) {
        Self::start_monitoring();

        let self_clone = Arc::clone(&self);

        Registry::spawn(async move {
            let status = worker.start_download().await;
            self_clone.dispatch(ManagerAction::UpdateDownloadStatus(
                status.to_string(),
                worker.download.id,
            ));
            self_clone.dispatch(ManagerAction::UpdateChunks(worker.download.id));
        });
    }

    pub(super) async fn pause_download_action(self: &Arc<Self>, download_id: i64) {
        let workers = Arc::clone(&Registry::get_state().workers);
        let maybe_worker = workers.get(&download_id);

        if let Some(worker) = maybe_worker {
            let worker = worker.lock().await;
            worker.cancel_token.cancel();
        }
    }

    pub(super) async fn update_chunks_action(self: &Arc<Self>, download_id: i64) {
        let workers = Arc::clone(&Registry::get_state().workers);
        let maybe_worker = workers.get(&download_id);
        let reports = Arc::clone(&Registry::get_state().reports);

        if let Some(worker) = maybe_worker {
            let worker = worker.lock().await.clone();
            let file_path = worker.download.file_path.clone();

            let update_chunks = worker
                .chunks
                .iter()
                .map(|chunk| {
                    let start_byte = chunk.start_byte as u64;
                    let chunk_index = chunk.chunk_index;

                    let wrote_bytes = reports
                        .get(&download_id)
                        .and_then(|r| {
                            r.chunks_wrote_bytes
                                .get(&chunk_index)
                                .map(|v| v.load(Ordering::Relaxed))
                        })
                        .unwrap_or(0);

                    //TODO: pref issue
                    // let expected_hash =
                    //     Self::compute_partial_hash(&file_path, start_byte, wrote_bytes).ok();

                    UpdateChunk {
                        downloaded_bytes: Some(wrote_bytes as i64),
                        error_message: None,
                        expected_hash: Some(String::new()),
                        has_error: Some(false),
                        chunk_index,
                    }
                })
                .collect::<Vec<_>>();

            if let Err(errors) = ChunkRepository::update_all(download_id, update_chunks).await {
                errors.iter().for_each(|err| {
                    Emitter::emit_error(err.to_string());
                });
            }
        }

        Registry::dispatch(RegistryAction::CleanDownloadedItemData(download_id));
    }

    pub(super) async fn validate_chunks_hash_action(self: &Arc<Self>, download_id: i64) {
        //TODO: perf issue
        // self.dispatch(ManagerAction::UpdateDownloadStatus(
        //     "validating".to_string(),
        //     download_id,
        // ));

        // let download = DownloadRepository::find(download_id).await.unwrap();

        // if download.downloaded_bytes == 0 {
        //     Registry::dispatch(RegistryAction::NewDownloadQueue(download_id));
        //     return;
        // }

        // let chunks = ChunkRepository::find_all(download_id).await.unwrap();

        // let file_path = download.file_path;

        // let invalid_chunks = Self::validate_chunks_hash(&file_path, chunks);

        // if !invalid_chunks.is_empty() {
        //     self.dispatch(ManagerAction::ResetChunks(download_id, invalid_chunks));
        // } else {
        //     Registry::dispatch(RegistryAction::NewDownloadQueue(download_id));
        // }

        Registry::dispatch(RegistryAction::NewDownloadQueue(download_id));
    }

    pub(super) async fn reset_chunks_action(
        self: &Arc<Self>,
        download_id: i64,
        chunks_index: Vec<i64>,
    ) {
        let update_chunks = chunks_index
            .into_iter()
            .map(|chunk_index| UpdateChunk {
                chunk_index,
                downloaded_bytes: Some(0),
                error_message: Some("invalid chunk hash".to_string()),
                expected_hash: None,
                has_error: Some(true),
            })
            .collect::<Vec<UpdateChunk>>();

        if let Err(errors) = ChunkRepository::update_all(download_id, update_chunks).await {
            errors.iter().for_each(|err| {
                Emitter::emit_error(err.to_string());
            });
        }

        Registry::dispatch(RegistryAction::NewDownloadQueue(download_id));
    }
}
