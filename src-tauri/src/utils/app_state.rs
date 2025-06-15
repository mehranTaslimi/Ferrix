use sqlx::SqlitePool;
use tokio::sync::broadcast;

use crate::{
    db::init_db,
    models::{
        Chunk, ChunkCount, ChunkIndex, Download, DownloadId, DownloadedBytes, FileInfo, TotalBytes,
    },
};

pub struct AppState {
    pub broadcast_tx: broadcast::Sender<AppEvent>,
    pub pool: SqlitePool,
}

impl AppState {
    pub async fn new(broadcast_tx: broadcast::Sender<AppEvent>) -> Self {
        let db_url = Self::get_db_url();
        let pool = init_db(&db_url).await.expect("init db error");
        Self { broadcast_tx, pool }
    }

    fn get_db_url() -> String {
        std::env::var("DATABASE_URL").unwrap_or("sqlite://./app.db?mode=rwc".to_string())
    }
}

#[derive(Clone, Debug)]
pub enum AppEvent {
    StartNewDownloadProcess(FileInfo, ChunkCount),
    CreateNewDownloadRecordInDB(FileInfo, ChunkCount),
    InsertDownloadFromDBToDownloadingList(DownloadId),
    CreateDownloadChunkInDB(DownloadId, TotalBytes, ChunkCount),
    StartDownload(DownloadId, Download),
    SendDownloadList,

    UpdateChunk(DownloadId, ChunkIndex, DownloadedBytes, String),
    DownloadFinished(DownloadId),

    PauseDownload(DownloadId),
    ResumeDownload(DownloadId),

    ValidateExistingFile(DownloadId, Vec<Chunk>),

    // MakeChunkHash(u64, Chunk),
    ReportChunkReceivedBytes(DownloadId, u64, u64),

    WorkerFinished(DownloadId),
    WorkerPaused(DownloadId),
}
