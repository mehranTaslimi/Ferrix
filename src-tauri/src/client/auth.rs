use serde::{Deserialize, Serialize};
use tauri::http::{
    header::{AUTHORIZATION, COOKIE},
    HeaderName, HeaderValue,
};
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
            Some(AuthType::Bearer { token }) => {
                return request.bearer_auth(token);
            }
            Some(AuthType::CustomToken { scheme, token }) => {
                let value = format!("{} {}", scheme, token);
                return request.header(AUTHORIZATION, value);
            }
            Some(AuthType::ApiKeyHeader { header_name, key }) => {
                if let Ok(header_name) = HeaderName::from_bytes(header_name.as_bytes()) {
                    if let Ok(header_value) = HeaderValue::from_str(key) {
                        return request.header(header_name.clone(), header_value);
                    }
                }
            }
            Some(AuthType::ApiKeyQuery { key_name, key }) => {
                return request.query(&[(key_name, key)]);
            }
            Some(AuthType::Cookie { cookie }) => {
                return request.header(COOKIE, cookie);
            }
            None => {}
        }

        return request;
    }
}
