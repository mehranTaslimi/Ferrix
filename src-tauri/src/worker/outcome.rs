use tokio::task::JoinError;

use crate::client::ClientError;

#[derive(Debug)]
pub enum DownloadStatus {
    Paused,
    Finished,
}

#[derive(Debug)]
pub enum WorkerOutcome {
    Finished,
    Paused,
    Errored,
    Mixed,
}

impl super::DownloadWorker {
    pub(super) fn classify_results(
        &self,
        results: Vec<Result<Option<Result<DownloadStatus, ClientError>>, JoinError>>,
    ) -> WorkerOutcome {
        let mut has_finished = false;
        let mut has_paused = false;
        let mut has_error = false;

        for result in &results {
            match result {
                Ok(Some(Ok(DownloadStatus::Finished))) => has_finished = true,
                Ok(Some(Ok(DownloadStatus::Paused))) => has_paused = true,
                Ok(Some(Err(_))) | Err(_) => has_error = true,
                Ok(None) => has_finished = true,
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
