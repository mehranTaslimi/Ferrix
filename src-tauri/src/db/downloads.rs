use sqlx::SqlitePool;

use crate::models::{Download, DownloadChunk};

pub async fn insert_new_download(
    pool: &SqlitePool,
    url: &str,
    total_bytes: i64,
) -> Result<i64, String> {
    let status = "queued";

    sqlx::query!(
        "INSERT INTO downloads (url, total_bytes, status) VALUES (?, ?, ?)",
        url,
        total_bytes,
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

        sqlx::query!("INSERT INTO download_chunks (download_id, chunk_index, start_byte, end_byte) VALUES (?, ?, ?, ?)",id,chunk_index,start,end)
        .execute(pool)
        .await
        .map_err(|e|e.to_string())?;
    }

    Ok(())
}

pub async fn get_downloads_list(pool: &SqlitePool) -> Result<Vec<Download>, String> {
    sqlx::query_as!(
        Download,
        r#"
        SELECT 
            d.id,
            d.url,
            d.created_at,
            d.total_bytes,
            COALESCE(
                (SELECT SUM(dc.downloaded_bytes)
                 FROM download_chunks dc
                 WHERE dc.download_id = d.id),
                0
            ) AS downloaded_bytes,
            CASE 
                WHEN d.total_bytes > 0 THEN 
                    ROUND(
                        COALESCE(
                            (SELECT SUM(dc.downloaded_bytes)
                             FROM download_chunks dc
                             WHERE dc.download_id = d.id),
                            0
                        ) * 100.0 / d.total_bytes,
                        2
                    )
                ELSE 0
            END AS progress_percent,
            d.status
        FROM downloads d
        ORDER BY d.created_at DESC
    "#
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
            dc.start_byte,
            dc.end_byte,
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
