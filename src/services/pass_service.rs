use crate::{
    models::responses::PassInfo,
    repository::{ground_station::GroundStationRepository, satellite::SatelliteRepository},
    services::errors::ServiceError,
};
use chrono::{DateTime, Duration, Utc};
use tracking::{Elements, Observer, Tracker};

pub struct PassService {
    satellite_repository: SatelliteRepository,
    ground_station_repository: GroundStationRepository,
}

impl PassService {
    pub fn new(
        satellite_repository: SatelliteRepository,
        ground_station_repository: GroundStationRepository,
    ) -> Self {
        Self {
            satellite_repository,
            ground_station_repository,
        }
    }

    /// Get upcoming passes for a satellite across all ground stations
    pub async fn get_satellite_passes(
        &self,
        sat_id: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<PassInfo>, ServiceError> {
        // Get satellite
        let satellite = self
            .satellite_repository
            .get_satellite(sat_id)
            .await?
            .ok_or_else(|| ServiceError::NotFound(format!("Satellite {} not found", sat_id)))?;

        // Parse TLE
        let tle_lines: Vec<&str> = satellite.tle.lines().collect();
        if tle_lines.len() < 3 {
            return Err(ServiceError::BadRequest(
                "Invalid TLE format: expected 3 lines (name, line1, line2)".to_string(),
            ));
        }

        // Get all ground stations
        let ground_stations = self
            .ground_station_repository
            .get_all_ground_stations()
            .await?;

        if ground_stations.is_empty() {
            return Ok(Vec::new());
        }

        // Calculate passes for each ground station
        let mut all_passes = Vec::new();
        let duration = end.signed_duration_since(start);
        let window = std::time::Duration::from_secs(duration.num_seconds() as u64);

        for gs in ground_stations {
            let observer = Observer::new(gs.latitude, gs.longitude, gs.altitude as f64);

            // Parse TLE for each ground station
            let gs_elements = match Elements::from_tle(
                Some(tle_lines[0].to_string()),
                tle_lines[1].as_bytes(),
                tle_lines[2].as_bytes(),
            ) {
                Ok(e) => e,
                Err(e) => {
                    log::error!("Error parsing TLE for GS {}: {:?}", gs.id, e);
                    continue;
                }
            };

            match Tracker::new(&observer, gs_elements) {
                Ok(tracker) => {
                    // Get all passes within the time window using next_passes
                    if let Some(passes) = tracker.next_passes(start, window) {
                        for pass in passes.passes {
                            if let (Some(aos), Some(los)) = (pass.aos, pass.los) {
                                let aos_time =
                                    DateTime::from_timestamp(aos.time as i64, 0).unwrap_or(start);
                                let los_time =
                                    DateTime::from_timestamp(los.time as i64, 0).unwrap_or(start);

                                // Calculate max elevation during the pass
                                let mut max_elevation = 0.0;
                                let mut check_time = aos_time;
                                while check_time <= los_time {
                                    if let Ok(observation) = tracker.track(check_time) {
                                        if observation.elevation > max_elevation {
                                            max_elevation = observation.elevation;
                                        }
                                    }
                                    check_time = check_time + Duration::seconds(30);
                                }

                                // Only include passes with elevation >= 10 degrees
                                if max_elevation >= 10.0 {
                                    all_passes.push(PassInfo {
                                        gs_id: gs.id.clone(),
                                        aos: aos_time,
                                        los: los_time,
                                        max_elevation,
                                    });
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    log::error!("Error creating tracker for GS {}: {:?}", gs.id, e);
                }
            }
        }

        // Sort by AOS time
        all_passes.sort_by(|a, b| a.aos.cmp(&b.aos));

        Ok(all_passes)
    }
}
