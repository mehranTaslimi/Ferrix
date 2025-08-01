use serde::{Deserialize, Serialize};

use super::*;
use crate::{emitter::Emitter, registry::Registry};
use std::sync::{atomic::Ordering, Arc};

#[derive(Clone, Serialize, Deserialize)]
struct SpeedAndRemaining {
    speed: u64,
    remaining_time: u64,
}

impl DownloadsManager {
    pub(super) fn report_downloaded_bytes() {
        let reports = Arc::clone(&Registry::get_state().reports);

        reports.iter().for_each(|report| {
            let download_id = report.key();
            let event = format!("downloaded_bytes_{}", download_id);
            Emitter::emit_event(
                &event,
                report.total_downloaded_bytes.load(Ordering::Relaxed),
            );
        });
    }

    pub(super) async fn report_network_speed() {
        let reports = Arc::clone(&Registry::get_state().reports);

        for report in reports.iter() {
            let download_id = report.key();

            let mut history = report.download_history.lock().await;
            history.push_back(report.downloaded_bytes.swap(0, Ordering::Relaxed));

            if history.len() > 10 {
                history.pop_front();

                // Stable download speed
                if !report.stable_speed.load(Ordering::Relaxed) {
                    report.stable_speed.store(true, Ordering::Relaxed);
                }
            };

            let speed_history_avg = history.iter().sum::<u64>() / history.len() as u64;

            report.speed_bps.swap(speed_history_avg, Ordering::Relaxed);

            let speed = speed_history_avg / 1024;

            let total_downloaded_bytes = report.total_downloaded_bytes.load(Ordering::Relaxed);
            let remaining = report.total_bytes.saturating_sub(total_downloaded_bytes);

            let remaining_time = if speed_history_avg == 0 {
                0
            } else {
                remaining / speed_history_avg
            };

            let event = format!("speed_and_remaining_{}", download_id);
            Emitter::emit_event(
                &event,
                SpeedAndRemaining {
                    speed,
                    remaining_time,
                },
            );
        }
    }

    pub(super) async fn report_disk_speed() {
        let reports = Arc::clone(&Registry::get_state().reports);

        for report in reports.iter() {
            let download_id = report.key();

            let mut history = report.wrote_history.lock().await;
            history.push_back(report.wrote_bytes.swap(0, Ordering::Relaxed));

            if history.len() > 10 {
                history.pop_front();
            };

            let speed_history_avg = history.iter().sum::<u64>() / history.len() as u64;

            let speed = speed_history_avg / 1024;

            let disk_speed_event = format!("disk_speed_{}", download_id);
            let wrote_bytes_event = format!("wrote_bytes_{}", download_id);

            Emitter::emit_event(&disk_speed_event, speed);
            Emitter::emit_event(
                &wrote_bytes_event,
                report.total_wrote_bytes.load(Ordering::Relaxed),
            );
        }
    }
}
