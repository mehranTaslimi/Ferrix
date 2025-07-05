CREATE TABLE IF NOT EXISTS downloads (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    url TEXT NOT NULL,
    total_bytes INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    chunk_count INTEGER NOT NULL DEFAULT 5,
    file_path TEXT NOT NULL,
    file_name TEXT NOT NULL,
    content_type TEXT NOT NULL,
    extension TEXT NOT NULL,
    auth TEXT,
    proxy TEXT,
    headers TEXT,
    cookies TEXT,
    speed_limit INTEGER,
    max_retries INTEGER NOT NULL DEFAULT 3,
    delay_secs REAL NOT NULL DEFAULT 2.0,
    backoff_factor REAL NOT NULL DEFAULT 2.0,
    timeout_secs INTEGER NOT NULL DEFAULT 30
);

CREATE TABLE IF NOT EXISTS download_chunks (
    download_id INTEGER NOT NULL REFERENCES downloads(id) ON DELETE CASCADE,
    chunk_index INTEGER NOT NULL,
    start_byte INTEGER NOT NULL,
    end_byte INTEGER NOT NULL,
    downloaded_bytes INTEGER NOT NULL DEFAULT 0,
    expected_hash TEXT DEFAULT NULL,
    error_message TEXT,
    has_error BOOLEAN DEFAULT FALSE,
    PRIMARY KEY (download_id, chunk_index)
);

CREATE INDEX IF NOT EXISTS idx_downloads_status ON downloads(status);

CREATE INDEX IF NOT EXISTS idx_downloads_created_at ON downloads(created_at);

CREATE INDEX IF NOT EXISTS idx_download_chunks_download_id ON download_chunks(download_id);

CREATE INDEX IF NOT EXISTS idx_download_chunks_has_error ON download_chunks(has_error);