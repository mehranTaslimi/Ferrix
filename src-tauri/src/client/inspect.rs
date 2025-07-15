use mime2ext::mime2ext;
use tauri::http::{
    header::{ACCEPT_RANGES, CONTENT_DISPOSITION, CONTENT_LENGTH, CONTENT_TYPE},
    Method,
};

#[derive(Clone, Debug)]
pub struct InspectResponse {
    pub supports_range: bool,
    pub content_length: u64,
    pub content_type: String,
    pub file_name: String,
    pub extension: String,
    pub url: String,
}

impl super::Client {
    pub async fn inspect(&self) -> Result<InspectResponse, super::ClientError> {
        let request = self.client.request(Method::HEAD, &self.url);
        let request = Self::auth_handler(request, &self.auth);

        let response = request.send().await?;
        let status = response.status();
        let final_url = response.url().clone();
        let headers = response.headers().clone();
        let body = response.text().await?;

        if !status.is_success() {
            return Err(super::ClientError::Http {
                status,
                message: body,
            });
        }

        let supports_range = headers
            .get(ACCEPT_RANGES)
            .map(|v| v == "bytes")
            .unwrap_or(false);

        let content_length = headers
            .get(CONTENT_LENGTH)
            .and_then(|f| f.to_str().ok())
            .and_then(|f| f.parse::<u64>().ok())
            .ok_or(super::ClientError::MissingContentLength)?;

        let content_type = headers
            .get(CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("application/octet-stream");

        let extension = mime2ext(content_type).unwrap_or("bin");

        let file_name = headers
            .get(CONTENT_DISPOSITION)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| {
                v.split(';')
                    .find_map(|part| part.trim().strip_prefix("filename="))
                    .map(|name| name.trim_matches('"').to_string())
            })
            .or_else(|| {
                final_url
                    .path_segments()
                    .and_then(|segments| segments.filter(|s| !s.is_empty()).last())
                    .map(|s| s.to_string())
            })
            .or_else(|| {
                final_url
                    .domain()
                    .map(|domain| format!("{domain}.{extension}"))
            })
            .unwrap_or_else(|| format!("file.{extension}"));

        Ok(InspectResponse {
            url: final_url.to_string(),
            file_name: file_name.to_string(),
            content_type: content_type.to_string(),
            content_length,
            extension: extension.to_string(),
            supports_range,
        })
    }
}
