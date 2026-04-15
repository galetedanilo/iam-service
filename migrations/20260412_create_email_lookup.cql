-- Migration: create_email_lookup
-- Timestamp: 20260412

-- Add your CQL queries here
CREATE TABLE IF NOT EXISTS iam_service.email_lookup (
    email text PRIMARY KEY,
    user_id uuid
);