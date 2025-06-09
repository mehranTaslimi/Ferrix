use sqlx::SqlitePool;

use crate::models::{Download, DownloadChunk, DownloadWithDownloadedBytes};

pub async fn insert_new_download(
    pool: &SqlitePool,
    url: &str,
    content_length: i64,
) -> Result<i64, String> {
    let status = "queued";

    sqlx::query!(
        "INSERT INTO downloads (url, content_length, status) VALUES (?, ?, ?)",
        url,
        content_length,
        status
    )
    .execute(pool)
    .await
    .map(|res| res.last_insert_rowid())
    .map_err(|e| e.to_string())
}

pub async fn insert_download_chunks(
    pool: &SqlitePool,
    id: i64,
    ranges: Vec<(u64, u64)>,
) -> Result<(), String> {
    for (i, (start, end)) in ranges.iter().enumerate() {
        let chunk_index = i as i64;
        let start = *start as i64;
        let end = *end as i64;

        sqlx::query!("INSERT INTO download_chunks (download_id, chunk_index, start, end) VALUES (?, ?, ?, ?)",id,chunk_index,start,end)
        .execute(pool)
        .await
        .map_err(|e|e.to_string())?;
    }

    Ok(())
}

pub async fn get_downloads_list(
    pool: &SqlitePool,
) -> Result<Vec<DownloadWithDownloadedBytes>, String> {
    sqlx::query_as!(
        DownloadWithDownloadedBytes,
        r#"SELECT d.id,
        d.url,
    d.content_length,
    (
        SELECT SUM(dc.downloaded_bytes)
        FROM download_chunks dc
        WHERE dc.download_id = d.id
    ) AS downloaded_bytes,
    d.status
FROM downloads d;"#
    )
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())
}

pub async fn update_download_status(
    pool: &SqlitePool,
    id: i64,
    status: &str,
) -> Result<(), String> {
    sqlx::query!("UPDATE downloads SET status = ? WHERE id = ?", status, id)
        .execute(pool)
        .await
        .map(|_| ())
        .map_err(|e| e.to_string())
}

pub async fn get_download_chunks(pool: &SqlitePool, id: i64) -> Result<Vec<DownloadChunk>, String> {
    sqlx::query_as!(
        DownloadChunk,
        r#"SELECT dc.download_id,
dc.chunk_index,
dc.start,
dc.
end,
dc.downloaded_bytes,
d.url
FROM download_chunks dc
    JOIN downloads d ON d.id = dc.download_id
WHERE dc.download_id = ?;"#,
        id
    )
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())
}

pub async fn update_chunk_downloaded(
    pool: &SqlitePool,
    download_id: i64,
    chunk_index: i64,
    downloaded_bytes: i64,
) -> Result<(), String> {
    sqlx::query!(
        "UPDATE download_chunks SET downloaded_bytes = ? WHERE download_id = ? AND chunk_index = ?",
        downloaded_bytes,
        download_id,
        chunk_index
    )
    .execute(pool)
    .await
    .map(|_| ())
    .map_err(|e| e.to_string())
}

pub async fn get_download_item_info_by_id(
    pool: &SqlitePool,
    id: i64,
) -> Result<Vec<Download>, String> {
    sqlx::query_as!(Download, "SELECT * FROM downloads")
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())
}
