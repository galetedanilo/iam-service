use std::{sync::Arc, time::Duration};

use chrono::Utc;

use crate::infrastructure::{
    message::event_publisher::EventPublisher, repository::outbox::OutboxRepo,
};

#[derive(Clone)]
pub struct EventWorker<R: OutboxRepo, P: EventPublisher> {
    outbox_repo: Arc<R>,
    event_publisher: Arc<P>,
}

impl<R: OutboxRepo, P: EventPublisher> EventWorker<R, P> {
    pub fn new(outbox_repo: Arc<R>, event_publisher: Arc<P>) -> Self {
        Self {
            outbox_repo,
            event_publisher,
        }
    }

    pub async fn process(&self) {
        let base_interval = Duration::from_millis(500);
        let max_interval = Duration::from_secs(10);
        let mut current_interval = base_interval;

        loop {
            let shard_id = rand::random::<u8>() % 10;
            let bucket_id = format!("outbox-{}", shard_id);

            match self.outbox_repo.select_all(&bucket_id).await {
                Ok(outboxes) => {
                    if outboxes.is_empty() {
                        current_interval = base_interval;
                        tokio::time::sleep(current_interval).await;

                        continue;
                    }

                    let now = Utc::now();

                    current_interval = base_interval;

                    for outbox in outboxes {
                        let is_lease_expired = outbox.lease_expires < now;

                        if !is_lease_expired {
                            continue;
                        }

                        match self.outbox_repo.mark_as_processing(&outbox).await {
                            Ok(_) => {
                                tracing::info!("Outbox marked as processing");

                                match self.event_publisher.publish_event(&outbox).await {
                                    Ok(_) => {
                                        tracing::info!("Event published");
                                        match self.outbox_repo.remove(&outbox).await {
                                            Ok(_) => {
                                                tracing::info!("Outbox removed");
                                            }
                                            Err(e) => {
                                                tracing::error!("Error removing outbox: {}", e);
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        tracing::error!("Error publishing event: {}", e);
                                        match self.outbox_repo.mark_as_pending(&outbox).await {
                                            Ok(_) => {
                                                tracing::info!("Outbox marked as pending");
                                            }
                                            Err(e) => {
                                                tracing::error!(
                                                    "Error marking outbox as pending: {}",
                                                    e
                                                );
                                            }
                                        }
                                        continue;
                                    }
                                }
                            }
                            Err(e) => {
                                tracing::error!("Error marking outbox as processing: {}", e);
                                match self.outbox_repo.mark_as_pending(&outbox).await {
                                    Ok(_) => {
                                        tracing::info!("Outbox marked as pending");
                                    }
                                    Err(e) => {
                                        tracing::error!("Error marking outbox as pending: {}", e);
                                    }
                                }
                                continue;
                            }
                        }
                    }
                }
                Err(e) => {
                    tracing::error!("Error selecting pending outboxes: {}", e);

                    current_interval = std::cmp::min(current_interval * 2, max_interval);
                    tokio::time::sleep(current_interval).await;
                }
            }
        }
    }
}
