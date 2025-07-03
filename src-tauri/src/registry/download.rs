use std::collections::HashMap;

use serde::Deserialize;

use crate::client::{AuthType, Client, ProxyType};

#[derive(Debug, Deserialize)]
pub struct DownloadOptions {
    save_path: Option<String>,
    chunk_count: usize,
    proxy: Option<ProxyType>,
    auth: Option<AuthType>,
    headers: Option<HashMap<String, String>>,
    cookies: Option<HashMap<String, String>>,
    speed_limit: Option<u64>,
    max_retries: Option<u8>,
    delay_secs: Option<f64>,
    backoff_factor: Option<f64>,
    timeout_secs: Option<f64>,
}

impl super::Registry {
    pub(super) async fn add_new_download(
        url: String,
        options: DownloadOptions,
    ) -> Result<(), String> {
        let client = Client::new(&url, AuthType::None, ProxyType::None)?;
        let response = client.inspect().await?;

        Ok(())
    }
}
