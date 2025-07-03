use tokio::sync::broadcast;

pub struct AppState {
    pub broadcast_tx: broadcast::Sender<AppEvent>,
}

impl AppState {
    pub async fn new(broadcast_tx: broadcast::Sender<AppEvent>) -> Self {
        Self { broadcast_tx }
    }
}

#[derive(Clone, Debug)]
pub enum AppEvent {
    // StartNewDownload(FileInfo, ChunkCount),
    // PauseDownload(DownloadId),
    // ResumeDownload(DownloadId),
    // DownloadFinished(DownloadId),
    // DownloadPaused(DownloadId),
    // DownloadFailed(DownloadId),
    // PauseAllDownload,
    // ForcePauseAllDownloadWorkers,
}
