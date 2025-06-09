CREATE TABLE IF NOT EXISTS downloads (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    url TEXT NOT NULL,
    content_length INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL -- "queued", "downloading", "completed", "failed"
);
CREATE TABLE IF NOT EXISTS download_chunks (
    download_id TEXT NOT NULL REFERENCES downloads(id) ON DELETE CASCADE,
    chunk_index INTEGER NOT NULL,
    start INTEGER NOT NULL,
end INTEGER NOT NULL,
downloaded_bytes INTEGER NOT NULL DEFAULT 0,
PRIMARY KEY (download_id, chunk_index)
);