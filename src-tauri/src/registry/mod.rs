use crate::{dispatch, emitter::Emitter, manager::DownloadsManager, spawn, worker::Worker};
use atomic_float::AtomicF64;
use dashmap::DashMap;
use log::debug;
use once_cell::sync::OnceCell;
use sqlx::SqlitePool;
use std::{
    collections::VecDeque,
    sync::{
        atomic::{AtomicBool, AtomicU64, AtomicUsize},
        Arc,
    },
    time::Instant,
};
use tauri::{AppHandle, Manager};
use tokio::sync::{
    mpsc::{self, UnboundedReceiver},
    Mutex, RwLock, Semaphore,
};
use tokio_util::{bytes::BytesMut, sync::CancellationToken};

mod actions;
mod event;
mod pool;

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
    pub last_update_downloaded_bytes: AtomicU64,
    pub last_update_time: Arc<Mutex<Instant>>,
    pub stable_speed: AtomicBool,
    pub buffer: Arc<DashMap<i64, Arc<Mutex<BytesMut>>>>,
}

#[derive(Debug)]
pub struct State {
    pub pool: SqlitePool,
    pub current_tasks: Arc<Semaphore>,
    pub app_handle: Arc<AppHandle>,
    pub available_permits: Arc<AtomicUsize>,
    pub pending_queue: Arc<Mutex<VecDeque<i64>>>,
    pub workers: Arc<DashMap<i64, Arc<RwLock<Worker>>>>,
    pub reports: Arc<DashMap<i64, Arc<Report>>>,
    pub monitor_running: Arc<AtomicBool>,
    pub bandwidth_limit: Arc<AtomicF64>,
    pub download_speed: Arc<AtomicF64>,
    pub spawn_cancellation_token: Arc<CancellationToken>,
    queue_listener_running: Arc<AtomicBool>,
    mpsc_sender: Arc<mpsc::UnboundedSender<RegistryAction>>,
    manager: OnceCell<Arc<DownloadsManager>>,
}

static STATE: OnceCell<Arc<State>> = OnceCell::new();

pub struct Registry;

impl Registry {
    pub async fn new(app_handle: AppHandle) {
        let max_concurrent_tasks = Self::detect_max_concurrent_tasks();
        let current_tasks = Arc::new(Semaphore::new(max_concurrent_tasks));
        let available_permits = Arc::new(AtomicUsize::new(0));
        let pending_queue = Arc::new(Mutex::new(VecDeque::new()));
        let workers = Arc::new(DashMap::new());
        let app_handle = Arc::new(app_handle);
        let pool = Self::init_db(&app_handle.path().app_data_dir()).await;
        let reports = Arc::new(DashMap::new());
        let (tx, rx) = mpsc::unbounded_channel::<RegistryAction>();
        let mpsc_sender = Arc::new(tx);
        let queue_listener_running = Arc::new(AtomicBool::new(false));
        let manager = OnceCell::new();
        let monitor_running = Arc::new(AtomicBool::new(false));
        let bandwidth_limit = Arc::new(AtomicF64::new(0.0));
        let spawn_cancellation_token = Arc::new(CancellationToken::new());
        let download_speed = Arc::new(AtomicF64::new(0.0));

        let state = Arc::new(State {
            pool,
            reports,
            workers,
            manager,
            app_handle,
            mpsc_sender,
            current_tasks,
            pending_queue,
            download_speed,
            monitor_running,
            bandwidth_limit,
            available_permits,
            queue_listener_running,
            spawn_cancellation_token,
        });

        STATE.set(state).unwrap();
        Self::initialize_mpsc_action(rx);
        Self::initialize_manager();

        dispatch!(registry, RecoverDownloads);
    }

    fn initialize_mpsc_action(mut rx: UnboundedReceiver<RegistryAction>) {
        spawn!("registry_mpsc", {
            while let Some(action) = rx.recv().await {
                if let Err(err) = Self::reducer(action).await {
                    debug!("{}", err.to_string());
                    Emitter::emit_error(err.to_string());
                }
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

    pub fn get_manager() -> &'static Arc<DownloadsManager> {
        Self::get_state()
            .manager
            .get()
            .expect("manager not initialized")
    }

    fn detect_max_concurrent_tasks() -> usize {
        num_cpus::get() * 10
    }
}
