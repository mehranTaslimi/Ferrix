use std::path::Path;

use dirs_next;
use tokio::fs;

pub struct FileName {
    pub full_path: String,
    pub file_name: String,
}

pub async fn create_download_dir() -> Result<String, String> {
    let home_dir = dirs_next::home_dir().ok_or("Cannot find home directory".to_string())?;
    let download_dir = home_dir
        .join("Downloads".to_string())
        .join("download-manager".to_string());

    if fs::metadata(&download_dir).await.is_err() {
        fs::create_dir_all(&download_dir)
            .await
            .map_err(|e| e.to_string())?;
    }

    let result = download_dir
        .to_str()
        .ok_or("failed to convert path to string".to_string())?
        .to_string();

    Ok(result)
}

pub async fn get_available_filename(full_path: &str) -> Result<FileName, String> {
    let path = Path::new(full_path);

    if fs::metadata(path).await.is_err() {
        return Ok(FileName {
            full_path: full_path.to_string(),
            file_name: path
                .file_name()
                .ok_or("cannot get file name from full path")?
                .to_string_lossy()
                .into_owned(),
        });
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
            return Ok(FileName {
                full_path: candidate.to_string_lossy().into_owned(),
                file_name: candidate
                    .file_name()
                    .ok_or("cannot get file name from candidate files")?
                    .to_string_lossy()
                    .into_owned(),
            });
        }
    }

    Err("error".to_string())
}
