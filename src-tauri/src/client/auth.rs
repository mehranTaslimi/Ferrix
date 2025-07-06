use serde::{Deserialize, Serialize};
use tauri_plugin_http::reqwest::RequestBuilder;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum AuthType {
    Basic {
        username: String,
        password: String,
    },
    Bearer {
        token: String,
    },
    CustomToken {
        scheme: String,
        token: String,
    },
    ApiKeyHeader {
        header_name: String,
        key: String,
    },
    ApiKeyQuery {
        key_name: String,
        key: String,
    },
    Cookie {
        cookie: String,
    },
    ClientCertificate {
        pem_path: String,
        #[serde(default)]
        password: Option<String>,
    },
    None,
}

impl super::Client {
    pub(super) fn auth_handler(request: RequestBuilder, auth: &AuthType) -> RequestBuilder {
        match auth {
            AuthType::Basic { username, password } => {
                return request.basic_auth(username, Some(password));
            }
            _ => {}
        }

        request
    }
}
