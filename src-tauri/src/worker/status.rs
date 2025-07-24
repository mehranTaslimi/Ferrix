use std::collections::HashMap;

use crate::dispatch;

use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DownloadStatus {
    Queued,
    Paused,
    Completed,
    Failed,
    Downloading,
    Trying,
}

impl ToString for DownloadStatus {
    fn to_string(&self) -> String {
        match self {
            DownloadStatus::Paused => "paused".to_string(),
            DownloadStatus::Completed => "completed".to_string(),
            DownloadStatus::Failed => "failed".to_string(),
            DownloadStatus::Downloading => "downloading".to_string(),
            DownloadStatus::Trying => "trying".to_string(),
            DownloadStatus::Queued => "queued".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChunkDownloadStatus {
    Paused,
    Finished,
    Downloading,
    Trying(String),
    Errored(String),
}

impl DownloadWorker {
    pub(super) async fn update_chunk_status(&self, index: i64, status: ChunkDownloadStatus) {
        self.chunks_status.entry(index).insert(status);

        let status_snapshot = self
            .chunks_status
            .iter()
            .map(|s| s.value().clone())
            .collect::<Vec<_>>();

        self.calculate_worker_status(&status_snapshot).await;
    }

    async fn calculate_worker_status(&self, statuses: &[ChunkDownloadStatus]) {
        use ChunkDownloadStatus::*;

        let last_status = Arc::clone(&self.last_status);
        let mut last_status = last_status.lock().await;

        if statuses.iter().any(|s| matches!(s, Errored(_))) {
            let err_msg = self.get_error_message(statuses);

            println!("{err_msg:?}");

            *last_status = DownloadStatus::Failed;
            dispatch!(
                manager,
                UpdateDownloadStatus,
                (DownloadStatus::Failed, err_msg, self.download_id)
            );
        } else if statuses.iter().any(|s| matches!(s, Trying(_))) {
            let err_msg = self.get_error_message(statuses);

            println!("{err_msg:?}");

            *last_status = DownloadStatus::Trying;
            dispatch!(
                manager,
                UpdateDownloadStatus,
                (DownloadStatus::Trying, err_msg, self.download_id)
            );
        } else if statuses.iter().all(|s| matches!(s, Downloading)) {
            if *last_status != DownloadStatus::Downloading {
                *last_status = DownloadStatus::Downloading;
                dispatch!(
                    manager,
                    UpdateDownloadStatus,
                    (DownloadStatus::Downloading, None, self.download_id)
                );
            }
        } else if statuses.iter().all(|s| matches!(s, Paused)) {
            if *last_status != DownloadStatus::Paused {
                *last_status = DownloadStatus::Paused;
                dispatch!(
                    manager,
                    UpdateDownloadStatus,
                    (DownloadStatus::Paused, None, self.download_id)
                );
            }
        } else if statuses.iter().all(|s| matches!(s, Finished)) {
            if *last_status != DownloadStatus::Completed {
                *last_status = DownloadStatus::Completed;
                dispatch!(
                    manager,
                    UpdateDownloadStatus,
                    (DownloadStatus::Completed, None, self.download_id)
                );
            }
        }
    }

    fn get_error_message(&self, statuses: &[ChunkDownloadStatus]) -> Option<String> {
        use ChunkDownloadStatus::*;

        let mut error_message = String::new();
        let chunk_count = self.chunks_status.len();
        let mut errors: HashMap<String, u8> = HashMap::new();

        statuses.iter().for_each(|s| match s {
            Errored(err) | Trying(err) => {
                errors
                    .entry(err.clone())
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
