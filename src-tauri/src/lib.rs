use tokio::{spawn, sync::broadcast};

mod command;
mod db;
mod downloader;
mod events;
mod manager;
mod models;
mod utils;

use crate::{
    command::{add_download_queue, get_download_list, pause_download, resume_download},
    manager::downloads_manager::DownloadsManager,
    utils::app_state::AppState,
};

#[tokio::main]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    let (tx, _) = broadcast::channel(1000);
    let app_state = AppState::new(tx.clone()).await;

    let pool = app_state.pool.clone();
    let tx = tx.clone();
    let mut rx = tx.subscribe();

    tauri::Builder::default()
        .manage(app_state)
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            add_download_queue,
            get_download_list,
            resume_download,
            pause_download
        ])
        .setup(move |app| {
            let manager = DownloadsManager::new(tx, pool, app.handle().clone());

            spawn(async move {
                while let Ok(app_event) = rx.recv().await {
                    let result = manager.manage(app_event).await;
                    if let Err(err) = result {
                        println!("Broadcast error: {}", err);
                    }
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
