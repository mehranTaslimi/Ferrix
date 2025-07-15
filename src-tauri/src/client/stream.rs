use std::pin::Pin;

use futures_util::{Stream, StreamExt};
use tauri::http::{header::RANGE, Method};
use tokio_util::bytes::Bytes;

impl super::Client {
    pub async fn stream(
        &self,
        range: Option<(i64, i64)>,
    ) -> Result<
        Pin<Box<dyn Stream<Item = Result<Bytes, super::ClientError>> + Send>>,
        super::ClientError,
    > {
        let request = self.client.request(Method::GET, &self.url);
        let request = Self::auth_handler(request, &self.auth);

        let request = if let Some(range) = range {
            let (start, end) = range;
            let range_header = format!("bytes={}-{}", start, end);
            request.header(RANGE, range_header)
        } else {
            request
        };

        let response = request.send().await?;

        let stream = response
            .bytes_stream()
            .map(|res| res.map_err(super::ClientError::from));

        return Ok(Box::pin(stream));
    }
}
