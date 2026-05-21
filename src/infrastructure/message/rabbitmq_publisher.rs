use lapin::{
    BasicProperties, Channel, Connection, ConnectionProperties, ExchangeKind,
    options::{BasicPublishOptions, ConfirmSelectOptions, ExchangeDeclareOptions},
    types::FieldTable,
};
use tokio_retry::{
    Retry,
    strategy::{ExponentialBackoff, jitter},
};

use crate::infrastructure::{message::event_publisher::EventPublisher, repository::outbox::Outbox};

#[derive(Clone)]
pub struct RabbitmqPublisher {
    channel: Channel,
}

impl RabbitmqPublisher {
    pub async fn create_publisher(uri: &str, exchange_name: &str) -> anyhow::Result<Self> {
        let connection =
            Connection::connect(uri, ConnectionProperties::default().enable_auto_recover()).await?;

        let channel = connection.create_channel().await?;

        channel
            .exchange_declare(
                exchange_name.into(),
                ExchangeKind::Topic,
                ExchangeDeclareOptions {
                    passive: false,
                    durable: true,
                    auto_delete: false,
                    internal: false,
                    nowait: false,
                },
                FieldTable::default(),
            )
            .await?;

        channel
            .confirm_select(ConfirmSelectOptions::default())
            .await?;

        Ok(Self { channel })
    }
}

#[async_trait::async_trait]
impl EventPublisher for RabbitmqPublisher {
    async fn publish_event(&self, outbox_record: &Outbox) -> anyhow::Result<()> {
        let retry_strategy = ExponentialBackoff::from_millis(100).map(jitter).take(3);

        Retry::spawn(retry_strategy, || async {
            let confirm = self
                .channel
                .basic_publish(
                    outbox_record.exchange_name().into(),
                    outbox_record.routing_key().into(),
                    BasicPublishOptions::default(),
                    &outbox_record.raw_event().as_bytes(),
                    BasicProperties::default().with_delivery_mode(2),
                )
                .await?
                .await?;

            if confirm.is_ack() {
                Ok(())
            } else {
                tracing::warn!("RabbitMQ send NACK for event {}", outbox_record.event_id());
                Err(anyhow::anyhow!("NACK received"))
            }
        })
        .await
        .map_err(|e| {
            tracing::error!("Failed to publish message to RabbitMQ: {:?}", e);
            anyhow::anyhow!("Failed to publish message to RabbitMQ: {}", e)
        })
    }
}
