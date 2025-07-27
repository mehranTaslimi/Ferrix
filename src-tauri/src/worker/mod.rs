use chrono::Duration;
use dashmap::DashMap;
use std::{sync::Arc, time::Instant};
use tokio::sync::{mpsc::UnboundedSender, Mutex, Notify, RwLock};
use tokio_util::sync::CancellationToken;

use crate::{
    emitter::Emitter,
    file::WriteMessage,
    models::{Download, DownloadChunk},
    registry::{Registry, Report},
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
    report: Arc<Report>,
    chunks_status: Arc<DashMap<i64, status::ChunkDownloadStatus>>,
    notify: Arc<Notify>,
    stream_duration: Arc<Mutex<Option<Instant>>>,
    received_bytes: Arc<Mutex<u64>>,
}

impl DownloadWorker {
    pub async fn new(download_id: i64) -> Result<Arc<Self>, ()> {
        let chunks_status = Arc::new(DashMap::new());
        let notify = Arc::new(Notify::new());
        let state = Registry::get_state();
        let worker_ref = state.workers.get(&download_id);
        let report_ref = state.reports.get(&download_id);
        let stream_duration = Arc::new(Mutex::new(None));
        let received_bytes = Arc::new(Mutex::new(0));

        if let (Some(worker), Some(report)) = (worker_ref, report_ref) {
            let w = Arc::new(Self {
                download_id,
                data: Arc::clone(&worker),
                report: Arc::clone(&report),
                chunks_status,
                notify,
                stream_duration,
                received_bytes,
            });

            w.start_status_listener().await;

            Ok(w)
        } else {
            Emitter::emit_error("download not found");
            Err(())
        }
    }
}
