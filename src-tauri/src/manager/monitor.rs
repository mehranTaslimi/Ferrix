use crate::monitors_spawn;
use std::time::Duration;

impl super::DownloadsManager {
    pub(super) fn start_monitoring() {
        monitors_spawn!(
            ("report_downloaded_bytes", Duration::from_millis(100), {
                Self::report_downloaded_bytes();
            }),
            ("monitor_download_speed", Duration::from_secs(1), {
                Self::monitor_download_speed().await;
            }),
            ("report_network_speed", Duration::from_secs(1), {
                Self::report_network_speed().await;
            }),
            ("report_disk_speed", Duration::from_secs(1), {
                Self::report_disk_speed().await;
            }),
            ("update_chunks_monitor", Duration::from_secs(5), {
                Self::update_chunks_monitor().await;
            })
        );
    }
}
