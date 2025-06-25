use serde::Serialize;
use tauri::{AppHandle, Emitter};
use tokio::sync::broadcast::Sender;

pub fn dispatch<T>(tx: &Sender<T>, event: T) {
    let _ = tx.send(event).map(|_| ());
}

pub fn emit_app_event<S: Serialize + Clone>(app_handle: &AppHandle, event: &str, payload: S) {
    let _ = app_handle.emit(event, payload);
}
