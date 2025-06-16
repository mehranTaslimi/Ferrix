use std::{sync::Arc, time::Duration};

use futures_util::{future::join_all, StreamExt};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tauri::{http::header::RANGE, AppHandle};
use tauri_plugin_http::reqwest::Client;
use tokio::{
    spawn,
    sync::{broadcast::Sender, mpsc, Mutex},
    time::sleep,
};
use tokio_util::sync::CancellationToken;

use crate::{
    db::downloads::{
        get_download_chunks_by_download_id, get_downloads_by_id, insert_download_chunks,
        insert_new_download, update_chunk_downloaded, update_download_status,
    },
    events::{dispatch, emit_app_event},
    manager::{
        compute_partial_hash::compute_partial_hash,
        file_writer::{file_writer, WriteMessage},
        get_chunk_ranges::get_chunk_ranges,
    },
    models::{Chunk, ChunkCount, Download, DownloadId, FileInfo},
    utils::app_state::AppEvent,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct SpeedAndRemaining {
    speed: f64,
    remaining_time: f64,
}

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

#[derive(Debug, Clone)]
struct Report {
    downloaded_bytes: u64,
    received_bytes: f64,
}

#[derive(Clone, Debug)]
pub struct DownloadWorker {
    pool: SqlitePool,
    app_handle: AppHandle,
    chunks: Vec<Chunk>,
    download: Download,
    file_writer_tx: mpsc::Sender<WriteMessage>,
    cancellation_token: CancellationToken,
    report: Arc<Mutex<Report>>,
    app_event: Sender<AppEvent>,
    pub download_id: DownloadId,
}

impl DownloadWorker {
    pub async fn new(
        pool: SqlitePool,
        app_handle: AppHandle,
        app_event: Sender<AppEvent>,
        download_id: Option<DownloadId>,
        file_info: Option<FileInfo>,
        chunk_count: Option<ChunkCount>,
    ) -> Result<Self, String> {
        let (download, chunks) = if let Some(download_id) = download_id {
            Self::get_download_and_chunks(&pool, download_id).await?
        } else {
            let file_info =
                file_info.ok_or_else(|| "missing file_info for new download".to_string())?;
            let chunk_count =
                chunk_count.ok_or_else(|| "missing chunk_count for new download".to_string())?;

            let download_id = Self::create_download(&pool, file_info, chunk_count).await?;
            Self::get_download_and_chunks(&pool, download_id).await?
        };

        let downloaded_bytes = download.downloaded_bytes as u64;
        let download_id = download.id;
        let file_writer_tx = file_writer(&download.file_path, download.total_bytes as u64).await?;

        Ok(Self {
            pool,
            download,
            chunks,
            download_id,
            app_event,
            file_writer_tx,
            app_handle,
            cancellation_token: CancellationToken::new(),
            report: Arc::new(Mutex::new(Report {
                downloaded_bytes,
                received_bytes: 0.0,
            })),
        })
    }

    pub async fn create_download(
        pool: &SqlitePool,
        file_info: FileInfo,
        chunk_count: i64,
    ) -> Result<DownloadId, String> {
        let download_id = insert_new_download(&pool, file_info.clone(), chunk_count).await?;
        let ranges = get_chunk_ranges(file_info.total_bytes as u64, chunk_count as u8)?;
        insert_download_chunks(&pool, download_id, ranges).await?;

        Ok(download_id)
    }

    async fn get_download_and_chunks(
        pool: &SqlitePool,
        download_id: i64,
    ) -> Result<(Download, Vec<Chunk>), String> {
        let download = get_downloads_by_id(&pool, download_id).await?;
        let chunks = get_download_chunks_by_download_id(&pool, download_id).await?;

        Ok((download, chunks))
    }

    pub async fn start_download(&self) {
        let max_retries = 3;
        let mut retries = 0;

        self.listen_to_report();

        loop {
            let futures = self
                .chunks
                .clone()
                .into_iter()
                .filter(|chunk| chunk.downloaded_bytes < chunk.end_byte - chunk.start_byte)
                .map(|chunk| async move { self.download_chunk(chunk).await });

            let results = join_all(futures).await;

            let outcome = Self::classify_results(results);

            match outcome {
                WorkerOutcome::Finished => {
                    dispatch(&self.app_event, AppEvent::WorkerFinished(self.download_id));
                    self.cancellation_token.cancel();
                    break;
                }
                WorkerOutcome::Paused => {
                    dispatch(&self.app_event, AppEvent::WorkerPaused);
                    self.cancellation_token.cancel();
                    break;
                }
                WorkerOutcome::Errored | WorkerOutcome::Mixed => {
                    if retries < max_retries {
                        retries += 1;
                        continue;
                    } else {
                        self.cancellation_token.cancel();
                        break;
                    }
                }
            };
        }
    }

    async fn download_chunk(&self, chunk: Chunk) -> Result<DownloadStatus, String> {
        let downloaded_bytes = chunk.downloaded_bytes as u64;
        let start_byte = chunk.start_byte as u64;
        let end_byte = chunk.end_byte as u64;
        let url = self.download.url.clone();

        let start_byte = start_byte + downloaded_bytes;

        let client = Client::new();

        let range_header = format!("bytes={}-{}", start_byte, end_byte);

        let response = client
            .get(url)
            .header(RANGE, range_header)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let stream = response.bytes_stream();
        let mut stream_fuse = stream.fuse();
        let mut downloaded = downloaded_bytes as u64;
        let report = Arc::clone(&self.report);
        let cancellation_token = self.cancellation_token.clone();

        loop {
            tokio::select! {
                biased;

                _ = cancellation_token.cancelled() => {
                    let _ = self.update_chunk_and_download_status(
                        chunk.chunk_index,
                        chunk.start_byte,
                        downloaded,
                        "paused",
                    ).await?;
                    return Ok(DownloadStatus::Paused);
                }

                maybe_bytes = stream_fuse.next() => {
                    match maybe_bytes {
                        Some(Ok(bytes)) => {
                            let chunk_len = bytes.len() as u64;

                            self.file_writer_tx
                                .send((start_byte as u64 + downloaded, bytes.to_vec()))
                                .await
                                .map_err(|e| e.to_string())?;

                            downloaded += chunk_len;

                            let mut report = report.lock().await;
                            report.downloaded_bytes += chunk_len;
                            report.received_bytes += chunk_len as f64;

                        },
                        Some(Err(e)) => {
                            let _ = self.update_chunk_and_download_status(
                                chunk.chunk_index,
                                chunk.start_byte,
                                downloaded,
                                "failed",
                            ).await?;
                            return Err(e.to_string());
                        },
                        None => {
                            let _ = self.update_chunk_and_download_status(
                                chunk.chunk_index,
                                chunk.start_byte,
                                downloaded,
                                "completed",
                            ).await?;
                            return Ok(DownloadStatus::Finished);
                        }
                    }
                }

            }
        }
    }

    pub async fn update_chunk_and_download_status(
        &self,
        chunk_index: i64,
        start_byte: i64,
        downloaded_bytes: u64,
        status: &str,
    ) -> Result<(), String> {
        let hash = compute_partial_hash(
            &self.download.file_path,
            start_byte as u64,
            downloaded_bytes,
        )
        .await?;
        let _ = update_chunk_downloaded(
            &self.pool,
            self.download_id,
            chunk_index,
            downloaded_bytes as i64,
            hash,
        )
        .await?;
        let _ = update_download_status(&self.pool, self.download_id, status).await?;

        Ok(())
    }

    pub fn pause_download(&self) {
        self.cancellation_token.cancel();
    }

    fn listen_to_report(&self) {
        let download_id = self.download.id.clone();
        let total_bytes = self.download.total_bytes.clone();

        let cancellation_token = self.cancellation_token.clone();
        let app_handle = self.app_handle.clone();
        let report = Arc::clone(&self.report);

        spawn(async move {
            loop {
                if cancellation_token.is_cancelled() {
                    break;
                }

                sleep(Duration::from_millis(100)).await;

                let report = report.lock().await;

                println!("{}", report.downloaded_bytes);

                let event = format!("downloaded_bytes_{}", download_id);

                emit_app_event(&app_handle, &event, report.downloaded_bytes);
            }
        });

        let cancellation_token = self.cancellation_token.clone();
        let app_handle = self.app_handle.clone();
        let report = Arc::clone(&self.report);

        spawn(async move {
            loop {
                if cancellation_token.is_cancelled() {
                    break;
                }

                sleep(Duration::from_secs(1)).await;

                let mut report = report.lock().await;

                let speed = report.received_bytes as f64 / 1024.0;

                let remaining_time = total_bytes.saturating_sub(report.downloaded_bytes as i64)
                    as f64
                    / (speed * 1024.0);

                let event = format!("speed_and_remaining_{}", download_id);

                println!("Speed: {}, Remaining time: {}", speed, remaining_time);

                emit_app_event(
                    &app_handle,
                    &event,
                    SpeedAndRemaining {
                        speed,
                        remaining_time,
                    },
                );

                report.received_bytes = 0.0;
            }
        });
    }

    fn classify_results(results: Vec<Result<DownloadStatus, String>>) -> WorkerOutcome {
        let mut has_finished = false;
        let mut has_paused = false;
        let mut has_error = false;

        for result in results {
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
