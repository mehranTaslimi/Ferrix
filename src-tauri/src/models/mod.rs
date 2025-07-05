use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Download {
    pub id: i64,
    pub url: String,
    pub total_bytes: i64,
    pub status: String,
    pub created_at: Option<NaiveDateTime>,
    pub chunk_count: i64,
    pub file_path: String,
    pub file_name: String,
    pub content_type: String,
    pub extension: String,
    pub auth: Option<String>,
    pub proxy: Option<String>,
    pub headers: Option<String>,
    pub cookies: Option<String>,
    pub speed_limit: Option<i64>,
    pub max_retries: i64,
    pub delay_secs: f64,
    pub backoff_factor: f64,
    pub timeout_secs: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewDownload {
    pub url: String,
    pub total_bytes: i64,
    pub status: String,
    pub chunk_count: i64,
    pub file_path: String,
    pub file_name: String,
    pub content_type: String,
    pub extension: String,
    pub auth: Option<String>,
    pub proxy: Option<String>,
    pub headers: Option<String>,
    pub cookies: Option<String>,
    pub speed_limit: Option<i64>,
    pub max_retries: Option<i64>,
    pub delay_secs: Option<f64>,
    pub backoff_factor: Option<f64>,
    pub timeout_secs: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDownload {
    pub status: Option<String>,
    pub total_bytes: Option<i64>,
    pub speed_limit: Option<i64>,
    pub auth: Option<String>,
    pub proxy: Option<String>,
    pub headers: Option<String>,
    pub cookies: Option<String>,
    pub max_retries: Option<i64>,
    pub delay_secs: Option<f64>,
    pub backoff_factor: Option<f64>,
    pub timeout_secs: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DownloadChunk {
    pub download_id: i64,
    pub chunk_index: i64,
    pub start_byte: i64,
    pub end_byte: i64,
    pub downloaded_bytes: i64,
    pub expected_hash: Option<String>,
    pub error_message: Option<String>,
    pub has_error: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewChunk {
    pub download_id: i64,
    pub chunk_index: i64,
    pub start_byte: i64,
    pub end_byte: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateChunk {
    pub id: i64,
    pub downloaded_bytes: Option<i64>,
    pub error_message: Option<String>,
    pub expected_hash: Option<String>,
    pub has_error: Option<bool>,
}
