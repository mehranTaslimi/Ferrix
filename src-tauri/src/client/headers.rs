use std::{collections::HashMap, env};

use tauri::http::{header::USER_AGENT, HeaderMap, HeaderName, HeaderValue};

use crate::emitter::Emitter;

impl super::Client {
    fn default_user_agent() -> String {
        format!(
            "Ferrix/{version} ({os} {arch}; +https://github.com/mehranTaslimi/Ferrix)",
            version = env!("CARGO_PKG_VERSION"),
            os = env::consts::OS,
            arch = env::consts::ARCH,
        )
    }

    pub(super) fn get_default_headers(
        custom_headers: &Option<HashMap<String, String>>,
    ) -> HeaderMap {
        let mut headers = HeaderMap::new();

        let val = HeaderValue::from_str(&Self::default_user_agent()).unwrap();
        headers.insert(USER_AGENT, val);

        if let Some(custom_headers) = custom_headers {
            for (key, value) in custom_headers {
                if let (Ok(header_name), Ok(header_value)) = (
                    HeaderName::from_bytes(key.as_bytes()),
                    HeaderValue::from_str(value),
                ) {
                    headers.insert(header_name, header_value);
                } else {
                    Emitter::emit_error(format!("Invalid header: {}: {}", key, value));
                }
            }
        }

        headers
    }
}
