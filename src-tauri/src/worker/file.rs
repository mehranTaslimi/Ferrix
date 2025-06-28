use std::sync::Arc;
use std::time::Duration;

use tokio::fs;
use tokio::io::{AsyncSeekExt, AsyncWriteExt, SeekFrom};
use tokio::sync::{mpsc, Mutex};
use tokio::time::sleep;
use tokio::{fs::OpenOptions, spawn};

use crate::worker::DiskReport;

pub type WriteMessage = (u64, u64, u64, Vec<u8>);

impl super::DownloadWorker {
    pub(super) async fn file_writer(
        file_path: &str,
        total_bytes: u64,
        report: Arc<Mutex<DiskReport>>,
    ) -> Result<mpsc::UnboundedSender<WriteMessage>, String> {
        let file_exists = fs::metadata(file_path).await.is_ok();

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path)
            .await
            .map_err(|e| e.to_string())?;

        if !file_exists {
            file.set_len(total_bytes as u64)
                .await
                .map_err(|e| e.to_string())?;
        }

        let (tx, mut rx) = mpsc::unbounded_channel::<WriteMessage>();

        spawn(async move {
            while let Some((chunk_index, start_byte, downloaded_bytes, bytes)) = rx.recv().await {
                file.seek(SeekFrom::Start(start_byte)).await.unwrap();
                file.write_all(&bytes).await.unwrap();

                let mut report = report.lock().await;
                report.wrote_bytes += downloaded_bytes;
                report.received_bytes += downloaded_bytes;
                report
                    .chunks
                    .entry(chunk_index)
                    .and_modify(|f| *f += downloaded_bytes)
                    .or_insert(downloaded_bytes);

                // Slow hard simulation
                println!("{downloaded_bytes}");
                sleep(Duration::from_millis(10)).await;
            }
        });

        Ok(tx)
    }
}
