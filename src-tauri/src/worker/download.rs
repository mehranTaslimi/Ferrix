use futures_util::{future::join_all, StreamExt};
use std::{sync::Arc, time::Duration};
use tokio::time::timeout;

use crate::{
    client::Client,
    file::WriteMessage,
    models::DownloadChunk,
    registry::{Registry, RegistryAction},
};

impl super::DownloadWorker {
    pub async fn start_download(&self) -> super::WorkerOutcome {
        let mut futures = self.chunks.clone().into_iter().map(|chunk| {
            let self_clone = self.clone();
            let task_name = format!(
                "chunk_download: {}, {}",
                self_clone.download.id, chunk.chunk_index
            );
            Registry::spawn(
                &task_name,
                async move { self_clone.download_chunk(chunk).await },
            )
        });

        let results = join_all(&mut futures).await;
        self.classify_results(results)
    }

    async fn download_chunk(&self, chunk: DownloadChunk) -> Result<super::DownloadStatus, i64> {
        let chunk_index = chunk.chunk_index;
        let mut downloaded_bytes = chunk.downloaded_bytes;
        let start_byte = chunk.start_byte;
        let end_byte = chunk.end_byte;

        let client = Client::new(
            &self.download.url,
            &self.download.auth,
            &self.download.proxy,
            &self.download.headers,
            &self.download.cookies,
        )
        .map_err(|_| chunk_index)?;

        let range = match self.download.supports_range {
            true => Some((start_byte + downloaded_bytes, end_byte)),
            false => None,
        };

        let mut stream = client.stream(range).await.map_err(|_| chunk_index)?;

        let cancellation_token = Arc::clone(&self.cancel_token);

        let timeout_secs = self.download.timeout_secs as u64;

        loop {
            tokio::select! {
                biased;

                _ = cancellation_token.cancelled() => {
                    return Ok(super::DownloadStatus::Paused);
                }

                maybe_bytes = timeout(Duration::from_secs(timeout_secs), stream.next()) => {
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

                            Registry::dispatch(RegistryAction::UpdateNetworkReport(self.download.id, bytes_len));
                        },
                        Ok(Some(Err(_))) => {
                            return Err(chunk_index);
                        },
                        Ok(None) => {
                            return Ok(super::DownloadStatus::Finished);
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
