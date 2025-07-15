use tauri::http::StatusCode;
use tauri_plugin_http::reqwest;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Network error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Request failed with status {status}: {message}")]
    Http { status: StatusCode, message: String },

    #[error("Deserialization error: {0}")]
    Deserialize(#[from] serde_json::Error),

    #[error("Missing or invalid Content-Length header")]
    MissingContentLength,
}
