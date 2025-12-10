use serde::Serialize;
use utoipa::ToSchema;

#[derive(ToSchema, Debug, Serialize)]
#[schema(example = json!({
    "timestamp": 1702219200,
    "temperature": 25.5,
    "voltage": 12.6,
    "current": 2.3,
    "battery_level": 85
}))]
pub struct TelemetryResponse {
    #[schema(example = 1702219200)]
    pub timestamp: i64, // ISO timestamp
    #[schema(example = 25.5)]
    pub temperature: f32,
    #[schema(example = 12.6)]
    pub voltage: f32,
    #[schema(example = 2.3)]
    pub current: f32,
    #[schema(example = 85)]
    pub battery_level: i32, // percentage
}

#[derive(ToSchema, Debug, Serialize)]
pub struct ConfigResponse {
    pub server: crate::config::ServerConfig,
    pub database: crate::config::DatabaseConfig,
    pub message_broker: crate::config::BrokerConfig,
}
