use std::time::Duration;

use tokio::{task::JoinError, time::sleep};

// use crate::{events::dispatch, utils::app_state::AppEvent};

#[derive(Debug)]
pub enum DownloadStatus {
    Paused,
    Finished,
}

#[derive(Debug)]
pub(super) enum WorkerOutcome {
    Finished,
    Paused,
    Errored,
    Mixed,
}

impl super::DownloadWorker {
    pub(super) async fn handle_finished(&self) {
        // if self.disk_report.lock().await.wrote_bytes
        //     < self.internet_report.lock().await.downloaded_bytes
        // {
        //     let _ = self.emit_and_update_download_status("writing").await;
        // }

        // loop {
        //     let wrote = self.disk_report.lock().await.wrote_bytes;
        //     let downloaded = self.internet_report.lock().await.downloaded_bytes;

        //     if wrote == downloaded {
        //         break;
        //     }

        //     sleep(Duration::from_millis(100)).await;
        // }

        // dispatch(
        //     &self.app_event,
        //     AppEvent::DownloadFinished(self.download_id),
        // );
        // let _ = self.emit_and_update_download_status("completed").await;
    }

    pub(super) async fn handle_paused(&self) {
        // dispatch(&self.app_event, AppEvent::DownloadPaused(self.download_id));
        // let _ = self.emit_and_update_download_status("paused").await;
    }

    pub(super) async fn handle_retry(&self) {
        sleep(Duration::from_secs(5)).await;
    }

    pub(super) async fn handle_failed(&self) {
        // let _ = self.emit_and_update_download_status("failed").await;
        // dispatch(&self.app_event, AppEvent::DownloadFailed(self.download_id));
    }

    pub(super) fn classify_results(
        &self,
        results: Vec<Result<Result<DownloadStatus, i64>, JoinError>>,
    ) -> WorkerOutcome {
        let mut has_finished = false;
        let mut has_paused = false;
        let mut has_error = false;

        for result in &results {
            match result {
                Ok(Ok(DownloadStatus::Finished)) => has_finished = true,
                Ok(Ok(DownloadStatus::Paused)) => has_paused = true,
                Ok(Err(_)) | Err(_) => has_error = true,
            }
        }

        match (has_finished, has_paused, has_error) {
            (true, false, false) => WorkerOutcome::Finished,
            (false, true, false) => WorkerOutcome::Paused,
            (false, false, true) => WorkerOutcome::Errored,
            _ => WorkerOutcome::Mixed,
        }
    }
}
