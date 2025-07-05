use crate::{
    manager::{DownloadOptions, DownloadsManager},
    models::Download,
    repository::download::DownloadRepository,
};

#[tauri::command]
pub async fn add_new_download(url: String, options: DownloadOptions) -> Result<(), String> {
    DownloadsManager::add_new_download(url, options).await
}

#[tauri::command]
pub async fn get_download_list() -> Result<Vec<Download>, String> {
    DownloadRepository::find_all()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn pause_download() {}

#[tauri::command]
pub fn resume_download() {}
