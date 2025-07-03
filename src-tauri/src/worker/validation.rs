use std::path::Path;

use tauri::{
    http::header::{ACCEPT_RANGES, CONTENT_DISPOSITION, CONTENT_LENGTH, CONTENT_TYPE},
    Url,
};
use tauri_plugin_http::reqwest::Client;

use crate::utils::path::{create_download_dir, get_available_filename};

impl super::DownloadWorker {
    pub(super) fn invalid_chunks_hash(file_path: &str, chunks: Vec<Chunk>) -> Vec<i64> {
        chunks
            .into_iter()
            .filter(|chunk| chunk.downloaded_bytes > 0)
            .filter(|chunk| {
                let hash = Self::compute_partial_hash(
                    file_path,
                    chunk.start_byte as u64,
                    chunk.downloaded_bytes as u64,
                )
                .ok();

                Some(hash) != Some(chunk.expected_hash.clone())
            })
            .map(|chunk| chunk.chunk_index)
            .collect::<Vec<i64>>()
    }
}
