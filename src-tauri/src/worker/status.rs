use std::{collections::HashMap, time::Duration};
use tokio::{select, time::sleep};

use crate::{dispatch, spawn};

use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DownloadStatus {
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
    pub(super) async fn start_status_listener(self: &Arc<Self>) {
        let self_clone = Arc::clone(self);

        let worker = self.data.read().await;
        let notify = Arc::clone(&self.notify);
        let cancel_token = Arc::clone(&worker.cancel_token);
        let chunks_status = Arc::clone(&self.chunks_status);

        spawn!("download_status_update", {
            let mut cancelled = false;

            loop {
                select! {
                    _ = notify.notified() => {
                        sleep(Duration::from_millis(100)).await;

                        let status_snapshot = chunks_status
                            .iter()
                            .map(|s| s.value().clone())
                            .collect::<Vec<_>>();

                        self_clone.calculate_worker_status(&status_snapshot).await;

                        if cancelled {
                            break;
                        }
                    }
                    _ = cancel_token.cancelled() => {
                        cancelled = true;
                    }
                };
            }
        });
    }

    pub(super) async fn update_chunk_status(&self, index: i64, status: ChunkDownloadStatus) {
        self.chunks_status.entry(index).insert(status);
        self.notify.notify_one();
    }

    async fn calculate_worker_status(&self, statuses: &[ChunkDownloadStatus]) {
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
                let msg = self.get_error_message(statuses);
                self.update_download_status(DownloadStatus::Failed, msg)
                    .await;
            }
            (_, true, _, _, _) => {
                let msg = self.get_error_message(statuses);
                self.update_download_status(DownloadStatus::Trying, msg)
                    .await;
            }
            (_, _, true, _, _) => {
                self.update_download_status(DownloadStatus::Downloading, None)
                    .await;
            }
            (_, _, _, true, _) => {
                self.update_download_status(DownloadStatus::Paused, None)
                    .await;
            }
            (_, _, _, _, true) => {
                self.update_download_status(DownloadStatus::Completed, None)
                    .await;
            }
            _ => {}
        }
    }

    async fn update_download_status(&self, status: DownloadStatus, err_msg: Option<String>) {
        dispatch!(
            manager,
            UpdateDownloadStatus,
            (status, err_msg, self.download_id)
        );
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
