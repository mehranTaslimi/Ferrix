use futures_util::{future::join_all, StreamExt};
use std::sync::Arc;

use crate::{
    client::{Client, ClientError},
    file::WriteMessage,
    models::DownloadChunk,
    registry::{Registry, RegistryAction},
};

impl super::DownloadWorker {
    pub async fn start_download(&self) -> super::WorkerOutcome {
        let mut futures = self.chunks.clone().into_iter().map(|chunk| {
            let self_clone = self.clone();
            Registry::spawn(async move { self_clone.download_chunk(chunk).await })
        });

        let results = join_all(&mut futures).await;
        self.classify_results(results)
    }

    async fn download_chunk(
        &self,
        chunk: DownloadChunk,
    ) -> Result<super::DownloadStatus, ClientError> {
        let chunk_index = chunk.chunk_index;
        let mut downloaded_bytes = chunk.downloaded_bytes;
        let start_byte = chunk.start_byte;
        let end_byte = chunk.end_byte;

        let client = Client::new(
            &self.download.url,
            self.download.timeout_secs as f64,
            &self.download.auth,
            &self.download.proxy,
            &self.download.headers,
            &self.download.cookies,
        )?;

        let range = match self.download.supports_range {
            true => Some((start_byte + downloaded_bytes, end_byte)),
            false => None,
        };

        let mut stream = client.stream(range).await?;

        let cancellation_token = Arc::clone(&self.cancel_token);

        loop {
            tokio::select! {
                biased;

                _ = cancellation_token.cancelled() => {
                    return Ok(super::DownloadStatus::Paused);
                }

                maybe_bytes = stream.next() => {
                    match maybe_bytes {
                        Some(Ok(bytes)) => {

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

                            downloaded_bytes += bytes_len as i64;

                            Registry::dispatch(RegistryAction::UpdateNetworkReport(self.download.id, bytes_len));
                        },
                        Some(Err(err)) => {
                            return Err(err);
                        },
                        None => {
                            return Ok(super::DownloadStatus::Finished);
                        },
                    }
                }

            }
        }
    }
}
