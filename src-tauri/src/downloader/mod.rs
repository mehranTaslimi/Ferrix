use futures_util::{future::join_all, StreamExt};
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use tauri::http::header::RANGE;
use tauri_plugin_http::reqwest::Client;
use tokio::{
    fs::{File, OpenOptions},
    io::{AsyncSeekExt, AsyncWriteExt},
    sync::{broadcast::Sender, Mutex},
};

use crate::{
    models::{Chunk, Download},
    utils::{app_state::AppEvent, broadcast_event::dispatch},
};

pub async fn get_file_content_length(url: &str) -> Result<u64, String> {
    let client = Client::new();

    client
        .head(url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .headers()
        .get("content-length")
        .and_then(|f| f.to_str().ok())
        .and_then(|f| f.parse::<u64>().ok())
        .ok_or("get content length error".to_string())
}

pub fn get_chunk_ranges(content_length: u64, chunk: u8) -> Result<Vec<(u64, u64)>, String> {
    let chunk = chunk as u64;
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

    Ok(ranges)
}

pub async fn download_chunks(
    tx: Sender<AppEvent>,
    chunks: Vec<Chunk>,
    download: Download,
) -> Result<(), String> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&download.file_path)
        .await
        .map_err(|e| format!("file creation failed: {}", e))?;

    file.set_len(download.total_bytes as u64)
        .await
        .map_err(|e| format!("file allocation failed: {}", e))?;

    let shared_file = Arc::new(Mutex::new(file));

    let futures = chunks.into_iter().map(|chunk| {
        let tx = tx.clone();
        let file_clone = Arc::clone(&shared_file);
        async move { download_chunk(tx, chunk, file_clone).await }
    });

    let results = join_all(futures).await;

    println!("{:?}", results);

    Ok(())
}

pub async fn validate_url() {}

async fn download_chunk(
    tx: Sender<AppEvent>,
    chunk: Chunk,
    file: Arc<Mutex<File>>,
) -> Result<(), String> {
    let Chunk {
        downloaded_bytes: _,
        chunk_index,
        download_id,
        end_byte,
        start_byte,
        url,
    } = chunk;
    let download_id = download_id.parse::<i64>().map_err(|e| e.to_string())?;
    let client = Client::new();

    let range_header = format!("bytes={}-{}", start_byte, end_byte);

    let response = client
        .get(url)
        .header(RANGE, range_header)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let mut last_send_event = Instant::now();
    let mut stream = response.bytes_stream();
    let mut downloaded = 0u64;
    let mut received_bytes = 0u64;
    let mut last_downloaded_chunk = Instant::now();
    let mut speed_kbps = 0f64;
    let mut file_guard = file.lock().await;

    while let Some(bytes) = stream.next().await {
        let bytes = bytes.map_err(|e| e.to_string())?;
        let now = Instant::now();
        let chunk_len = bytes.len() as u64;

        file_guard
            .seek(std::io::SeekFrom::Start(
                chunk.start_byte as u64 + downloaded,
            ))
            .await
            .map_err(|e| e.to_string())?;

        file_guard
            .write_all(&bytes)
            .await
            .map_err(|e| e.to_string())?;

        downloaded += chunk_len;
        received_bytes += chunk_len;

        let elapsed = now.duration_since(last_downloaded_chunk).as_secs_f64();
        speed_kbps = if elapsed >= 0.001 {
            (received_bytes as f64 / elapsed) / 1024.0
        } else {
            speed_kbps
        };
        last_downloaded_chunk = Instant::now();
        received_bytes = 0;

        if last_send_event.elapsed() >= Duration::from_secs(1) {
            dispatch(
                &tx,
                AppEvent::ReportChunkSpeed(download_id, chunk_index, speed_kbps),
            )?;
            dispatch(
                &tx,
                AppEvent::ReportChunkDownloadedBytes(download_id, chunk_index, downloaded as i64),
            )?;
            last_send_event = Instant::now();
        }
    }

    Ok(())
}
