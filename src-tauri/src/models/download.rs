use std::{collections::HashMap, path::Path};

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::client::{AuthType, ProxyType};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DownloadRaw {
    pub id: i64,
    pub url: String,
    pub total_bytes: i64,
    pub downloaded_bytes: i64,
    pub status: String,
    pub created_at: Option<NaiveDateTime>,
    pub modified_at: Option<NaiveDateTime>,
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
    pub supports_range: bool,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Download {
    pub id: i64,
    pub url: String,
    pub total_bytes: i64,
    pub downloaded_bytes: i64,
    pub status: String,
    pub created_at: Option<NaiveDateTime>,
    pub modified_at: Option<NaiveDateTime>,
    pub chunk_count: i64,
    pub file_path: String,
    pub file_name: String,
    pub content_type: String,
    pub extension: String,
    pub auth: Option<AuthType>,
    pub proxy: Option<ProxyType>,
    pub headers: Option<HashMap<String, String>>,
    pub cookies: Option<HashMap<String, String>>,
    pub speed_limit: Option<i64>,
    pub max_retries: i64,
    pub delay_secs: f64,
    pub backoff_factor: f64,
    pub timeout_secs: i64,
    pub file_exist: bool,
    pub supports_range: bool,
    pub error_message: Option<String>,
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
    pub supports_range: i64,
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
    pub error_message: Option<String>,
}

impl TryFrom<DownloadRaw> for Download {
    type Error = sqlx::Error;

    fn try_from(raw: DownloadRaw) -> Result<Self, Self::Error> {
        let proxy = match raw.proxy {
            Some(str) => {
                Some(serde_json::from_str(&str).map_err(|e| sqlx::Error::Decode(Box::new(e)))?)
            }
            None => None,
        };
        let auth = match raw.auth {
            Some(str) => {
                Some(serde_json::from_str(&str).map_err(|e| sqlx::Error::Decode(Box::new(e)))?)
            }
            None => None,
        };
        let headers = match raw.headers {
            Some(str) => {
                Some(serde_json::from_str(&str).map_err(|e| sqlx::Error::Decode(Box::new(e)))?)
            }
            None => None,
        };
        let cookies = match raw.cookies {
            Some(str) => {
                Some(serde_json::from_str(&str).map_err(|e| sqlx::Error::Decode(Box::new(e)))?)
            }
            None => None,
        };

        Ok(Download {
            id: raw.id,
            url: raw.url,
            total_bytes: raw.total_bytes,
            downloaded_bytes: raw.downloaded_bytes,
            status: raw.status,
            created_at: raw.created_at,
            modified_at: raw.modified_at,
            chunk_count: raw.chunk_count,
            file_exist: Path::new(&raw.file_path).exists(),
            file_path: raw.file_path,
            file_name: raw.file_name,
            content_type: raw.content_type,
            extension: raw.extension,
            speed_limit: raw.speed_limit,
            max_retries: raw.max_retries,
            delay_secs: raw.delay_secs,
            backoff_factor: raw.backoff_factor,
            timeout_secs: raw.timeout_secs,
            supports_range: raw.supports_range,
            error_message: raw.error_message,
            auth,
            proxy,
            headers,
            cookies,
        })
    }
}
