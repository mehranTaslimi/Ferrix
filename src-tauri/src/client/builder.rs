use tauri_plugin_http::reqwest::Client as ReqwestClient;

impl super::Client {
    pub fn new(
        url: &str,
        auth: &Option<super::AuthType>,
        proxy: &Option<super::ProxyType>,
    ) -> Result<Self, String> {
        let client = ReqwestClient::builder()
            .build()
            .map_err(|e| e.to_string())?;

        Ok(Self {
            url: url.to_string(),
            client,
            auth: auth.clone(),
            proxy: proxy.clone(),
        })
    }
}
