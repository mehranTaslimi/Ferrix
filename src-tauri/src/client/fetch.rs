use super::*;

impl Client {
    pub async fn fetch_bytes(&self) -> anyhow::Result<tokio_util::bytes::Bytes> {
        let request = self.client.get(&self.url);
        let response = request.send().await?;
        let status = response.status();
        let bytes = response.bytes().await?;

        if !status.is_success() {
            return Err(super::ClientError::Http { status }.into());
        }

        Ok(bytes)
    }
}
