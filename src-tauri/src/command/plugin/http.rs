use tauri_plugin_http::reqwest::Client;

#[tauri::command]
pub async fn api_http_head(url: String) -> Result<(), String> {
    let client = Client::new();
    client.head(&url).send().await.map_err(|e| e.to_string())?;
    Ok(())
}
