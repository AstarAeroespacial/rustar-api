use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

use validator::Validate;

#[derive(ToSchema, IntoParams, Debug, Deserialize)]
#[into_params(style = Form)]
#[serde(rename_all = "camelCase")]
pub struct HistoricTelemetryRequest {
    #[param(example = 1640995200)]
    pub start_time: Option<i64>,
    #[param(example = 1640998800)]
    pub end_time: Option<i64>,
}

#[derive(ToSchema, IntoParams, Debug, Deserialize)]
#[into_params(style = Form)]
#[serde(rename_all = "camelCase")]
pub struct LatestTelemetryRequest {
    #[param(example = 10)]
    pub amount: Option<i32>,
}

#[derive(ToSchema, IntoParams, Debug, Deserialize)]
#[into_params(style=Form)]
#[serde(rename_all = "camelCase")]
pub struct GroundStationCreateRequest {
    pub id: i64,
    pub name: String,
    pub latitude: f32,
    pub longitude: f32,
    pub altitude: i32,
}

#[derive(ToSchema, IntoParams, Debug, Deserialize)]
#[into_params(style=Form)]
#[serde(rename_all = "camelCase")]
pub struct JobCreateRequest {
    #[param(example = 1)]
    pub sat_id: i64,
    #[param(example = 1)]
    pub gs_id: i64,
    #[param(example = json!(["command1", "command2"]))]
    pub commands: Vec<String>,
}

#[derive(ToSchema, IntoParams, Debug, Deserialize, Validate)]
#[into_params(style = Form)]
#[serde(rename_all = "camelCase")]
pub struct SatelliteCreateRequest {
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

#[derive(ToSchema, IntoParams, Debug, Deserialize, Validate)]
#[into_params(style = Form)]
#[serde(rename_all = "camelCase")]
pub struct TleUpdateRequest {
    #[validate(length(min = 1, message = "TLE cannot be empty"))]
    #[schema(
        example = "1 33591U 09005A   24305.51234567  .00000020  00000-0  12000-4 0  9993\n2 33591  99.1234 123.4567 0012345 123.4567 234.5678 14.12345678901234"
    )]
    pub tle: String,
}
