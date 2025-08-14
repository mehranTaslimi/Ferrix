use anyhow::anyhow;

use crate::manager::DownloadsManager;

use super::*;

impl DownloadWorker {
    pub(super) async fn validate_chunk(
        self: &Arc<Self>,
        current_hash: Option<String>,
        chunk_index: i64,
    ) -> anyhow::Result<()> {
        if let Some(current_hash) = current_hash {
            let hash = DownloadsManager::hash_chunk(self.download_id, chunk_index).await?;

            if hash.to_string() == current_hash {
                return Ok(());
            }

            return Err(anyhow!("hash not match"));
        }

        Ok(())
    }
}
