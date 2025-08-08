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
                let data_path = path.as_ref().expect("failed to get app data directory");
                let is_exist = std::fs::metadata(data_path).is_ok();

                if !is_exist {
                    std::fs::create_dir_all(data_path)
                        .expect("faild to create ferrix data directory");
                };

                let db_path = data_path
                    .join("database.db?mode=rwc")
                    .to_string_lossy()
                    .to_string();

                format!("sqlite://{}", db_path)
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
