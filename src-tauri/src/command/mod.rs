use crate::{
    dispatch,
    manager::{DownloadOptions, DownloadsManager},
    models::Download,
    repository::download::DownloadRepository,
};

pub mod plugin;

#[tauri::command]
pub async fn add_new_download(url: String, options: DownloadOptions) -> Result<(), String> {
    DownloadsManager::add_new_download(url, options).await
}

#[tauri::command]
pub async fn get_download_list() -> Result<Vec<Download>, String> {
    DownloadRepository::find_all(None)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn pause_download(id: i64) {
    dispatch!(registry, PauseDownload, (id));
}

#[tauri::command]
pub fn resume_download(id: i64) {
    dispatch!(registry, ResumeDownload, (id));
}

#[tauri::command]
pub fn remove_download(id: i64, remove_file: bool) {
    dispatch!(registry, RemoveDownload, (id, remove_file));
}
