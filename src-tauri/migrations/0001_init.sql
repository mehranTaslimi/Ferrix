CREATE TABLE IF NOT EXISTS downloads (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    url TEXT NOT NULL,
    total_bytes INTEGER NOT NULL DEFAULT 0,
    -- "queued", "downloading", "completed", "failed"
    status TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    chunk_count INTEGER NOT NULL DEFAULT 5,
    file_path TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS download_chunks (
    download_id TEXT NOT NULL REFERENCES downloads(id) ON DELETE CASCADE,
    chunk_index INTEGER NOT NULL,
    start_byte INTEGER NOT NULL,
    end_byte INTEGER NOT NULL,
    downloaded_bytes INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (download_id, chunk_index)
);