use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(ToSchema, Debug, Serialize)]
#[schema(example = json!({
    "gs_id": "gs-001",
    "sat_id": "sat-001",
    "aos": "2025-12-10T12:00:00Z",
    "los": "2025-12-10T12:15:00Z",
    "max_elevation": 45.5
}))]
pub struct PassInfo {
    #[schema(example = "gs-001")]
    pub gs_id: String,
    #[schema(example = "sat-001")]
    pub sat_id: String,
    #[schema(value_type = String, example = "2025-12-10T12:00:00Z")]
    pub aos: DateTime<Utc>,
    #[schema(value_type = String, example = "2025-12-10T12:15:00Z")]
    pub los: DateTime<Utc>,
    #[schema(example = 45.5)]
    pub max_elevation: f64,
}

#[derive(ToSchema, Debug, Serialize)]
pub struct SatellitePassesResponse {
    pub passes: Vec<PassInfo>,
}

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
