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
    ShallowUpdateDownloadStatus(/* Download ID */ i64, /* Status */ &'static str),
}

impl super::Registry {
    pub fn dispatch(action: RegistryAction) {
        let mpsc_sender = Arc::clone(&Self::get_state().mpsc_sender);
        mpsc_sender.send(action).unwrap()
    }

    pub(super) async fn reducer(action: RegistryAction) {
        match action {
            RegistryAction::NewDownloadQueue(download_id) => {
                Self::add_download_to_queue(download_id).await;
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
            RegistryAction::ShallowUpdateDownloadStatus(download_id, status) => {
                Self::shallow_update_download_status_action(download_id, status).await;
            }
        }
    }
}
