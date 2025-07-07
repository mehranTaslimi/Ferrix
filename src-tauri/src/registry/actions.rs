use std::{
    collections::VecDeque,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::Duration,
};

use dashmap::DashMap;
use tokio::{sync::Mutex, time::sleep};
use tokio_util::sync::CancellationToken;

use crate::{
    emitter::Emitter,
    file::File,
    manager::ManagerAction,
    models::{Download, DownloadChunk},
    registry::{Registry, RegistryAction, Report},
    repository::{chunk::ChunkRepository, download::DownloadRepository},
    worker::Worker,
};

impl super::Registry {
    pub async fn add_download_to_queue(download_id: i64) {
        let pending_queue = Arc::clone(&Self::get_state().pending_queue);
        let mut pending_queue = pending_queue.lock().await;
        pending_queue.push_back(download_id);

        let download = DownloadRepository::find(download_id).await.unwrap();
        Emitter::emit_event("download_item", download);

        Registry::dispatch(super::RegistryAction::CheckAvailablePermit);
    }

    pub async fn check_available_permit_action() {
        let state = Arc::clone(&Registry::get_state());
        let pending_queue = Arc::clone(&state.pending_queue);
        let queue_listener_running = Arc::clone(&state.queue_listener_running);

        if !pending_queue.lock().await.is_empty()
            && !queue_listener_running.swap(true, Ordering::SeqCst)
        {
            Registry::spawn("check_available_permit_action", async move {
                loop {
                    let download_id = {
                        let pending_queue = pending_queue.lock().await;
                        pending_queue.front().cloned()
                    };

                    match download_id {
                        Some(download_id) => {
                            let download_result = DownloadRepository::find(download_id).await;

                            match download_result {
                                Ok(download) => {
                                    let chunk_count = download.chunk_count as usize;

                                    let available_permits = Registry::get_state()
                                        .available_permits
                                        .load(Ordering::SeqCst);
                                    /*
                                        The app reserves 10 permits for other operations.
                                        Before starting a download, we check if available permits
                                        are sufficient: the required `chunk_count` plus 5 extra permits as buffer.
                                        If enough permits are available, the ID is popped from the queue
                                        and the download is dispatched.
                                    */
                                    if available_permits >= chunk_count + 5 {
                                        pending_queue.lock().await.pop_front();
                                        Registry::dispatch(
                                            RegistryAction::AddDownloadToWorkersMap(download_id),
                                        );
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
                        None => {
                            /*
                                This `None` match means the queue is empty.
                                There’s no reason to continue the loop, so we break it.
                            */
                            queue_listener_running.store(false, Ordering::SeqCst);
                            break;
                        }
                    }

                    sleep(Duration::from_secs(1)).await;
                }
            });
        }
    }

    pub(super) async fn add_download_workers_map_action(download_id: i64) {
        let workers = Arc::clone(&Registry::get_state().workers);
        let manager = Arc::clone(&Registry::get_manager());

        let download = DownloadRepository::find(download_id).await.unwrap();
        let chunks = ChunkRepository::find_all(download_id).await.unwrap();

        let not_downloaded_chunks = chunks
            .into_iter()
            .filter(|chunk| chunk.downloaded_bytes < chunk.end_byte - chunk.start_byte)
            .collect::<Vec<_>>();

        Registry::dispatch(RegistryAction::CreateDownloadReport(
            download.clone(),
            not_downloaded_chunks.clone(),
        ));

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
            Arc::new(Mutex::new(Worker {
                download,
                chunks: not_downloaded_chunks.clone(),
                cancel_token: Arc::new(CancellationToken::new()),
                file,
            })),
        );

        manager.dispatch(ManagerAction::StartDownload(download_id));
    }

    pub(super) async fn create_download_report_action(
        download: Download,
        chunks: Vec<DownloadChunk>,
    ) {
        let report = Arc::clone(&Registry::get_state().report);

        let chunks_wrote_bytes: DashMap<i64, AtomicU64> = chunks
            .iter()
            .map(|f| (f.chunk_index, AtomicU64::new(f.downloaded_bytes as u64)))
            .collect();

        report.insert(
            download.id,
            Report {
                total_downloaded_bytes: AtomicU64::new(download.downloaded_bytes as u64),
                downloaded_bytes: AtomicU64::new(0),
                total_wrote_bytes: AtomicU64::new(download.downloaded_bytes as u64),
                wrote_bytes: AtomicU64::new(0),
                download_history: Mutex::new(VecDeque::with_capacity(10)),
                wrote_history: Mutex::new(VecDeque::with_capacity(10)),
                chunks_wrote_bytes,
                total_bytes: download.total_bytes as u64,
                speed_bps: AtomicU64::new(0),
            },
        );
    }

    pub(super) async fn update_network_report_action(download_id: i64, bytes_len: u64) {
        let reports = Arc::clone(&Registry::get_state().report);
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
        let reports = Arc::clone(&Registry::get_state().report);
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
        let reports = Arc::clone(&Registry::get_state().report);
        let workers = Arc::clone(&Registry::get_state().workers);

        reports.remove(&download_id);
        workers.remove(&download_id);
    }

    pub(super) async fn pause_download_action(download_id: i64) {
        let manager = Arc::clone(&Registry::get_manager());
        manager.dispatch(ManagerAction::PauseDownload(download_id));
    }

    pub(super) async fn resume_download_action(download_id: i64) {
        Registry::get_manager().dispatch(ManagerAction::ValidateChunksHash(download_id));
    }
}
