mod command;
mod db;
mod downloader;
mod models;
mod utils;
use command::add_download_queue;
use tokio::{spawn, sync::broadcast};

use crate::{
    command::{get_download_list, pause_download, resume_download},
    utils::{app_state::AppState, broadcast_event::EventHandler},
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
            let event_handler = EventHandler {
                app_handle: app.handle().clone(),
                pool,
                tx,
            };

            spawn(async move {
                loop {
                    match rx.recv().await {
                        Ok(app_event) => {
                            let result = event_handler.event_reducer(app_event).await;
                            if let Err(err) = result {
                                println!("Error: {}", err);
                            }
                        }
                        Err(e) => {
                            println!("Error Error Error Error {}", e);
                        }
                    }
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
