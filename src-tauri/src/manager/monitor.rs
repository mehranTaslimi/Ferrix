use crate::registry::Registry;
use std::time::Duration;
use tokio::time::sleep;

impl super::DownloadsManager {
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
