use tauri::State;

use crate::{
    events::dispatch,
    models::Download,
    registry::{DownloadOptions, Registry, RegistryAction},
    utils::app_state::{AppEvent, AppState},
};

#[tauri::command]
pub async fn add_new_download(url: String, options: DownloadOptions) -> Result<(), String> {
    Registry::dispatch(RegistryAction::AddNewDownload(url, options)).await
}

#[tauri::command]
pub async fn get_download_list(state: State<'_, AppState>) -> Result<Vec<Download>, String> {
    crate::db::downloads::get_downloads_list().await
}

#[tauri::command]
pub fn pause_download(state: State<'_, AppState>, id: i64) {
    dispatch(&state.broadcast_tx, AppEvent::PauseDownload(id));
}

#[tauri::command]
pub fn resume_download(state: State<'_, AppState>, id: i64) {
    dispatch(&state.broadcast_tx, AppEvent::ResumeDownload(id));
}
