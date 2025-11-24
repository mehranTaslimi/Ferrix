use anyhow::{anyhow, Context};
use dashmap::DashMap;
use std::{sync::Arc, time::Instant};
use tokio::sync::{mpsc::UnboundedSender, Mutex, RwLock};
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

#[derive(Debug)]
pub struct Worker {
    pub download: Download,
    pub chunks: Vec<DownloadChunk>,
    pub cancel_token: CancellationToken,
    pub file: UnboundedSender<WriteMessage>,
}

#[derive(Debug)]
pub struct DownloadWorker {
    pub download_id: i64,
    data: Arc<RwLock<Worker>>,
    report: Arc<Report>,
    chunks_status: DashMap<i64, status::ChunkDownloadStatus>,
    stream_duration: Mutex<Option<Instant>>,
    received_bytes: Mutex<u64>,
    last_worker_status: Mutex<DownloadStatus>,
}

impl DownloadWorker {
    pub async fn new(download_id: i64) -> anyhow::Result<Arc<Self>> {
        let chunks_status = DashMap::new();
        let state = Registry::get_state();
        let worker = state
            .workers
            .get(&download_id)
            .context(anyhow!("cannot find worker with id {}", download_id))?;
        let report = state
            .reports
            .get(&download_id)
            .context(anyhow!("cannot find report with id {}", download_id))?;
        let stream_duration = Mutex::new(None);
        let received_bytes = Mutex::new(0);
        let last_worker_status = Mutex::new(DownloadStatus::Unknown);

        let w = Arc::new(Self {
            download_id,
            data: Arc::clone(&worker),
            report: Arc::clone(&report),
            chunks_status,
            stream_duration,
            received_bytes,
            last_worker_status,
        });

        Ok(w)
    }
}
