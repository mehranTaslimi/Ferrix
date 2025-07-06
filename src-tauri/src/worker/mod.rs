use std::{sync::Arc, time::Instant};

use futures_util::lock::Mutex;
use tokio::sync::mpsc::UnboundedSender;
use tokio_util::sync::CancellationToken;

use crate::{
    file::WriteMessage,
    manager::DownloadsManager,
    models::{Download, DownloadChunk},
};

mod bandwidth;
mod download;
mod outcome;
pub mod validation;

use outcome::DownloadStatus;
pub use outcome::WorkerOutcome;

#[derive(Clone, Debug)]
pub struct Worker {
    pub download: Download,
    pub chunks: Vec<DownloadChunk>,
    pub cancel_token: Arc<CancellationToken>,
    pub download_id: i64,
    pub file: Arc<UnboundedSender<WriteMessage>>,
}

#[derive(Debug, Clone)]
pub struct DownloadWorker {
    download: Download,
    chunks: Vec<DownloadChunk>,
    cancel_token: Arc<CancellationToken>,
    file: Arc<UnboundedSender<WriteMessage>>,
    manager: Arc<DownloadsManager>,
    pub download_id: i64,
}

impl DownloadWorker {
    pub fn new(
        download: Download,
        chunks: Vec<DownloadChunk>,
        cancel_token: Arc<CancellationToken>,
        download_id: i64,
        file: Arc<UnboundedSender<WriteMessage>>,
        manager: Arc<DownloadsManager>,
    ) -> Self {
        Self {
            download,
            chunks,
            cancel_token,
            download_id,
            file,
            manager,
        }
    }
}
