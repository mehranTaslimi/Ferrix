use once_cell::sync::Lazy;
use serde::Serialize;
use sqlx::SqlitePool;
use std::{collections::HashMap, sync::Mutex};
use tauri::{AppHandle, Emitter};
use tokio::{spawn, sync::broadcast::Sender};

use crate::{
    db::downloads::{
        get_download_chunks, get_downloads_list, insert_download_chunks, insert_new_download,
        update_chunk_downloaded, update_download_status,
    },
    downloader::{download_chunks, get_chunk_ranges, get_file_content_length},
    models::{Download, DownloadReport, DownloadSpeed},
    utils::app_state::AppEvent,
};

type ChunkCount = i64;
type DownloadId = i64;
type ChunkIndex = i64;
type DownloadedBytes = i64;
type SpeedKbps = f64;
type DownloadTracker = HashMap<DownloadId, HashMap<ChunkIndex, DownloadedBytes>>;
type SpeedTracker = HashMap<DownloadId, HashMap<ChunkIndex, SpeedKbps>>;

static DOWNLOAD_TRACKER: Lazy<Mutex<DownloadTracker>> = Lazy::new(|| Mutex::new(HashMap::new()));
static SPEED_TRACKER: Lazy<Mutex<SpeedTracker>> = Lazy::new(|| Mutex::new(HashMap::new()));

fn download_tracker_report(
    chunk_count: ChunkCount,
    download_id: DownloadId,
    chunk_index: ChunkIndex,
    downloaded_bytes: DownloadedBytes,
) -> Option<Vec<i64>> {
    let mut tracker = DOWNLOAD_TRACKER.lock().unwrap();
    let downloaded_bytes_chunks = tracker.entry(download_id).or_insert_with(HashMap::new);

    downloaded_bytes_chunks.insert(chunk_index, downloaded_bytes);

    let is_all_chunk_reported = downloaded_bytes_chunks.len() as i64 == chunk_count;

    let aggregate_downloaded_bytes = downloaded_bytes_chunks
        .values()
        .cloned()
        .collect::<Vec<i64>>();

    if is_all_chunk_reported {
        tracker.remove(&download_id);
        Some(aggregate_downloaded_bytes)
    } else {
        None
    }
}

fn speed_tracker_report(
    chunk_count: ChunkCount,
    download_id: DownloadId,
    chunk_index: ChunkIndex,
    speed_kbps: SpeedKbps,
) -> Option<f64> {
    let mut tracker = SPEED_TRACKER.lock().unwrap();
    let download_speed_chunks = tracker.entry(download_id).or_insert_with(HashMap::new);

    download_speed_chunks.insert(chunk_index, speed_kbps);

    let is_all_chunk_reported = download_speed_chunks.len() as i64 == chunk_count;

    let aggregate_speed: f64 = download_speed_chunks.values().sum();

    if is_all_chunk_reported {
        tracker.remove(&download_id);
        Some(aggregate_speed)
    } else {
        None
    }
}

pub fn dispatch(tx: &Sender<AppEvent>, app_event: AppEvent) -> Result<(), String> {
    tx.send(app_event).map(|_| ()).map_err(|e| e.to_string())
}

pub fn dispatch_client_event<S: Serialize + Clone>(
    app_handle: &AppHandle,
    event: &str,
    payload: S,
) -> Result<(), String> {
    app_handle.emit(event, payload).map_err(|e| e.to_string())
}

pub async fn handle(
    app_event: AppEvent,
    tx: &Sender<AppEvent>,
    pool: &SqlitePool,
    app_handle: &AppHandle,
) -> Result<(), String> {
    match app_event {
        AppEvent::StartNewDownloadProcess(download_data) => {
            dispatch(&tx, AppEvent::ValidateUrl(download_data))
        }
        AppEvent::ValidateUrl(download_data) => {
            dispatch(&tx, AppEvent::GetFileContentLength(download_data))
        }
        AppEvent::GetFileContentLength(download_data) => {
            let content_length = get_file_content_length(&download_data.url).await?;
            dispatch(
                &tx,
                AppEvent::CreateNewDownloadRecord(download_data, content_length),
            )
        }
        AppEvent::CreateNewDownloadRecord(download_data, content_length) => {
            let id = insert_new_download(&pool, &download_data.url, content_length as i64).await?;
            dispatch(
                &tx,
                AppEvent::CreateDownloadChunk(id, content_length, download_data.chunk),
            )?;
            dispatch(&tx, AppEvent::SendDownloadList)
        }
        AppEvent::CreateDownloadChunk(id, content_length, chunk) => {
            let ranges = get_chunk_ranges(content_length, chunk)?;
            insert_download_chunks(&pool, id, ranges).await?;
            dispatch(&tx, AppEvent::StartDownload(id))
        }
        AppEvent::StartDownload(id) => {
            update_download_status(&pool, id, "downloading").await?;
            let chunks = get_download_chunks(&pool, id).await?;
            let tx = tx.clone();
            spawn(async move { download_chunks(&tx, chunks).await });
            Ok(())
        }
        AppEvent::UpdateDownloadedChunk(download_id, chunk_index, downloaded) => {
            update_chunk_downloaded(&pool, download_id, chunk_index, downloaded as i64).await?;
            let downloaded_bytes_chunks =
                download_tracker_report(5, download_id, chunk_index, downloaded as i64);
            if let Some(downloaded_bytes_chunks) = downloaded_bytes_chunks {
                dispatch(
                    &tx,
                    AppEvent::SendDownloadItemUpdate(download_id, downloaded_bytes_chunks),
                )?;
            }

            Ok(())
        }
        AppEvent::SendDownloadItemUpdate(download_id, downloaded_bytes_chunks) => {
            dispatch_client_event(
                &app_handle,
                "downloads_tracker",
                DownloadReport {
                    id: download_id,
                    downloaded_bytes_chunks,
                },
            )
        }
        AppEvent::SendDownloadList => {
            let list: HashMap<i64, Download> = get_downloads_list(&pool)
                .await?
                .into_iter()
                .map(|f| (f.id, f))
                .collect();

            dispatch_client_event(&app_handle, "download_list", &list)
        }
        AppEvent::SendDownloadSpeed(download_id, chunk_index, speed_kbps) => {
            let speed = speed_tracker_report(5, download_id, chunk_index, speed_kbps);
            if let Some(speed) = speed {
                dispatch_client_event(
                    &app_handle,
                    "download_speed_tracker",
                    DownloadSpeed {
                        id: download_id,
                        speed_kbps: speed,
                    },
                )?;
            }
            Ok(())
        }
    }
}
