use std::collections::HashMap;
use tauri_plugin_http::reqwest::Client as ReqwestClient;

impl super::Client {
    pub fn new(
        url: &str,
        auth: &Option<super::AuthType>,
        proxy: &Option<super::ProxyType>,
        headers: &Option<HashMap<String, String>>,
        cookies: &Option<HashMap<String, String>>,
    ) -> Result<Self, String> {
        let mut builder =
            ReqwestClient::builder().default_headers(Self::get_default_headers(&headers));

        if let Some(cookies) = cookies {
            let jar = Self::get_cookie_jar(url, cookies.to_owned());
            builder = builder.cookie_provider(jar);
        }

        let client = builder.build().map_err(|e| e.to_string())?;

        Ok(Self {
            client,
            url: url.to_string(),
            auth: auth.clone(),
            proxy: proxy.clone(),
            headers: headers.clone(),
            cookies: cookies.clone(),
        })
    }
}
