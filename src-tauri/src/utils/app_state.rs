use sqlx::SqlitePool;
use tokio::sync::broadcast;

use crate::db::init_db;

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
pub struct DownloadData {
    pub url: String,
    pub chunk: u8,
}

#[derive(Clone, Debug)]
pub enum AppEvent {
    StartNewDownloadProcess(DownloadData),
    ValidateUrl(DownloadData),
    CreateNewDownloadRecord(DownloadData, u64),
    GetFileContentLength(DownloadData),
    CreateDownloadChunk(i64, u64, u8),
    UpdateDownloadedChunk(i64, i64, u64),
    StartDownload(i64),
    SendDownloadList,
    SendDownloadItemUpdate(i64, i64, u64),
}
