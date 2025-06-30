use std::{sync::Arc, time::Duration};

use tokio::{spawn, time::sleep};

impl super::BandwidthManager {
    pub(super) fn monitor_bandwidth(&self) {
        let workers = Arc::clone(&self.workers);
        let bandwidth_limit = Arc::clone(&self.bandwidth_limit);
        let last_total_speed_bps = Arc::clone(&self.last_total_speed_bps);

        spawn(async move {
            loop {
                let workers = workers.lock().await;
                let mut last_total_speed_bps = last_total_speed_bps.lock().await;
                let download_count = workers.len();

                if download_count == 0 {
                    continue;
                }

                let chunk_count: u16 = workers.values().map(|f| f.chunk_count as u16).sum();

                let mut current_total_speed_bps = 0u32;

                for worker in workers.values() {
                    let bps = *worker.speed_bps.lock().await;
                    current_total_speed_bps += bps as u32;
                }

                if current_total_speed_bps > *last_total_speed_bps {
                    *last_total_speed_bps = current_total_speed_bps
                } else {
                    current_total_speed_bps = (*last_total_speed_bps as f32 * 1.1) as u32
                }

                *bandwidth_limit.lock().await = current_total_speed_bps as f32 / chunk_count as f32;

                sleep(Duration::from_secs(1)).await;
            }
        });
    }
}
