use opentelemetry::trace::TraceContextExt;
use strum_macros::{AsRefStr, Display, EnumString};
use tracing_opentelemetry::OpenTelemetrySpanExt;

pub trait EventPayload: Send + Sync {
    fn get_payload(&self) -> String;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, EnumString, Display, AsRefStr)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum EventType {
    UserRegistered,
    UserActivated,
    PasswordResetRequested,
    PasswordResetCompleted,
    // Outros tipos de eventos podem ser adicionados aqui
}

#[derive(Debug, Clone)]
pub struct Metadata {
    pub trace_id: uuid::Uuid,
    pub source: String,
    pub correlation_id: Option<String>,
    pub causation_id: Option<String>,
}

impl Metadata {
    #[tracing::instrument(name = "Generating event metadata", skip(source))]
    pub fn current(source: &str) -> Self {
        // Obtém o contexto do span atual do tracing
        let context = tracing::Span::current().context();
        let span_ref = context.span();
        let span_context = span_ref.span_context();

        // Extrai o Trace ID (se existir e estiver ativo)
        let trace_id = if span_context.is_valid() {
            let bytes = span_context.trace_id().to_bytes();
            uuid::Uuid::from_bytes(bytes)
        } else {
            uuid::Uuid::now_v7() // Fallback se não houver tracing ativo
        };

        // O causation_id geralmente é o ID do Span atual
        let causation_id = if span_context.is_valid() {
            Some(span_context.span_id().to_string())
        } else {
            None
        };

        Self {
            trace_id,
            source: source.to_string(),
            correlation_id: None, // Pode vir de um Header específico via Axum
            causation_id,
        }
    }

    pub fn get_payload(&self) -> String {
        format!(
            "{{\"trace_id\": \"{}\", \"source\": \"{}\", \"correlation_id\": {:?}, \"causation_id\": {:?}}}",
            self.trace_id, self.source, self.correlation_id, self.causation_id
        )
    }
}

#[derive(Debug, Clone)]
pub struct Event<T: EventPayload> {
    id: uuid::Uuid,
    event_type: EventType,
    exchange_name: String,
    payload: T,
    metadata: Metadata,
    occurred_at: chrono::DateTime<chrono::Utc>,
}

impl<E: EventPayload> Event<E> {
    pub fn new(event_type: EventType, exchange_name: String, payload: E) -> Self {
        Self {
            id: uuid::Uuid::now_v7(),
            event_type,
            exchange_name,
            payload,
            metadata: Metadata::current("iam-service"),
            occurred_at: chrono::Utc::now(),
        }
    }

    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn event_type(&self) -> &EventType {
        &self.event_type
    }

    pub fn exchange_name(&self) -> String {
        self.exchange_name.clone()
    }

    pub fn metadata(&self) -> String {
        self.metadata.get_payload()
    }

    pub fn occurred_at(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.occurred_at
    }

    pub fn build_payload_json(&self) -> String {
        format!(
            "{{\"event_id\": \"{}\", \"event_type\": \"{}\", \"exchange_name\": \"{}\", \"occurred_at\": \"{}\", \"payload\": {}, \"metadata\": {}}}",
            self.id,
            self.event_type,
            self.exchange_name,
            self.occurred_at.format("%Y-%m-%d %H:%M:%S"),
            self.payload.get_payload(),
            self.metadata.get_payload()
        )
    }
}
