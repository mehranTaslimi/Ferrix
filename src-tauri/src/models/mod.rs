use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Download {
    pub id: i64,
    pub url: String,
    pub content_length: i64,
    pub status: String,
}

#[derive(Serialize, Deserialize)]
pub struct DownloadWithDownloadedBytes {
    pub id: i64,
    pub url: String,
    pub content_length: i64,
    pub status: String,
    pub downloaded_bytes: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DownloadChunk {
    pub download_id: String,
    pub chunk_index: i64,
    pub start: i64,
    pub end: i64,
    pub downloaded_bytes: i64,
    pub url: String,
}

impl DownloadChunk {
    pub fn percent(&self) -> f64 {
        let total = (self.end - self.start + 1) as f64;
        (self.downloaded_bytes as f64 / total) * 100.0
    }
}
