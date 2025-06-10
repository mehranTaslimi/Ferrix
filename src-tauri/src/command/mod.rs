use tauri::State;

use crate::utils::{
    app_state::{AppEvent, AppState},
    broadcast_event::dispatch,
};

#[tauri::command]
pub fn add_download_queue(
    state: State<'_, AppState>,
    url: String,
    chunk: Option<u8>,
) -> Result<(), String> {
    let chunk = chunk.unwrap_or(6).clamp(1, 6);
    dispatch(
        &state.broadcast_tx,
        AppEvent::StartNewDownloadProcess(url, chunk as i64),
    )
}

#[tauri::command]
pub fn get_download_list(state: State<'_, AppState>) -> Result<(), String> {
    dispatch(&state.broadcast_tx, AppEvent::SendDownloadList)
}
