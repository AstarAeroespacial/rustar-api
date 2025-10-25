use crate::models::requests::GroundStationCreateRequest;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct GroundStation {
    pub id: String,
    pub name: String,
    pub latitude: f32,
    pub longitude: f32,
    pub altitude: i32,
    pub tle: Option<String>,
}

impl GroundStation {
    pub fn from_request(req: GroundStationCreateRequest) -> Self {
        Self {
            id: req.id,
            name: req.name,
            latitude: req.latitude,
            longitude: req.longitude,
            altitude: req.altitude,
            tle: None,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Job {
    pub id: String,
    pub gs_id: String,
    pub sat_id: String,
    pub start_time: i32,
    pub end_time: i32,
    pub commands: Vec<String>,
}

impl Job {
    pub fn new(gs_id: &String, sat_id: &String, commands: &Vec<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            gs_id: gs_id.clone(),
            sat_id: sat_id.clone(),
            start_time: 0,
            end_time: 0,
            commands: commands.clone(),
        }
    }
}
