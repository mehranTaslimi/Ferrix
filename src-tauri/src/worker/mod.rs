mod chunk;
mod download;
mod file;
mod outcome;
mod reporter;
pub mod validation;

use crate::{
    db::downloads::{
        get_download_chunks_by_download_id, get_downloads_by_id, insert_download_chunks,
        insert_new_download, reset_downloaded_chunks, update_download_status,
    },
    events::emit_app_event,
    manager::task::TaskManager,
    models::{Chunk, ChunkCount, Download, DownloadId, FileInfo},
    utils::app_state::AppEvent,
    worker::{
        file::WriteMessage,
        reporter::{DiskReport, InternetReport},
    },
};
use sqlx::SqlitePool;
use std::sync::Arc;
use tauri::AppHandle;
use tokio::sync::{broadcast::Sender, mpsc, Mutex};
use tokio_util::sync::CancellationToken;

#[derive(Clone, Debug)]
pub struct DownloadWorker {
    pool: SqlitePool,
    app_handle: AppHandle,
    chunks: Arc<Mutex<Vec<Chunk>>>,
    download: Arc<Mutex<Download>>,
    file_writer: mpsc::UnboundedSender<WriteMessage>,
    internet_report: Arc<Mutex<InternetReport>>,
    disk_report: Arc<Mutex<DiskReport>>,
    app_event: Sender<AppEvent>,
    bandwidth_limit: Arc<Mutex<f32>>,
    task: Arc<TaskManager>,
    pub cancellation_token: CancellationToken,
    pub download_id: DownloadId,
    pub chunk_count: ChunkCount,
    pub speed_bps: Arc<Mutex<u64>>,
}

impl DownloadWorker {
    pub async fn new(
        pool: SqlitePool,
        app_handle: AppHandle,
        app_event: Sender<AppEvent>,
        bandwidth_limit: Arc<Mutex<f32>>,
        task: Arc<TaskManager>,
        download_id: Option<DownloadId>,
        file_info: Option<FileInfo>,
        chunk_count: Option<ChunkCount>,
    ) -> Result<Self, String> {
        let (download, chunks) = if let Some(download_id) = download_id {
            Self::get_download_and_chunks(&pool, download_id).await?
        } else {
            let file_info =
                file_info.ok_or_else(|| "missing file_info for new download".to_string())?;
            let chunk_count =
                chunk_count.ok_or_else(|| "missing chunk_count for new download".to_string())?;

            let download_id = Self::create_download(&pool, file_info, chunk_count).await?;
            Self::get_download_and_chunks(&pool, download_id).await?
        };

        let downloaded_bytes = download.downloaded_bytes as u64;
        let download_id = download.id;
        let chunk_count = chunks.len() as i64;

        let internet_report = Arc::new(Mutex::new(InternetReport {
            downloaded_bytes,
            received_bytes: 0.0,
        }));

        let disk_report = Arc::new(Mutex::new(DiskReport {
            received_bytes: 0,
            wrote_bytes: downloaded_bytes,
            chunks: chunks
                .clone()
                .into_iter()
                .map(|f| (f.chunk_index as u64, f.downloaded_bytes as u64))
                .collect(),
        }));

        let file_writer = Self::file_writer(
            &download.file_path,
            download.total_bytes as u64,
            Arc::clone(&disk_report),
            Arc::clone(&task),
        )
        .await?;

        let cancellation_token = CancellationToken::new();

        Ok(Self {
            pool,
            download: Arc::new(Mutex::new(download)),
            chunks: Arc::new(Mutex::new(chunks)),
            download_id,
            app_event,
            file_writer,
            app_handle,
            cancellation_token,
            bandwidth_limit,
            internet_report,
            disk_report,
            task,
            chunk_count,
            speed_bps: Arc::new(Mutex::new(0)),
        })
    }

    async fn create_download(
        pool: &SqlitePool,
        file_info: FileInfo,
        chunk_count: i64,
    ) -> Result<DownloadId, String> {
        let download_id = insert_new_download(&pool, file_info.clone(), chunk_count).await?;
        let ranges = Self::get_chunk_ranges(file_info.total_bytes as u64, chunk_count as u8)?;
        insert_download_chunks(&pool, download_id, ranges).await?;

        Ok(download_id)
    }

    pub(super) async fn get_download_and_chunks(
        pool: &SqlitePool,
        download_id: i64,
    ) -> Result<(Download, Vec<Chunk>), String> {
        let download = get_downloads_by_id(&pool, download_id).await?;
        let chunks = get_download_chunks_by_download_id(&pool, download_id).await?;

        let invalid_chunks_index = Self::invalid_chunks_hash(&download.file_path, chunks.clone());

        reset_downloaded_chunks(&pool, download_id, invalid_chunks_index).await?;

        Ok((download, chunks))
    }

    async fn emit_and_update_download_status(&self, status: &str) -> Result<(), String> {
        update_download_status(&self.pool, self.download_id, status).await?;
        let result = get_downloads_by_id(&self.pool, self.download_id).await?;
        emit_app_event(&self.app_handle, "download_item", result);

        Ok(())
    }
}
