use std::{collections::HashMap, sync::Arc, thread::spawn};

use serde::Deserialize;

use crate::{
    client::{AuthType, Client, ProxyType},
    emitter::Emitter,
    file::File,
    manager::ManagerAction,
    models::{NewDownload, UpdateDownload},
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

        let chunk_count = match response.supports_range {
            true => options.chunk_count as u64,
            false => 1,
        };

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

        let range = Self::get_chunk_ranges(response.content_length, chunk_count);

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
        self.clone()
            .dispatch(ManagerAction::UpdateDownloadStatus(download_id));

        let worker = Registry::get_state().workers.get(&download_id).unwrap();
        let worker = worker.lock().await;

        let worker = DownloadWorker::new(
            worker.download.clone(),
            worker.chunks.clone(),
            Arc::clone(&worker.cancel_token),
            worker.download_id,
            Arc::clone(&worker.file),
            Arc::clone(self),
        );

        self.clone()
            .dispatch(ManagerAction::ManageWorkerResult(worker));
    }

    pub(super) async fn update_download_status_action(self: &Arc<Self>, download_id: i64) {
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

        let download = DownloadRepository::find(download_id).await.unwrap();
        Emitter::emit_event("download_item", download);
    }

    pub(super) async fn manage_worker_result_action(self: &Arc<Self>, worker: DownloadWorker) {
        Registry::spawn("start_download", async move {
            let result = worker.start_download().await;
            println!("{:?}", result);
        });
    }

    pub(super) async fn update_worker_network_report(
        self: &Arc<Self>,
        download_id: i64,
        bytes_len: u64,
    ) {
        Registry::dispatch(RegistryAction::UpdateNetworkReport(download_id, bytes_len));
    }
}
