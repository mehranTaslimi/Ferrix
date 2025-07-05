use std::sync::Arc;

use tokio_util::bytes;

use crate::worker::DownloadWorker;

#[derive(Debug)]
pub enum ManagerAction {
    StartDownload(/*Download ID */ i64),
    UpdateDownloadStatus(/*Download ID */ i64),
    ManageWorkerResult(DownloadWorker),
    ReportNetworkWorker(/*Download ID */ i64, /*Bytes len*/ u64),
}

impl super::DownloadsManager {
    pub fn dispatch(self: Arc<Self>, action: ManagerAction) {
        let mpsc_sender = Arc::clone(&self.mpsc_sender);
        mpsc_sender.send(action).unwrap()
    }

    pub(super) async fn reducer(self: &Arc<Self>, action: ManagerAction) {
        let self_clone = Arc::clone(&self);

        match action {
            ManagerAction::StartDownload(download_id) => {
                self_clone.start_download_action(download_id).await;
            }
            ManagerAction::UpdateDownloadStatus(download_id) => {
                self_clone.update_download_status_action(download_id).await;
            }
            ManagerAction::ManageWorkerResult(worker) => {
                self_clone.manage_worker_result_action(worker).await
            }
            ManagerAction::ReportNetworkWorker(download_id, bytes_len) => {
                self_clone
                    .update_worker_network_report(download_id, bytes_len)
                    .await;
            }
        }
    }
}
