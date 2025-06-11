use once_cell::sync::Lazy;
use serde::Serialize;
use sqlx::SqlitePool;
use std::collections::HashMap;
use tauri::{AppHandle, Emitter};
use tokio::{
    spawn,
    sync::{broadcast::Sender, Mutex},
};

use crate::{
    db::downloads::{
        get_download_chunks_by_download_id, get_downloads_by_id, get_downloads_list,
        insert_download_chunks, insert_new_download, update_chunk_downloaded,
        update_download_status,
    },
    downloader::{download_chunks, get_chunk_ranges, validate_and_inspect_url},
    models::{ChunkIndex, Download, DownloadId, DownloadedBytes, SpeedKbps},
    utils::app_state::AppEvent,
};

pub fn dispatch(tx: &Sender<AppEvent>, app_event: AppEvent) -> Result<(), String> {
    tx.send(app_event).map(|_| ()).map_err(|e| e.to_string())
}

pub fn emit_app_event<S: Serialize + Clone>(
    app_handle: &AppHandle,
    event: &str,
    payload: S,
) -> Result<(), String> {
    app_handle.emit(event, payload).map_err(|e| e.to_string())
}

#[derive(Debug, Clone)]
struct DownloadMeta {
    chunk_speed: HashMap<ChunkIndex, SpeedKbps>,
    chunk_downloaded_bytes: HashMap<ChunkIndex, DownloadedBytes>,
}

#[derive(Debug, Clone)]
struct Downloading {
    download: Download,
    meta: DownloadMeta,
}

static DOWNLOADING_LIST: Lazy<Mutex<HashMap<DownloadId, Downloading>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub struct EventHandler {
    pub tx: Sender<AppEvent>,
    pub pool: SqlitePool,
    pub app_handle: AppHandle,
}

impl EventHandler {
    async fn download_reporter(
        &self,
        download_id: DownloadId,
        chunk_index: ChunkIndex,
        downloaded_bytes: DownloadedBytes,
    ) -> Result<(), String> {
        let mut downloading_list = DOWNLOADING_LIST.lock().await;

        let download = downloading_list
            .get_mut(&download_id)
            .ok_or("cannot defined downloading list to report".to_string())?;

        download
            .meta
            .chunk_downloaded_bytes
            .insert(chunk_index, downloaded_bytes);

        let is_all_chunk_reported =
            download.meta.chunk_downloaded_bytes.len() as i64 == download.download.chunk_count;

        if is_all_chunk_reported {
            let downloaded_bytes = download.meta.chunk_downloaded_bytes.values().sum::<i64>();
            download.meta.chunk_downloaded_bytes.clear();
            dispatch(
                &self.tx,
                AppEvent::FullReportChunksDownloadedBytes(download_id, downloaded_bytes),
            )?;
        }

        Ok(())
    }

    async fn speed_reporter(
        &self,
        download_id: DownloadId,
        chunk_index: ChunkIndex,
        speed_kbps: SpeedKbps,
    ) -> Result<(), String> {
        let mut downloading_list = DOWNLOADING_LIST.lock().await;

        let download = downloading_list
            .get_mut(&download_id)
            .ok_or("cannot defined downloading list to report".to_string())?;

        download.meta.chunk_speed.insert(chunk_index, speed_kbps);

        let is_all_chunk_reported =
            download.meta.chunk_speed.len() as i64 == download.download.chunk_count;

        if is_all_chunk_reported {
            let chunk_speeds = download.meta.chunk_speed.values().sum::<f64>();
            download.meta.chunk_speed.clear();
            dispatch(
                &self.tx,
                AppEvent::FullReportChunksSpeed(download_id, chunk_speeds),
            )?;
        }

        Ok(())
    }

    async fn flush_report(&self, download_id: DownloadId) -> Result<(), String> {
        let mut downloading_list = DOWNLOADING_LIST.lock().await;
        downloading_list.remove(&download_id);

        Ok(())
    }

    pub async fn event_reducer(&self, app_event: AppEvent) -> Result<(), String> {
        match app_event {
            AppEvent::StartNewDownloadProcess(download_url, chunk_count) => {
                println!("Start new download process");
                dispatch(
                    &self.tx,
                    AppEvent::ValidateAndInspectLink(download_url, chunk_count),
                )
            }
            AppEvent::ValidateAndInspectLink(download_url, chunk_count) => {
                let file_info = validate_and_inspect_url(&download_url).await?;
                dispatch(
                    &self.tx,
                    AppEvent::CreateNewDownloadRecordInDB(file_info, chunk_count),
                )
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
                )
            }
            AppEvent::CreateDownloadChunkInDB(download_id, total_bytes, chunk_count) => {
                let ranges = get_chunk_ranges(total_bytes as u64, chunk_count as u8)?;
                insert_download_chunks(&self.pool, download_id, ranges).await?;
                dispatch(
                    &self.tx,
                    AppEvent::InsertDownloadFromDBToDownloadingList(download_id),
                )
            }
            AppEvent::InsertDownloadFromDBToDownloadingList(id) => {
                let mut downloading_list = DOWNLOADING_LIST.lock().await;
                let download = get_downloads_by_id(&self.pool, id).await?;
                update_download_status(&self.pool, id, "downloading").await?;
                downloading_list.insert(
                    download.id,
                    Downloading {
                        download: download.clone(),
                        meta: DownloadMeta {
                            chunk_downloaded_bytes: HashMap::new(),
                            chunk_speed: HashMap::new(),
                        },
                    },
                );
                dispatch(&self.tx, AppEvent::StartDownload(id, download))
            }
            AppEvent::StartDownload(id, download) => {
                let chunks = get_download_chunks_by_download_id(&self.pool, id).await?;
                let tx = self.tx.clone();

                spawn(async move { download_chunks(tx, chunks, download).await });

                dispatch(&self.tx, AppEvent::SendDownloadList)
            }

            AppEvent::ReportChunkSpeed(download_id, chunk_index, speed_kbps) => {
                self.speed_reporter(download_id, chunk_index, speed_kbps)
                    .await
            }
            AppEvent::ReportChunkDownloadedBytes(download_id, chunk_index, downloaded_bytes) => {
                dispatch(
                    &self.tx,
                    AppEvent::UpdateChunkDownloadedBytes(
                        download_id,
                        chunk_index,
                        downloaded_bytes,
                    ),
                )?;
                self.download_reporter(download_id, chunk_index, downloaded_bytes)
                    .await
            }

            AppEvent::FullReportChunksSpeed(download_id, speed_kbps) => {
                let mut payload: HashMap<DownloadId, SpeedKbps> = HashMap::new();
                payload.insert(download_id, speed_kbps);
                emit_app_event(&self.app_handle, "download_speed", payload)
            }
            AppEvent::FullReportChunksDownloadedBytes(download_id, downloaded_bytes) => {
                let mut payload: HashMap<DownloadId, DownloadedBytes> = HashMap::new();
                payload.insert(download_id, downloaded_bytes);
                emit_app_event(&self.app_handle, "downloaded_bytes", payload)
            }

            AppEvent::UpdateChunkDownloadedBytes(download_id, chunk_index, downloaded_bytes) => {
                update_chunk_downloaded(&self.pool, download_id, chunk_index, downloaded_bytes)
                    .await
            }

            AppEvent::SendDownloadList => {
                let results = get_downloads_list(&self.pool).await?;
                emit_app_event(&self.app_handle, "download_list", results)
            }

            AppEvent::DownloadFinished(download_id) => {
                update_download_status(&self.pool, download_id, "completed").await?;
                self.flush_report(download_id).await?;

                dispatch(&self.tx, AppEvent::SendDownloadList)?;

                Ok(())
            }
        }
    }
}
