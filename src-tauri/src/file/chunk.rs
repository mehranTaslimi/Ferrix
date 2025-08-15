use super::*;
use crate::registry::{Buffer, Registry};

use anyhow::Context;
use dashmap::DashMap;
use std::{
    fs,
    io::{Read, Seek, SeekFrom},
    sync::Arc,
};
use tokio::sync::Mutex;
use tokio_util::bytes::BytesMut;

impl File {
    pub async fn get_chunks_bytes_from_file(
        download_id: i64,
    ) -> anyhow::Result<DashMap<i64, Arc<Mutex<Buffer>>>> {
        let workers = Arc::clone(&Registry::get_state().workers);
        let worker = workers
            .get(&download_id)
            .with_context(|| format!("worker not found for download id {}", download_id))?;
        let worker_lock = worker.read().await;

        let mut file = fs::File::open(&worker_lock.download.file_path)
            .with_context(|| format!("failed to open file {}", worker_lock.download.file_name))?;

        let buffers: DashMap<i64, Arc<Mutex<Buffer>>> = DashMap::new();

        for chunk in &worker_lock.chunks {
            let mut buffer = Buffer {
                first: BytesMut::with_capacity(1024),
                last: BytesMut::with_capacity(1024),
            };

            if chunk.downloaded_bytes < 2048 {
                buffers.insert(chunk.chunk_index, Arc::new(Mutex::new(buffer)));
                continue;
            }

            buffer.first.resize(1024, 0);
            buffer.last.resize(1024, 0);

            file.seek(SeekFrom::Start(chunk.start_byte as u64))
                .context("failed to seek to chunk start")?;
            file.read_exact(&mut buffer.first[..1024])
                .context("failed to read first 1024 bytes")?;

            let end_start = (chunk.start_byte + chunk.downloaded_bytes - 1024) as u64;

            file.seek(SeekFrom::Start(end_start))
                .context("failed to seek to chunk end region")?;

            file.read_exact(&mut buffer.last[..1024])
                .context("failed to read last 1024 bytes")?;

            buffers.insert(chunk.chunk_index, Arc::new(Mutex::new(buffer)));
        }

        Ok(buffers)
    }
}
