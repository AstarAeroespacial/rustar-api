use chrono::{DateTime, Utc};
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

use validator::Validate;

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
#[into_params(parameter_in = Query)]
pub struct PassesQueryParams {
    #[param(example = "2025-12-10T00:00:00Z")]
    #[schema(example = "2025-12-10T00:00:00Z")]
    pub start: DateTime<Utc>,
    #[param(example = "2025-12-17T23:59:59Z")]
    #[schema(example = "2025-12-17T23:59:59Z")]
    pub end: DateTime<Utc>,
}
#[derive(ToSchema, Debug, Deserialize, Validate)]
#[schema(example = json!({
    "id": "gs-001",
    "name": "Ground Station Buenos Aires",
    "latitude": -34.6037,
    "longitude": -58.3816,
    "altitude": 25
}))]
pub struct GroundStationCreateRequest {
    #[validate(length(min = 1, message = "ID cannot be empty"))]
    #[schema(example = "gs-001")]
    pub id: String,
    #[validate(length(min = 1, message = "Name cannot be empty"))]
    #[schema(example = "Ground Station Buenos Aires")]
    pub name: String,

    #[validate(range(min = -90.0, max = 90.0, message = "Latitude must be between -90 and 90"))]
    #[schema(example = json!(-34.6037))]
    pub latitude: f32,

    #[validate(range(min = -180.0, max = 180.0, message = "Longitude must be between -180 and 180"))]
    #[schema(example = json!(-58.3816))]
    pub longitude: f32,

    #[validate(range(min = -500, max = 9000, message = "Altitude must be within realistic range (-500 to 9000 meters)"))]
    #[schema(example = 25i32)]
    pub altitude: i32,
}

#[derive(ToSchema, Debug, Deserialize, Validate)]
#[schema(example = json!({
    "sat_id": "sat-001",
    "gs_id": "gs-001",
    "start": "2025-12-10T12:00:00Z",
    "end": "2025-12-10T12:15:00Z",
    "commands": ["command1", "command2"]
}))]
pub struct JobCreateRequest {
    #[validate(length(min = 1, message = "Satellite ID cannot be empty"))]
    #[schema(example = "sat-001")]
    pub sat_id: String,

    #[validate(length(min = 1, message = "Ground station ID cannot be empty"))]
    #[schema(example = "gs-001")]
    pub gs_id: String,

    #[serde(rename = "start")]
    #[schema(value_type = String, example = "2025-12-10T12:00:00Z")]
    pub start_time: DateTime<Utc>,

    #[serde(rename = "end")]
    #[schema(value_type = String, example = "2025-12-10T12:15:00Z")]
    pub end_time: DateTime<Utc>,

    #[schema(example = json!(["command1", "command2"]))]
    pub commands: Option<Vec<String>>,
}

#[derive(ToSchema, Debug, Deserialize, Validate)]
#[schema(example = json!({
    "id": "sat-001",
    "name": "NOAA 19",
    "tle": "1 33591U 09005A   24304.41234567  .00000023  00000-0  12345-4 0  9992\n2 33591  99.1234 123.4567 0012345 123.4567 234.5678 14.12345678901234",
    "downlink_frequency": 137.1,
    "uplink_frequency": 145.8
}))]
pub struct SatelliteCreateRequest {
    #[validate(length(min = 1, message = "ID cannot be empty"))]
    #[schema(example = "sat-001")]
    pub id: String,

    #[validate(length(min = 1, message = "Name cannot be empty"))]
    #[schema(example = "NOAA 19")]
    pub name: String,

    #[validate(length(min = 1, message = "TLE cannot be empty"))]
    #[schema(
        example = "1 33591U 09005A   24304.41234567  .00000023  00000-0  12345-4 0  9992\n2 33591  99.1234 123.4567 0012345 123.4567 234.5678 14.12345678901234"
    )]
    pub tle: String,

    #[validate(range(min = 1.0, message = "Downlink frequency must be positive"))]
    #[schema(example = 137.1)]
    pub downlink_frequency: f64,

    #[validate(range(min = 1.0, message = "Uplink frequency must be positive"))]
    #[schema(example = 145.8)]
    pub uplink_frequency: f64,
}

#[derive(ToSchema, Debug, Deserialize, Validate)]
#[schema(example = json!({
    "tle": "1 33591U 09005A   24305.51234567  .00000020  00000-0  12000-4 0  9993\n2 33591  99.1234 123.4567 0012345 123.4567 234.5678 14.12345678901234"
}))]
pub struct TleUpdateRequest {
    #[validate(length(min = 1, message = "TLE cannot be empty"))]
    #[schema(
        example = "1 33591U 09005A   24305.51234567  .00000020  00000-0  12000-4 0  9993\n2 33591  99.1234 123.4567 0012345 123.4567 234.5678 14.12345678901234"
    )]
    pub tle: String,
}

#[derive(ToSchema, IntoParams, Debug, Deserialize, Validate)]
#[into_params(parameter_in = Query)]
pub struct TelemetryQueryParams {
    #[validate(range(min = 1, max = 1000, message = "Limit must be between 1 and 1000"))]
    #[schema(example = 50)]
    pub limit: Option<i64>,

    #[validate(range(min = 1, message = "Page must be at least 1"))]
    #[schema(example = 1)]
    pub page: Option<i64>,
}
