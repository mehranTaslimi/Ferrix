mod bandwidth;
pub mod task;

use std::{collections::HashMap, sync::Arc, time::Duration};

use sqlx::SqlitePool;
use tauri::AppHandle;
use tokio::{
    sync::{broadcast::Sender, Mutex},
    time::sleep,
};
use tokio_util::sync::CancellationToken;

use crate::{
    events::dispatch,
    manager::{bandwidth::BandwidthManager, task::TaskManager},
    models::{ChunkCount, DownloadId, FileInfo},
    utils::app_state::AppEvent,
    worker::DownloadWorker,
};

#[derive(Clone)]
struct WorkerData {
    chunk_count: i64,
    speed_bps: Arc<Mutex<u64>>,
    cancellation_token: CancellationToken,
}

pub struct DownloadsManager {
    pool: SqlitePool,
    app_handle: AppHandle,
    app_event: Sender<AppEvent>,
    workers: Arc<Mutex<HashMap<DownloadId, WorkerData>>>,
    bandwidth: BandwidthManager,
    task: Arc<TaskManager>,
}

impl DownloadsManager {
    pub fn new(app_event: Sender<AppEvent>, pool: SqlitePool, app_handle: AppHandle) -> Self {
        let task = TaskManager::new();
        let workers = Arc::new(Mutex::new(HashMap::new()));
        let workers_clone = Arc::clone(&workers);

        Self {
            app_handle,
            pool,
            app_event,
            task: Arc::clone(&task),
            workers,
            bandwidth: BandwidthManager::new(workers_clone),
        }
    }

    pub async fn manage(&self, app_event: AppEvent) -> Result<(), String> {
        match app_event {
            AppEvent::StartNewDownload(file_info, chunk_count) => {
                self.create_worker(None, Some(file_info), Some(chunk_count))
                    .await
            }

            AppEvent::ResumeDownload(download_id) => {
                self.create_worker(Some(download_id), None, None).await
            }

            AppEvent::PauseDownload(download_id) => {
                if let Some(worker) = self.workers.lock().await.get(&download_id) {
                    worker.cancellation_token.cancel();
                };
                Ok(())
            }

            AppEvent::PauseAllDownload => {
                let workers = self.workers.lock().await;

                for (_, worker) in workers.iter() {
                    worker.cancellation_token.cancel();
                }

                Ok(())
            }

            AppEvent::ForcePauseAllDownloadWorkers => {
                dispatch(&self.app_event, AppEvent::PauseAllDownload);

                let app_handle = self.app_handle.clone();
                let workers = Arc::clone(&self.workers);

                self.task.spawn(async move {
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

    async fn create_worker(
        &self,
        download_id: Option<DownloadId>,
        file_info: Option<FileInfo>,
        chunk_count: Option<ChunkCount>,
    ) -> Result<(), String> {
        let worker = DownloadWorker::new(
            self.pool.clone(),
            self.app_handle.clone(),
            self.app_event.clone(),
            Arc::clone(&self.bandwidth.bandwidth_limit),
            Arc::clone(&self.task),
            download_id,
            file_info,
            chunk_count,
        )
        .await?;

        self.workers.lock().await.insert(
            worker.download_id,
            WorkerData {
                chunk_count: worker.chunk_count,
                speed_bps: Arc::clone(&worker.speed_bps),
                cancellation_token: worker.cancellation_token.clone(),
            },
        );

        self.task.spawn(async move {
            worker.start_download().await;
        });

        Ok(())
    }
}
