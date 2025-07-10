use sqlx::SqlitePool;

impl super::Registry {
    pub fn get_pool() -> &'static SqlitePool {
        &Self::get_state().pool
    }

    pub(super) async fn init_db() -> SqlitePool {
        let db_url = Self::get_db_url();

        let pool = SqlitePool::connect(&db_url).await.unwrap();

        sqlx::query!("PRAGMA foreign_keys = ON;")
            .execute(&pool)
            .await
            .unwrap();

        sqlx::migrate!().run(&pool).await.unwrap();

        pool
    }

    fn get_db_url() -> String {
        if cfg!(debug_assertions) {
            format!(
                "{}",
                std::env::var("DATABASE_URL").unwrap_or("sqlite://./app.db?mode=rwc".to_string())
            )
        } else {
            let db_path = dirs_next::data_local_dir()
                .expect("no data dir")
                .join("ferrix")
                .join("app.db?mode=rwc");

            std::fs::create_dir_all(db_path.parent().unwrap()).expect("failed to create db dir");

            format!("sqlite:{}", db_path.display())
        }
    }
}
