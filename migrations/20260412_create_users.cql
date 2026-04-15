-- Migration: create_users
-- Timestamp: 20260412

-- Add your CQL queries here
CREATE TABLE IF NOT EXISTS iam_service.users (
    id uuid PRIMARY KEY,
    email text,
    password text,
    status text,
    audiences set<text>,
    scopes set<text>,
    token_hash text,
    token_expires_at timestamp,
    created_at timestamp,
    updated_at timestamp,
);

CREATE MATERIALIZED VIEW IF NOT EXISTS iam_service.view_users_by_email AS
    SELECT id, email, password, status, token_hash, token_expires_at 
    FROM iam_service.users
    WHERE email IS NOT NULL AND id IS NOT NULL
    PRIMARY KEY (email, id);

CREATE MATERIALIZED VIEW IF NOT EXISTS iam_service.view_users_by_status AS
    SELECT id, email, created_at, status 
    FROM iam_service.users
    WHERE status IS NOT NULL AND id IS NOT NULL 
    PRIMARY KEY (status, id) 
    WITH CLUSTERING ORDER BY (id DESC);