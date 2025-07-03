use std::{sync::Arc, time::Duration};

use tokio::time::sleep;

use crate::registry::Registry;

impl super::BandwidthManager {
    pub(super) fn monitor_bandwidth(&self) {
        let workers = Arc::clone(&self.workers);
        let bandwidth_limit = Arc::clone(&self.bandwidth_limit);
        let prev_bps = Arc::clone(&self.prev_bps);

        Registry::spawn("monitor bandwidth", async move {
            loop {
                let workers = workers.lock().await;
                let mut prev_bps = prev_bps.lock().await;
                let download_count = workers.len();

                if download_count == 0 {
                    continue;
                }

                let mut current_bps = 0u32;

                for worker in workers.values() {
                    let bps = *worker.speed_bps.lock().await;
                    current_bps += bps as u32;
                }

                if current_bps > *prev_bps {
                    *prev_bps = current_bps
                } else {
                    current_bps *= 2
                }

                *bandwidth_limit.lock().await = current_bps as f32 / download_count as f32;

                sleep(Duration::from_secs(1)).await;
            }
        });
    }
}
