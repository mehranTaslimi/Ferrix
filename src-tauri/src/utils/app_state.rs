use sqlx::SqlitePool;
use tokio::sync::broadcast;

use crate::{
    db::init_db,
    models::{
        ChunkCount, ChunkIndex, Download, DownloadId, DownloadUrl, DownloadedBytes, SpeedKbps,
        TotalBytes,
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
    StartNewDownloadProcess(DownloadUrl, ChunkCount),
    ValidateLink(DownloadUrl, ChunkCount),
    GetFileTotalBytes(DownloadUrl, ChunkCount),
    CreateNewDownloadRecordInDB(DownloadUrl, ChunkCount, TotalBytes),
    InsertDownloadFromDBToDownloadingList(DownloadId),
    CreateDownloadChunkInDB(DownloadId, TotalBytes, ChunkCount),
    StartDownload(DownloadId, Download),
    SendDownloadList,
    ReportChunkDownloadedBytes(DownloadId, ChunkIndex, DownloadedBytes),
    ReportChunksDownloadedBytes(DownloadId, DownloadedBytes),
    ReportChunkSpeed(DownloadId, ChunkIndex, SpeedKbps),
    ReportChunksSpeed(DownloadId, SpeedKbps),
    UpdateChunkDownloadedBytes(DownloadId, ChunkIndex, DownloadedBytes),
}
