CREATE TABLE downloads (
    id TEXT PRIMARY KEY,
    url TEXT NOT NULL,
    file_path TEXT NOT NULL,
    total_size INTEGER,
    status TEXT NOT NULL DEFAULT 'queued',
    started_at DATETIME,
    completed_at DATETIME,
    error TEXT
);

CREATE TABLE download_chunks (
    download_id TEXT NOT NULL REFERENCES downloads(id) ON DELETE CASCADE,
    chunk_index INTEGER NOT NULL,
    start_offset INTEGER NOT NULL,
    end_offset INTEGER NOT NULL,
    bytes_downloaded INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'queued',
    PRIMARY KEY (download_id, chunk_index)
);