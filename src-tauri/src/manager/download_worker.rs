use crate::{
    db::downloads::{
        get_download_chunks_by_download_id, get_downloads_by_id, insert_download_chunks,
        insert_new_download, reset_downloaded_chunks, update_chunk_downloaded,
        update_download_status,
    },
    events::{dispatch, emit_app_event},
    manager::{
        compute_partial_hash::compute_partial_hash,
        file_writer::{file_writer, WriteMessage},
        get_chunk_ranges::get_chunk_ranges,
        validation::invalid_chunks_hash,
    },
    models::{Chunk, ChunkCount, Download, DownloadId, FileInfo},
    utils::app_state::AppEvent,
};
use futures_util::{future::join_all, StreamExt};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::{collections::HashMap, sync::Arc, time::Duration};
use tauri::{http::header::RANGE, AppHandle};
use tauri_plugin_http::reqwest::Client;
use tokio::{
    spawn,
    sync::{broadcast::Sender, mpsc, Mutex},
    time::sleep,
};
use tokio_util::sync::CancellationToken;

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
struct InternetReport {
    downloaded_bytes: u64,
    received_bytes: f64,
}

#[derive(Debug, Clone)]
pub struct DiskReport {
    pub total: u64,
    pub chunks: HashMap<u64, u64>,
}

#[derive(Clone, Debug)]
pub struct DownloadWorker {
    pool: SqlitePool,
    app_handle: AppHandle,
    chunks: Vec<Chunk>,
    download: Download,
    file_writer: mpsc::UnboundedSender<WriteMessage>,
    internet_report: Arc<Mutex<InternetReport>>,
    disk_report: Arc<Mutex<DiskReport>>,
    app_event: Sender<AppEvent>,
    pub cancellation_token: CancellationToken,
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

        let internet_report = Arc::new(Mutex::new(InternetReport {
            downloaded_bytes,
            received_bytes: 0.0,
        }));

        let disk_report = Arc::new(Mutex::new(DiskReport {
            total: 0,
            chunks: chunks
                .clone()
                .into_iter()
                .map(|f| (f.chunk_index as u64, f.downloaded_bytes as u64))
                .collect(),
        }));

        let file_writer = file_writer(
            &download.file_path,
            download.total_bytes as u64,
            Arc::clone(&disk_report),
        )
        .await?;

        let cancellation_token = CancellationToken::new();

        Ok(Self {
            pool,
            download,
            chunks,
            download_id,
            app_event,
            file_writer,
            app_handle,
            cancellation_token,
            internet_report,
            disk_report,
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

        let invalid_chunks_index = invalid_chunks_hash(&download.file_path, chunks.clone());

        reset_downloaded_chunks(&pool, download_id, invalid_chunks_index).await?;

        Ok((download, chunks))
    }

    pub async fn start_download(&self) {
        let max_retries = 5;
        let mut retries = 0;

        self.listen_to_report_internet();
        self.listen_to_report_disk();

        loop {
            let futures = self
                .chunks
                .clone()
                .into_iter()
                .filter(|chunk| chunk.downloaded_bytes < chunk.end_byte - chunk.start_byte)
                .map(|chunk| async move { self.download_chunk(chunk).await });

            let _ = self.emit_and_update_download_status("downloading").await;
            let results = join_all(futures).await;

            let outcome = Self::classify_results(results);

            match outcome {
                WorkerOutcome::Finished => {
                    dispatch(
                        &self.app_event,
                        AppEvent::DownloadFinished(self.download_id),
                    );
                    let _ = self.emit_and_update_download_status("completed").await;
                    self.cancellation_token.cancel();
                    break;
                }
                WorkerOutcome::Paused => {
                    let _ = self.emit_and_update_download_status("paused").await;
                    self.cancellation_token.cancel();
                    break;
                }
                WorkerOutcome::Errored | WorkerOutcome::Mixed => {
                    if retries < max_retries {
                        retries += 1;
                        continue;
                    } else {
                        let _ = self.emit_and_update_download_status("failed").await;
                        self.cancellation_token.cancel();
                        break;
                    }
                }
            };
        }
    }

    async fn download_chunk(&self, chunk: Chunk) -> Result<DownloadStatus, String> {
        let downloaded_bytes = chunk.downloaded_bytes;
        let start_byte = chunk.start_byte;
        let end_byte = chunk.end_byte;
        let chunk_index = chunk.chunk_index;
        let url = self.download.url.clone();
        let range_header = format!("bytes={}-{}", start_byte + downloaded_bytes, end_byte);

        let client = Client::new();

        let response = client
            .get(url)
            .header(RANGE, range_header)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let mut stream = response.bytes_stream();
        let mut downloaded = downloaded_bytes as u64;
        let report = Arc::clone(&self.internet_report);
        let cancellation_token = self.cancellation_token.clone();

        loop {
            tokio::select! {
                biased;

                _ = cancellation_token.cancelled() => {
                    self.update_chunk_hash(chunk_index, start_byte).await?;
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
                                .map_err(|e| e.to_string())?;

                            downloaded += bytes_len;

                            let mut report = report.lock().await;
                            report.downloaded_bytes += bytes_len;
                            report.received_bytes += bytes_len as f64;

                        },
                        Some(Err(err)) => {
                            self.update_chunk_hash(chunk_index, start_byte).await?;
                            return Err(err.to_string());
                        },
                        None => {
                            self.update_chunk_hash(chunk_index, start_byte).await?;
                            return Ok(DownloadStatus::Finished);
                        }
                    }
                }

            }
        }
    }

    pub async fn update_chunk_hash(&self, chunk_index: i64, start_byte: i64) -> Result<(), String> {
        let downloaded_bytes = self
            .disk_report
            .lock()
            .await
            .chunks
            .get(&(chunk_index as u64))
            .cloned()
            .unwrap_or(0);

        let hash = compute_partial_hash(
            &self.download.file_path,
            start_byte as u64,
            downloaded_bytes,
        )?;
        update_chunk_downloaded(
            &self.pool,
            self.download_id,
            chunk_index,
            downloaded_bytes as i64,
            hash,
        )
        .await?;
        Ok(())
    }

    async fn emit_and_update_download_status(&self, status: &str) -> Result<(), String> {
        update_download_status(&self.pool, self.download_id, status).await?;
        let result = get_downloads_by_id(&self.pool, self.download_id).await?;
        emit_app_event(&self.app_handle, "download_item", result);

        Ok(())
    }

    fn listen_to_report_internet(&self) {
        let download_id = self.download.id.clone();
        let total_bytes = self.download.total_bytes.clone();

        let cancellation_token = self.cancellation_token.clone();
        let app_handle = self.app_handle.clone();
        let report = Arc::clone(&self.internet_report);

        spawn(async move {
            loop {
                if cancellation_token.is_cancelled() {
                    break;
                }

                sleep(Duration::from_millis(100)).await;

                let report = report.lock().await;

                let event = format!("downloaded_bytes_{}", download_id);

                emit_app_event(&app_handle, &event, report.downloaded_bytes);
            }
        });

        let cancellation_token = self.cancellation_token.clone();
        let app_handle = self.app_handle.clone();
        let report = Arc::clone(&self.internet_report);

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

    fn listen_to_report_disk(&self) {
        let cancellation_token = self.cancellation_token.clone();
        let disk_report = self.disk_report.clone();
        let app_handle = self.app_handle.clone();
        let download_id = self.download.id.clone();
        spawn(async move {
            loop {
                if cancellation_token.is_cancelled() {
                    break;
                }

                sleep(Duration::from_secs(1)).await;

                let mut report = disk_report.lock().await;

                let speed = report.total as f64 / 1024.0;

                let event = format!("disk_speed_{}", download_id);

                emit_app_event(&app_handle, &event, speed);

                report.total = 0;
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
