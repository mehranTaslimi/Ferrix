use std::{collections::HashMap, str::FromStr, sync::Arc};

use cookie::Cookie;
use tauri::Url;
use tauri_plugin_http::reqwest::cookie::Jar;

impl super::Client {
    pub(super) fn get_cookie_jar(url: &str, custom_cookies: HashMap<String, String>) -> Arc<Jar> {
        let jar = Jar::default();
        let url = Url::from_str(url).unwrap();

        for (name, value) in custom_cookies {
            let cookie = Cookie::new(name, value);
            jar.add_cookie_str(&cookie.to_string(), &url);
        }

        Arc::new(jar)
    }
}
