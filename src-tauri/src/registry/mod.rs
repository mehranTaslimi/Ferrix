use std::{
    collections::{HashMap, VecDeque},
    sync::{
        atomic::{AtomicU64, AtomicUsize},
        Arc,
    },
};

use dashmap::DashMap;
use once_cell::sync::OnceCell;
use sqlx::SqlitePool;

mod chunk;
mod download;
mod pool;
mod task;

pub use download::DownloadOptions;
use tauri::AppHandle;
use tokio::sync::{Mutex, Semaphore};

use crate::{manager::DownloadsManager, models::DownloadWithChunk};

#[derive(Debug)]
pub struct Report {
    pub total_downloaded_bytes: AtomicU64,
    pub downloaded_bytes: AtomicU64,
    pub total_wrote_bytes: AtomicU64,
    pub wrote_bytes: AtomicU64,
    pub download_history: Mutex<VecDeque<u64>>,
    pub wrote_history: Mutex<VecDeque<u64>>,
    pub chunks_wrote_bytes: DashMap<i64, AtomicU64>,
    pub total_bytes: u64,
}

#[derive(Debug)]
pub struct State {
    pub pool: SqlitePool,
    pub current_tasks: Arc<Semaphore>,
    pub app_handle: Arc<AppHandle>,
    pub available_permits: Arc<AtomicUsize>,
    pub pending_queue: Arc<Mutex<VecDeque<i64>>>,
    pub downloading_map: Arc<Mutex<HashMap<i64, Arc<Mutex<DownloadWithChunk>>>>>,
    pub report: Arc<DashMap<i64, Report>>,
}

static STATE: OnceCell<Arc<State>> = OnceCell::new();

pub struct Registry;

impl Registry {
    pub async fn new(app_handle: AppHandle) {
        let pool = Self::init_db().await;
        let max_concurrent_tasks = Self::detect_max_concurrent_tasks();
        let current_tasks = Arc::new(Semaphore::new(max_concurrent_tasks));
        let available_permits = Arc::new(AtomicUsize::new(0));
        let pending_queue = Arc::new(Mutex::new(VecDeque::new()));
        let downloading_map = Arc::new(Mutex::new(HashMap::new()));
        let app_handle = Arc::new(app_handle);
        let report = Arc::new(DashMap::new());

        let state = Arc::new(State {
            pool,
            app_handle,
            current_tasks,
            available_permits,
            pending_queue,
            downloading_map,
            report,
        });

        STATE.set(state).expect("initialized STATE error");

        DownloadsManager::new();
    }

    pub fn get_state() -> &'static Arc<State> {
        STATE.get().expect("STATE not initialized")
    }
}
