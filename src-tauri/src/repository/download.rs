use crate::{
    models::{Download, NewDownload, UpdateDownload},
    registry::Registry,
};

pub struct DownloadRepository;

impl DownloadRepository {
    pub async fn find_all() -> Result<Vec<Download>, sqlx::Error> {
        let pool = Registry::get_pool();
        sqlx::query_as!(
            Download,
            r#"
    SELECT
        id,
        url,
        total_bytes,
        status,
        created_at,
        chunk_count,
        file_path,
        file_name,
        content_type,
        extension,
        auth,
        proxy,
        headers,
        cookies,
        speed_limit,
        max_retries,
        delay_secs,
        backoff_factor,
        timeout_secs
    FROM downloads
    ORDER BY created_at DESC
    "#
        )
        .fetch_all(pool)
        .await
    }

    pub async fn find(id: i64) -> Result<Download, sqlx::Error> {
        let pool = Registry::get_pool();
        sqlx::query_as!(
            Download,
            r#"
    SELECT
        id,
        url,
        total_bytes,
        status,
        created_at,
        chunk_count,
        file_path,
        file_name,
        content_type,
        extension,
        auth,
        proxy,
        headers,
        cookies,
        speed_limit,
        max_retries,
        delay_secs,
        backoff_factor,
        timeout_secs
    FROM downloads
    WHERE id = ?
    "#,
            id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn add(new: NewDownload) -> Result<i64, sqlx::Error> {
        let pool = Registry::get_pool();

        sqlx::query_as!(
            Download,
            r#"
        INSERT INTO downloads (
            url,
            status,
            chunk_count,
            file_path,
            file_name,
            content_type,
            extension,
            auth,
            proxy,
            headers,
            cookies,
            speed_limit,
            max_retries,
            delay_secs,
            backoff_factor,
            timeout_secs
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
            new.url,
            new.status,
            new.chunk_count,
            new.file_path,
            new.file_name,
            new.content_type,
            new.extension,
            new.auth,
            new.proxy,
            new.headers,
            new.cookies,
            new.speed_limit,
            new.max_retries,
            new.delay_secs,
            new.backoff_factor,
            new.timeout_secs
        )
        .execute(pool)
        .await
        .map(|result| result.last_insert_rowid())
    }

    pub async fn update(id: i64, update: UpdateDownload) -> Result<(), sqlx::Error> {
        let pool = Registry::get_pool();

        let mut fields = Vec::new();
        let mut binds: Vec<String> = Vec::new();

        if let Some(status) = &update.status {
            fields.push("status = ?");
            binds.push(status.clone().into());
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
}
