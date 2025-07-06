// use crate::manager::WorkerData;
// use std::{collections::HashMap, sync::Arc, u32};
// use tokio::sync::Mutex;
// mod monitor;

// pub struct BandwidthManager {
//     workers: Arc<Mutex<HashMap<i64, WorkerData>>>,
//     prev_bps: Arc<Mutex<u32>>,
//     pub bandwidth_limit: Arc<Mutex<f32>>,
// }

// impl BandwidthManager {
//     pub fn new(workers: Arc<Mutex<HashMap<i64, WorkerData>>>) -> Self {
//         let bandwidth = Self {
//             workers,
//             prev_bps: Arc::new(Mutex::new(0)),
//             bandwidth_limit: Arc::new(Mutex::new(f32::MAX)),
//         };

//         bandwidth.monitor_bandwidth();

//         bandwidth
//     }
// }
