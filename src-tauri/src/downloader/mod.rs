mod validate;
pub use validate::validate_and_inspect_url;

mod utils;
pub use utils::get_chunk_ranges;

mod file;
pub use file::compute_partial_hash;
use file::setup_file_write;

use futures_util::{future::join_all, StreamExt};
use tauri::http::header::RANGE;
use tauri_plugin_http::reqwest::Client;
use tokio::sync::{broadcast::Sender, mpsc};

use crate::{
    downloader::file::WriteMessage,
    events::dispatch,
    models::{Chunk, Download},
    utils::app_state::AppEvent,
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
        dispatch(&tx, AppEvent::DownloadFinished(download.id));
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
        chunk_index: _,
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

    loop {
        tokio::select! {
            Ok(app_event) = rx.recv() => {
                match app_event {
                    AppEvent::PauseDownload(id) if id == download_id => {
                        dispatch(
                            &tx,
                            AppEvent::MakeChunkHash(downloaded, chunk.clone()),
                        );
                        return Ok(DownloadChunkStatus::Paused);
                    },
                    _ => {}
                }
            }
            maybe_bytes = stream.next() => {
                match maybe_bytes {
                    Some(Ok(bytes)) => {
                        // println!("{:?}", "downloading...");
                        let chunk_len = bytes.len() as u64;

                        file_tx
                            .send((start_byte as u64 + downloaded, bytes.to_vec()))
                            .await
                            .map_err(|e| e.to_string())?;

                        downloaded += chunk_len;

                        dispatch(&tx, AppEvent::ReportChunkReceivedBytes(download_id, chunk_len));

                    },
                    Some(Err(e)) => {
                        return Err(e.to_string());
                    },
                    None => {
                        return Ok(DownloadChunkStatus::Finished);
                    }
                }
            }
        }
    }
}
