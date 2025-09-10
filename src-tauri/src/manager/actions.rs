use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};

use anyhow::{anyhow, Context};
use futures_util::future::join_all;

use crate::{
    dispatch,
    emitter::Emitter,
    models::{UpdateChunk, UpdateDownload},
    registry::Registry,
    repository::{chunk::ChunkRepository, download::DownloadRepository},
    worker::{DownloadStatus, DownloadWorker},
};

impl super::DownloadsManager {
    pub(super) async fn start_download_action(
        self: &Arc<Self>,
        download_id: i64,
    ) -> anyhow::Result<()> {
        Self::start_monitoring();
        let download_worker = DownloadWorker::new(download_id).await?;

        download_worker.start_download().await;

        Ok(())
    }

    pub(super) async fn update_download_status_action(
        self: &Arc<Self>,
        status: DownloadStatus,
        error_message: Option<String>,
        download_id: i64,
    ) -> anyhow::Result<()> {
        match status {
            DownloadStatus::Failed | DownloadStatus::Completed | DownloadStatus::Paused => {
                dispatch!(manager, UpdateChunks, (download_id, true));
            }
            _ => {}
        }

        DownloadRepository::update(
            download_id,
            UpdateDownload {
                status: Some(status.to_string()),
                error_message,
                auth: None,
                backoff_factor: None,
                cookies: None,
                delay_secs: None,
                headers: None,
                max_retries: None,
                proxy: None,
                speed_limit: None,
                timeout_secs: None,
                total_bytes: None,
            },
        )
        .await?;

        let download = DownloadRepository::find(download_id).await?;
        Emitter::emit_event("download_item", &download);

        if matches!(status, DownloadStatus::Completed) {
            Emitter::emit_notification("Download Completed", download.file_name);
        }

        Ok(())
    }

    pub(super) async fn pause_download_action(
        self: &Arc<Self>,
        download_id: i64,
    ) -> anyhow::Result<()> {
        let workers = Arc::clone(&Registry::get_state().workers);
        let worker = workers.get(&download_id).ok_or(anyhow!(
            "pause download error: cannot find worker with download id {}",
            download_id
        ))?;

        let worker = worker.read().await;
        worker.cancel_token.cancel();

        Ok(())
    }

    pub(super) async fn update_chunks_action(
        self: &Arc<Self>,
        download_id: i64,
        clean_after_update: bool,
    ) -> anyhow::Result<()> {
        let workers = Arc::clone(&Registry::get_state().workers);
        let worker = workers.get(&download_id).context(anyhow!(
            "update chunk error: cannot find worker with download id {}",
            download_id
        ))?;

        let reports = Arc::clone(&Registry::get_state().reports);

        let worker = worker.write().await;
        let update_chunks_futures = worker.chunks.iter().map(|chunk| {
            let chunk_index = chunk.chunk_index;
            let wrote_bytes = reports
                .get(&download_id)
                .and_then(|r| {
                    r.chunks_wrote_bytes
                        .get(&chunk_index)
                        .map(|v| v.load(Ordering::Relaxed))
                })
                .unwrap_or(0);

            async move {
                let expected_hash = Self::hash_chunk(download_id, chunk_index)
                    .await
                    .ok()
                    .map(|h| h.to_string());

                UpdateChunk {
                    downloaded_bytes: Some(wrote_bytes as i64),
                    expected_hash,
                    chunk_index,
                }
            }
        });

        let update_chunks = join_all(update_chunks_futures).await;

        if let Err(errors) = ChunkRepository::update_all(download_id, update_chunks).await {
            errors.iter().for_each(|err| {
                Emitter::emit_error(err.to_string());
            });
        }

        if clean_after_update {
            dispatch!(registry, CleanDownloadedItemData { download_id });
        }

        Ok(())
    }

    pub(super) async fn reset_chunk_action(
        self: &Arc<Self>,
        download_id: i64,
        chunk_index: i64,
    ) -> anyhow::Result<()> {
        ChunkRepository::update(
            download_id,
            UpdateChunk {
                chunk_index,
                downloaded_bytes: Some(0),
                expected_hash: None,
            },
        )
        .await?;

        let reports = Arc::clone(&Registry::get_state().reports);

        let report = reports.get(&download_id).context(anyhow!(
            "reset chunk error: cannot find report with download id {}",
            download_id
        ))?;

        let chunk_counter = report.chunks_wrote_bytes.get(&chunk_index)
        .context(
            anyhow!(
                "reset chunk error: cannot find chunk wrote bytes in report with download id {} and chunk index {}",
                download_id, chunk_index
            )
        )?;

        let prev_chunk_bytes = chunk_counter.swap(0, Ordering::AcqRel);

        let saturating_fetch_sub = |atom: &AtomicU64, amount: u64| {
            let _ = atom.fetch_update(Ordering::AcqRel, Ordering::Acquire, |cur| {
                Some(cur.saturating_sub(amount))
            });
        };

        saturating_fetch_sub(&report.total_downloaded_bytes, prev_chunk_bytes);
        saturating_fetch_sub(&report.total_wrote_bytes, prev_chunk_bytes);

        Ok(())
    }
}
