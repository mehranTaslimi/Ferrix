use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::{mpsc::UnboundedSender, Notify, RwLock};
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
    notify: Arc<Notify>,
}

impl DownloadWorker {
    pub async fn new(download_id: i64) -> Result<Arc<Self>, ()> {
        let worker = Registry::get_state().workers.get(&download_id);
        let chunks_status = Arc::new(DashMap::new());
        let notify = Arc::new(Notify::new());

        match worker {
            Some(worker) => {
                let w = Arc::new(Self {
                    download_id,
                    data: Arc::clone(&worker),
                    chunks_status,
                    notify,
                });

                w.start_status_listener().await;

                Ok(w)
            }
            None => {
                Emitter::emit_error("download not found");
                Err(())
            }
        }
    }
}
