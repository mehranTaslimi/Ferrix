use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

pub type ChunkCount = i64;
pub type DownloadId = i64;
pub type ChunkIndex = i64;
pub type DownloadedBytes = i64;
pub type TotalBytes = i64;
pub type DownloadUrl = String;
pub type DownloadStatus = String;
pub type CreatedAt = NaiveDateTime;
pub type FilePath = String;
pub type FileName = String;
pub type ContentType = String;
pub type Extension = String;
pub type ExpectedHash = Option<String>;

#[derive(Debug, Clone)]
pub struct FileInfo {
    pub url: DownloadUrl,
    pub file_name: FileName,
    pub content_type: ContentType,
    pub total_bytes: TotalBytes,
    pub extension: Extension,
    pub file_path: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Download {
    pub id: DownloadId,
    pub url: DownloadUrl,
    pub total_bytes: TotalBytes,
    pub status: DownloadStatus,
    pub created_at: Option<CreatedAt>,
    pub downloaded_bytes: DownloadedBytes,
    pub chunk_count: ChunkCount,
    pub file_path: FilePath,
    pub file_name: FileName,
    pub content_type: ContentType,
    pub extension: Extension,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Chunk {
    pub download_id: String,
    pub chunk_index: i64,
    pub start_byte: i64,
    pub end_byte: i64,
    pub downloaded_bytes: i64,
    pub url: String,
    pub expected_hash: ExpectedHash,
    pub file_path: FilePath,
    pub total_bytes: TotalBytes,
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
