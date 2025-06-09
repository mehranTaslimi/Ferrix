use tauri::State;

use crate::utils::{
    app_state::{AppEvent, AppState, DownloadData},
    event_handler::dispatch,
};

#[tauri::command]
pub fn add_download_queue(
    state: State<'_, AppState>,
    url: String,
    chunk: Option<u8>,
) -> Result<(), String> {
    let chunk = chunk.unwrap_or(5).clamp(1, 5);
    dispatch(
        &state.broadcast_tx,
        AppEvent::StartNewDownloadProcess(DownloadData { chunk, url }),
    )
}

#[tauri::command]
pub fn get_download_list(state: State<'_, AppState>) -> Result<(), String> {
    dispatch(&state.broadcast_tx, AppEvent::SendDownloadList)
}
