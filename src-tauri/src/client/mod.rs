use tauri_plugin_http::reqwest::Client as ReqwestClient;

mod auth;
mod builder;
mod inspect;
mod proxy;
mod stream;

pub use auth::AuthType;
pub use proxy::ProxyType;

pub struct Client {
    url: String,
    client: ReqwestClient,
    auth: AuthType,
    proxy: ProxyType,
}
