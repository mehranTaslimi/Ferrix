use std::{
    collections::VecDeque,
    sync::{atomic::AtomicUsize, Arc},
};

use once_cell::sync::OnceCell;
use sqlx::SqlitePool;

mod chunk;
mod dispatch;
mod download;
mod pool;
mod task;

pub use dispatch::RegistryAction;
pub use download::DownloadOptions;
use tauri::AppHandle;
use tokio::sync::{Mutex, Semaphore};

use crate::manager::DownloadsManager;

#[derive(Debug)]
pub struct State {
    pub pool: SqlitePool,
    pub current_tasks: Arc<Semaphore>,
    pub app_handle: AppHandle,
    pub available_permits: Arc<AtomicUsize>,
    pub download_queue: Arc<Mutex<VecDeque<i64>>>,
}

static STATE: OnceCell<Arc<State>> = OnceCell::new();

pub struct Registry;

impl Registry {
    pub async fn new(app_handle: AppHandle) {
        let pool = Self::init_db().await;
        let max_concurrent_tasks = Self::detect_max_concurrent_tasks();
        let current_tasks = Arc::new(Semaphore::new(max_concurrent_tasks));
        let available_permits = Arc::new(AtomicUsize::new(0));
        let download_queue = Arc::new(Mutex::new(VecDeque::new()));

        let state = Arc::new(State {
            pool,
            app_handle,
            current_tasks,
            available_permits,
            download_queue,
        });

        STATE.set(state).expect("initialized STATE error");

        DownloadsManager::monitor();
    }

    pub fn get_state() -> &'static Arc<State> {
        STATE.get().expect("STATE not initialized")
    }
}
