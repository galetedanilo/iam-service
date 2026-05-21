use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    application::events::metadata::Metadata,
    domain::events::domain_event::{DomainEvent, EventType},
};

pub const SERVICE_NAME: &str = "iam_service";

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Event<P> {
    id: Uuid,
    event_type: String,
    exchange_name: String,
    payload: P,
    metadata: Metadata,
    occurred_at: DateTime<Utc>,
}

impl<P: Send + Sync + Serialize> Event<P> {
    pub fn new(
        event_type: EventType,
        exchange_name: String,
        correlation_id: Uuid,
        payload: P,
    ) -> Self {
        Self {
            id: uuid::Uuid::now_v7(),
            event_type: event_type.as_ref().to_string(),
            exchange_name,
            payload,
            metadata: Metadata::current(SERVICE_NAME.to_string(), correlation_id),
            occurred_at: chrono::Utc::now(),
        }
    }
}

impl<P: Send + Sync + Serialize> DomainEvent for Event<P> {
    fn id(&self) -> Uuid {
        self.id
    }
    
    fn raw_event(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".to_string())
    }

    fn exchange_name(&self) -> String {
        self.exchange_name.clone()
    }

    fn routing_key(&self) -> String {
        self.event_type.clone()
    }
}
