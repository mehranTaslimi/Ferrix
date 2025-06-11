use std::path::Path;

use tauri::{
    http::header::{ACCEPT_RANGES, CONTENT_DISPOSITION, CONTENT_LENGTH, CONTENT_TYPE},
    Url,
};
use tauri_plugin_http::reqwest::Client;

use crate::models::FileInfo;

pub async fn validate_and_inspect_url(url: &str) -> Result<FileInfo, String> {
    let parsed_url = Url::parse(url).map_err(|e| e.to_string())?;

    let client = Client::new();

    let response = client
        .head(parsed_url.clone())
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let headers = response.headers();

    let supports_range = headers
        .get(ACCEPT_RANGES)
        .map(|v| v == "bytes")
        .unwrap_or(false);

    if !supports_range {
        return Err("server does not support ranged downloads".to_string());
    };

    let content_length = headers
        .get(CONTENT_LENGTH)
        .and_then(|f| f.to_str().ok())
        .and_then(|f| f.parse::<u64>().ok())
        .ok_or("invalid header content-length".to_string())?;

    let content_type = headers
        .get(CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");

    let file_name = headers
        .get(CONTENT_DISPOSITION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| {
            v.split(';')
                .find_map(|part| part.trim().strip_prefix("filename="))
                .map(|name| name.trim_matches('"'))
        })
        .or_else(|| {
            parsed_url
                .path_segments()
                .and_then(|segments| segments.last())
        })
        .ok_or("invalid file name".to_string())?;

    let file_extension = Path::new(file_name)
        .extension()
        .and_then(|f| f.to_str())
        .ok_or("invalid file extension")?;

    Ok(FileInfo {
        url: parsed_url.to_string(),
        file_name: file_name.to_string(),
        content_type: content_type.to_string(),
        total_bytes: content_length as i64,
        extension: file_extension.to_string(),
    })
}
