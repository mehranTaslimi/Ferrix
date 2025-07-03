use std::{sync::Arc, time::Duration};

use tokio::time::sleep;

use crate::registry::Registry;

impl super::DownloadsManager {
    pub fn monitor() {
        let download_queue = Arc::clone(&Registry::get_state().download_queue);
        Registry::spawn("manager_monitor", async move {
            loop {
                let download_queue = download_queue.lock().await;

                if download_queue.len() > 0 {
                    println!("{:?}", download_queue);
                }

                sleep(Duration::from_secs(1)).await;
            }
        });
    }
}
