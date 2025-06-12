ALTER TABLE
    download_chunks
ADD
    COLUMN expected_hash TEXT DEFAULT NULL;