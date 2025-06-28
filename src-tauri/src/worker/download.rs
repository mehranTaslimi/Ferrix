use std::{sync::Arc, time::Duration};

use futures_util::{future::join_all, StreamExt};
use tauri::http::header::RANGE;
use tauri_plugin_http::reqwest::Client;
use tokio::time::{interval, sleep, MissedTickBehavior};

use crate::{events::dispatch, models::Chunk, utils::app_state::AppEvent};

#[derive(Debug)]
enum DownloadStatus {
    Paused,
    Finished,
}

#[derive(Debug)]
enum WorkerOutcome {
    Finished,
    Paused,
    Errored,
    Mixed,
}

impl super::DownloadWorker {
    pub async fn start_download(&self) {
        let max_retries = 5;
        let mut retries = 0;

        self.listen_to_report_internet().await;
        self.listen_to_report_disk().await;
        let _ = self.emit_and_update_download_status("downloading").await;

        loop {
            let chunks = self
                .chunks
                .lock()
                .await
                .clone()
                .into_iter()
                .filter(|chunk| chunk.downloaded_bytes < chunk.end_byte - chunk.start_byte);

            let mut futures = chunks.map(|chunk| async move { self.download_chunk(chunk).await });

            let results = join_all(&mut futures).await;
            let outcome = Self::classify_results(results);

            match outcome {
                WorkerOutcome::Finished => {
                    if self.disk_report.lock().await.total_wrote_bytes
                        < self.internet_report.lock().await.downloaded_bytes
                    {
                        let _ = self.emit_and_update_download_status("writing").await;
                    }

                    loop {
                        let wrote = self.disk_report.lock().await.total_wrote_bytes;
                        let downloaded = self.internet_report.lock().await.downloaded_bytes;

                        if wrote == downloaded {
                            break;
                        }

                        sleep(Duration::from_millis(100)).await;
                    }

                    dispatch(
                        &self.app_event,
                        AppEvent::DownloadFinished(self.download_id),
                    );
                    let _ = self.emit_and_update_download_status("completed").await;
                    break;
                }
                WorkerOutcome::Paused => {
                    dispatch(&self.app_event, AppEvent::DownloadPaused(self.download_id));
                    let _ = self.emit_and_update_download_status("paused").await;
                    break;
                }
                WorkerOutcome::Errored | WorkerOutcome::Mixed => {
                    if retries <= max_retries {
                        sleep(Duration::from_secs(5)).await;

                        retries += 1;

                        continue;
                    } else {
                        let _ = self.emit_and_update_download_status("failed").await;
                        dispatch(&self.app_event, AppEvent::DownloadFailed(self.download_id));
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
        let report = Arc::clone(&self.internet_report);
        let cancellation_token = self.cancellation_token.clone();

        let mut interval = interval(Duration::from_millis(100));
        interval.set_missed_tick_behavior(MissedTickBehavior::Delay);

        loop {
            tokio::select! {
                biased;

                _ = cancellation_token.cancelled() => {
                    self.update_chunk(chunk_index, false, "").await.map_err(|_| chunk_index)?;
                    return Ok(DownloadStatus::Paused);
                }

                _ = interval.tick() => {}

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

    fn classify_results(results: Vec<Result<DownloadStatus, i64>>) -> WorkerOutcome {
        let mut has_finished = false;
        let mut has_paused = false;
        let mut has_error = false;

        for result in &results {
            match result {
                Ok(DownloadStatus::Finished) => has_finished = true,
                Ok(DownloadStatus::Paused) => has_paused = true,
                Err(_) => has_error = true,
            }
        }

        match (has_finished, has_paused, has_error) {
            (true, false, false) => WorkerOutcome::Finished,
            (false, true, false) => WorkerOutcome::Paused,
            (false, false, true) => WorkerOutcome::Errored,
            _ => WorkerOutcome::Mixed,
        }
    }
}
