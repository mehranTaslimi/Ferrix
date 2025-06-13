use futures_util::future::try_join_all;
use once_cell::sync::Lazy;
use serde::Serialize;
use sqlx::SqlitePool;
use std::{collections::HashMap, time::Duration};
use tauri::{AppHandle, Emitter};
use tokio::{
    spawn,
    sync::{broadcast::Sender, Mutex},
    time::sleep,
};

use crate::{
    db::downloads::{
        get_download_chunks_by_download_id, get_downloads_by_id, get_downloads_list,
        insert_download_chunks, insert_new_download, update_chunk_downloaded,
        update_download_status,
    },
    downloader::{
        compute_partial_hash, download_chunks, get_chunk_ranges, validate_and_inspect_url,
    },
    models::DownloadId,
    utils::app_state::AppEvent,
};

pub struct EventHandler {
    tx: Sender<AppEvent>,
    pool: SqlitePool,
    app_handle: AppHandle,
}

#[derive(Clone, Debug)]
struct Report {
    downloaded_bytes: u64,
    received_bytes: f64,
}

static REPORTER: Lazy<Mutex<HashMap<DownloadId, Report>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub fn dispatch(tx: &Sender<AppEvent>, app_event: AppEvent) {
    let _ = tx.send(app_event).map(|_| ());
}

pub fn emit_app_event<S: Serialize + Clone>(app_handle: &AppHandle, event: &str, payload: S) {
    let _ = app_handle.emit(event, payload);
}

fn downloading_meta_reporter(app_handle: &AppHandle) {
    let app_handle_1 = app_handle.clone();
    let app_handle_2 = app_handle.clone();
    spawn(async move {
        loop {
            sleep(Duration::from_millis(100)).await;

            let reporter = REPORTER.lock().await;
            let clone = reporter.clone();

            if clone.len() == 0 {
                continue;
            }

            let mut report_map: HashMap<DownloadId, u64> = HashMap::new();

            for (&id, report) in clone.iter() {
                report_map.insert(id, report.downloaded_bytes);
            }

            emit_app_event(&app_handle_1, "downloaded_bytes", report_map);
        }
    });
    spawn(async move {
        loop {
            sleep(Duration::from_secs(1)).await;

            let mut reporter = REPORTER.lock().await;

            if reporter.len() == 0 {
                continue;
            }

            println!("reporter: {:?}", reporter);

            let mut report_map: HashMap<DownloadId, f64> = HashMap::new();

            for (&id, report) in reporter.iter_mut() {
                let speed_kbps = report.received_bytes as f64 / 1024.0;
                report_map.insert(id, speed_kbps);
                report.received_bytes = 0.0;
            }

            emit_app_event(&app_handle_2, "download_speed", report_map);
        }
    });
}

async fn flush_report(download_id: DownloadId) {
    let mut reporter = REPORTER.lock().await;
    reporter.remove(&download_id);
}

impl EventHandler {
    pub fn new(tx: Sender<AppEvent>, pool: SqlitePool, app_handle: AppHandle) -> Self {
        downloading_meta_reporter(&app_handle);
        Self {
            app_handle,
            pool,
            tx,
        }
    }

    pub async fn event_reducer(&self, app_event: AppEvent) -> Result<(), String> {
        match app_event {
            AppEvent::StartNewDownloadProcess(download_url, chunk_count) => {
                dispatch(
                    &self.tx,
                    AppEvent::ValidateAndInspectLink(download_url, chunk_count),
                );
                Ok(())
            }

            AppEvent::ValidateAndInspectLink(download_url, chunk_count) => {
                let file_info = validate_and_inspect_url(&download_url).await?;
                dispatch(
                    &self.tx,
                    AppEvent::CreateNewDownloadRecordInDB(file_info, chunk_count),
                );
                Ok(())
            }

            AppEvent::CreateNewDownloadRecordInDB(file_info, chunk_count) => {
                let total_bytes = file_info.total_bytes;
                let file_name = file_info.file_name.clone();
                let file_path = format!("/Users/mehrantaslimi/Downloads/{}", file_name);
                let id =
                    insert_new_download(&self.pool, file_info, chunk_count, &file_path).await?;

                dispatch(
                    &self.tx,
                    AppEvent::CreateDownloadChunkInDB(id, total_bytes, chunk_count),
                );
                Ok(())
            }

            AppEvent::CreateDownloadChunkInDB(download_id, total_bytes, chunk_count) => {
                let ranges = get_chunk_ranges(total_bytes as u64, chunk_count as u8)?;
                insert_download_chunks(&self.pool, download_id, ranges).await?;
                dispatch(
                    &self.tx,
                    AppEvent::InsertDownloadFromDBToDownloadingList(download_id),
                );
                Ok(())
            }

            AppEvent::InsertDownloadFromDBToDownloadingList(id) => {
                let download = get_downloads_by_id(&self.pool, id).await?;
                update_download_status(&self.pool, id, "downloading").await?;
                dispatch(&self.tx, AppEvent::StartDownload(id, download));
                Ok(())
            }

            AppEvent::UpdateChunk(download_id, chunk_index, downloaded_bytes, expected_hash) => {
                update_chunk_downloaded(
                    &self.pool,
                    download_id,
                    chunk_index,
                    downloaded_bytes,
                    expected_hash,
                )
                .await
            }

            AppEvent::SendDownloadList => {
                let results = get_downloads_list(&self.pool).await?;
                emit_app_event(&self.app_handle, "download_list", results);
                Ok(())
            }

            AppEvent::DownloadFinished(download_id) => {
                update_download_status(&self.pool, download_id, "completed").await?;
                flush_report(download_id).await;

                dispatch(&self.tx, AppEvent::SendDownloadList);

                Ok(())
            }

            AppEvent::StartDownload(id, download) => {
                let chunks = get_download_chunks_by_download_id(&self.pool, id).await?;
                let tx = self.tx.clone();

                spawn(async move { download_chunks(tx, chunks, download).await });

                dispatch(&self.tx, AppEvent::SendDownloadList);

                Ok(())
            }

            AppEvent::MakeChunkHash(downloaded_bytes, chunk) => {
                let hash = compute_partial_hash(
                    &chunk.file_path,
                    chunk.start_byte as u64,
                    downloaded_bytes,
                )
                .await?;

                let download_id = chunk
                    .download_id
                    .parse::<i64>()
                    .map_err(|e| e.to_string())?;

                dispatch(
                    &self.tx,
                    AppEvent::UpdateChunk(
                        download_id,
                        chunk.chunk_index,
                        downloaded_bytes as i64,
                        hash,
                    ),
                );
                Ok(())
            }

            AppEvent::PauseDownload(download_id) => {
                update_download_status(&self.pool, download_id, "paused").await?;
                dispatch(&self.tx, AppEvent::SendDownloadList);
                Ok(())
            }

            AppEvent::ResumeDownload(download_id) => {
                let chunks = get_download_chunks_by_download_id(&self.pool, download_id).await?;
                dispatch(
                    &self.tx,
                    AppEvent::ValidateExistingFile(download_id, chunks),
                );
                Ok(())
            }

            AppEvent::ValidateExistingFile(download_id, chunks) => {
                let results: Result<Vec<bool>, String> =
                    try_join_all(chunks.iter().map(|f| async move {
                        let hash = compute_partial_hash(
                            &f.file_path,
                            f.start_byte as u64,
                            f.downloaded_bytes as u64,
                        )
                        .await
                        .map_err(|e| e.to_string())?;

                        Ok(f.expected_hash == Some(hash))
                    }))
                    .await;

                let is_all_match = match results {
                    Ok(vec) => vec.into_iter().all(|f| f),
                    Err(_) => false,
                };

                if is_all_match {
                    dispatch(
                        &self.tx,
                        AppEvent::InsertDownloadFromDBToDownloadingList(download_id),
                    );
                    Ok(())
                } else {
                    Err("file not match".to_string())
                }
            }

            AppEvent::ReportChunkReceivedBytes(download_id, received_bytes) => {
                let mut reporter = REPORTER.lock().await;
                reporter
                    .entry(download_id)
                    .and_modify(|report| {
                        report.downloaded_bytes += received_bytes;
                        report.received_bytes += received_bytes as f64;
                    })
                    .or_insert(Report {
                        received_bytes: received_bytes as f64,
                        downloaded_bytes: received_bytes,
                    });
                Ok(())
            }
        }
    }
}
