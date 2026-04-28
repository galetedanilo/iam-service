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

CREATE TABLE IF NOT EXISTS iam_service.users_by_email (
    email text PRIMARY KEY,
    id uuid
);