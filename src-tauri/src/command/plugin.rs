use crate::{
    dispatch,
    registry::{ActionKind, EventName},
};

#[tauri::command]
pub fn register_event(event: EventName, id: String) {
    dispatch!(registry, RegisterEvent, (event, id));
}

#[tauri::command]
pub fn unregister_event(event: EventName, id: String) {
    dispatch!(registry, UnRegisterEvent, (event, id));
}

#[tauri::command]
pub fn event_job_completed(action_kind: ActionKind, id: String) {
    dispatch!(registry, EventJobCompleted, (action_kind, id));
}
