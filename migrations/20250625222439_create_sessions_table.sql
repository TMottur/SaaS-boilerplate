-- Add migration script here
CREATE TABLE IF NOT EXISTS sessions (
    id TEXT PRIMARY KEY,
    session BYTEA NOT NULL,
    expires TIMESTAMPTZ NOT NULL
);
