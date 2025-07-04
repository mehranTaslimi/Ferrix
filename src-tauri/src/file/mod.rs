use tokio::sync::mpsc;

mod writer;
pub use writer::WriteMessage;

#[derive(Clone, Debug)]
pub struct File;

impl File {
    pub async fn new(
        download_id: i64,
        file_path: &str,
        total_bytes: u64,
    ) -> Result<mpsc::UnboundedSender<WriteMessage>, String> {
        Self::setup_file_writer(download_id, file_path, total_bytes).await
    }
}
