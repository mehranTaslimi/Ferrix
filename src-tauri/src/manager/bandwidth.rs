use std::sync::{atomic::Ordering, Arc};

use crate::registry::Registry;

impl super::DownloadsManager {
    pub(super) async fn monitor_download_speed() {
        let reports = Arc::clone(&Registry::get_state().reports);
        let bandwidth_limit = Arc::clone(&Registry::get_state().bandwidth_limit);

        let download_len = reports.len().max(1) as u64;

        if download_len == 1 {
            if bandwidth_limit.load(Ordering::Relaxed) > 0.0 {
                bandwidth_limit.store(0.0, Ordering::Relaxed);
            }
            return;
        }

        let all_stable_speed = reports
            .iter()
            .all(|r| r.stable_speed.load(Ordering::Relaxed));

        if !all_stable_speed {
            if bandwidth_limit.load(Ordering::Relaxed) > 0.0 {
                bandwidth_limit.store(0.0, Ordering::Relaxed);
            }
            return;
        }

        let download_speed = Arc::clone(&Registry::get_state().download_speed);

        let current_speed = reports
            .iter()
            .map(|r| r.speed_bps.load(Ordering::Relaxed))
            .sum::<u64>() as f64;

        let mut last_speed = download_speed.load(Ordering::Relaxed);

        if current_speed > last_speed {
            last_speed = current_speed as f64;
        } else if current_speed < last_speed {
            last_speed = current_speed * 1.1;
        }

        download_speed.store(last_speed, Ordering::Relaxed);
        bandwidth_limit.store(current_speed / download_len as f64, Ordering::Relaxed);
    }
}
