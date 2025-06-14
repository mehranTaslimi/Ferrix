use std::{collections::HashMap, time::Duration};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};
use tokio::{
    spawn,
    sync::{broadcast::Sender, Mutex},
    time::sleep,
};

use crate::{models::DownloadId, utils::app_state::AppEvent};

#[derive(Clone, Debug)]
struct Report {
    total_bytes: u64,
    downloaded_bytes: u64,
    received_bytes: f64,
}

static REPORTER: Lazy<Mutex<HashMap<DownloadId, Report>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub fn dispatch(tx: &Sender<AppEvent>, app_event: AppEvent) {
    let _ = tx.send(app_event).map(|_| ());
}

pub fn emit_app_event<S: Serialize + Clone>(app_handle: &AppHandle, event: &str, payload: S) {
    let _ = app_handle.emit(event, payload);
}

#[derive(Clone)]
pub struct Reporter {
    app_handle: AppHandle,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SpeedAndRemaining {
    speed: f64,
    remaining_time: f64,
}

impl Reporter {
    pub fn new(app_handle: AppHandle) -> Self {
        Self { app_handle }
    }

    pub async fn flush_report(&self, download_id: DownloadId) {
        spawn(async move {
            let mut reporter = REPORTER.lock().await;
            reporter.remove(&download_id);
        });
    }

    pub async fn add_report(&self, download_id: DownloadId, received_bytes: u64, total_bytes: u64) {
        spawn(async move {
            let mut reporter = REPORTER.lock().await;
            reporter
                .entry(download_id)
                .and_modify(|report| {
                    report.downloaded_bytes += received_bytes;
                    report.received_bytes += received_bytes as f64;
                })
                .or_insert(Report {
                    total_bytes,
                    received_bytes: received_bytes as f64,
                    downloaded_bytes: received_bytes,
                });
        });
    }

    pub fn listen_to_report(&self) {
        let downloaded_bytes_handle = self.app_handle.clone();
        let speed_handle = self.app_handle.clone();
        spawn(async move {
            loop {
                sleep(Duration::from_millis(100)).await;

                let reporter = REPORTER.lock().await;
                let clone = reporter.clone();

                if clone.len() == 0 {
                    continue;
                }

                for (&id, report) in clone.iter() {
                    let event = format!("downloaded_bytes_{}", id);
                    emit_app_event(&downloaded_bytes_handle, &event, report.downloaded_bytes);
                }
            }
        });
        spawn(async move {
            loop {
                sleep(Duration::from_secs(1)).await;

                let mut reporter = REPORTER.lock().await;

                if reporter.len() == 0 {
                    continue;
                }

                for (&id, report) in reporter.iter_mut() {
                    let speed = report.received_bytes as f64 / 1024.0;
                    let remaining_time = report.total_bytes.saturating_sub(report.downloaded_bytes)
                        as f64
                        / (speed * 1024.0);

                    let event = format!("speed_and_remaining_{}", id);

                    emit_app_event(
                        &speed_handle,
                        &event,
                        SpeedAndRemaining {
                            speed,
                            remaining_time,
                        },
                    );

                    report.received_bytes = 0.0;
                }
            }
        });
    }
}
