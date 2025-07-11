use serde::{Deserialize, Serialize};
use tauri_plugin_http::reqwest::{self, Proxy};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ProxyType {
    Http {
        host: String,
        port: u16,
        #[serde(default)]
        username: Option<String>,
        #[serde(default)]
        password: Option<String>,
    },
    Socks5 {
        host: String,
        port: u16,
        #[serde(default)]
        username: Option<String>,
        #[serde(default)]
        password: Option<String>,
    },
    Https {
        host: String,
        port: u16,
        #[serde(default)]
        username: Option<String>,
        #[serde(default)]
        password: Option<String>,
    },
    System,
}

impl super::Client {
    fn build_proxy_url(
        scheme: &str,
        host: &str,
        port: u16,
        username: &Option<String>,
        password: &Option<String>,
    ) -> String {
        let mut url = format!("{scheme}://{host}:{port}");

        if let (Some(user), Some(pass)) = (username, password) {
            url = format!("{scheme}://{user}:{pass}@{host}:{port}");
        }

        url
    }

    fn build_proxy(proxy: &ProxyType) -> Option<String> {
        match proxy {
            ProxyType::Http {
                host,
                port,
                username,
                password,
            } => Some(Self::build_proxy_url(
                "http", host, *port, username, password,
            )),
            ProxyType::Https {
                host,
                port,
                username,
                password,
            } => Some(Self::build_proxy_url(
                "https", host, *port, username, password,
            )),
            ProxyType::Socks5 {
                host,
                port,
                username,
                password,
            } => Some(Self::build_proxy_url(
                "socks5h", host, *port, username, password,
            )),
            _ => None,
        }
    }

    pub(super) fn get_proxy(proxy: &ProxyType) -> Result<Proxy, reqwest::Error> {
        let proxy = Self::build_proxy(proxy).unwrap();
        Proxy::all(proxy)
    }
}
