use std::{
    collections::VecDeque,
    sync::{atomic::AtomicU64, Arc},
};

use dashmap::DashMap;
use tokio::sync::{mpsc, Mutex};
use tokio_util::sync::CancellationToken;

use crate::{
    emitter::Emitter,
    file::{File, WriteMessage},
    models::DownloadWithChunk,
    registry::{Registry, Report},
};

mod bandwidth_limiter;
mod chunk;
mod download;
mod outcome;
pub mod validation;

#[derive(Clone, Debug)]
pub struct DownloadWorker {
    download_id: i64,
    download_ref: Arc<Mutex<DownloadWithChunk>>,
    file: mpsc::UnboundedSender<WriteMessage>,
    pub cancellation_token: CancellationToken,
}

impl DownloadWorker {
    pub async fn new(download_id: i64) -> Result<(), ()> {
        let cancellation_token = CancellationToken::new();

        let state = Registry::get_state();
        let downloading_map = state.downloading_map.lock().await;
        let download_ref = Arc::clone(downloading_map.get(&download_id).unwrap());

        let download_ref_clone = Arc::clone(&download_ref);
        let download_ref_clone = download_ref_clone.lock().await;

        if let Err(_) = download_ref_clone
            .cancel_token
            .set(cancellation_token.clone())
        {
            let err = format!("cancel token already set for download ID {}", download_id);
            Emitter::emit_error(err);
            return Err(());
        }

        let report = Arc::clone(&Registry::get_state().report);

        report.insert(
            download_id,
            Report {
                download_history: Mutex::new(VecDeque::with_capacity(10)),
                downloaded_bytes: AtomicU64::new(0),
                total_downloaded_bytes: AtomicU64::new(0),
                total_wrote_bytes: AtomicU64::new(0),
                wrote_bytes: AtomicU64::new(0),
                wrote_history: Mutex::new(VecDeque::with_capacity(10)),
                chunks_wrote_bytes: DashMap::new(),
                total_bytes: download_ref_clone.download.total_bytes as u64,
            },
        );

        let file = match File::new(
            download_id,
            &download_ref_clone.download.file_path,
            download_ref_clone.download.total_bytes as u64,
        )
        .await
        {
            Ok(f) => f,
            Err(err) => {
                Emitter::emit_error(err);
                return Err(());
            }
        };

        let worker = Self {
            cancellation_token,
            download_id,
            file,
            download_ref,
        };

        Registry::spawn("start_download", async move {
            worker.start_download().await;
        });

        Ok(())
    }

    // async fn create_download(file_info: FileInfo, chunk_count: i64) -> Result<DownloadId, String> {
    //     let download_id = insert_new_download(file_info.clone(), chunk_count).await?;
    //     let ranges = Self::get_chunk_ranges(file_info.total_bytes as u64, chunk_count as u8)?;
    //     insert_download_chunks(download_id, ranges).await?;

    //     Ok(download_id)
    // }

    // pub(super) async fn get_download_and_chunks(
    //     download_id: i64,
    // ) -> Result<(Download, Vec<Chunk>), String> {
    //     let download = get_downloads_by_id(download_id).await?;
    //     let chunks = get_download_chunks_by_download_id(download_id).await?;

    //     let invalid_chunks_index = Self::invalid_chunks_hash(&download.file_path, chunks.clone());

    //     reset_downloaded_chunks(download_id, invalid_chunks_index).await?;

    //     Ok((download, chunks))
    // }

    // async fn emit_and_update_download_status(&self, status: &str) -> Result<(), String> {
    //     update_download_status(self.download_id, status).await?;
    //     let result = get_downloads_by_id(self.download_id).await?;
    //     emit_app_event(&self.app_handle, "download_item", result);

    //     Ok(())
    // }
}
