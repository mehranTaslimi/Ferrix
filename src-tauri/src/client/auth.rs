use serde::{Deserialize, Serialize};
use tauri_plugin_http::reqwest::RequestBuilder;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum AuthType {
    Basic { username: String, password: String },
    Bearer { token: String },
    CustomToken { scheme: String, token: String },
    ApiKeyHeader { header_name: String, key: String },
    ApiKeyQuery { key_name: String, key: String },
    Cookie { cookie: String },
}

impl super::Client {
    pub(super) fn auth_handler(request: RequestBuilder, auth: &Option<AuthType>) -> RequestBuilder {
        match auth {
            Some(AuthType::Basic { username, password }) => {
                return request.basic_auth(username, Some(password));
            }
            _ => {}
        }

        request
    }
}
