-- Migration: create_outbox
-- Timestamp: 20260412

-- Add your CQL queries here
CREATE TABLE IF NOT EXISTS iam_service.outbox (
    bucket_id text, 
    event_id uuid,
    status text, 
    lease_expires timestamp,
    occurred_at timestamp,
    payload text,
    metadata text,
    exchange_name text,
    event_type text,
    PRIMARY KEY (bucket_id, event_id)
) WITH CLUSTERING ORDER BY (event_id ASC)
  AND compaction = {'class': 'TimeWindowCompactionStrategy', 'compaction_window_unit': 'HOURS', 'compaction_window_size': 1};

-- Índice para busca rápida de eventos não processados
CREATE INDEX IF NOT EXISTS outbox_status_idx ON iam_service.outbox (status);
