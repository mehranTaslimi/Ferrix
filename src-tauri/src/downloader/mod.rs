mod validate;
pub use validate::validate_and_inspect_url;

mod utils;
use utils::calc_download_speed;
pub use utils::get_chunk_ranges;

mod file;
use file::setup_file_write;

use futures_util::{future::join_all, StreamExt};
use std::time::{Duration, Instant};
use tauri::http::header::RANGE;
use tauri_plugin_http::reqwest::Client;
use tokio::sync::{broadcast::Sender, mpsc};

use crate::{
    downloader::file::WriteMessage,
    models::{Chunk, Download},
    utils::{app_state::AppEvent, broadcast_event::dispatch},
};

pub async fn download_chunks(
    tx: Sender<AppEvent>,
    chunks: Vec<Chunk>,
    download: Download,
) -> Result<(), String> {
    let file_tx = setup_file_write(&download.file_path, download.total_bytes as u64).await?;

    let futures = chunks.into_iter().map(|chunk| {
        let tx = tx.clone();
        let file_tx = file_tx.clone();
        async move { download_chunk(tx, file_tx, chunk).await }
    });

    let results = join_all(futures).await;
    let all_ok = results.iter().all(|f| f.is_ok());

    if all_ok {
        dispatch(&tx, AppEvent::DownloadFinished(download.id))?
    }

    Ok(())
}

async fn download_chunk(
    tx: Sender<AppEvent>,
    file_tx: mpsc::Sender<WriteMessage>,
    chunk: Chunk,
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

    loop {
        match stream.next().await {
            Some(bytes) => {
                let bytes = bytes.map_err(|e| e.to_string())?;
                let chunk_len = bytes.len() as u64;

                file_tx
                    .send((start_byte as u64 + downloaded, bytes.to_vec()))
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
                        AppEvent::ReportChunkDownloadedBytes(
                            download_id,
                            chunk_index,
                            downloaded as i64,
                        ),
                    )?;
                    last_send_event = Instant::now();
                }
            }
            None => {
                dispatch(
                    &tx,
                    AppEvent::ReportChunkSpeed(download_id, chunk_index, speed_kbps),
                )?;
                dispatch(
                    &tx,
                    AppEvent::ReportChunkDownloadedBytes(
                        download_id,
                        chunk_index,
                        downloaded as i64,
                    ),
                )?;
                break;
            }
        }
    }

    Ok(())
}
