use std::path::Path;
use tauri::Manager;
use tokio::fs;

use crate::registry::Registry;

impl super::File {
    pub async fn get_default_path(file_name: &str) -> Result<String, String> {
        let home_dir = Registry::get_state()
            .app_handle
            .path()
            .home_dir()
            .map_err(|e| e.to_string())?;

        let mut download_dir = home_dir
            .join("Downloads".to_string())
            .join("ferrix".to_string());

        if fs::metadata(&download_dir).await.is_err() {
            fs::create_dir_all(&download_dir)
                .await
                .map_err(|e| e.to_string())?;
        }

        download_dir.push(file_name);

        let result = download_dir.to_str().unwrap().to_string();

        Ok(result)
    }

    pub fn get_file_name(file_path: &str) -> Result<String, String> {
        let path = Path::new(file_path);
        let file_name = path
            .file_name()
            .ok_or("cannot get filename from full path")?;

        Ok(file_name.to_string_lossy().into_owned())
    }

    pub async fn get_available_filename(full_path: &str) -> Result<String, String> {
        let path = Path::new(full_path);

        if fs::metadata(path).await.is_err() {
            return Ok(full_path.to_string());
        }

        let file_stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("file");
        let ext = path.extension().and_then(|e| e.to_str());
        let dir = path.parent().unwrap_or_else(|| Path::new("."));

        for i in 1.. {
            let file_name = match ext {
                Some(e) => format!("{} ({}).{}", file_stem, i, e),
                None => format!("{} ({})", file_stem, i),
            };

            let candidate = dir.join(file_name);

            if fs::metadata(&candidate).await.is_err() {
                return Ok(candidate.to_string_lossy().into_owned());
            }
        }

        Err("cannot available filename".to_string())
    }
}
