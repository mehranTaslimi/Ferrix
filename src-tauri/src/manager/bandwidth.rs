use std::sync::{atomic::Ordering, Arc};

use crate::registry::Registry;

impl super::DownloadsManager {
    pub(super) async fn monitor_download_speed() {
        let workers = Arc::clone(&Registry::get_state().workers);
        let chunk_count = {
            let mut count = 1;
            for worker in workers.iter() {
                let worker = worker.read().await;
                count += worker.download.chunk_count
            }
            count as u64
        };

        let reports = Arc::clone(&Registry::get_state().reports);
        let bandwidth_limit = Arc::clone(&Registry::get_state().bandwidth_limit);

        let download_speed = reports
            .iter()
            .map(|report| report.speed_bps.load(Ordering::Relaxed))
            .sum::<u64>();

        bandwidth_limit.swap(download_speed / chunk_count, Ordering::Relaxed);
    }
}
