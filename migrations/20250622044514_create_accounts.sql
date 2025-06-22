-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "pgcrypto";
CREATE TABLE accounts (
    email TEXT PRIMARY KEY,
    password TEXT NOT NULL
);
