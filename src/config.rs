use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct DatabaseConfig {
    pub url: String,
    pub pool_size: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct BrokerConfig {
    pub host: String,
    pub port: u16,
    pub keep_alive: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub broker: BrokerConfig,
}

impl Config {
    pub fn load() -> Result<Self, config::ConfigError> {
        let _ = dotenvy::dotenv();

        let settings = config::Config::builder()
            .add_source(config::File::with_name("config"))
            .add_source(config::Environment::separator(
                config::Environment::with_prefix("API"),
                "_",
            ))
            .build()?;

        let mut config: Self = settings.try_deserialize()?;

        // Check if the "PORT" environment variable is set, and override the server.port if so
        if let Ok(port_str) = std::env::var("PORT") {
            if let Ok(port) = port_str.parse::<u16>() {
                config.server.port = port;
                println!("PORT environment variable found, overriding server.port to {}", port);
            }
        }
        println!("server.port: {}", config.server.port);
        Ok(config)
    }

    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }
}

// Type alias for shared configuration
pub type SharedConfig = Arc<Config>;
