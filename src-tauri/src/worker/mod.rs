use anyhow::{anyhow, Context};
use dashmap::DashMap;
use std::{sync::Arc, time::Instant};
use tokio::sync::{mpsc::UnboundedSender, Mutex, Notify, RwLock};
use tokio_util::sync::CancellationToken;

use crate::{
    file::WriteMessage,
    models::{Download, DownloadChunk},
    registry::{Registry, Report},
};

mod backoff;
mod bandwidth;
mod download;
mod status;
mod validation;

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
    pub async fn new(download_id: i64) -> anyhow::Result<Arc<Self>> {
        let chunks_status = Arc::new(DashMap::new());
        let notify = Arc::new(Notify::new());
        let state = Registry::get_state();
        let worker = state
            .workers
            .get(&download_id)
            .context(anyhow!("cannot find worker with id {}", download_id))?;
        let report = state
            .reports
            .get(&download_id)
            .context(anyhow!("cannot find report with id {}", download_id))?;
        let stream_duration = Arc::new(Mutex::new(None));
        let received_bytes = Arc::new(Mutex::new(0));

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
    }
}
