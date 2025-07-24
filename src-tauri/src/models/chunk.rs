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
    pub expected_hash: Option<String>,
}
