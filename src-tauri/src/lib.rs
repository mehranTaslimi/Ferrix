use tauri::Manager;
use tokio::{spawn, sync::broadcast};

mod client;
mod command;
mod db;
mod events;
mod manager;
mod models;
mod registry;
mod utils;
mod worker;

use crate::{
    events::dispatch,
    manager::DownloadsManager,
    registry::Registry,
    utils::app_state::{AppEvent, AppState},
};

#[tokio::main]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    Registry::new().await;

    let (tx, _) = broadcast::channel(1000);
    let app_state = AppState::new(tx.clone()).await;

    let tx = tx.clone();
    let mut rx = tx.subscribe();

    tauri::Builder::default()
        .manage(app_state)
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            command::add_new_download,
            command::get_download_list,
            command::resume_download,
            command::pause_download
        ])
        .setup(move |app| {
            let window = app.get_webview_window("main").unwrap();

            #[cfg(target_os = "macos")]
            window_vibrancy::apply_vibrancy(
                &window,
                window_vibrancy::NSVisualEffectMaterial::Sidebar,
                None,
                None,
            )
            .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

            let manager = DownloadsManager::new(tx, app.handle().clone());

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
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let state = window.app_handle().state::<AppState>();
                dispatch(&state.broadcast_tx, AppEvent::ForcePauseAllDownloadWorkers);
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
