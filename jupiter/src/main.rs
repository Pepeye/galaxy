// use module dependencies
use jupiter::configuration::get_configuration;
use jupiter::{
    startup::Application,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_name = env!("CARGO_PKG_NAME").to_string();

    // Get app config settings
    let config = get_configuration().expect("Failed to load configuration");

    // setup tracing subscriber
    let tracing_endpoint = format!("{}:{}", config.telemetry.host, config.telemetry.port);
    let subscriber = get_subscriber(app_name, "info".into(), std::io::stdout, tracing_endpoint);
    init_subscriber(subscriber);

    let application = Application::build(config).await?;
    application.run().await?;

    Ok(())
}
