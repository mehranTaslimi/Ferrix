use sqlx::SqlitePool;
use std::path::PathBuf;

impl super::Registry {
    pub fn get_pool() -> &'static SqlitePool {
        &Self::get_state().pool
    }

    pub(super) async fn init_db(path: &Result<PathBuf, tauri::Error>) -> SqlitePool {
        let db_url = {
            if cfg!(debug_assertions) {
                format!(
                    "{}",
                    std::env::var("DATABASE_URL").expect("DATABASE_URL in .env file not set")
                )
            } else {
                path.as_ref()
                    .map(|p| p.join("database.db?mode=rwc").to_string_lossy().to_string())
                    .expect("failed to get app data directory")
            }
        };

        let pool = SqlitePool::connect(&db_url).await.unwrap();

        sqlx::query!("PRAGMA foreign_keys = ON;")
            .execute(&pool)
            .await
            .unwrap();

        sqlx::migrate!().run(&pool).await.unwrap();

        pool
    }
}
