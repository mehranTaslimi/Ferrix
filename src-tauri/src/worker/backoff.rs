use std::time::Duration;

use super::*;

impl DownloadWorker {
    pub(super) async fn backoff_delay(self: &Arc<Self>, retries: i64) -> Duration {
        let base = self.data.read().await.download.backoff_factor;
        let mut secs = base.powf(retries as f64);

        let jitter = fastrand::f64() * 0.25;
        secs *= 1.0 + jitter;

        Duration::from_secs_f64(secs)
    }
}
