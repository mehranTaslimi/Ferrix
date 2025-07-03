use tauri::State;

use crate::{
    models::Download,
    registry::{DownloadOptions, Registry, RegistryAction},
    repository::download::DownloadRepository,
    utils::app_state::AppState,
};

#[tauri::command]
pub async fn add_new_download(url: String, options: DownloadOptions) -> Result<(), String> {
    Registry::dispatch(RegistryAction::AddNewDownload(url, options)).await
}

#[tauri::command]
pub async fn get_download_list() -> Result<Vec<Download>, String> {
    DownloadRepository::find_all()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn pause_download(state: State<'_, AppState>, id: i64) {}

#[tauri::command]
pub fn resume_download(state: State<'_, AppState>, id: i64) {}
