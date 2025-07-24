use dashmap::DashMap;
use futures_util::lock::Mutex;
use std::sync::Arc;
use tokio::sync::{mpsc::UnboundedSender, RwLock};
use tokio_util::sync::CancellationToken;

use crate::{
    emitter::Emitter,
    file::WriteMessage,
    models::{Download, DownloadChunk},
    registry::Registry,
};

mod bandwidth;
mod download;
mod status;

pub use status::DownloadStatus;

#[derive(Clone, Debug)]
pub struct Worker {
    pub download: Download,
    pub chunks: Vec<DownloadChunk>,
    pub cancel_token: Arc<CancellationToken>,
    pub file: Arc<UnboundedSender<WriteMessage>>,
}

#[derive(Debug, Clone)]
pub struct DownloadWorker {
    pub download_id: i64,
    data: Arc<RwLock<Worker>>,
    chunks_status: Arc<DashMap<i64, status::ChunkDownloadStatus>>,
    last_status: Arc<Mutex<status::DownloadStatus>>,
}

impl DownloadWorker {
    pub fn new(download_id: i64) -> Result<Arc<Self>, ()> {
        let worker = Registry::get_state().workers.get(&download_id);
        let chunks_status = Arc::new(DashMap::new());

        match worker {
            Some(worker) => Ok(Arc::new(Self {
                download_id,
                data: Arc::clone(&worker),
                chunks_status,
                last_status: Arc::new(Mutex::new(status::DownloadStatus::Queued)),
            })),
            None => {
                Emitter::emit_error("download not found");
                Err(())
            }
        }
    }
}
