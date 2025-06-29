use std::{collections::HashMap, sync::Arc, time::Duration};

use serde::{Deserialize, Serialize};
use tokio::{spawn, time::sleep};

use crate::events::emit_app_event;

#[derive(Clone, Serialize, Deserialize)]
pub(super) struct SpeedAndRemaining {
    speed: f64,
    remaining_time: f64,
}

#[derive(Debug, Clone)]
pub(super) struct InternetReport {
    pub(super) downloaded_bytes: u64,
    pub(super) received_bytes: f64,
}

#[derive(Debug, Clone)]
pub(super) struct DiskReport {
    pub(super) wrote_bytes: u64,
    pub(super) received_bytes: u64,
    pub(super) chunks: HashMap<u64, u64>,
}

impl super::DownloadWorker {
    pub(super) async fn listen_to_report_internet(&self) {
        let download_id = self.download.lock().await.id.clone();
        let total_bytes = self.download.lock().await.total_bytes.clone();

        let cancellation_token = self.cancellation_token.clone();
        let app_handle = self.app_handle.clone();
        let report = Arc::clone(&self.internet_report);

        spawn(async move {
            loop {
                if cancellation_token.is_cancelled() {
                    break;
                }

                sleep(Duration::from_millis(100)).await;

                let report = report.lock().await;

                let event = format!("downloaded_bytes_{}", download_id);

                emit_app_event(&app_handle, &event, report.downloaded_bytes);
            }
        });

        let cancellation_token = self.cancellation_token.clone();
        let app_handle = self.app_handle.clone();
        let report = Arc::clone(&self.internet_report);
        let speed_bps = Arc::clone(&self.speed_bps);

        spawn(async move {
            loop {
                if cancellation_token.is_cancelled() {
                    break;
                }

                sleep(Duration::from_secs(1)).await;

                let mut report = report.lock().await;

                *speed_bps.lock().await = report.received_bytes as u64;

                let speed = report.received_bytes as f64 / 1024.0;

                let remaining_time = total_bytes.saturating_sub(report.downloaded_bytes as i64)
                    as f64
                    / (speed * 1024.0);

                let event = format!("speed_and_remaining_{}", download_id);

                emit_app_event(
                    &app_handle,
                    &event,
                    SpeedAndRemaining {
                        speed,
                        remaining_time,
                    },
                );

                report.received_bytes = 0.0;
            }
        });
    }

    pub(super) async fn listen_to_report_disk(&self) {
        let cancellation_token = self.cancellation_token.clone();
        let disk_report = self.disk_report.clone();
        let app_handle = self.app_handle.clone();
        let download_id = self.download.lock().await.id.clone();
        spawn(async move {
            loop {
                if cancellation_token.is_cancelled() {
                    break;
                }

                sleep(Duration::from_secs(1)).await;

                let mut report = disk_report.lock().await;

                let speed = report.received_bytes as f64 / 1024.0;

                let disk_speed_event = format!("disk_speed_{}", download_id);
                let wrote_bytes_event = format!("wrote_bytes_{}", download_id);

                emit_app_event(&app_handle, &disk_speed_event, speed);
                emit_app_event(&app_handle, &wrote_bytes_event, report.wrote_bytes);

                report.received_bytes = 0;
            }
        });
    }
}
