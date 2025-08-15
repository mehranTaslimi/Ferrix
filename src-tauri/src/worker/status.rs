use std::collections::HashMap;

use crate::{client::ClientError, dispatch};

use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DownloadStatus {
    Paused,
    Completed,
    Failed,
    Downloading,
    Trying,
    Unknown,
}

impl ToString for DownloadStatus {
    fn to_string(&self) -> String {
        match self {
            DownloadStatus::Paused => "paused".to_string(),
            DownloadStatus::Completed => "completed".to_string(),
            DownloadStatus::Failed => "failed".to_string(),
            DownloadStatus::Downloading => "downloading".to_string(),
            DownloadStatus::Trying => "trying".to_string(),
            DownloadStatus::Unknown => "unknown".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ChunkDownloadStatus {
    Paused,
    Finished,
    Downloading,
    Trying(ClientError),
    Errored(ClientError),
}

impl DownloadWorker {
    async fn update_worker_status(&self) {
        let chunks_status = Arc::clone(&self.chunks_status);

        let status_snapshot = chunks_status
            .iter()
            .map(|s| s.value().clone())
            .collect::<Vec<_>>();

        let last_status = Arc::clone(&self.last_worker_status);
        let mut last_status_lock = last_status.lock().await;

        let (st, msg) = self.calculate_worker_status(&status_snapshot).await;

        if (*last_status_lock != st || st == DownloadStatus::Trying)
            && st != DownloadStatus::Unknown
        {
            *last_status_lock = st.clone();
            dispatch!(manager, UpdateDownloadStatus, (st, msg, self.download_id));
        }
    }

    pub(super) async fn update_chunk_status(&self, index: i64, status: ChunkDownloadStatus) {
        self.chunks_status.entry(index).insert(status);
        self.update_worker_status().await;
    }

    async fn calculate_worker_status(
        &self,
        statuses: &[ChunkDownloadStatus],
    ) -> (DownloadStatus, Option<String>) {
        use ChunkDownloadStatus::*;

        let mut has_errored = false;
        let mut has_trying = false;
        let mut all_downloading = true;
        let mut all_paused = true;
        let mut all_finished = true;

        for status in statuses {
            match status {
                Errored(_) => {
                    has_errored = true;
                    all_downloading = false;
                    all_paused = false;
                    all_finished = false;
                }
                Trying(_) => {
                    has_trying = true;
                    all_downloading = false;
                    all_paused = false;
                    all_finished = false;
                }
                Downloading => {
                    all_paused = false;
                    all_finished = false;
                }
                Paused => {
                    all_downloading = false;
                    all_finished = false;
                }
                Finished => {
                    all_downloading = false;
                    all_paused = false;
                }
            }
        }

        match (
            has_errored,
            has_trying,
            all_downloading,
            all_paused,
            all_finished,
        ) {
            (true, _, _, _, _) => {
                let msg = self.generate_error_message(statuses);
                (DownloadStatus::Failed, msg)
            }
            (_, true, _, _, _) => {
                let msg = self.generate_error_message(statuses);
                (DownloadStatus::Trying, msg)
            }
            (_, _, true, _, _) => (DownloadStatus::Downloading, None),
            (_, _, _, true, _) => (DownloadStatus::Paused, None),
            (_, _, _, _, true) => (DownloadStatus::Completed, None),
            _ => (DownloadStatus::Unknown, None),
        }
    }

    fn generate_error_message(&self, statuses: &[ChunkDownloadStatus]) -> Option<String> {
        use ChunkDownloadStatus::*;

        let mut error_message = String::new();
        let chunk_count = self.chunks_status.len();
        let mut errors: HashMap<String, u8> = HashMap::new();

        statuses.iter().for_each(|s| match s {
            Errored(err) | Trying(err) => {
                errors
                    .entry(err.to_string())
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
            _ => {}
        });

        if errors.is_empty() {
            return None;
        }

        for (error, count) in errors {
            if count == chunk_count as u8 {
                error_message.push_str(&error);
            } else {
                let chunk = if count == 1 { "chunk" } else { "chunks" };
                error_message.push_str(&format!("{} {}: {} ", count, chunk, error));
            }
        }

        Some(error_message)
    }
}
