use crate::models::requests::{GroundStationCreateRequest, SatelliteCreateRequest};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "job_status", rename_all = "PascalCase")]
#[schema(example = "Sent")]
pub enum JobStatus {
    Sent,
    Received,
    Scheduled,
    Started,
    Completed,
    Error,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "id": "gs-001",
    "name": "Ground Station Buenos Aires",
    "latitude": -34.6037,
    "longitude": -58.3816,
    "altitude": 25
}))]
pub struct GroundStation {
    #[schema(example = "gs-001")]
    pub id: String,
    #[schema(example = "Ground Station Buenos Aires")]
    pub name: String,
    #[schema(example = -34.6037)]
    pub latitude: f64,
    #[schema(example = -58.3816)]
    pub longitude: f64,
    #[schema(example = 25)]
    pub altitude: i64,
}

impl GroundStation {
    pub fn from_request(req: GroundStationCreateRequest) -> Self {
        Self {
            id: req.id,
            name: req.name,
            latitude: req.latitude as f64,
            longitude: req.longitude as f64,
            altitude: req.altitude as i64,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
#[schema(example = json!({
    "id": "sat-001",
    "name": "NOAA 19",
    "tle": "1 33591U 09005A   24304.41234567  .00000023  00000-0  12345-4 0  9992\n2 33591  99.1234 123.4567 0012345 123.4567 234.5678 14.12345678901234",
    "downlink_frequency": 137.1,
    "uplink_frequency": 145.8
}))]
pub struct Satellite {
    #[schema(example = "sat-001")]
    pub id: String,
    #[schema(example = "NOAA 19")]
    pub name: String,
    #[schema(
        example = "1 33591U 09005A   24304.41234567  .00000023  00000-0  12345-4 0  9992\n2 33591  99.1234 123.4567 0012345 123.4567 234.5678 14.12345678901234"
    )]
    pub tle: String,
    #[schema(example = 137.1)]
    pub downlink_frequency: f64,
    #[schema(example = 145.8)]
    pub uplink_frequency: f64,
}

impl Satellite {
    pub fn from_request(req: SatelliteCreateRequest) -> Self {
        Self {
            id: req.id,
            name: req.name,
            tle: req.tle,
            downlink_frequency: req.downlink_frequency,
            uplink_frequency: req.uplink_frequency,
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "id": 1,
    "sat_id": "sat-001",
    "gs_id": "gs-001",
    "start": "2024-12-10T12:00:00Z",
    "end": "2024-12-10T12:15:00Z",
    "commands": ["command1", "command2"]
}))]
pub struct Job {
    #[schema(example = 1)]
    pub id: i64,
    #[schema(example = "sat-001")]
    pub sat_id: String,
    #[schema(example = "gs-001")]
    pub gs_id: String,
    #[schema(value_type = String, example = "2024-12-10T12:00:00Z")]
    pub start: DateTime<Utc>,
    #[schema(value_type = String, example = "2024-12-10T12:15:00Z")]
    pub end: DateTime<Utc>,
    #[schema(example = json!(["command1", "command2"]))]
    pub commands: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "job_id": 1,
    "timestamp": "2024-12-10T12:00:00Z",
    "status": "Sent"
}))]
pub struct JobStatusUpdate {
    #[schema(example = 1)]
    pub job_id: i64,
    #[schema(value_type = String, example = "2024-12-10T12:00:00Z")]
    pub timestamp: DateTime<Utc>,
    #[schema(example = "Sent")]
    pub status: JobStatus,
}
