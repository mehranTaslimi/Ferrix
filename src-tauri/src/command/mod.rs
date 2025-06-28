use tauri::State;

use crate::{
    events::dispatch,
    models::Download,
    utils::app_state::{AppEvent, AppState},
    worker::DownloadWorker,
};

#[tauri::command]
pub async fn add_download_queue(
    state: State<'_, AppState>,
    url: String,
    chunk: Option<u8>,
) -> Result<(), String> {
    let chunk = chunk.unwrap_or(5).clamp(1, 5);
    let file_info = DownloadWorker::validate_and_inspect_url(&url).await?;
    dispatch(
        &state.broadcast_tx,
        AppEvent::StartNewDownload(file_info, chunk as i64),
    );

    Ok(())
}

#[tauri::command]
pub async fn get_download_list(state: State<'_, AppState>) -> Result<Vec<Download>, String> {
    crate::db::downloads::get_downloads_list(&state.pool).await
}

#[tauri::command]
pub fn pause_download(state: State<'_, AppState>, id: i64) {
    dispatch(&state.broadcast_tx, AppEvent::PauseDownload(id));
}

#[tauri::command]
pub fn resume_download(state: State<'_, AppState>, id: i64) {
    dispatch(&state.broadcast_tx, AppEvent::ResumeDownload(id));
}
