use futures_util::{future::try_join_all, StreamExt};
use serde::{Deserialize, Serialize};
use tauri::{async_runtime::spawn, http::header::RANGE, AppHandle, Emitter};
use tauri_plugin_http::reqwest::Client;

#[derive(Serialize, Deserialize, Clone)]
struct Progress {
    chunk: u8,
    progress: f64,
}

#[tauri::command]
pub async fn download_file(
    app_handle: AppHandle,
    url: String,
    chunk_count: Option<u8>,
) -> Result<(), String> {
    let chunk_count = chunk_count.unwrap_or(5).clamp(1, 5);

    let ranges = get_ranges(&url, chunk_count).await?;

    let tasks = ranges.into_iter().enumerate().map(|(i, range)| {
        let url = url.clone();
        let app_handle_clone = app_handle.clone();
        spawn(async move {
            download_chunk(&url, range, |progress| {
                let _ = app_handle_clone.emit(
                    "download_progress",
                    Progress {
                        progress,
                        chunk: i as u8,
                    },
                );
            })
            .await
        })
    });

    try_join_all(tasks).await.map_err(|e| e.to_string())?;

    Ok(())
}

async fn get_ranges(url: &String, chunk_count: u8) -> Result<Vec<(u64, u64)>, String> {
    let client = Client::new();
    let content_len = client
        .head(url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .headers()
        .get("content-length")
        .and_then(|f| f.to_str().ok())
        .and_then(|f| f.parse::<u64>().ok())
        .unwrap_or(0);

    let chunk_size = content_len / chunk_count as u64;

    let ranges: Vec<(u64, u64)> = (0..chunk_count)
        .map(|i| {
            let start = i as u64 * chunk_size;
            let end = (((i + 1) as u64) * chunk_size) - if i + 1 == chunk_count { 0 } else { 1 };

            (start, end)
        })
        .collect();

    Ok(ranges)
}

async fn download_chunk<T>(url: &String, range: (u64, u64), on_progress: T) -> Result<(), String>
where
    T: Fn(f64) + Send + Sync,
{
    let client = Client::new();

    let (start, end) = range;
    let range_header = format!("bytes={}-{}", start, end);

    let response = client
        .get(url)
        .header(RANGE, range_header)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let total_size = response.content_length().unwrap_or(0) as f64;
    let mut downloaded = 0f64;
    let mut progress = 0f64;

    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| e.to_string())?;
        downloaded += chunk.len() as f64;
        calc_progress(&downloaded, &total_size, &mut progress, |prog| {
            on_progress(prog);
        });
    }

    Ok(())
}

fn to_fixed(value: f64, decimals: u32) -> f64 {
    let factor = 10f64.powi(decimals as i32);
    (value * factor).round() / factor
}

fn calc_progress<T>(downloaded: &f64, total_size: &f64, progress: &mut f64, callback: T)
where
    T: Fn(f64) + Send + Sync,
{
    let new_progress = to_fixed((downloaded / total_size) * 100.0, 2);
    if *progress < new_progress {
        *progress = new_progress;
        callback(*progress)
    };
}
