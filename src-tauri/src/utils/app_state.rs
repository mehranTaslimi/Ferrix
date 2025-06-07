use sqlx::SqlitePool;
use tokio::sync::broadcast;

use crate::db::init_db;

pub struct AppState {
    pub tx: broadcast::Sender<AppEvent>,
    pub pool: SqlitePool,
}

impl AppState {
    pub async fn new(db_url: String) -> Self {
        let (tx, _) = broadcast::channel(100);
        let pool = init_db(&db_url).await.expect("init db error");
        Self { tx, pool }
    }
}

#[derive(Clone, Debug)]
pub enum AppEvent {
    CancelDownload(String),
}
