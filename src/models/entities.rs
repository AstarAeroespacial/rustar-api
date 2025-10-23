use crate::models::requests::GroundStationCreateRequest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GroundStation {
    pub id: String,
    pub name: String,
    pub latitude: f32,
    pub longitude: f32,
    pub altitude: i32,
}

impl GroundStation {
    pub fn from_request(req: GroundStationCreateRequest) -> Self {
        Self {
            id: req.id,
            name: req.name,
            latitude: req.latitude,
            longitude: req.longitude,
            altitude: req.altitude,
        }
    }
}
