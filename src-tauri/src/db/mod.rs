use sqlx::SqlitePool;

pub async fn init_db(db_url: &str) -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePool::connect(db_url).await?;

    sqlx::migrate!().run(&pool).await?;

    Ok(pool)
}
