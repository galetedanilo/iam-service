use anyhow::Result;
use opentelemetry::{KeyValue, global};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{Resource, trace::RandomIdGenerator};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_telemetry() -> Result<()> {
    let otlp_exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_endpoint("http://localhost:4317")
        .build()?;

    // Create a tracer provider with the exporter
    let tracer_provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
        .with_batch_exporter(otlp_exporter)
        .with_id_generator(RandomIdGenerator::default())
        .with_max_events_per_span(64)
        .with_max_attributes_per_span(16)
        .with_resource(
            Resource::builder_empty()
                .with_attributes(vec![
                    KeyValue::new("service.name", "iam-service"),
                    KeyValue::new("service.version", "1.0.0"),
                ])
                .build(),
        )
        .build();

    // Set it as the global provider
    global::set_tracer_provider(tracer_provider);
    let tracer = opentelemetry::global::tracer("iam-service");

    // // 4. Integra com o ecossistema Tracing
    let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("info"))
        .with(tracing_subscriber::fmt::layer())
        .with(telemetry_layer)
        .init();

    Ok(())
}
