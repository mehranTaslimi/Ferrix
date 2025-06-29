use crate::{manager::WorkerData, models::DownloadId};
use std::{collections::HashMap, sync::Arc, u32};
use tokio::sync::Mutex;
mod monitor;

pub struct BandwidthManager {
    workers: Arc<Mutex<HashMap<DownloadId, WorkerData>>>,
    last_total_speed_bps: Arc<Mutex<u32>>,
    // download_speed_limit: Arc<Mutex<u32>>,
    pub bandwidth_limit: Arc<Mutex<f32>>,
}

impl BandwidthManager {
    pub fn new(workers: Arc<Mutex<HashMap<DownloadId, WorkerData>>>) -> Self {
        let bandwidth = Self {
            workers,
            last_total_speed_bps: Arc::new(Mutex::new(u32::MAX)),
            bandwidth_limit: Arc::new(Mutex::new(0.0)),
            // download_speed_limit: Arc::new(Mutex::new(u32::MAX)),
        };

        bandwidth.monitor_bandwidth();

        bandwidth
    }
}
