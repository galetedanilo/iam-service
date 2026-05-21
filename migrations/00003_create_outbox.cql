-- Migration: create_outbox
-- Timestamp: 20260412

-- Add your CQL queries here
CREATE TABLE iam_service.outbox (
    bucket_id text,
    event_id uuid,
    status text,
    lease_expires timestamp,
    exchange_name text,
    routing_key text,
    raw_event text,
    PRIMARY KEY (bucket_id, event_id)
) WITH CLUSTERING ORDER BY (event_id ASC)
  AND compaction = {'class': 'TimeWindowCompactionStrategy', 'compaction_window_unit': 'HOURS', 'compaction_window_size': 1};
