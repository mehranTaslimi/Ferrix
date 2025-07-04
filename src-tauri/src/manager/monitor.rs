use crate::{registry::Registry, worker::DownloadWorker};
use std::{sync::Arc, time::Duration};
use tokio::time::sleep;

impl super::DownloadsManager {
    pub(super) fn pending_queue_monitor() {
        let pending_queue = Arc::clone(&Registry::get_state().pending_queue);

        Registry::spawn("manager_monitor", async move {
            loop {
                let pending_queue = pending_queue.lock().await;

                if pending_queue.len() > 0 {
                    Registry::add_download_to_downloading_queue()
                }

                sleep(Duration::from_secs(1)).await;
            }
        });
    }

    pub(super) fn downloading_monitor() {
        let downloading_map = Arc::clone(&Registry::get_state().downloading_map);

        Registry::spawn("preparing_monitor", async move {
            loop {
                let downloading_map = downloading_map.lock().await;

                for (download_id, download_info) in downloading_map.iter() {
                    let mut download_info = download_info.lock().await;
                    if !download_info.worker_created {
                        download_info.worker_created = true;

                        let download_id = download_id.clone();

                        Registry::spawn("new_download_worker", async move {
                            DownloadWorker::new(download_id).await;
                        });

                        break;
                    }
                }

                sleep(Duration::from_secs(1)).await;
            }
        });
    }

    pub(super) fn reporting_monitor() {
        Registry::spawn("downloaded_bytes_report", async move {
            loop {
                sleep(Duration::from_millis(100)).await;
                Self::report_downloaded_bytes();
            }
        });
        Registry::spawn("speed_report", async move {
            loop {
                sleep(Duration::from_secs(1)).await;
                Self::report_network_speed().await;
            }
        });
        Registry::spawn("disk_report", async move {
            loop {
                sleep(Duration::from_secs(1)).await;
                Self::report_disk_speed().await;
            }
        });
    }
}
