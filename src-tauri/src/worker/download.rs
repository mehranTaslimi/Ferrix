use std::{
    sync::{atomic::Ordering, Arc},
    time::Duration,
};

use futures_util::{future::join_all, StreamExt};
use tauri::http::header::RANGE;
use tauri_plugin_http::reqwest::Client;
use tokio::time::timeout;

use crate::{
    // models::Chunk,
    models::DownloadChunk,
    registry::Registry,
    worker::outcome::{DownloadStatus, WorkerOutcome},
};

impl super::DownloadWorker {
    pub async fn start_download(&self) {
        let max_retries = self.download_ref.lock().await.download.max_retries;
        let mut retries = 0;

        // let _ = self.emit_and_update_download_status("downloading").await;

        loop {
            let mut futures = self.not_downloaded_chunks().await.map(|chunk| {
                let self_clone = self.clone();
                let task_name = format!(
                    "chunk download: {}, index: {}",
                    self_clone.download_id, chunk.chunk_index
                );
                Registry::spawn(
                    &task_name,
                    async move { self_clone.download_chunk(chunk).await },
                )
            });

            let results = join_all(&mut futures).await;

            let outcome = self.classify_results(results);

            match outcome {
                WorkerOutcome::Finished => {
                    self.handle_finished().await;
                    break;
                }
                WorkerOutcome::Paused => {
                    self.handle_paused().await;
                    break;
                }
                WorkerOutcome::Errored | WorkerOutcome::Mixed => {
                    if retries < max_retries {
                        retries += 1;
                        self.handle_retry().await;
                        continue;
                    } else {
                        self.handle_failed().await;
                        break;
                    }
                }
            };
        }
    }

    async fn download_chunk(&self, chunk: DownloadChunk) -> Result<DownloadStatus, i64> {
        let chunk_index = chunk.chunk_index;
        let mut downloaded_bytes = chunk.downloaded_bytes;
        let start_byte = chunk.start_byte;
        let end_byte = chunk.end_byte;
        let url = self.download_ref.lock().await.download.url.clone();
        let range_header = format!("bytes={}-{}", start_byte + downloaded_bytes, end_byte);

        let client = Client::builder().build().map_err(|_| chunk_index)?;

        let response = client
            .get(url)
            .header(RANGE, range_header)
            .send()
            .await
            .map_err(|_| chunk_index)?;

        let mut stream = response.bytes_stream();
        let cancellation_token = self.cancellation_token.clone();
        let report = Arc::clone(&Registry::get_state().report);

        loop {
            tokio::select! {
                biased;

                _ = cancellation_token.cancelled() => {
                    // self.update_chunk(chunk_index, false, "").await.map_err(|_| chunk_index)?;
                    return Ok(DownloadStatus::Paused);
                }

                maybe_bytes = timeout(Duration::from_secs(1), stream.next()) => {
                    match maybe_bytes {
                        Ok(Some(Ok(bytes))) => {

                            let bytes_len = bytes.len() as u64;

                            // self.file
                            //     .send((
                            //         chunk_index as u64,
                            //         (start_byte + downloaded_bytes) as u64,
                            //         bytes_len,
                            //         bytes.to_vec(),
                            //     ))
                            //     .map_err(|e| e.to_string()).map_err(|_| chunk_index)?;

                            // self.limiter(bytes_len as u32).await;

                            downloaded_bytes += bytes_len as i64;

                            if let Some(report) = report.get(&self.download_id) {
                                report.downloaded_bytes.fetch_add(bytes_len, Ordering::Relaxed);
                                report.total_downloaded_bytes.fetch_add(bytes_len, Ordering::Relaxed);
                            };
                        },
                        Ok(Some(Err(err))) => {
                            // self.update_chunk(chunk_index, true, &err.to_string()).await.map_err(|_| chunk_index)?;
                            return Err(chunk_index);
                        },
                        Ok(None) => {
                            // self.update_chunk(chunk_index, false, "").await.map_err(|_| chunk_index)?;
                            return Ok(DownloadStatus::Finished);
                        },
                        Err(_) => {
                            // self.update_chunk(chunk_index, true, "timeout").await.map_err(|_| chunk_index)?;
                            return Err(chunk_index);
                        }
                    }
                }

            }
        }
    }
}
