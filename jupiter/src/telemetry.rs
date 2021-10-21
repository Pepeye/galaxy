use opentelemetry::global;
use opentelemetry::sdk::propagation::TraceContextPropagator;
use opentelemetry::sdk::trace::{self, Sampler};
use tracing::{subscriber::set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

// credit zerotoprod; works a charm

/// Compose multiple layers into a `tracing`'s subscriber.
///
/// # Implementation Notes
///
/// We are using `impl Subscriber` as return type to avoid having to
/// spell out the actual type of the returned subscriber, which is
/// indeed quite complex.
/// We need to explicitly call out that the returned subscriber is
/// `Send` and `Sync` to make it possible to pass it to `init_subscriber`
/// later on.
pub fn get_subscriber(
    name: String,
    env_filter: String,
    sink: impl MakeWriter + Send + Sync + 'static,
    tracing_endpoint: String,
) -> impl Subscriber + Send + Sync {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    let formatting_layer = BunyanFormattingLayer::new(name.clone(), sink);

    // global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    global::set_text_map_propagator(TraceContextPropagator::new());

    // Install an otel pipeline with a simple span processor that exports data one at a time when
    // spans end. See the `install_batch` option on each exporter's pipeline builder to see how to
    // export in batches.
    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name(name)
        .with_trace_config(trace::config().with_sampler(Sampler::AlwaysOn))
        .with_agent_endpoint(tracing_endpoint)
        .with_max_packet_size(65000)
        // .with_auto_split_batch(true)
        .install_batch(opentelemetry::runtime::Tokio)
        .expect("Unable to build Jaeger pipeline");

    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    // The `with` method is provided by `SubscriberExt`, an extension
    // trait for `Subscriber` exposed by `tracing_subscriber`
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
        .with(opentelemetry)
}

/// Register a subscriber as global default to process span data.
///
/// It should only be called once!
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    // Redirect all `log`'s events to our subscriber
    LogTracer::init().expect("Failed to set logger");

    // `set_global_default` can be used by applications to specify
    // what subscriber should be used to process spans.
    set_global_default(subscriber).expect("Failed to set subscriber");
}
