use opentelemetry::trace::TraceContextExt;
use serde::{Deserialize, Serialize};
use tracing_opentelemetry::OpenTelemetrySpanExt;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Metadata {
    pub trace_id: Uuid,
    pub source: String,
    pub correlation_id: Uuid,
    pub causation_id: Option<String>,
}

impl Metadata {
    #[tracing::instrument(name = "Generating event metadata", skip(source))]
    pub fn current(source: String, correlation_id: Uuid) -> Self {
        // Obtém o contexto do span atual do tracing
        let context = tracing::Span::current().context();
        let span_ref = context.span();
        let span_context = span_ref.span_context();

        // Extrai o Trace ID (se existir e estiver ativo)
        let trace_id = if span_context.is_valid() {
            let bytes = span_context.trace_id().to_bytes();
            Uuid::from_bytes(bytes)
        } else {
            Uuid::now_v7() // Fallback se não houver tracing ativo
        };

        // O causation_id geralmente é o ID do Span atual
        let causation_id = if span_context.is_valid() {
            Some(span_context.span_id().to_string())
        } else {
            None
        };

        Self {
            trace_id,
            source,
            correlation_id,
            causation_id,
        }
    }
}
