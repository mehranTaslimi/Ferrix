use std::{sync::Arc, time::Duration};

use futures_util::{future::join_all, StreamExt};
use tauri::http::header::RANGE;
use tauri_plugin_http::reqwest::Client;
use tokio::time::timeout;

use crate::{
    file::WriteMessage, manager::ManagerAction, models::DownloadChunk, registry::Registry,
    worker::outcome::DownloadStatus,
};

impl super::DownloadWorker {
    pub async fn start_download(
        &self,
    ) -> Vec<Result<Result<DownloadStatus, i64>, tokio::task::JoinError>> {
        let mut futures = self.not_downloaded_chunks().await.map(|chunk| {
            let self_clone = self.clone();
            let task_name = format!(
                "chunk download: {}, index: {}",
                self_clone.download_id, chunk.chunk_index
            );
            Registry::spawn(
                &task_name,
                async move { self_clone.download_chunk(chunk).await },
            )
        });

        join_all(&mut futures).await

        // let outcome = self.classify_results(results);

        // match outcome {
        //     WorkerOutcome::Finished => {
        //         self.handle_finished().await;
        //         break;
        //     }
        //     WorkerOutcome::Paused => {
        //         self.handle_paused().await;
        //         break;
        //     }
        //     WorkerOutcome::Errored | WorkerOutcome::Mixed => {
        //         if retries < max_retries {
        //             retries += 1;
        //             self.handle_retry().await;
        //             continue;
        //         } else {
        //             self.handle_failed().await;
        //             break;
        //         }
        //     }
        // };
    }

    async fn download_chunk(&self, chunk: DownloadChunk) -> Result<DownloadStatus, i64> {
        let chunk_index = chunk.chunk_index;
        let mut downloaded_bytes = chunk.downloaded_bytes;
        let start_byte = chunk.start_byte;
        let end_byte = chunk.end_byte;
        let url = self.download.url.clone();
        let range_header = format!("bytes={}-{}", start_byte + downloaded_bytes, end_byte);

        let client = Client::builder().build().map_err(|_| chunk_index)?;

        let response = client
            .get(url)
            .header(RANGE, range_header)
            .send()
            .await
            .map_err(|_| chunk_index)?;

        let mut stream = response.bytes_stream();
        let cancellation_token = self.cancel_token.clone();

        loop {
            tokio::select! {
                biased;

                _ = cancellation_token.cancelled() => {
                    return Ok(DownloadStatus::Paused);
                }

                maybe_bytes = timeout(Duration::from_secs(1), stream.next()) => {
                    match maybe_bytes {
                        Ok(Some(Ok(bytes))) => {

                            let bytes_len = bytes.len() as u64;

                            let write_message: WriteMessage = (
                                chunk_index,
                                (start_byte + downloaded_bytes) as u64,
                                bytes_len,
                                bytes.to_vec(),
                            );

                            self.file
                                .send(write_message)
                                .unwrap();

                            // Limiter

                            downloaded_bytes += bytes_len as i64;

                            Arc::clone(&self.manager).dispatch(ManagerAction::ReportNetworkWorker(self.download_id, bytes_len));
                        },
                        Ok(Some(Err(_))) => {
                            return Err(chunk_index);
                        },
                        Ok(None) => {
                            return Ok(DownloadStatus::Finished);
                        },
                        Err(_) => {
                            return Err(chunk_index);
                        }
                    }
                }

            }
        }
    }
}
