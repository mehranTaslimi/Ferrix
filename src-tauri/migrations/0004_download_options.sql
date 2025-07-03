ALTER TABLE
    downloads
ADD
    COLUMN auth TEXT;

ALTER TABLE
    downloads
ADD
    COLUMN proxy TEXT;

ALTER TABLE
    downloads
ADD
    COLUMN headers TEXT;

ALTER TABLE
    downloads
ADD
    COLUMN cookies TEXT;

ALTER TABLE
    downloads
ADD
    COLUMN speed_limit INTEGER;

ALTER TABLE
    downloads
ADD
    COLUMN max_retries INTEGER NOT NULL DEFAULT 3;

ALTER TABLE
    downloads
ADD
    COLUMN delay_secs REAL NOT NULL DEFAULT 2.0;

ALTER TABLE
    downloads
ADD
    COLUMN backoff_factor REAL NOT NULL DEFAULT 2.0;

ALTER TABLE
    downloads
ADD
    COLUMN timeout_secs INTEGER NOT NULL DEFAULT 30;