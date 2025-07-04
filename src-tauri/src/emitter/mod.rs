use serde::Serialize;
use std::sync::Arc;
use tauri::Emitter as TauriEmmiter;

use crate::registry::Registry;

pub struct Emitter;

impl Emitter {
    pub fn emit_error(err: String) {
        let app_handle = Arc::clone(&Registry::get_state().app_handle);
        let _ = app_handle.emit("error", err);
    }

    pub fn emit_event<S>(event: &str, payload: S)
    where
        S: Serialize + Clone,
    {
        let app_handle = Arc::clone(&Registry::get_state().app_handle);
        let _ = app_handle.emit(event, payload);
    }
}
