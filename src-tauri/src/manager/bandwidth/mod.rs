use crate::{
    manager::{task::TaskManager, WorkerData},
    models::DownloadId,
};
use std::{collections::HashMap, sync::Arc, u32};
use tokio::sync::Mutex;
mod monitor;

pub struct BandwidthManager {
    workers: Arc<Mutex<HashMap<DownloadId, WorkerData>>>,
    prev_bps: Arc<Mutex<u32>>,
    task: Arc<TaskManager>,
    pub bandwidth_limit: Arc<Mutex<f32>>,
}

impl BandwidthManager {
    pub fn new(
        workers: Arc<Mutex<HashMap<DownloadId, WorkerData>>>,
        task: Arc<TaskManager>,
    ) -> Self {
        let bandwidth = Self {
            workers,
            task,
            prev_bps: Arc::new(Mutex::new(0)),
            bandwidth_limit: Arc::new(Mutex::new(f32::MAX)),
        };

        bandwidth.monitor_bandwidth();

        bandwidth
    }
}
