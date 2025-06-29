use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use futures_util::{future::join_all, StreamExt};
use tauri::http::header::RANGE;
use tauri_plugin_http::reqwest::Client;
use tokio::time::sleep;

use crate::{
    models::Chunk,
    worker::outcome::{DownloadStatus, WorkerOutcome},
};

impl super::DownloadWorker {
    pub async fn start_download(&self) {
        let max_retries = 5;
        let mut retries = 0;

        self.listen_to_report_internet().await;
        self.listen_to_report_disk().await;
        let _ = self.emit_and_update_download_status("downloading").await;

        loop {
            let mut futures = self
                .not_downloaded_chunks()
                .await
                .map(|chunk| async move { self.download_chunk(chunk).await });

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

    async fn download_chunk(&self, chunk: Chunk) -> Result<DownloadStatus, i64> {
        let chunk_index = chunk.chunk_index;
        let downloaded_bytes = chunk.downloaded_bytes;
        let start_byte = chunk.start_byte;
        let end_byte = chunk.end_byte;
        let url = self.download.lock().await.url.clone();
        let range_header = format!("bytes={}-{}", start_byte + downloaded_bytes, end_byte);

        let client = Client::new();

        let response = client
            .get(url)
            .header(RANGE, range_header)
            .send()
            .await
            .map_err(|_| chunk_index)?;

        let mut stream = response.bytes_stream();
        let mut downloaded = downloaded_bytes as u64;
        let mut downloaded_in_one_sec = 0f32;
        let report = Arc::clone(&self.internet_report);
        let cancellation_token = self.cancellation_token.clone();
        let mut start = Instant::now();

        loop {
            tokio::select! {
                biased;

                _ = cancellation_token.cancelled() => {
                    self.update_chunk(chunk_index, false, "").await.map_err(|_| chunk_index)?;
                    return Ok(DownloadStatus::Paused);
                }

                maybe_bytes = stream.next() => {
                    match maybe_bytes {
                        Some(Ok(bytes)) => {

                            let bytes_len = bytes.len() as u64;

                            self.file_writer
                                .send((
                                    chunk_index as u64,
                                    (start_byte as u64) + downloaded,
                                    bytes_len,
                                    bytes.to_vec(),
                                ))
                                .map_err(|e| e.to_string()).map_err(|_| chunk_index)?;

                            downloaded += bytes_len;
                            downloaded_in_one_sec += bytes_len as f32;

                            let bandwidth_limit = *self.bandwidth_limit.lock().await;
                            let elapsed = start.elapsed().as_secs_f32();
                            let current_speed = downloaded_in_one_sec / elapsed;

                            if current_speed > bandwidth_limit {
                                let expected_time = downloaded_in_one_sec / bandwidth_limit;
                                let sleep_duration = expected_time - elapsed;
                                if sleep_duration > 0.001 {
                                    sleep(Duration::from_secs_f32(sleep_duration)).await;
                                }
                            }

                            if elapsed >= 1.0 {
                                start = Instant::now();
                                downloaded_in_one_sec = 0.0;
                            }

                            let mut report = report.lock().await;
                            report.downloaded_bytes += bytes_len;
                            report.received_bytes += bytes_len as f64;


                        },
                        Some(Err(err)) => {
                            self.update_chunk(chunk_index, true, &err.to_string()).await.map_err(|_| chunk_index)?;
                            return Err(chunk_index);
                        },
                        None => {
                            self.update_chunk(chunk_index, false, "").await.map_err(|_| chunk_index)?;
                            return Ok(DownloadStatus::Finished);
                        },
                    }
                }

            }
        }
    }
}
