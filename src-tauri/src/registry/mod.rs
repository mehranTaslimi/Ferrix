use crate::{emitter::Emitter, manager::DownloadsManager, worker::Worker};
use dashmap::DashMap;
use once_cell::sync::OnceCell;
use sqlx::SqlitePool;
use std::{
    collections::VecDeque,
    sync::{
        atomic::{AtomicBool, AtomicU64, AtomicUsize},
        Arc,
    },
};
use tauri::AppHandle;
use tokio::sync::{
    mpsc::{self, UnboundedReceiver},
    Mutex, Semaphore,
};

mod actions;
mod event;
mod pool;
mod task;

pub use event::RegistryAction;

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
    pub speed_bps: AtomicU64,
}

#[derive(Debug)]
pub struct State {
    pub pool: SqlitePool,
    pub current_tasks: Arc<Semaphore>,
    pub app_handle: Arc<AppHandle>,
    pub available_permits: Arc<AtomicUsize>,
    pub pending_queue: Arc<Mutex<VecDeque<i64>>>,
    pub workers: Arc<DashMap<i64, Arc<Mutex<Worker>>>>,
    pub report: Arc<DashMap<i64, Report>>,
    pub monitor_running: Arc<AtomicBool>,
    pub bandwidth_limit: Arc<AtomicU64>,
    queue_listener_running: Arc<AtomicBool>,
    mpsc_sender: Arc<mpsc::UnboundedSender<RegistryAction>>,
    manager: OnceCell<Arc<DownloadsManager>>,
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
        let workers = Arc::new(DashMap::new());
        let app_handle = Arc::new(app_handle);
        let report = Arc::new(DashMap::new());
        let (tx, rx) = mpsc::unbounded_channel::<RegistryAction>();
        let mpsc_sender = Arc::new(tx);
        let queue_listener_running = Arc::new(AtomicBool::new(false));
        let manager = OnceCell::new();
        let monitor_running = Arc::new(AtomicBool::new(false));
        let bandwidth_limit = Arc::new(AtomicU64::new(0));

        let state = Arc::new(State {
            pool,
            app_handle,
            current_tasks,
            available_permits,
            pending_queue,
            workers,
            report,
            mpsc_sender,
            queue_listener_running,
            manager,
            monitor_running,
            bandwidth_limit,
        });

        STATE.set(state).unwrap();
        Self::initialize_mpsc_action(rx);
        Self::initialize_manager();
    }

    fn initialize_mpsc_action(mut rx: UnboundedReceiver<RegistryAction>) {
        Self::spawn("registry_action", async move {
            while let Some(action) = rx.recv().await {
                Self::reducer(action).await
            }
        });
    }

    fn initialize_manager() {
        let manager = DownloadsManager::new();
        if let Err(_) = Self::get_state().manager.set(manager) {
            Emitter::emit_error("manager is already initialized");
        }
    }

    pub fn get_state() -> &'static Arc<State> {
        STATE.get().expect("STATE not initialized")
    }

    fn get_manager() -> &'static Arc<DownloadsManager> {
        Self::get_state()
            .manager
            .get()
            .expect("manager not initialized")
    }
}
