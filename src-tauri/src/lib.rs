mod command;
mod utils;

use command::download::{cancel_download, download_file};
use utils::AppState;

#[tokio::main]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    tauri::Builder::default()
        .manage(AppState::new())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![download_file, cancel_download])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
