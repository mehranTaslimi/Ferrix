use std::collections::HashMap;

use serde::Deserialize;

use crate::{
    client::{AuthType, Client, ProxyType},
    models::NewDownload,
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
    pub(super) async fn add_new_download(
        url: String,
        options: DownloadOptions,
    ) -> Result<(), String> {
        let client = Client::new(&url, AuthType::None, ProxyType::None)?;
        let response = client.inspect().await?;

        let new_download = NewDownload {
            auth: serde_json::to_string(&options.auth).ok(),
            backoff_factor: options.backoff_factor,
            chunk_count: options.chunk_count,
            content_type: response.content_type,
            cookies: serde_json::to_string(&options.cookies).ok(),
            delay_secs: options.delay_secs,
            extension: response.extension,
            file_name: response.file_name,
            file_path: options.file_path.unwrap_or("".to_string()),
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

        Self::dispatch(super::RegistryAction::AddDownloadToQueue(download_id));

        Ok(())
    }

    pub async fn add_download_queue(download_id: i64) {
        let mut download_queue = Self::get_state().download_queue.lock().await;
        download_queue.push_back(download_id);
    }
}
