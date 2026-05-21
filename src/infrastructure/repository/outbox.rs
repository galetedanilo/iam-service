use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Outbox {
    bucket_id: String,
    status: String,
    event_id: Uuid,
    lease_expires: DateTime<Utc>,
    exchange_name: String,
    routing_key: String,
    raw_event: String,
}

impl Outbox {
    pub fn new(
        bucket_id: String,
        event_id: Uuid,
        exchange_name: String,
        routing_key: String,
        raw_event: String,
    ) -> Self {
        Self {
            bucket_id,
            status: OutboxStatus::Pending.as_ref().to_string(),
            event_id,
            lease_expires: Utc::now(),
            exchange_name,
            routing_key,
            raw_event,
        }
    }

    pub fn from_parts(
        bucket_id: String,
        event_id: Uuid,
        status: String,
        lease_expires: DateTime<Utc>,
        exchange_name: String,
        routing_key: String,
        raw_event: String,
    ) -> Self {
        Self {
            bucket_id,
            status,
            event_id,
            lease_expires,
            exchange_name,
            routing_key,
            raw_event,
        }
    }

    pub fn bucket_id(&self) -> &str {
        &self.bucket_id
    }

    pub fn status(&self) -> &str {
        &self.status
    }

    pub fn event_id(&self) -> Uuid {
        self.event_id
    }

    pub fn lease_expires(&self) -> DateTime<Utc> {
        self.lease_expires
    }

    pub fn exchange_name(&self) -> &str {
        &self.exchange_name
    }

    pub fn routing_key(&self) -> &str {
        &self.routing_key
    }

    pub fn raw_event(&self) -> &str {
        &self.raw_event
    }
}

#[async_trait::async_trait]
pub trait OutboxRepo: Send + Sync {
    async fn mark_as_processing(&self, outbox: &Outbox) -> Result<()>;

    async fn mark_as_pending(&self, outbox: &Outbox) -> Result<()>;

    async fn remove(&self, outbox: &Outbox) -> Result<()>;

    async fn select_all(&self, bucket_id: &str) -> Result<Vec<Outbox>>;
}
