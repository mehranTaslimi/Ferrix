mod command;
mod db;
mod downloader;
mod models;
mod utils;

use command::add_download_queue;
use tokio::sync::broadcast;

use crate::{command::get_download_list, utils::app_state::AppState};

#[tokio::main]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    let (tx, _) = broadcast::channel(100);
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
            get_download_list
        ])
        .setup(move |app| {
            let app_handle = app.handle().clone();

            tokio::spawn(async move {
                while let Ok(app_event) = rx.recv().await {
                    let result =
                        utils::event_handler::handle(app_event, &tx, &pool, &app_handle).await;

                    if let Err(err) = result {
                        println!("Error: {}", err);
                    }
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
