use serde::Serialize;
use tauri::Emitter as TauriEmmiter;
use tauri_plugin_notification::NotificationExt;

use crate::registry::Registry;

pub struct Emitter;

impl Emitter {
    pub fn emit_error<S>(err: S)
    where
        S: Serialize + Clone,
    {
        let state = Registry::get_state();
        let _ = state.app_handle.emit("error", err);
    }

    pub fn emit_event<S>(event: &str, payload: S)
    where
        S: Serialize + Clone,
    {
        let state = Registry::get_state();
        let _ = state.app_handle.emit(event, payload);
    }

    pub fn emit_notification(title: impl Into<String>, body: impl Into<String>) {
        let state = Registry::get_state();
        let _ = state
            .app_handle
            .notification()
            .builder()
            .title(title)
            .body(body)
            .show();
    }
}
