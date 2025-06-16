use std::{collections::HashMap, sync::Arc};

use once_cell::sync::Lazy;
use sqlx::SqlitePool;
use tauri::AppHandle;
use tokio::sync::{broadcast::Sender, Mutex};

use crate::{
    db::downloads::get_downloads_list,
    events::emit_app_event,
    manager::download_worker::DownloadWorker,
    models::{Download, DownloadId},
    utils::app_state::AppEvent,
};

static WORKERS: Lazy<Mutex<HashMap<DownloadId, Arc<Mutex<DownloadWorker>>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub struct DownloadsManager {
    pool: SqlitePool,
    app_handle: AppHandle,
    app_event: Sender<AppEvent>,
}

impl DownloadsManager {
    pub fn new(app_event: Sender<AppEvent>, pool: SqlitePool, app_handle: AppHandle) -> Self {
        Self {
            app_handle,
            pool,
            app_event,
        }
    }

    pub async fn manage(&self, app_event: AppEvent) -> Result<(), String> {
        match app_event {
            AppEvent::StartNewDownloadProcess(file_info, chunk_count) => {
                let worker = Arc::new(Mutex::new(
                    DownloadWorker::new(
                        self.pool.clone(),
                        self.app_handle.clone(),
                        self.app_event.clone(),
                        None,
                        Some(file_info),
                        Some(chunk_count),
                    )
                    .await?,
                ));

                WORKERS
                    .lock()
                    .await
                    .insert(worker.lock().await.download_id, Arc::clone(&worker));

                tokio::spawn(async move {
                    worker.lock().await.start_download().await;
                });

                Ok(())
            }

            AppEvent::PauseDownload(download_id) => {
                if let Some(worker_arc) = WORKERS.lock().await.get(&download_id).cloned() {
                    let worker = worker_arc.lock().await;
                    worker.pause_download();
                }

                WORKERS.lock().await.remove(&download_id);

                Ok(())
            }

            AppEvent::ResumeDownload(download_id) => {
                let worker = Arc::new(Mutex::new(
                    DownloadWorker::new(
                        self.pool.clone(),
                        self.app_handle.clone(),
                        self.app_event.clone(),
                        Some(download_id),
                        None,
                        None,
                    )
                    .await?,
                ));

                WORKERS
                    .lock()
                    .await
                    .insert(worker.lock().await.download_id, Arc::clone(&worker));

                tokio::spawn(async move {
                    worker.lock().await.start_download().await;
                });

                Ok(())
            }

            AppEvent::SendDownloadList => {
                let results: HashMap<i64, Download> = get_downloads_list(&self.pool)
                    .await?
                    .into_iter()
                    .map(|f| (f.id, f))
                    .collect();
                emit_app_event(&self.app_handle, "download_list", results);
                Ok(())
            }

            AppEvent::WorkerFinished(download_id) => {
                WORKERS.lock().await.remove(&download_id);
                Ok(())
            }

            _ => Ok(()),
        }
    }
}

// AppEvent::CreateNewDownloadRecordInDB(file_info, chunk_count) => {
//     let total_bytes = file_info.total_bytes;
//     let id = insert_new_download(&self.pool, file_info, chunk_count).await?;

//     dispatch(
//         &self.tx,
//         AppEvent::CreateDownloadChunkInDB(id, total_bytes, chunk_count),
//     );
//     Ok(())
// }

// AppEvent::CreateDownloadChunkInDB(download_id, total_bytes, chunk_count) => {
//     let ranges = get_chunk_ranges(total_bytes as u64, chunk_count as u8)?;
//     insert_download_chunks(&self.pool, download_id, ranges).await?;
//     dispatch(
//         &self.tx,
//         AppEvent::InsertDownloadFromDBToDownloadingList(download_id),
//     );
//     Ok(())
// }

// AppEvent::InsertDownloadFromDBToDownloadingList(id) => {
//     let download = get_downloads_by_id(&self.pool, id).await?;
//     update_download_status(&self.pool, id, "downloading").await?;
//     dispatch(&self.tx, AppEvent::StartDownload(id, download));
//     Ok(())
// }

// AppEvent::UpdateChunk(download_id, chunk_index, downloaded_bytes, expected_hash) => {
//     update_chunk_downloaded(
//         &self.pool,
//         download_id,
//         chunk_index,
//         downloaded_bytes,
//         expected_hash,
//     )
//     .await
// }

// AppEvent::SendDownloadList => {

// }

// AppEvent::DownloadFinished(download_id) => {
//     update_download_status(&self.pool, download_id, "completed").await?;
//     self.reporter.flush_report(download_id).await;
//     dispatch(&self.tx, AppEvent::SendDownloadList);

//     Ok(())
// }

// AppEvent::StartDownload(id, download) => {
//     let chunks = get_download_chunks_by_download_id(&self.pool, id).await?;
//     let tx = self.tx.clone();

//     spawn(async move { download_chunks(tx, chunks, download).await });

//     dispatch(&self.tx, AppEvent::SendDownloadList);

//     Ok(())
// }

// AppEvent::MakeChunkHash(downloaded_bytes, chunk) => {
//     let hash = compute_partial_hash(
//         &chunk.file_path,
//         chunk.start_byte as u64,
//         downloaded_bytes,
//     )
//     .await?
//     let download_id = chunk
// //         .download_id
// //         .parse::<i64>()
// //         .map_err(|e| e.to_string())?;

//     dispatch(
//         &self.tx,
//         AppEvent::UpdateChunk(
//             download_id,
//             chunk.chunk_index,
//             downloaded_bytes as i64,
//             hash,
//         ),
//     );
//     Ok(())
// }
// AppEvent::ResumeDownload(download_id) => {
//     let chunks = get_download_chunks_by_download_id(&self.pool, download_id).await?;
//     dispatch(
//         &self.tx,
//         AppEvent::ValidateExistingFile(download_id, chunks),
//     );
//     Ok(())
// }

// AppEvent::ValidateExistingFile(download_id, chunks) => {
//     let results: Result<Vec<bool>, String> =
//         try_join_all(chunks.iter().map(|f| async move {
//             let hash = compute_partial_hash(
//                 &f.file_path,
//                 f.start_byte as u64,
//                 f.downloaded_bytes as u64,
//             )
//             .await
//             .map_err(|e| e.to_string())?;

//             Ok(f.expected_hash == Some(hash))
//         }))
//         .await;

//     let is_all_match = match results {
//         Ok(vec) => vec.into_iter().all(|f| f),
//         Err(_) => false,
//     };

//     if is_all_match {
//         dispatch(
//             &self.tx,
//             AppEvent::InsertDownloadFromDBToDownloadingList(download_id),
//         );
//         Ok(())
//     } else {
//         Err("file not match".to_string())
//     }
// }

// AppEvent::ReportChunkReceivedBytes(download_id, received_bytes, total_bytes) => {
//     self.reporter
//         .add_report(download_id, received_bytes, total_bytes)
//         .await;
//     Ok(())
// }
