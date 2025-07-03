use once_cell::sync::OnceCell;
use sqlx::SqlitePool;

mod dispatch;
mod download;
mod pool;

pub use dispatch::RegistryAction;
pub use download::DownloadOptions;

#[derive(Debug)]
pub struct State {
    pub pool: SqlitePool,
}

pub static STATE: OnceCell<&'static State> = OnceCell::new();

pub struct Registry;

impl Registry {
    pub async fn new() {
        let pool = Self::init_db().await;

        let state = Box::leak(Box::new(State { pool }));

        STATE.set(state).unwrap();
    }
}
