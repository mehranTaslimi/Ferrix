use crate::{
    dispatch,
    models::Download,
    registry::{DownloadOptions, RegistryAction},
    repository::download::DownloadRepository,
};

#[tauri::command]
pub fn add_new_download(opt_id: String, url: String, options: DownloadOptions) {
    dispatch!(
        registry,
        NewDownload {
            opt_id,
            url,
            options
        }
    );
}

#[tauri::command]
pub async fn get_download_list() -> Result<Vec<Download>, String> {
    DownloadRepository::find_all(None)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn pause_download(id: i64) {
    dispatch!(registry, PauseDownload { download_id: id });
}

#[tauri::command]
pub fn resume_download(id: i64) {
    dispatch!(registry, ResumeDownload { download_id: id });
}

#[tauri::command]
pub fn remove_download(id: i64, remove_file: bool) {
    dispatch!(
        registry,
        RemoveDownload {
            download_id: id,
            remove_file
        }
    );
}

#[tauri::command]
pub fn dispatch(action: RegistryAction) -> Result<(), String> {
    dispatch!(registry, ::action).map_err(|e| e.to_string())
}
