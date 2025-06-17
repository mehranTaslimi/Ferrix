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
