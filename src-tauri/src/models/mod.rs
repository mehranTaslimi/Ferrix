use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Download {
    pub id: i64,
    pub url: String,
    pub total_bytes: i64,
    pub status: String,
    pub created_at: Option<NaiveDateTime>,
    pub downloaded_bytes: Option<i64>,
    pub progress_percent: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DownloadChunk {
    pub download_id: String,
    pub chunk_index: i64,
    pub start_byte: i64,
    pub end_byte: i64,
    pub downloaded_bytes: i64,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DownloadReport {
    pub id: i64,
    pub downloaded_bytes_chunks: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DownloadSpeed {
    pub id: i64,
    pub speed_kbps: f64,
}
