use anyhow::Result;
use chrono::{DateTime, Utc};
use strum_macros::{AsRefStr, Display, EnumString};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, EnumString, Display, AsRefStr)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum OutboxStatus {
    Pending,
    Processing,
    Processed,
}

impl TryFrom<String> for OutboxStatus {
    type Error = anyhow::Error;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        match value.as_str() {
            "PENDING" => Ok(Self::Pending),
            "PROCESSING" => Ok(Self::Processing),
            "PROCESSED" => Ok(Self::Processed),
            _ => Err(anyhow::anyhow!("Invalid status: {}", value)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Outbox {
    pub bucket_id: String,
    pub event_id: Uuid,
    pub status: OutboxStatus,
    pub lease_expires: DateTime<Utc>,
    pub payload: String,
    pub metadata: String,
    pub event_type: String,
    pub occurred_at: DateTime<Utc>,
    pub exchange_name: String,
}

impl Outbox {
    pub fn new(
        bucket_id: String,
        event_id: Uuid,
        status: OutboxStatus,
        lease_expires: DateTime<Utc>,
        payload: String,
        metadata: String,
        event_type: String,
        occurred_at: DateTime<Utc>,
        exchange_name: String,
    ) -> Self {
        Self {
            bucket_id,
            event_id,
            status,
            lease_expires,
            payload,
            metadata,
            event_type,
            occurred_at,
            exchange_name,
        }
    }
}

#[async_trait::async_trait]
pub trait OutboxRepo: Send + Sync {
    async fn mark_as_processing(&self, outbox: &Outbox) -> Result<()>;

    async fn mark_as_pending(&self, outbox: &Outbox) -> Result<()>;

    async fn remove(&self, outbox: &Outbox) -> Result<()>;

    async fn select_all(&self, bucket_id: &str) -> Result<Vec<Outbox>>;
}
