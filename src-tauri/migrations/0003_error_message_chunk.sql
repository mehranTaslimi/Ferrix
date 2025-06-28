ALTER TABLE
    download_chunks
ADD
    COLUMN error_message TEXT;

ALTER TABLE
    download_chunks
ADD
    COLUMN has_error BOOLEAN DEFAULT FALSE;