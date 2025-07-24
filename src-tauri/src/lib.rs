use tauri::Manager;
use tokio::spawn;

mod client;
mod command;
mod emitter;
mod file;
mod manager;
mod models;
mod registry;
mod repository;
mod worker;

#[macro_use]
mod macros;

use crate::registry::Registry;

#[tokio::main]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    dotenvy::dotenv().ok();
    env_logger::builder()
        .format_module_path(false)
        .format_level(false)
        .format_timestamp(None)
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            command::add_new_download,
            command::get_download_list,
            command::resume_download,
            command::pause_download,
            command::remove_download
        ])
        .setup(move |app| {
            let window = app.get_webview_window("main").unwrap();
            let app_handle = app.app_handle().clone();

            #[cfg(target_os = "macos")]
            window_vibrancy::apply_vibrancy(
                &window,
                window_vibrancy::NSVisualEffectMaterial::Sidebar,
                None,
                None,
            )
            .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

            spawn(async move {
                Registry::new(app_handle).await;
            });

            Ok(())
        })
        .on_window_event(|_, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                dispatch!(registry, CloseRequested);
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
