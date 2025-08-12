use futures_util::StreamExt;
use std::{
    sync::{atomic::Ordering, Arc},
    time::Duration,
};
use tokio::{
    select,
    time::{sleep, timeout},
};

use crate::{
    client::{Client, ClientError},
    dispatch,
    file::WriteMessage,
    spawn,
    worker::status::ChunkDownloadStatus,
};

use super::*;

impl DownloadWorker {
    pub async fn start_download(self: &Arc<Self>) {
        use ChunkDownloadStatus::*;

        let worker = self.data.read().await;

        let chunks = worker.chunks.clone();
        let cancel_token = Arc::clone(&worker.cancel_token);
        let max_retries = worker.download.max_retries;

        let cancelable_sleep = async move |d: Duration, c: Arc<CancellationToken>| {
            select! {
                _ = sleep(d) => true,
                _ = c.cancelled() => false,
            }
        };

        for chunk in chunks {
            let worker_clone = Arc::clone(self);
            let cancel_token = Arc::clone(&cancel_token);

            spawn!("download_chunk", {
                let max_retries = max_retries;
                let mut retries = 0;

                let set = async |st| {
                    worker_clone
                        .update_chunk_status(chunk.chunk_index, st)
                        .await
                };

                loop {
                    set(Downloading).await;

                    let st = select! {
                        status = worker_clone.download_chunk(&chunk) => {
                            match status {
                                Ok(()) => Finished,
                                Err(err) if err.is_retryable() => Trying(err.to_string()),
                                Err(err) => Errored(err.to_string())
                            }
                        },
                        _ = cancel_token.cancelled() => Paused,
                    };

                    match st {
                        Paused | Finished => {
                            set(st).await;
                            break;
                        }
                        Errored(err) => {
                            set(Errored(err)).await;
                            cancel_token.cancel();
                            break;
                        }
                        Trying(err) => {
                            if cancel_token.is_cancelled() {
                                set(Paused).await;
                                break;
                            }

                            if retries < max_retries {
                                set(Trying(err)).await;

                                retries += 1;

                                let delay = worker_clone.backoff_delay(retries).await;
                                let cancel_clone = Arc::clone(&cancel_token);

                                if !cancelable_sleep(delay, cancel_clone).await {
                                    set(Paused).await;
                                    break;
                                }
                            } else {
                                set(Errored(err)).await;
                                cancel_token.cancel();
                                break;
                            }
                        }

                        _ => {}
                    }
                }
            });
        }
    }

    async fn download_chunk(self: &Arc<Self>, chunk: &DownloadChunk) -> Result<(), ClientError> {
        let report = Arc::clone(&self.report);

        let mut downloaded_bytes = match report.chunks_wrote_bytes.get(&chunk.chunk_index) {
            Some(bytes) => bytes.load(Ordering::SeqCst) as i64,
            None => 0,
        };

        // if let Err(_) = self
        //     .validate_chunk(chunk.expected_hash.clone(), chunk.chunk_index)
        //     .await
        // {
        //     return Err(ClientError::UnexpectedChunkHash);
        // }

        let start_byte = chunk.start_byte;
        let end_byte = chunk.end_byte;

        let (client, range, timeout_secs, file) = {
            let w = self.data.read().await;

            let range = match &w.download.supports_range {
                true => Some((start_byte + downloaded_bytes, end_byte)),
                false => None,
            };

            let client = match Client::new(
                &w.download.url,
                &w.download.auth,
                &w.download.proxy,
                &w.download.headers,
                &w.download.cookies,
            ) {
                Ok(c) => c,
                Err(err) => {
                    return Err(err);
                }
            };

            let file = Arc::clone(&w.file);

            let timeout_secs = w.download.timeout_secs;

            (client, range, timeout_secs, file)
        };

        if chunk.chunk_index == 2 {
            return Err(ClientError::StreamTimeout);
        }

        let mut stream = client.stream(range).await?;

        loop {
            match timeout(Duration::from_secs(timeout_secs as u64), stream.next()).await {
                Ok(Some(Ok(bytes))) => {
                    let bytes_len = bytes.len() as u64;

                    let write_message: WriteMessage = (
                        chunk.chunk_index,
                        (start_byte + downloaded_bytes) as u64,
                        bytes_len,
                        bytes.to_vec(),
                    );

                    file.send(write_message).unwrap();

                    self.limiter(bytes_len).await;

                    downloaded_bytes += bytes_len as i64;

                    dispatch!(registry, UpdateNetworkReport, (self.download_id, bytes_len));
                }
                Ok(Some(Err(err))) => {
                    return Err(err);
                }
                Ok(None) => {
                    return Ok(());
                }
                Err(_) => {
                    return Err(ClientError::StreamTimeout);
                }
            }
        }
    }
}
