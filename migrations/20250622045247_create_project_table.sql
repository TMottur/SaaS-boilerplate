-- Add migration script here
CREATE TABLE projects (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_email TEXT NOT NULL REFERENCES accounts(email),
    name TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMP DEFAULT now(),
    last_updated TIMESTAMP
);
