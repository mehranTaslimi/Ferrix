use std::collections::HashMap;

use tauri_plugin_http::reqwest::Client as ReqwestClient;

mod auth;
mod builder;
mod cookies;
mod headers;
mod inspect;
mod proxy;
mod stream;

pub use auth::AuthType;
pub use proxy::*;

pub struct Client {
    url: String,
    client: ReqwestClient,
    auth: Option<AuthType>,
    proxy: Option<ProxyType>,
    headers: Option<HashMap<String, String>>,
    cookies: Option<HashMap<String, String>>,
}
