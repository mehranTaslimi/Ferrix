use sqlx::SqlitePool;
use tokio::sync::broadcast;

use crate::{
    db::init_db,
    models::{ChunkCount, DownloadId, FileInfo},
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
        if cfg!(debug_assertions) {
            format!(
                "{}",
                std::env::var("DATABASE_URL").unwrap_or("sqlite://./app.db?mode=rwc".to_string())
            )
        } else {
            let db_path = dirs_next::data_local_dir()
                .expect("no data dir")
                .join("ferrix")
                .join("app.db?mode=rwc");

            std::fs::create_dir_all(db_path.parent().unwrap()).expect("failed to create db dir");

            format!("sqlite:{}", db_path.display())
        }
    }
}

#[derive(Clone, Debug)]
pub enum AppEvent {
    StartNewDownload(FileInfo, ChunkCount),
    PauseDownload(DownloadId),
    ResumeDownload(DownloadId),
    DownloadFinished(DownloadId),
}
