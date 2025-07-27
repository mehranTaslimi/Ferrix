use crate::registry::Registry;

use std::{
    sync::{atomic::Ordering, Arc},
    time::{Duration, Instant},
};
use tokio::time::sleep;

impl super::DownloadWorker {
    pub async fn limiter(&self, bytes_len: u64) {
        let bandwidth_limit =
            Arc::clone(&Registry::get_state().bandwidth_limit).load(Ordering::Relaxed);

        if bandwidth_limit == 0.0 {
            return;
        }

        let mut received_bytes = self.received_bytes.lock().await;
        let mut duration = self.stream_duration.lock().await;

        if duration.is_none() {
            *duration = Some(Instant::now());
        }

        if bandwidth_limit <= *received_bytes as f64 {
            if let Some(instant) = *duration {
                let time_overflow = 1.0 - instant.elapsed().as_secs_f32();
                let bytes_overflow = 1.0 - ((bandwidth_limit as f32) / (*received_bytes as f32));
                if time_overflow < 1.0 && time_overflow > 0.0 {
                    let added_time = time_overflow * (bytes_overflow / 100.0);
                    let final_delay = time_overflow + added_time;
                    sleep(Duration::from_secs_f32(final_delay)).await;
                }
            };

            *duration = Some(Instant::now());
            *received_bytes = 0;
        }

        *received_bytes += bytes_len;

        if let Some(instant) = *duration {
            if instant.elapsed().as_secs_f32() > 1.0 {
                *received_bytes = 0;
                *duration = Some(Instant::now());
            }
        }
    }
}
