use crate::registry::{Registry, RegistryAction};
use tokio::fs;
use tokio::fs::OpenOptions;
use tokio::io::{AsyncSeekExt, AsyncWriteExt, SeekFrom};
use tokio::sync::mpsc;

pub type WriteMessage = (i64, u64, u64, Vec<u8>);

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

        let (tx, mut rx) = mpsc::unbounded_channel::<WriteMessage>();

        Registry::spawn(async move {
            while let Some((chunk_index, start_byte, downloaded_bytes, bytes)) = rx.recv().await {
                file.seek(SeekFrom::Start(start_byte)).await.unwrap();
                file.write_all(&bytes).await.unwrap();
                file.flush().await.unwrap();

                Registry::dispatch(RegistryAction::UpdateDiskReport(
                    download_id,
                    chunk_index,
                    downloaded_bytes,
                ));
            }
        });

        Ok(tx)
    }
}
