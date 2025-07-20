use futures_util::{future::join_all, StreamExt};
use std::{sync::Arc, time::Duration};
use tauri::http::StatusCode;

use crate::{
    client::{Client, ClientError},
    file::WriteMessage,
    manager,
    models::DownloadChunk,
    registry::{Registry, RegistryAction},
    worker::outcome::NormalizedDownloadStatus,
};

use super::*;

impl DownloadWorker {
    pub async fn start_download(self: &Arc<Self>) -> DownloadStatus {
        let backoff_factor = self.download.backoff_factor;
        let max_retries = self.download.max_retries as u64;
        let mut retries = 0u64;

        loop {
            let mut futures = self.chunks.clone().into_iter().map(|chunk| {
                let self_clone = Arc::clone(self);
                Registry::spawn(async move {
                    (chunk.chunk_index, self_clone.download_chunk(chunk).await)
                })
            });

            let results = join_all(&mut futures).await;
            let classify_results = self.classify_results(results);

            if classify_results
                .keys()
                .all(|s| *s == NormalizedDownloadStatus::Finished)
            {
                return DownloadStatus::Completed;
            }

            if classify_results
                .keys()
                .any(|s| *s == NormalizedDownloadStatus::Error)
            {
                return DownloadStatus::Failed;
            }

            if classify_results
                .keys()
                .any(|s| *s == NormalizedDownloadStatus::Paused)
            {
                return DownloadStatus::Paused;
            }

            if classify_results
                .keys()
                .any(|s| *s == NormalizedDownloadStatus::Retry)
            {
                if retries == max_retries {
                    return DownloadStatus::Failed;
                }

                self.manager
                    .dispatch(manager::ManagerAction::UpdateDownloadStatus(
                        DownloadStatus::Error.to_string(),
                        self.download.id,
                    ));

                retries += 1;
                let wait_time = backoff_factor.powf(retries as f64);
                tokio::time::sleep(Duration::from_secs_f64(wait_time)).await;
                continue;
            }
        }
    }

    async fn download_chunk(
        self: &Arc<Self>,
        chunk: DownloadChunk,
    ) -> Result<outcome::ChunkDownloadStatus, ClientError> {
        let chunk_index = chunk.chunk_index;
        let mut downloaded_bytes = chunk.downloaded_bytes;
        let start_byte = chunk.start_byte;
        let end_byte = chunk.end_byte;

        let cancellation_token = Arc::clone(&self.cancel_token);

        if chunk_index == 2 {
            return Err(ClientError::Http {
                status: StatusCode::GATEWAY_TIMEOUT,
                message: "Error".to_string(),
            });
        }

        let client = match Client::new(
            &self.download.url,
            self.download.timeout_secs as f64,
            &self.download.auth,
            &self.download.proxy,
            &self.download.headers,
            &self.download.cookies,
        ) {
            Ok(c) => c,
            Err(err) => {
                if !err.is_retryable() {
                    cancellation_token.cancel();
                }
                return Err(err);
            }
        };

        let range = match self.download.supports_range {
            true => Some((start_byte + downloaded_bytes, end_byte)),
            false => None,
        };

        let mut stream = client.stream(range).await?;

        loop {
            tokio::select! {

                _ = cancellation_token.cancelled() => {
                    return Ok(outcome::ChunkDownloadStatus::Paused);
                }

                maybe_bytes = stream.next() => {
                    match maybe_bytes {
                        Some(Ok(bytes)) => {

                            let bytes_len = bytes.len() as u64;

                            let write_message: WriteMessage = (
                                chunk_index,
                                (start_byte + downloaded_bytes) as u64,
                                bytes_len,
                                bytes.to_vec(),
                            );

                            self.file
                                .send(write_message)
                                .unwrap();

                            downloaded_bytes += bytes_len as i64;

                            Registry::dispatch(RegistryAction::UpdateNetworkReport(self.download.id, bytes_len));
                        },
                        Some(Err(err)) => {
                            if !err.is_retryable() {
                                cancellation_token.cancel();
                            }
                            return Err(err);
                        },
                        None => {
                            return Ok(outcome::ChunkDownloadStatus::Finished);
                        },
                    }
                }

            }
        }
    }
}
