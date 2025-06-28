use sqlx::SqlitePool;

use crate::models::{Chunk, ChunkCount, Download, FileInfo};

pub async fn insert_new_download(
    pool: &SqlitePool,
    file_info: FileInfo,
    chunk_count: ChunkCount,
) -> Result<i64, String> {
    let status = "queued";
    sqlx::query!(
        "INSERT INTO downloads (status, file_path, chunk_count, url, total_bytes, file_name, extension, content_type) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        status,
        file_info.file_path,
        chunk_count,
        file_info.url,
        file_info.total_bytes,
        file_info.file_name,
        file_info.extension,
        file_info.content_type,
    )
    .execute(pool)
    .await
    .map(|op| op.last_insert_rowid())
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

pub async fn get_downloads_by_id(pool: &SqlitePool, id: i64) -> Result<Download, String> {
    sqlx::query_as!(
        Download,
        r#"
        SELECT
	d.id,
	d.url,
	d.created_at,
	d.total_bytes,
	d.chunk_count,
	d.status,
    d.file_path,
    d.file_name,
    d.content_type,
    d.extension,
	COALESCE(
		(
			SELECT
				SUM(dc.downloaded_bytes)
			FROM
				download_chunks dc
			WHERE
				dc.download_id = d.id
		),
		0
	) AS downloaded_bytes
FROM
	downloads d
WHERE
	d.id = ?
    "#,
        id,
    )
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())
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
	d.chunk_count,
	d.status,
    d.file_path,
    d.file_name,
    d.content_type,
    d.extension,
	COALESCE(
		(
			SELECT
				SUM(dc.downloaded_bytes)
			FROM
				download_chunks dc
			WHERE
				dc.download_id = d.id
		),
		0
	) AS downloaded_bytes
FROM
	downloads d
ORDER BY
	d.created_at DESC;

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

pub async fn get_download_chunks_by_download_id(
    pool: &SqlitePool,
    id: i64,
) -> Result<Vec<Chunk>, String> {
    sqlx::query_as!(
        Chunk,
        r#"SELECT dc.download_id,
            dc.chunk_index,
            dc.start_byte,
            dc.end_byte,
            dc.downloaded_bytes,
            dc.expected_hash,
            dc.has_error,
            dc.error_message,
            d.url,
            d.file_path,
            d.total_bytes
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
    expected_hash: String,
    has_error: bool,
    error_message: &str,
) -> Result<(), String> {
    sqlx::query!(
        "UPDATE download_chunks SET (downloaded_bytes, expected_hash, has_error, error_message) = (?, ?, ?, ?) WHERE download_id = ? AND chunk_index = ?",
        downloaded_bytes,
        expected_hash,
        has_error,
        error_message,
        download_id,
        chunk_index
    )
    .execute(pool)
    .await
    .map(|_| ())
    .map_err(|e| e.to_string())
}

pub async fn reset_downloaded_chunks(
    pool: &SqlitePool,
    download_id: i64,
    chunk_indexes: Vec<i64>,
) -> Result<(), String> {
    let query = format!(
        "UPDATE download_chunks SET downloaded_bytes = 0, expected_hash = NULL WHERE download_id = ? AND chunk_index IN ({})",
        chunk_indexes.iter().map(|_| "?").collect::<Vec<_>>().join(", ")
    );

    let mut query_builder = sqlx::query(&query).bind(download_id);

    for idx in &chunk_indexes {
        query_builder = query_builder.bind(idx);
    }

    query_builder
        .execute(pool)
        .await
        .map(|_| ())
        .map_err(|e| e.to_string())
}
