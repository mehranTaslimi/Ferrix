use std::sync::Arc;

use crate::models::{Download, DownloadChunk};

#[derive(Debug)]
pub enum RegistryAction {
    NewDownloadQueue(/*Download ID */ i64),
    CheckAvailablePermit,
    AddDownloadToWorkersMap(/*Download ID */ i64),
    CreateDownloadReport(Download, Vec<DownloadChunk>),
    UpdateNetworkReport(/*Download ID */ i64, /*Bytes len*/ u64),
    UpdateDiskReport(
        /* Download ID */ i64,
        /* Chunk Index */ i64,
        /* Bytes len */ u64,
    ),
    CleanDownloadedItemData(/* Download ID */ i64),
    PauseDownload(/* Download ID */ i64),
    ResumeDownload(/* Download ID */ i64),
    RecoverQueuedDownloadFromRepository,
    RemoveDownload(/* Download ID */ i64, /* Remove File */ bool),
    CloseRequested,
}

impl super::Registry {
    pub fn dispatch(action: RegistryAction) {
        let mpsc_sender = Arc::clone(&Self::get_state().mpsc_sender);
        mpsc_sender.send(action).unwrap()
    }

    pub(super) async fn reducer(action: RegistryAction) {
        // println!("Registry: {action:?}");
        match action {
            RegistryAction::NewDownloadQueue(download_id) => {
                Self::add_download_to_queue(download_id).await;
            }
            RegistryAction::RecoverQueuedDownloadFromRepository => {
                Self::recover_queued_download_from_repository_action().await;
            }
            RegistryAction::CheckAvailablePermit => {
                Self::check_available_permit_action().await;
            }
            RegistryAction::AddDownloadToWorkersMap(download_id) => {
                Self::add_download_workers_map_action(download_id).await;
            }
            RegistryAction::CreateDownloadReport(download, download_chunk) => {
                Self::create_download_report_action(download, download_chunk).await;
            }
            RegistryAction::UpdateNetworkReport(download_id, bytes_len) => {
                Self::update_network_report_action(download_id, bytes_len).await;
            }
            RegistryAction::UpdateDiskReport(download_id, chunk_index, bytes_len) => {
                Self::update_disk_report_action(download_id, chunk_index, bytes_len).await;
            }
            RegistryAction::CleanDownloadedItemData(download_id) => {
                Self::clean_downloaded_item_data(download_id).await;
            }
            RegistryAction::PauseDownload(download_id) => {
                Self::pause_download_action(download_id).await;
            }
            RegistryAction::ResumeDownload(download_id) => {
                Self::resume_download_action(download_id).await;
            }
            RegistryAction::RemoveDownload(download_id, remove_file) => {
                Self::remove_download_action(download_id, remove_file).await;
            }
            RegistryAction::CloseRequested => {
                Self::close_requested_action().await;
            }
        }
    }
}
