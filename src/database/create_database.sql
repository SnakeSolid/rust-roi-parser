CREATE TABLE IF NOT EXISTS initiatives (
    initiative_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    archived INTEGER NOT NULL,
    enabled INTEGER NOT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS initiatives_id ON initiatives ( initiative_id );
CREATE INDEX IF NOT EXISTS initiatives_archived ON initiatives ( archived );

CREATE TABLE IF NOT EXISTS votes (
    initiative_id INTEGER NOT NULL,
    timestamp INTEGER NOT NULL,
    positive INTEGER NOT NULL,
    negative INTEGER NOT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS initiatives_id_timestamp ON votes ( initiative_id, timestamp );
