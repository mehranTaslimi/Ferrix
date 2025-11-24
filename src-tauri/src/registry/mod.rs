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

pub use actions::TaskStatus;
pub use event::RegistryAction;

#[derive(Debug)]
pub struct Buffer {
    pub first: BytesMut,
    pub last: BytesMut,
}

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
    pub buffer: Arc<DashMap<i64, Arc<Mutex<Buffer>>>>,
}

#[derive(Debug)]
pub struct State {
    pub pool: SqlitePool,
    pub current_tasks: Semaphore,
    pub tasks: DashMap<u64, actions::Task>,
    pub task_id: AtomicU64,
    pub app_handle: AppHandle,
    pub available_permits: AtomicUsize,
    pub pending_queue: Mutex<VecDeque<i64>>,
    pub workers: DashMap<i64, Arc<RwLock<Worker>>>,
    pub reports: DashMap<i64, Arc<Report>>,
    pub monitor_running: AtomicBool,
    pub bandwidth_limit: AtomicF64,
    pub download_speed: AtomicF64,
    pub spawn_cancellation_token: CancellationToken,
    queue_listener_running: AtomicBool,
    mpsc_sender: mpsc::UnboundedSender<RegistryAction>,
    manager: OnceCell<DownloadsManager>,
}

static STATE: OnceCell<Arc<State>> = OnceCell::new();

pub struct Registry;

impl Registry {
    pub async fn new(app_handle: AppHandle) {
        let max_concurrent_tasks = Self::detect_max_concurrent_tasks();
        let current_tasks = Semaphore::new(max_concurrent_tasks);
        let available_permits = AtomicUsize::new(0);
        let pending_queue = Mutex::new(VecDeque::new());
        let workers = DashMap::new();
        let app_handle = app_handle;
        let pool = Self::init_db(&app_handle.path().app_data_dir()).await;
        let reports = DashMap::new();
        let (tx, rx) = mpsc::unbounded_channel::<RegistryAction>();
        let mpsc_sender = tx;
        let queue_listener_running = AtomicBool::new(false);
        let manager = OnceCell::new();
        let monitor_running = AtomicBool::new(false);
        let bandwidth_limit = AtomicF64::new(0.0);
        let spawn_cancellation_token = CancellationToken::new();
        let download_speed = AtomicF64::new(0.0);
        let tasks = DashMap::new();
        let task_id = AtomicU64::new(0);

        let state = Arc::new(State {
            pool,
            tasks,
            reports,
            workers,
            task_id,
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
        if let Err(_) = Self::get_state_ref().manager.set(manager) {
            Emitter::emit_error("manager is already initialized");
        }
    }

    pub fn get_state_ref() -> &'static Arc<State> {
        STATE.get().expect("STATE not initialized")
    }

    pub fn get_state() -> Arc<State> {
        Arc::clone(Self::get_state_ref())
    }

    pub fn get_manager() -> DownloadsManager {
        Self::get_state_ref()
            .manager
            .get()
            .expect("manager not initialized")
            .clone()
    }

    fn detect_max_concurrent_tasks() -> usize {
        num_cpus::get() * 10
    }
}
