use crate::models::requests::{GroundStationCreateRequest, SatelliteCreateRequest};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "job_status", rename_all = "PascalCase")]
pub enum JobStatus {
    Sent,
    Received,
    Started,
    Completed,
    Error,
}

#[derive(Serialize, Deserialize)]
pub struct GroundStation {
    pub id: i64,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: i64,
}

impl GroundStation {
    pub fn from_request(req: GroundStationCreateRequest) -> Self {
        Self {
            id: 0,
            name: req.name,
            latitude: req.latitude as f64,
            longitude: req.longitude as f64,
            altitude: req.altitude as i64,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Satellite {
    pub id: i64,
    pub name: String,
    pub tle: String,
    pub downlink_frequency: f64,
    pub uplink_frequency: f64,
}

impl Satellite {
    pub fn from_request(req: SatelliteCreateRequest) -> Self {
        Self {
            id: 0,
            name: req.name,
            tle: req.tle,
            downlink_frequency: req.downlink_frequency,
            uplink_frequency: req.uplink_frequency,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Job {
    pub id: i64,
    pub gs_id: i64,
    pub sat_id: i64,
    pub start_time: i32,
    pub end_time: i32,
    pub commands: Vec<String>,
}

impl Job {
    pub fn new(gs_id: &i64, sat_id: &i64, commands: &Vec<String>) -> Self {
        Self {
            id: 0,
            gs_id: *gs_id,
            sat_id: *sat_id,
            start_time: 0,
            end_time: 0,
            commands: commands.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct JobStatusUpdate {
    pub job_id: i64,
    pub timestamp: DateTime<Utc>,
    pub status: JobStatus,
}
