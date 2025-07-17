use std::collections::HashMap;

use tokio::task::JoinError;

use crate::client::ClientError;

#[derive(Debug)]
pub enum ChunkDownloadStatus {
    Paused,
    Finished,
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum NormalizedDownloadStatus {
    Finished,
    Paused,
    Error,
}

impl super::DownloadWorker {
    pub(super) fn classify_results(
        &self,
        results: Vec<Result<Option<(i64, Result<ChunkDownloadStatus, ClientError>)>, JoinError>>,
    ) -> HashMap<NormalizedDownloadStatus, Vec<i64>> {
        let mut counter: HashMap<NormalizedDownloadStatus, Vec<i64>> = HashMap::new();

        for result in results.iter() {
            let (maybe_chunk_index, normalized_statuses) = match result {
                Ok(Some((_, Ok(ChunkDownloadStatus::Finished)))) => {
                    (None, NormalizedDownloadStatus::Finished)
                }
                Ok(Some((_, Ok(ChunkDownloadStatus::Paused)))) => {
                    (None, NormalizedDownloadStatus::Paused)
                }
                Ok(Some((chunk_index, Err(_)))) => {
                    (Some(chunk_index), NormalizedDownloadStatus::Error)
                }
                Ok(None) => (None, NormalizedDownloadStatus::Paused),
                Err(_) => (None, NormalizedDownloadStatus::Error),
            };

            let entry = counter.entry(normalized_statuses).or_default();

            if let Some(chunk_index) = maybe_chunk_index {
                entry.push(*chunk_index);
            }
        }

        counter
    }
}
