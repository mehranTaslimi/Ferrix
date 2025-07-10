-- 1. Add the column without default
ALTER TABLE
    downloads
ADD
    COLUMN modified_at TIMESTAMP;

UPDATE
    downloads
SET
    modified_at = CURRENT_TIMESTAMP
WHERE
    modified_at IS NULL;

CREATE INDEX IF NOT EXISTS idx_downloads_modified_at ON downloads(modified_at);

CREATE TRIGGER IF NOT EXISTS trg_downloads_modified_at
AFTER
UPDATE
    ON downloads FOR EACH ROW BEGIN
UPDATE
    downloads
SET
    modified_at = CURRENT_TIMESTAMP
WHERE
    id = OLD.id;

END;