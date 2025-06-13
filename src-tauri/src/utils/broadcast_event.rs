use futures_util::future::try_join_all;
use sqlx::SqlitePool;
use tauri::AppHandle;
use tokio::{spawn, sync::broadcast::Sender};

use crate::{
    db::downloads::{
        get_download_chunks_by_download_id, get_downloads_by_id, get_downloads_list,
        insert_download_chunks, insert_new_download, update_chunk_downloaded,
        update_download_status,
    },
    downloader::{
        compute_partial_hash, download_chunks, get_chunk_ranges, validate_and_inspect_url,
    },
    events::{dispatch, emit_app_event, Reporter},
    utils::app_state::AppEvent,
};

pub struct EventHandler {
    tx: Sender<AppEvent>,
    pool: SqlitePool,
    app_handle: AppHandle,
    reporter: Reporter,
}

impl EventHandler {
    pub fn new(tx: Sender<AppEvent>, pool: SqlitePool, app_handle: AppHandle) -> Self {
        let reporter = Reporter::new(app_handle.clone());
        reporter.listen_to_report();

        Self {
            app_handle,
            pool,
            tx,
            reporter,
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
                self.reporter.flush_report(download_id).await;
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

            // AppEvent::PauseDownload(download_id) => {
            //     update_download_status(&self.pool, download_id, "paused").await?;
            //     dispatch(&self.tx, AppEvent::SendDownloadList);
            //     Ok(())
            // }
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
                self.reporter.add_report(download_id, received_bytes).await;
                Ok(())
            }

            _ => Ok(()),
        }
    }
}
