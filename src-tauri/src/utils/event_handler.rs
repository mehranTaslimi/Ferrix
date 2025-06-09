use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use serde::Serialize;
use sqlx::SqlitePool;
use tauri::{AppHandle, Emitter};
use tokio::{spawn, sync::broadcast::Sender};

use crate::{
    db::downloads::{
        get_download_chunks, get_downloads_list, insert_download_chunks, insert_new_download,
        update_chunk_downloaded, update_download_status,
    },
    downloader::{download_chunks, get_chunk_ranges, get_file_content_length},
    models::{Download, DownloadChunk, DownloadWithDownloadedBytes},
    utils::app_state::AppEvent,
};

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
            )
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
            dispatch(
                &tx,
                AppEvent::SendDownloadItemUpdate(download_id, chunk_index, downloaded),
            )
        }
        AppEvent::SendDownloadList => {
            let list: HashMap<i64, DownloadWithDownloadedBytes> = get_downloads_list(&pool)
                .await?
                .into_iter()
                .map(|f| (f.id, f))
                .collect();

            dispatch_client_event(&app_handle, "download_list", &list)
        }
        AppEvent::SendDownloadItemUpdate(download_id, chunk_index, downloaded_bytes) => {
            dispatch_client_event(
                &app_handle,
                "process",
                DownloadChunk {
                    chunk_index,
                    download_id: download_id.to_string(),
                    downloaded_bytes: downloaded_bytes as i64,
                    end: 0,
                    start: 0,
                    url: "".to_string(),
                },
            )
        }
    }
}
