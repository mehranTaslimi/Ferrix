mod bandwidth;
mod monitor;
mod repoter;

pub struct DownloadsManager;

impl DownloadsManager {
    pub fn new() {
        Self::downloading_monitor();
        Self::pending_queue_monitor();
        Self::reporting_monitor();
    }

    // pub async fn manage(&self, app_event: AppEvent) -> Result<(), String> {
    //     match app_event {
    //         AppEvent::StartNewDownload(file_info, chunk_count) => {
    //             self.create_worker(None, Some(file_info), Some(chunk_count))
    //                 .await
    //         }

    //         AppEvent::ResumeDownload(download_id) => {
    //             self.create_worker(Some(download_id), None, None).await
    //         }

    //         AppEvent::PauseDownload(download_id) => {
    //             if let Some(worker) = self.workers.lock().await.get(&download_id) {
    //                 worker.cancellation_token.cancel();
    //             };
    //             Ok(())
    //         }

    //         AppEvent::PauseAllDownload => {
    //             let workers = self.workers.lock().await;

    //             for (_, worker) in workers.iter() {
    //                 worker.cancellation_token.cancel();
    //             }

    //             Ok(())
    //         }

    //         AppEvent::ForcePauseAllDownloadWorkers => {
    //             dispatch(&self.app_event, AppEvent::PauseAllDownload);

    //             let app_handle = self.app_handle.clone();
    //             let workers = Arc::clone(&self.workers);

    //             self.task.spawn("exit_app", async move {
    //                 loop {
    //                     if workers.lock().await.is_empty() {
    //                         app_handle.exit(0);
    //                         break;
    //                     }
    //                     sleep(Duration::from_millis(100)).await;
    //                 }
    //             });

    //             Ok(())
    //         }

    //         AppEvent::DownloadFinished(download_id)
    //         | AppEvent::DownloadPaused(download_id)
    //         | AppEvent::DownloadFailed(download_id) => {
    //             self.workers.lock().await.remove(&download_id);
    //             Ok(())
    //         }
    //     }
    // }

    // async fn create_worker(
    //     &self,
    //     download_id: Option<DownloadId>,
    //     file_info: Option<FileInfo>,
    //     chunk_count: Option<ChunkCount>,
    // ) -> Result<(), String> {
    //     let worker = DownloadWorker::new(
    //         self.app_handle.clone(),
    //         self.app_event.clone(),
    //         Arc::clone(&self.bandwidth.bandwidth_limit),
    //         Arc::clone(&self.task),
    //         download_id,
    //         file_info,
    //         chunk_count,
    //     )
    //     .await?;

    //     self.workers.lock().await.insert(
    //         worker.download_id,
    //         WorkerData {
    //             speed_bps: Arc::clone(&worker.speed_bps),
    //             cancellation_token: worker.cancellation_token.clone(),
    //         },
    //     );

    //     let task_name = format!("start download: {}", worker.download_id);
    //     self.task.spawn(&task_name, async move {
    //         worker.start_download().await;
    //     });

    // Ok(())
    // }
}
