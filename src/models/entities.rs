use crate::models::requests::{GroundStationCreateRequest, SatelliteCreateRequest};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "job_status", rename_all = "PascalCase")]
pub enum JobStatus {
    Sent,
    Received,
    Scheduled,
    Started,
    Completed,
    Error,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct GroundStation {
    pub id: String,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
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
pub struct Satellite {
    pub id: String,
    pub name: String,
    pub tle: String,
    pub downlink_frequency: f64,
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
pub struct Job {
    pub id: i64,
    pub sat_id: String,
    pub gs_id: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub commands: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct JobStatusUpdate {
    pub job_id: i64,
    pub timestamp: DateTime<Utc>,
    pub status: JobStatus,
}
