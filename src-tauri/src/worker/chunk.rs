use crate::db::downloads::update_chunk_downloaded;
use md5::{Digest, Md5};
use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
};

impl super::DownloadWorker {
    pub(super) async fn update_chunk(
        &self,
        chunk_index: i64,
        has_error: bool,
        error_message: &str,
    ) -> Result<(), String> {
        let mut chunks_guard = self.chunks.lock().await;
        let chunk = chunks_guard
            .get_mut(chunk_index as usize)
            .ok_or_else(|| format!("chunk index {} not found", chunk_index))?;

        let downloaded_bytes = self
            .disk_report
            .lock()
            .await
            .chunks
            .get(&(chunk_index as u64))
            .cloned()
            .unwrap_or(0);

        let hash =
            Self::compute_partial_hash(&self.download.lock().await.file_path, 0, downloaded_bytes)?;
        update_chunk_downloaded(
            self.download_id,
            chunk_index,
            downloaded_bytes as i64,
            hash,
            has_error,
            error_message,
        )
        .await?;

        chunk.downloaded_bytes = downloaded_bytes as i64;
        chunk.has_error = Some(has_error);
        chunk.error_message = Some(error_message.to_string());

        Ok(())
    }

    pub(super) fn compute_partial_hash(
        file_path: &str,
        start_byte: u64,
        downloaded_bytes: u64,
    ) -> Result<String, String> {
        let mut file = File::open(file_path).map_err(|e| e.to_string())?;

        file.seek(SeekFrom::Start(start_byte))
            .map_err(|e| e.to_string())?;

        let mut hasher = Md5::new();
        let mut remaining = downloaded_bytes as usize;
        let mut buffer = vec![0; 8192];

        while remaining > 0 {
            let read_size = std::cmp::min(buffer.len(), remaining);
            let n = file
                .read(&mut buffer[..read_size])
                .map_err(|e| e.to_string())?;
            if n == 0 {
                break;
            }
            hasher.update(&buffer[..n]);
            remaining -= n;
        }

        Ok(format!("{:x}", hasher.finalize()))
    }

    pub(super) async fn not_downloaded_chunks(&self) -> impl Iterator<Item = Chunk> {
        self.chunks
            .lock()
            .await
            .clone()
            .into_iter()
            .filter(|chunk| chunk.downloaded_bytes < chunk.end_byte - chunk.start_byte)
    }
}
