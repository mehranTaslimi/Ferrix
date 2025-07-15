use tauri_plugin_http::reqwest::Client as ReqwestClient;

mod auth;
mod builder;
mod cookies;
mod error;
mod headers;
mod inspect;
mod proxy;
mod stream;

pub use auth::AuthType;
pub use error::*;
pub use proxy::*;

pub struct Client {
    url: String,
    client: ReqwestClient,
    auth: Option<AuthType>,
}
