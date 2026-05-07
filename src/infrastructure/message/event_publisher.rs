use anyhow::Result;

use crate::infrastructure::repository::outbox::Outbox;

#[async_trait::async_trait]
pub trait EventPublisher: Send + Sync {
    async fn publish_event(&self, event: &Outbox) -> Result<()>;
}
