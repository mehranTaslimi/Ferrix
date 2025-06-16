use serde::Serialize;
use tauri::{AppHandle, Emitter};
use tokio::sync::broadcast::Sender;

use crate::utils::app_state::AppEvent;

pub fn dispatch(tx: &Sender<AppEvent>, app_event: AppEvent) {
    let _ = tx.send(app_event).map(|_| ());
}

pub fn emit_app_event<S: Serialize + Clone>(app_handle: &AppHandle, event: &str, payload: S) {
    let _ = app_handle.emit(event, payload);
}
