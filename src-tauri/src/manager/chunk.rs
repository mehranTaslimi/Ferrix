use log::debug;
use md5::{Digest, Md5};
use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    sync::{atomic::Ordering, Arc},
};

use crate::{models::DownloadChunk, registry::Registry};

impl super::DownloadsManager {
    pub(super) fn get_chunk_ranges(content_length: u64, chunk_count: u64) -> Vec<(u64, u64)> {
        let chunk = chunk_count;
        let mut ranges = Vec::with_capacity(chunk as usize);

        let base_chunk_size = content_length / chunk;
        let remainder = content_length % chunk;

        let mut start = 0;

        for i in 0..chunk {
            let extra = if i < remainder { 1 } else { 0 };
            let end = start + base_chunk_size + extra - 1;

            ranges.push((start, end));
            start = end + 1;
        }

        ranges
    }

    // pub(super) async fn update_chunks_monitor() {
    //     let reports = Arc::clone(&Registry::get_state().reports);

    //     reports.iter().for_each(|r| {
    //         let total_downloaded_bytes = r.total_downloaded_bytes.load(Ordering::Relaxed);
    //         let last_chunk_percent = r.last_update_chunk_percent.load(Ordering::Relaxed);
    //         if total_downloaded_bytes == 0 {
    //             return;
    //         }

    //         let percent = ((total_downloaded_bytes * 100) / r.total_bytes) as u8;
    //         let is_checkpoint = percent % 5 == 0;

    //         if last_chunk_percent == percent || !is_checkpoint {
    //             return;
    //         }

    //         debug!("Updating chunk monitor for {}", percent);

    //         r.last_update_chunk_percent
    //             .store(percent, Ordering::Relaxed);
    //     });
    // }

    pub(super) fn compute_partial_hash(
        file_path: &str,
        start_byte: u64,
        wrote_bytes: u64,
    ) -> Result<String, String> {
        let mut file = File::open(file_path).map_err(|e| e.to_string())?;

        file.seek(SeekFrom::Start(start_byte))
            .map_err(|e| e.to_string())?;

        let mut hasher = Md5::new();
        let mut remaining = wrote_bytes as usize;
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

    pub fn validate_chunks_hash(file_path: &str, chunks: Vec<DownloadChunk>) -> Vec<i64> {
        chunks
            .into_iter()
            .filter(|chunk| chunk.downloaded_bytes > 0)
            .filter(|chunk| {
                let hash = Self::compute_partial_hash(
                    &file_path,
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
