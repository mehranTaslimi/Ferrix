use std::pin::Pin;

use futures_util::Stream;
use tauri::http::{header::RANGE, Method};
use tauri_plugin_http::reqwest::Error;
use tokio_util::bytes::Bytes;

impl super::Client {
    pub async fn stream(
        &self,
        range: Option<(i64, i64)>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<Bytes, Error>> + Send>>, String> {
        let request = self.client.request(Method::GET, &self.url);
        let request = Self::auth_handler(request, &self.auth);

        let request = if let Some(range) = range {
            let (start, end) = range;
            let range_header = format!("bytes={}-{}", start, end);
            request.header(RANGE, range_header)
        } else {
            request
        };

        let response = request.send().await.map_err(|e| e.to_string())?;

        if response.status().is_success() {
            Ok(Box::pin(response.bytes_stream()))
        } else {
            Err("error".to_string())
        }
    }
}
