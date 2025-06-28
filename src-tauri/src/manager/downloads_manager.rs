use std::{collections::HashMap, sync::Arc, time::Duration};

use sqlx::SqlitePool;
use tauri::AppHandle;
use tokio::{
    spawn,
    sync::{broadcast::Sender, Mutex},
    time::sleep,
};
use tokio_util::sync::CancellationToken;

use crate::{
    events::dispatch, models::DownloadId, utils::app_state::AppEvent, worker::DownloadWorker,
};

pub struct DownloadsManager {
    pool: SqlitePool,
    app_handle: AppHandle,
    app_event: Sender<AppEvent>,
    workers: Arc<Mutex<HashMap<DownloadId, CancellationToken>>>,
}

impl DownloadsManager {
    pub fn new(app_event: Sender<AppEvent>, pool: SqlitePool, app_handle: AppHandle) -> Self {
        Self {
            app_handle,
            pool,
            app_event,
            workers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn manage(&self, app_event: AppEvent) -> Result<(), String> {
        match app_event {
            AppEvent::StartNewDownload(file_info, chunk_count) => {
                let worker = DownloadWorker::new(
                    self.pool.clone(),
                    self.app_handle.clone(),
                    self.app_event.clone(),
                    None,
                    Some(file_info),
                    Some(chunk_count),
                )
                .await?;

                self.workers
                    .lock()
                    .await
                    .insert(worker.download_id, worker.cancellation_token.clone());

                tokio::spawn(async move {
                    worker.start_download().await;
                });

                Ok(())
            }

            AppEvent::ResumeDownload(download_id) => {
                let worker = DownloadWorker::new(
                    self.pool.clone(),
                    self.app_handle.clone(),
                    self.app_event.clone(),
                    Some(download_id),
                    None,
                    None,
                )
                .await?;

                self.workers
                    .lock()
                    .await
                    .insert(worker.download_id, worker.cancellation_token.clone());

                tokio::spawn(async move {
                    worker.start_download().await;
                });

                Ok(())
            }

            AppEvent::PauseDownload(download_id) => {
                if let Some(cancellation_token) = self.workers.lock().await.get(&download_id) {
                    cancellation_token.cancel();
                };
                Ok(())
            }

            AppEvent::PauseAllDownload => {
                let workers = self.workers.lock().await;

                for (_, cancellation_token) in workers.iter() {
                    cancellation_token.cancel();
                }

                Ok(())
            }

            AppEvent::ForcePauseAllDownloadWorkers => {
                dispatch(&self.app_event, AppEvent::PauseAllDownload);

                let app_handle = self.app_handle.clone();
                let workers = Arc::clone(&self.workers);

                spawn(async move {
                    loop {
                        if workers.lock().await.is_empty() {
                            app_handle.exit(0);
                            break;
                        }
                        sleep(Duration::from_millis(100)).await;
                    }
                });

                Ok(())
            }

            AppEvent::DownloadFinished(download_id)
            | AppEvent::DownloadPaused(download_id)
            | AppEvent::DownloadFailed(download_id) => {
                self.workers.lock().await.remove(&download_id);
                Ok(())
            }
        }
    }
}
