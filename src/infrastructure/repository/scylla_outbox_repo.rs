use std::sync::Arc;

use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use scylla::client::session::Session;
use uuid::Uuid;

use crate::infrastructure::repository::outbox::{Outbox, OutboxRepo, OutboxStatus};

#[derive(Clone)]
pub struct ScyllaOutboxRepo {
    session: Arc<Session>,
}

impl ScyllaOutboxRepo {
    pub fn new(session: Arc<Session>) -> Self {
        Self { session }
    }
}

#[async_trait::async_trait]
impl OutboxRepo for ScyllaOutboxRepo {
    async fn select_all(&self, bucket_id: &str) -> Result<Vec<Outbox>> {
        let query = r#"
            SELECT bucket_id, event_id, status, lease_expires, payload, metadata, event_type, occurred_at, exchange_name FROM outbox
            WHERE bucket_id = ?
            LIMIT 100
        "#;

        let into_rows_result = self
            .session
            .query_unpaged(query, (bucket_id,))
            .await?
            .into_rows_result()?;

        let mut outboxes: Vec<Outbox> = Vec::new();

        for row in into_rows_result.rows()? {
            let (
                bucket_id,
                event_id,
                status,
                lease_expires,
                payload,
                metadata,
                event_type,
                occurred_at,
                exchange_name,
            ): (
                String,
                Uuid,
                String,
                DateTime<Utc>,
                String,
                String,
                String,
                DateTime<Utc>,
                String,
            ) = row?;

            outboxes.push(Outbox {
                bucket_id,
                event_id,
                status: OutboxStatus::try_from(status)?,
                lease_expires,
                payload,
                metadata,
                event_type,
                occurred_at,
                exchange_name,
            });
        }

        Ok(outboxes)
    }

    async fn mark_as_processing(&self, outbox: &Outbox) -> Result<()> {
        let query = r#"
            UPDATE outbox
            SET status = ?, lease_expires = ?
            WHERE bucket_id = ?
            AND event_id = ?
            IF lease_expires = ?
        "#;

        let now = Utc::now();

        self.session
            .query_unpaged(
                query,
                (
                    OutboxStatus::Processing.as_ref(),
                    now + Duration::seconds(90),
                    &outbox.bucket_id,
                    &outbox.event_id,
                    &outbox.lease_expires,
                ),
            )
            .await?;

        Ok(())
    }

    async fn mark_as_pending(&self, outbox: &Outbox) -> Result<()> {
        let query = r#"
            UPDATE outbox
            SET status = ?, lease_expires = ?
            WHERE bucket_id = ?
            AND event_id = ?
            IF lease_expires = ?
        "#;

        let now = Utc::now();

        self.session
            .query_unpaged(
                query,
                (
                    OutboxStatus::Pending.as_ref(),
                    now + Duration::seconds(90),
                    &outbox.bucket_id,
                    &outbox.event_id,
                    &outbox.lease_expires,
                ),
            )
            .await?;

        Ok(())
    }

    async fn remove(&self, outbox: &Outbox) -> Result<()> {
        let query = r#"
            DELETE FROM outbox
            WHERE bucket_id = ?
            AND event_id = ?
        "#;

        self.session
            .query_unpaged(query, (&outbox.bucket_id, &outbox.event_id))
            .await?;

        Ok(())
    }
}
