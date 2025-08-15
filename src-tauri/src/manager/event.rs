use std::sync::Arc;

use anyhow::Context;

use crate::worker::DownloadStatus;

#[derive(Debug)]
pub enum ManagerAction {
    StartDownload(/*Download ID */ i64),
    UpdateDownloadStatus(
        /* Status */ DownloadStatus,
        /* Error Message */ Option<String>,
        /*Download ID */ i64,
    ),
    PauseDownload(/*Download ID */ i64),
    UpdateChunks(
        /*Download ID */ i64,
        /* Clean After Update */ bool,
    ),
    ResetChunk(/*Download ID */ i64, /* Chunk Index */ i64),
}

impl super::DownloadsManager {
    pub fn dispatch(self: &Arc<Self>, action: ManagerAction) -> anyhow::Result<()> {
        let mpsc_sender = Arc::clone(&self.mpsc_sender);
        mpsc_sender
            .send(action)
            .context("failed to dispatch manager action: receiver might be closed")
    }

    pub(super) async fn reducer(self: &Arc<Self>, action: ManagerAction) -> anyhow::Result<()> {
        let self_clone = Arc::clone(&self);

        use ManagerAction::*;

        match action {
            StartDownload(download_id) => self_clone.start_download_action(download_id).await,
            UpdateDownloadStatus(status, error_message, download_id) => {
                self_clone
                    .update_download_status_action(status, error_message, download_id)
                    .await
            }
            PauseDownload(download_id) => self_clone.pause_download_action(download_id).await,
            UpdateChunks(download_id, clean_after_update) => {
                self_clone
                    .update_chunks_action(download_id, clean_after_update)
                    .await
            }
            ResetChunk(download_id, chunk_index) => {
                self_clone
                    .reset_chunk_action(download_id, chunk_index)
                    .await
            }
        }
    }
}
