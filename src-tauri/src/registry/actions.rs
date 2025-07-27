use std::{
    collections::VecDeque,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};

use dashmap::DashMap;
use tokio::sync::{Mutex, RwLock};
use tokio_util::sync::CancellationToken;

use crate::{
    dispatch,
    emitter::Emitter,
    file::File,
    models::{Download, DownloadChunk},
    queue_spawn,
    repository::{chunk::ChunkRepository, download::DownloadRepository},
    worker::Worker,
};

use super::*;

impl Registry {
    pub async fn add_download_to_queue(download_id: i64) {
        let pending_queue = Arc::clone(&Self::get_state().pending_queue);
        let mut pending_queue = pending_queue.lock().await;
        pending_queue.push_back(download_id);

        let download = DownloadRepository::find(download_id).await.unwrap();
        Emitter::emit_event("download_item", download);

        dispatch!(registry, CheckAvailablePermit);
    }

    pub(super) async fn recover_queued_download_from_repository_action() {
        let download_ids = DownloadRepository::find_all(Some("queued"))
            .await
            .map(|downloads| {
                downloads
                    .iter()
                    .map(|download| download.id)
                    .collect::<Vec<i64>>()
            });

        if let Ok(download_ids) = download_ids {
            for download_id in download_ids.iter() {
                dispatch!(registry, NewDownloadQueue, (*download_id));
            }
        };
    }

    pub(super) async fn check_available_permit_action() {
        queue_spawn!("check_available_permit_action", {
            let state = Arc::clone(&Self::get_state());
            let pending_queue = Arc::clone(&state.pending_queue);

            let download_id = {
                let pending_queue = pending_queue.lock().await;
                pending_queue.front().cloned()
            };

            if let Some(download_id) = download_id {
                let download_result = DownloadRepository::find(download_id).await;

                match download_result {
                    Ok(download) => {
                        let chunk_count = download.chunk_count as usize;

                        let available_permits =
                            Self::get_state().available_permits.load(Ordering::SeqCst);
                        /*
                            The app reserves 10 permits for other operations.
                            Before starting a download, we check if available permits
                            are sufficient: the required `chunk_count` plus 5 extra permits as buffer.
                            If enough permits are available, the ID is popped from the queue
                            and the download is dispatched.
                        */

                        let remaining_bytes = download
                            .total_bytes
                            .saturating_sub(download.downloaded_bytes)
                            as u64;

                        let has_disk_space =
                            match File::check_disk_space(&download.file_path, remaining_bytes) {
                                Ok(is_available) => is_available,
                                Err(e) => {
                                    Emitter::emit_error(e);
                                    false
                                }
                            };

                        let permit_available = has_disk_space
                            && (available_permits >= 10 && available_permits - 5 >= chunk_count);

                        if permit_available {
                            pending_queue.lock().await.pop_front();
                            dispatch!(registry, AddDownloadToWorkersMap, (download_id));
                        }
                    }
                    Err(_) => {
                        /*
                            This section handles the case where a download is removed or cancelled from the queue.
                            The ID is removed from the queued list, and in the next loop iteration,
                            if the queue is empty, the loop will break and the listener will stop.
                        */
                        pending_queue.lock().await.pop_front();
                    }
                }
            }
        });
    }

    pub(super) async fn add_download_workers_map_action(download_id: i64) {
        let workers = Arc::clone(&Self::get_state().workers);
        let download = DownloadRepository::find(download_id).await.unwrap();
        let chunks = ChunkRepository::find_all(download_id).await.unwrap();

        let not_downloaded_chunks = chunks
            .into_iter()
            .filter(|chunk| chunk.downloaded_bytes < chunk.end_byte - chunk.start_byte)
            .collect::<Vec<_>>();

        dispatch!(
            registry,
            CreateDownloadReport,
            (download.clone(), not_downloaded_chunks.clone(),)
        );

        let file = match File::new(
            download_id,
            &download.file_path,
            download.total_bytes as u64,
        )
        .await
        {
            Ok(f) => Arc::new(f),
            Err(err) => {
                Emitter::emit_error(err);
                return;
            }
        };

        workers.insert(
            download.id,
            Arc::new(RwLock::new(Worker {
                download,
                chunks: not_downloaded_chunks.clone(),
                cancel_token: Arc::new(CancellationToken::new()),
                file,
            })),
        );

        dispatch!(manager, StartDownload, (download_id));
    }

    pub(super) async fn create_download_report_action(
        download: Download,
        chunks: Vec<DownloadChunk>,
    ) {
        let report = Arc::clone(&Self::get_state().reports);

        let chunks_wrote_bytes: DashMap<i64, AtomicU64> = chunks
            .iter()
            .map(|f| (f.chunk_index, AtomicU64::new(f.downloaded_bytes as u64)))
            .collect();

        report.insert(
            download.id,
            Arc::new(Report {
                total_downloaded_bytes: AtomicU64::new(download.downloaded_bytes as u64),
                downloaded_bytes: AtomicU64::new(0),
                total_wrote_bytes: AtomicU64::new(download.downloaded_bytes as u64),
                wrote_bytes: AtomicU64::new(0),
                download_history: Mutex::new(VecDeque::with_capacity(10)),
                wrote_history: Mutex::new(VecDeque::with_capacity(10)),
                chunks_wrote_bytes,
                total_bytes: download.total_bytes as u64,
                speed_bps: AtomicU64::new(0),
                last_update_chunk_percent: AtomicU8::new(0),
                stable_speed: AtomicBool::new(false),
            }),
        );
    }

    pub(super) async fn update_network_report_action(download_id: i64, bytes_len: u64) {
        let reports = Arc::clone(&Self::get_state().reports);
        let maybe_report = reports.get(&download_id);
        if let Some(report) = maybe_report {
            report
                .total_downloaded_bytes
                .fetch_add(bytes_len, Ordering::Relaxed);

            report
                .downloaded_bytes
                .fetch_add(bytes_len, Ordering::Relaxed);
        }
    }

    pub(super) async fn update_disk_report_action(
        download_id: i64,
        chunk_index: i64,
        bytes_len: u64,
    ) {
        let reports = Arc::clone(&Self::get_state().reports);
        let maybe_report = reports.get(&download_id);
        if let Some(report) = maybe_report {
            report
                .chunks_wrote_bytes
                .entry(chunk_index as i64)
                .and_modify(|atomic| {
                    atomic.fetch_add(bytes_len, Ordering::Relaxed);
                })
                .or_insert(AtomicU64::new(bytes_len));

            report
                .total_wrote_bytes
                .fetch_add(bytes_len, Ordering::Relaxed);

            report.wrote_bytes.fetch_add(bytes_len, Ordering::Relaxed);
        }
    }

    pub(super) async fn clean_downloaded_item_data(download_id: i64) {
        let reports = Arc::clone(&Self::get_state().reports);
        let workers = Arc::clone(&Self::get_state().workers);

        reports.remove(&download_id);
        workers.remove(&download_id);
    }

    pub(super) async fn pause_download_action(download_id: i64) {
        dispatch!(manager, PauseDownload, (download_id));
    }

    pub(super) async fn resume_download_action(download_id: i64) {
        dispatch!(manager, ValidateChunksHash, (download_id));
    }

    pub(super) async fn remove_download_action(download_id: i64, remove_file: bool) {
        if let Ok(file_path) = DownloadRepository::delete(download_id).await {
            if remove_file {
                if let Err(err) = crate::file::File::remove_file(&file_path) {
                    Emitter::emit_error(err.to_string());
                }
            }
        } else if let Err(err) = DownloadRepository::delete(download_id).await {
            Emitter::emit_error(err.to_string());
        }
    }

    pub(super) async fn close_requested_action() {
        //
    }
}
