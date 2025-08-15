use std::sync::Arc;

use tauri::http::StatusCode;
use tauri_plugin_http::reqwest;
use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum ClientError {
    #[error("{0}")]
    Reqwest(#[from] Arc<reqwest::Error>),

    #[error("{status}")]
    Http { status: StatusCode },

    #[error("{0}")]
    Deserialize(#[from] Arc<serde_json::Error>),

    #[error("missing or invalid Content-Length header")]
    MissingContentLength,

    #[error("timeout error")]
    StreamTimeout,

    #[error("unexpected chunk hash")]
    UnexpectedChunkHash,
}

impl From<reqwest::Error> for ClientError {
    fn from(value: reqwest::Error) -> Self {
        ClientError::Reqwest(Arc::new(value))
    }
}

impl From<serde_json::Error> for ClientError {
    fn from(value: serde_json::Error) -> Self {
        ClientError::Deserialize(Arc::new(value))
    }
}

impl ClientError {
    pub fn is_retryable(&self) -> bool {
        match self {
            ClientError::Reqwest(e) => e.is_timeout() || e.is_connect() || e.is_request(),
            ClientError::Http { status, .. } => {
                matches!(status.as_u16(), 408 | 429 | 500..=504)
            }
            ClientError::StreamTimeout | ClientError::UnexpectedChunkHash => true,
            _ => false,
        }
    }
}
