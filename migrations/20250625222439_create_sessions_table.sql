-- Add migration script here
CREATE SCHEMA IF NOT EXISTS tower_sessions;

CREATE TABLE IF NOT EXISTS tower_sessions.session (
    id TEXT PRIMARY KEY,
    data BYTEA NOT NULL,
    expiry_date TIMESTAMPTZ NOT NULL
);