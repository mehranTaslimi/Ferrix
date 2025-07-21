use std::collections::HashMap;

use log::{error, warn};
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
    Retry,
}

impl super::DownloadWorker {
    pub(super) fn classify_results(
        &self,
        results: Vec<Result<(i64, Result<ChunkDownloadStatus, ClientError>), JoinError>>,
    ) -> HashMap<NormalizedDownloadStatus, Vec<i64>> {
        let mut counter: HashMap<NormalizedDownloadStatus, Vec<i64>> = HashMap::new();

        for result in results.iter() {
            let (maybe_chunk_index, normalized_statuses) = match result {
                Ok((_, Ok(ChunkDownloadStatus::Finished))) => {
                    (None, NormalizedDownloadStatus::Finished)
                }
                Ok((_, Ok(ChunkDownloadStatus::Paused))) => {
                    (None, NormalizedDownloadStatus::Paused)
                }
                Ok((chunk_index, Err(err))) => {
                    if err.is_retryable() {
                        warn!("Chunk {} failed but is retryable: {:?}", chunk_index, err);
                        (Some(chunk_index), NormalizedDownloadStatus::Retry)
                    } else {
                        error!("Chunk {} failed permanently: {:?}", chunk_index, err);
                        (Some(chunk_index), NormalizedDownloadStatus::Error)
                    }
                }
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
