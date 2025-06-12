mod validate;
pub use validate::validate_and_inspect_url;

mod utils;
use utils::calc_download_speed;
pub use utils::get_chunk_ranges;

mod file;
pub use file::compute_partial_hash;
use file::setup_file_write;

use futures_util::{future::join_all, StreamExt};
use std::time::Duration;
use tauri::http::header::RANGE;
use tauri_plugin_http::reqwest::Client;
use tokio::{
    sync::{broadcast::Sender, mpsc},
    time::{sleep, Instant},
};

use crate::{
    downloader::file::WriteMessage,
    models::{Chunk, Download},
    utils::{app_state::AppEvent, broadcast_event::dispatch},
};

enum DownloadChunkStatus {
    Finished,
    Paused,
}

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

    let chunk_results = join_all(futures).await;

    let all_finished = chunk_results
        .iter()
        .all(|f| matches!(f, Ok(DownloadChunkStatus::Finished)));

    let all_paused = chunk_results
        .iter()
        .all(|f| matches!(f, Ok(DownloadChunkStatus::Paused)));

    if all_finished {
        dispatch(&tx, AppEvent::DownloadFinished(download.id))?;
        Ok(())
    } else if all_paused {
        return Ok(());
    } else {
        return Err("Some chunks failed".to_string());
    }
}

async fn download_chunk(
    tx: Sender<AppEvent>,
    file_tx: mpsc::Sender<WriteMessage>,
    chunk: Chunk,
) -> Result<DownloadChunkStatus, String> {
    let Chunk {
        downloaded_bytes,
        chunk_index,
        download_id,
        end_byte,
        start_byte,
        url,
        file_path: _,
        expected_hash: _,
    } = chunk.clone();

    let start_byte = start_byte + downloaded_bytes;

    let download_id = download_id.parse::<i64>().map_err(|e| e.to_string())?;
    let client = Client::new();

    let range_header = format!("bytes={}-{}", start_byte, end_byte);

    let response = client
        .get(url)
        .header(RANGE, range_header)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let mut rx = tx.subscribe();
    let mut stream = response.bytes_stream();
    let mut downloaded = downloaded_bytes as u64;
    let mut received_bytes = 0u64;
    let mut last_time_chunk_downloaded = Instant::now();
    let mut speed_kbps = 0f64;

    let download_report_interval = sleep(Duration::from_millis(100));
    tokio::pin!(download_report_interval);

    let internet_speed_report_interval = sleep(Duration::from_millis(1000));
    tokio::pin!(internet_speed_report_interval);

    loop {
        tokio::select! {
            Ok(app_event) = rx.recv() => {
                match app_event {
                    AppEvent::PauseDownload(id) if id == download_id => {
                        dispatch(
                            &tx,
                            AppEvent::MakeChunkHash(downloaded, chunk.clone()),
                        )?;
                        return Ok(DownloadChunkStatus::Paused);
                    },
                    _ => {}
                }
            }
            maybe_chunk = stream.next() => {
                match maybe_chunk {
                    Some(Ok(bytes)) => {
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
                    },
                    Some(Err(e)) => {
                        return Err(e.to_string());
                    },
                    None => {
                        break;
                    }
                }
            }
            () = &mut download_report_interval => {

                let _ = dispatch(
                    &tx,
                    AppEvent::ReportChunkDownloadedBytes(download_id, chunk_index, downloaded as i64),
                );
                download_report_interval.as_mut().reset(Instant::now() + Duration::from_millis(500));
            }
            () = &mut internet_speed_report_interval => {
                let _ = dispatch(
                    &tx,
                    AppEvent::ReportChunkSpeed(download_id, chunk_index, speed_kbps),
                );
                internet_speed_report_interval.as_mut().reset(Instant::now() + Duration::from_millis(1000));
            }
        }
    }

    Ok(DownloadChunkStatus::Finished)
}
