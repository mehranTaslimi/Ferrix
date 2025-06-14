use tauri::State;

use crate::{
    downloader::validate_and_inspect_url,
    events::dispatch,
    utils::app_state::{AppEvent, AppState},
};

#[tauri::command]
pub async fn add_download_queue(
    state: State<'_, AppState>,
    url: String,
    chunk: Option<u8>,
) -> Result<(), String> {
    let chunk = chunk.unwrap_or(5).clamp(1, 5);
    let file_info = validate_and_inspect_url(&url).await?;
    dispatch(
        &state.broadcast_tx,
        AppEvent::StartNewDownloadProcess(file_info, chunk as i64),
    );

    Ok(())
}

#[tauri::command]
pub fn get_download_list(state: State<'_, AppState>) {
    dispatch(&state.broadcast_tx, AppEvent::SendDownloadList);
}

#[tauri::command]
pub fn pause_download(state: State<'_, AppState>, id: i64) {
    dispatch(&state.broadcast_tx, AppEvent::PauseDownload(id));
}

#[tauri::command]
pub fn resume_download(state: State<'_, AppState>, id: i64) {
    dispatch(&state.broadcast_tx, AppEvent::ResumeDownload(id));
}
