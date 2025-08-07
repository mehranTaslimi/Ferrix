use super::actions::{DownloadActions, ReportActions, SystemActions};

use anyhow::Context;
use std::sync::Arc;

#[derive(Debug)]
pub enum RegistryAction {
    NewDownload(/*Download ID */ i64),
    CheckAvailablePermit,
    UpdateNetworkReport(/*Download ID */ i64, /*Bytes len*/ u64),
    UpdateDiskReport(
        /* Download ID */ i64,
        /* Chunk Index */ i64,
        /* Bytes len */ u64,
    ),
    CleanDownloadedItemData(/* Download ID */ i64),
    PauseDownload(/* Download ID */ i64),
    ResumeDownload(/* Download ID */ i64),
    RecoverDownloads,
    RemoveDownload(/* Download ID */ i64, /* Remove File */ bool),
    CloseRequested,
    PrepareDownloadData(/* Download ID */ i64),
    UpdateChunkBufferReport(
        /* Download ID */ i64,
        /* Chunk Index */ i64,
        /* Bytes */ Vec<u8>,
    ),
}

impl super::Registry {
    pub fn dispatch(action: RegistryAction) -> anyhow::Result<()> {
        let mpsc_sender = Arc::clone(&Self::get_state().mpsc_sender);
        mpsc_sender
            .send(action)
            .context("failed to dispatch registry action: receiver might be closed")
    }

    pub(super) async fn reducer(action: RegistryAction) -> anyhow::Result<()> {
        use RegistryAction::*;

        match action {
            // Download
            PauseDownload(download_id) => Self::pause_download(download_id).await,
            ResumeDownload(download_id) => Self::resume_download(download_id).await,
            RemoveDownload(download_id, remove_file) => {
                Self::remove_download(download_id, remove_file).await
            }
            NewDownload(download_id) => Self::new_download(download_id).await,
            RecoverDownloads => Self::recover_downloads().await,
            PrepareDownloadData(download_id) => Self::prepare_download_data(download_id).await,
            CleanDownloadedItemData(download_id) => Self::clean_download_data(download_id).await,

            // Report
            UpdateNetworkReport(download_id, bytes_len) => {
                Self::update_network_report(download_id, bytes_len).await
            }
            UpdateDiskReport(download_id, chunk_index, bytes_len) => {
                Self::update_disk_report(download_id, chunk_index, bytes_len).await
            }
            UpdateChunkBufferReport(download_id, chunk_index, bytes) => {
                Self::update_chunk_buffer_report(download_id, chunk_index, bytes).await
            }

            // System
            CheckAvailablePermit => Self::check_available_permit().await,
            CloseRequested => Self::close_request().await,
        }
    }
}
