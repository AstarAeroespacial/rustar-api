use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

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
    pub id: String,
    pub name: String,
    pub latitude: f32,
    pub longitude: f32,
    pub altitude: i32,
}
