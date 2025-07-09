use serde::{Deserialize, Serialize};
use sqlx::FromRow;

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
    pub chunk_index: i64,
    pub downloaded_bytes: Option<i64>,
    pub error_message: Option<String>,
    pub expected_hash: Option<String>,
    pub has_error: Option<bool>,
}
