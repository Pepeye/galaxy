use anyhow::{Context, Result};
use config::{Config, ConfigError};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
// use tracing_subscriber::EnvFilter;

#[derive(Serialize, Deserialize, Clone)]
pub struct Configuration {
    pub app: AppSettings,
    pub db: DatabaseSettings,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AppSettings {
    pub host: String,
    pub port: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DatabaseSettings {
    pub host: String,
    pub port: i32,
    pub user: String,
    pub pass: String,
}

impl Configuration {
    #[instrument]
    pub fn from_env() -> Result<Configuration> {
        // dotenv().ok();

        info!("Loading configuration");

        // initialise configuration reader
        let mut c = Config::default();

        // Add configuration values from a file named `app`.
        c.merge(config::File::with_name("app"))?;

        // Read config from .env variable
        // c.merge(Environment::default())?;
        c.try_into().context("loading config from app.yaml")
    }
}

pub fn get_configuration() -> Result<Configuration, ConfigError> {
    let c = Configuration::from_env().expect("Server configuration error");
    Ok(c)
}
