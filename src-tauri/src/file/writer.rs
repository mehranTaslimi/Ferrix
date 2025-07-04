use crate::registry::Registry;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::fs;
use tokio::fs::OpenOptions;
use tokio::io::{AsyncSeekExt, AsyncWriteExt, SeekFrom};
use tokio::sync::mpsc;

pub type WriteMessage = (u64, u64, u64, Vec<u8>);

impl super::File {
    pub(super) async fn setup_file_writer(
        download_id: i64,
        file_path: &str,
        total_bytes: u64,
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

        let report = Arc::clone(&Registry::get_state().report);

        let (tx, mut rx) = mpsc::unbounded_channel::<WriteMessage>();

        let task_name = format!("file writer: {}", file_path);

        Registry::spawn(&task_name, async move {
            while let Some((chunk_index, start_byte, downloaded_bytes, bytes)) = rx.recv().await {
                file.seek(SeekFrom::Start(start_byte)).await.unwrap();
                file.write_all(&bytes).await.unwrap();

                if let Some(report) = report.get(&download_id) {
                    report
                        .chunks_wrote_bytes
                        .entry(chunk_index as i64)
                        .and_modify(|atomic| {
                            atomic.fetch_add(downloaded_bytes, Ordering::Relaxed);
                        })
                        .or_insert(AtomicU64::new(downloaded_bytes));

                    report
                        .total_wrote_bytes
                        .fetch_add(downloaded_bytes, Ordering::Relaxed);

                    report
                        .wrote_bytes
                        .fetch_add(downloaded_bytes, Ordering::Relaxed);
                }
            }
        });

        Ok(tx)
    }
}
