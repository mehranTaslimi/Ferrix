use futures_util::lock::Mutex;
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;
use tokio_util::sync::CancellationToken;

use crate::{
    file::WriteMessage,
    models::{Download, DownloadChunk},
};

mod bandwidth;
mod download;
mod outcome;

pub use outcome::ChunkDownloadStatus;

#[derive(Debug)]
pub enum DownloadStatus {
    Paused,
    Completed,
    Failed,
    Downloading,
    // Queued,
    Error,
}

impl ToString for DownloadStatus {
    fn to_string(&self) -> String {
        match self {
            DownloadStatus::Paused => "paused".to_string(),
            DownloadStatus::Completed => "completed".to_string(),
            DownloadStatus::Failed => "failed".to_string(),
            DownloadStatus::Downloading => "downloading".to_string(),
            // DownloadStatus::Queued => "queued".to_string(),
            DownloadStatus::Error => "error".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Worker {
    pub download: Download,
    pub chunks: Vec<DownloadChunk>,
    pub cancel_token: Arc<CancellationToken>,
    pub file: Arc<UnboundedSender<WriteMessage>>,
}

#[derive(Debug, Clone)]
pub struct DownloadWorker {
    pub download: Download,
    chunks: Vec<DownloadChunk>,
    cancel_token: Arc<CancellationToken>,
    file: Arc<UnboundedSender<WriteMessage>>,
    retries_indexes: Arc<Mutex<Vec<i64>>>,
}

impl DownloadWorker {
    pub fn new(
        download: Download,
        chunks: Vec<DownloadChunk>,
        cancel_token: Arc<CancellationToken>,
        file: Arc<UnboundedSender<WriteMessage>>,
    ) -> Arc<Self> {
        let retries_indexes = Arc::new(Mutex::new(vec![]));

        Arc::new(Self {
            download,
            chunks,
            cancel_token,
            file,
            retries_indexes,
        })
    }
}
