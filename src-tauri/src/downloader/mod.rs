mod validate;
pub use validate::validate_and_inspect_url;

mod utils;
use utils::calc_download_speed;
pub use utils::get_chunk_ranges;

use futures_util::{future::join_all, StreamExt};
use std::{
    io::SeekFrom,
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
    let _ = results.iter().all(|f| f.is_ok());

    Ok(())
}

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
    let mut last_time_chunk_downloaded = Instant::now();
    let mut speed_kbps = 0f64;
    let mut file_guard = file.lock().await;

    while let Some(bytes) = stream.next().await {
        let bytes = bytes.map_err(|e| e.to_string())?;
        let chunk_len = bytes.len() as u64;

        file_guard
            .seek(SeekFrom::Start(chunk.start_byte as u64 + downloaded))
            .await
            .map_err(|e| e.to_string())?;

        file_guard
            .write_all(&bytes)
            .await
            .map_err(|e| e.to_string())?;

        downloaded += chunk_len;
        received_bytes += chunk_len;

        calc_download_speed(
            &mut last_time_chunk_downloaded,
            &mut received_bytes,
            &mut speed_kbps,
        );

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
