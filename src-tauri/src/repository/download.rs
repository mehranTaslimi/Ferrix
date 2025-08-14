use crate::{
    models::{Download, DownloadRaw, NewDownload, UpdateDownload},
    registry::Registry,
};

pub struct DownloadRepository;

impl DownloadRepository {
    pub async fn find_all(status: Option<&str>) -> anyhow::Result<Vec<Download>> {
        let pool = Registry::get_pool();

        let query = r#"
        SELECT
            d.id,
            d.url,
            d.total_bytes,
            d.status,
            d.created_at,
            d.modified_at,
            d.chunk_count,
            d.file_path,
            d.file_name,
            d.content_type,
            d.extension,
            d.auth,
            d.proxy,
            d.headers,
            d.cookies,
            d.speed_limit,
            d.max_retries,
            d.delay_secs,
            d.backoff_factor,
            d.timeout_secs,
            d.supports_range,
            d.error_message,
            COALESCE(
                (
                    SELECT SUM(c.downloaded_bytes)
                    FROM download_chunks c
                    WHERE c.download_id = d.id
                ),
                0
            ) AS downloaded_bytes
        FROM downloads d
        LEFT JOIN download_chunks c ON c.download_id = d.id
        WHERE ($1::TEXT IS NULL OR d.status = $1)
        GROUP BY d.id
        ORDER BY
            CASE d.status
                WHEN 'downloading' THEN 0
                WHEN 'queued' THEN 1
                ELSE 2
            END,
            CASE d.status
                WHEN 'downloading' THEN d.modified_at
                ELSE d.created_at
            END DESC
    "#;

        let raw = sqlx::query_as::<_, DownloadRaw>(query)
            .bind(status)
            .fetch_all(pool)
            .await?;

        raw.into_iter()
            .map(Download::try_from)
            .collect::<Result<Vec<_>, _>>()
    }

    pub async fn find(id: i64) -> anyhow::Result<Download> {
        let pool = Registry::get_pool();
        let raw = sqlx::query_as!(
            DownloadRaw,
            r#"
            SELECT
    d.id,
    d.url,
    d.total_bytes,
    d.status,
    d.created_at,
    d.modified_at,
    d.chunk_count,
    d.file_path,
    d.file_name,
    d.content_type,
    d.extension,
    d.auth,
    d.proxy,
    d.headers,
    d.cookies,
    d.speed_limit,
    d.max_retries,
    d.delay_secs,
    d.backoff_factor,
    d.timeout_secs,
    d.supports_range,
    d.error_message,
    COALESCE(
		(
			SELECT
				SUM(c.downloaded_bytes)
			FROM
				download_chunks c
			WHERE
				c.download_id = d.id
		),
		0
	) AS downloaded_bytes
FROM downloads d
LEFT JOIN download_chunks c ON c.download_id = d.id
WHERE d.id = ?
GROUP BY d.id;
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Download::try_from(raw)
    }

    pub async fn add(new: NewDownload) -> anyhow::Result<i64> {
        let pool = Registry::get_pool();

        let mut fields = vec![
            "url",
            "status",
            "chunk_count",
            "file_path",
            "file_name",
            "content_type",
            "extension",
            "total_bytes",
            "supports_range",
        ];

        let mut values = vec!["?", "?", "?", "?", "?", "?", "?", "?", "?"];
        let mut params = vec![
            new.url,
            new.status,
            new.chunk_count.to_string(),
            new.file_path,
            new.file_name,
            new.content_type,
            new.extension,
            new.total_bytes.to_string(),
            new.supports_range.to_string(),
        ];

        if let Some(auth) = new.auth {
            fields.push("auth");
            values.push("?");
            params.push(auth);
        }
        if let Some(proxy) = new.proxy {
            fields.push("proxy");
            values.push("?");
            params.push(proxy);
        }
        if let Some(headers) = new.headers {
            fields.push("headers");
            values.push("?");
            params.push(headers);
        }
        if let Some(cookies) = new.cookies {
            fields.push("cookies");
            values.push("?");
            params.push(cookies);
        }
        if let Some(speed_limit) = new.speed_limit {
            fields.push("speed_limit");
            values.push("?");
            params.push(speed_limit.to_string());
        }
        if let Some(max_retries) = new.max_retries {
            fields.push("speed_limit");
            values.push("?");
            params.push(max_retries.to_string());
        }
        if let Some(delay_secs) = new.delay_secs {
            fields.push("speed_limit");
            values.push("?");
            params.push(delay_secs.to_string());
        }
        if let Some(backoff_factor) = new.backoff_factor {
            fields.push("speed_limit");
            values.push("?");
            params.push(backoff_factor.to_string());
        }
        if let Some(timeout_secs) = new.timeout_secs {
            fields.push("speed_limit");
            values.push("?");
            params.push(timeout_secs.to_string());
        }
        if let Some(timeout_secs) = new.timeout_secs {
            fields.push("speed_limit");
            values.push("?");
            params.push(timeout_secs.to_string());
        }

        let query = format!(
            "INSERT INTO downloads ({}) VALUES ({})",
            fields.join(", "),
            values.join(", ")
        );

        let mut query_builder = sqlx::query(&query);

        for param in params {
            query_builder = query_builder.bind(param);
        }

        Ok(query_builder.execute(pool).await?.last_insert_rowid())
    }

    pub async fn update(id: i64, update: UpdateDownload) -> anyhow::Result<()> {
        let pool = Registry::get_pool();

        let mut fields = Vec::new();
        let mut binds: Vec<String> = Vec::new();

        if let Some(status) = &update.status {
            fields.push("status = ?");
            binds.push(status.to_string());
        }

        if let Some(total_bytes) = update.total_bytes {
            fields.push("total_bytes = ?");
            binds.push(total_bytes.to_string());
        }

        if let Some(speed_limit) = update.speed_limit {
            fields.push("speed_limit = ?");
            binds.push(speed_limit.to_string());
        }

        if let Some(auth) = &update.auth {
            fields.push("auth = ?");
            binds.push(auth.clone());
        }

        if let Some(proxy) = &update.proxy {
            fields.push("proxy = ?");
            binds.push(proxy.clone());
        }

        if let Some(headers) = &update.headers {
            fields.push("headers = ?");
            binds.push(headers.clone().into());
        }

        if let Some(cookies) = &update.cookies {
            fields.push("cookies = ?");
            binds.push(cookies.clone().into());
        }

        if let Some(max_retries) = update.max_retries {
            fields.push("max_retries = ?");
            binds.push(max_retries.to_string());
        }

        if let Some(delay_secs) = update.delay_secs {
            fields.push("delay_secs = ?");
            binds.push(delay_secs.to_string());
        }

        if let Some(backoff_factor) = update.backoff_factor {
            fields.push("backoff_factor = ?");
            binds.push(backoff_factor.to_string());
        }

        if let Some(timeout_secs) = update.timeout_secs {
            fields.push("timeout_secs = ?");
            binds.push(timeout_secs.to_string());
        }
        if let Some(error_message) = update.error_message {
            fields.push("error_message = ?");
            binds.push(error_message.to_string());
        }

        if fields.is_empty() {
            return Ok(());
        }

        let sql = format!("UPDATE downloads SET {} WHERE id = ?", fields.join(", "));

        let mut query = sqlx::query(&sql);

        for bind in binds {
            query = query.bind(bind);
        }

        query = query.bind(id);

        query.execute(pool).await?;

        Ok(())
    }

    pub async fn delete(id: i64) -> anyhow::Result<String> {
        let pool = Registry::get_pool();
        let record = sqlx::query!("DELETE FROM downloads WHERE id = ? RETURNING file_path", id)
            .fetch_one(pool)
            .await?;

        Ok(record.file_path)
    }
}
