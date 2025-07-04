// use std::{
//     sync::Arc,
//     time::{Duration, Instant},
// };
// use tokio::{sync::Mutex, time::sleep};

// #[derive(Clone, Debug)]
// pub(super) struct Limiter {
//     pub(super) duration: Arc<Mutex<Option<Instant>>>,
//     pub(super) downloaded_bytes: Arc<Mutex<u32>>,
// }

// impl super::DownloadWorker {
//     pub(super) async fn limiter(&self, bytes_len: u32) {
//         let mut duration = self.limiter.duration.lock().await;
//         let mut downloaded_bytes = self.limiter.downloaded_bytes.lock().await;
//         let bandwidth_limit = *self.bandwidth_limit.lock().await;

//         if duration.is_none() {
//             *duration = Some(Instant::now());
//         }

//         if bandwidth_limit <= *downloaded_bytes as f32 && *downloaded_bytes > 0 {
//             if let Some(instant) = *duration {
//                 let time_overflow = 1.0 - instant.elapsed().as_secs_f32();
//                 let bytes_overflow = 1.0 - (bandwidth_limit / *downloaded_bytes as f32);
//                 if time_overflow < 1.0 && time_overflow > 0.0 {
//                     let added_time = time_overflow * (bytes_overflow / 100.0);
//                     let final_delay = time_overflow + added_time;
//                     sleep(Duration::from_secs_f32(final_delay)).await;
//                 }
//             };

//             *duration = Some(Instant::now());
//             *downloaded_bytes = 0;
//         }

//         if bandwidth_limit > 0.0 {
//             *downloaded_bytes += bytes_len;
//         }
//     }
// }
