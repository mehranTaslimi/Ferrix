use std::sync::Arc;

use crate::worker::DownloadWorker;

#[derive(Debug)]
pub enum ManagerAction {
    StartDownload(/*Download ID */ i64),
    UpdateDownloadStatus(/* Status */ String, /*Download ID */ i64),
    ManageWorkerResult(Arc<DownloadWorker>),
    PauseDownload(/*Download ID */ i64),
    UpdateChunks(/*Download ID */ i64),
    ValidateChunksHash(/*Download ID */ i64),
    ResetChunks(/*Download ID */ i64, /* Chunk Index */ Vec<i64>),
}

impl super::DownloadsManager {
    pub fn dispatch(self: &Arc<Self>, action: ManagerAction) {
        let mpsc_sender = Arc::clone(&self.mpsc_sender);
        mpsc_sender.send(action).unwrap()
    }

    pub(super) async fn reducer(self: &Arc<Self>, action: ManagerAction) {
        println!("Manager: {action:?}");
        let self_clone = Arc::clone(&self);

        match action {
            ManagerAction::StartDownload(download_id) => {
                self_clone.start_download_action(download_id).await;
            }
            ManagerAction::UpdateDownloadStatus(status, download_id) => {
                self_clone
                    .update_download_status_action(status, download_id)
                    .await;
            }
            ManagerAction::ManageWorkerResult(worker) => {
                self_clone.manage_worker_result_action(worker).await
            }
            ManagerAction::PauseDownload(download_id) => {
                self_clone.pause_download_action(download_id).await;
            }
            ManagerAction::UpdateChunks(download_id) => {
                self_clone.update_chunks_action(download_id).await;
            }
            ManagerAction::ValidateChunksHash(download_id) => {
                self_clone.validate_chunks_hash_action(download_id).await;
            }
            ManagerAction::ResetChunks(download_id, chunk_index) => {
                self_clone
                    .reset_chunks_action(download_id, chunk_index)
                    .await;
            }
        }
    }
}
