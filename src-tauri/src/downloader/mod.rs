use futures_util::{future::join_all, StreamExt};
use std::time::{Duration, Instant};
use tauri::http::header::RANGE;
use tauri_plugin_http::reqwest::Client;
use tokio::sync::broadcast::Sender;

use crate::{
    models::DownloadChunk,
    utils::{app_state::AppEvent, event_handler::dispatch},
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
    let chunk_size: u64 = content_length / chunk as u64;

    let ranges: Vec<(u64, u64)> = (0..chunk)
        .map(|i| {
            let start = i as u64 * chunk_size;

            let end_index = (i + 1) as u64;
            let mut end = end_index * chunk_size;

            if (end_index as u8) == chunk {
                end += 1;
            } else {
                end -= 1;
            }

            (start, end)
        })
        .collect();

    Ok(ranges)
}

pub async fn download_chunks(
    tx: &Sender<AppEvent>,
    chunks: Vec<DownloadChunk>,
) -> Result<(), String> {
    let futures = chunks
        .into_iter()
        .map(|chunk| async move { download_chunk(&tx, chunk).await });

    let results = join_all(futures).await;

    println!("{:?}", results);

    Ok(())
}

async fn download_chunk(tx: &Sender<AppEvent>, chunk: DownloadChunk) -> Result<(), String> {
    let DownloadChunk {
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

    while let Some(bytes) = stream.next().await {
        let bytes = bytes.map_err(|e| e.to_string())?;
        let now = Instant::now();
        let chunk_len = bytes.len() as u64;
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
                AppEvent::SendDownloadSpeed(download_id, chunk_index, speed_kbps),
            )?;
            dispatch(
                &tx,
                AppEvent::UpdateDownloadedChunk(download_id, chunk_index, downloaded),
            )?;
            last_send_event = Instant::now();
        }
    }

    Ok(())
}
