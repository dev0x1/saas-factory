use opentelemetry::{
    global,
    runtime::TokioCurrentThread,
    sdk::propagation::TraceContextPropagator,
};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

pub fn config_telemetry(app_name: &str, jaeger_url: &str) {
    // Redirect all `log`'s events to tracing subscriber
    LogTracer::init().expect("Failed to set logger");

    // Start a new Jaeger trace pipeline.
    // Spans are exported in batch - recommended setup for a production
    // application.
    global::set_text_map_propagator(TraceContextPropagator::new());
    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_agent_endpoint(jaeger_url)
        .with_service_name(app_name)
        .install_batch(TokioCurrentThread)
        .expect("Failed to install OpenTelemetry tracer.");

    // Filter based on level - trace, debug, info, warn, error
    // Tunable via `RUST_LOG` env variable
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    // Create a `tracing` layer using the Jaeger tracer
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    // Create a `tracing` layer to emit spans as structured logs to stdout
    let (non_blocking_writer, _guard) = tracing_appender::non_blocking(std::io::stdout());
    let formatting_layer = BunyanFormattingLayer::new(app_name.into(), non_blocking_writer);

    // Combined them all together in a `tracing` subscriber
    let subscriber = Registry::default()
        .with(env_filter)
        .with(telemetry)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to install `tracing` subscriber.")
}
