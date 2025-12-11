use crate::{
    models::responses::PassInfo,
    repository::{ground_station::GroundStationRepository, satellite::SatelliteRepository},
    services::errors::ServiceError,
};
use chrono::{DateTime, Utc};
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
                            if let (Some(aos), Some(los), Some(max_elevation)) =
                                (pass.aos, pass.los, pass.max_elevation)
                            {
                                let aos_time =
                                    DateTime::from_timestamp(aos.time as i64, 0).unwrap_or(start);
                                let los_time =
                                    DateTime::from_timestamp(los.time as i64, 0).unwrap_or(start);

                                // Convert max_elevation from radians to degrees
                                let max_elevation_deg = max_elevation.to_degrees();

                                // Only include passes with elevation >= 10 degrees
                                if max_elevation_deg >= 10.0 {
                                    all_passes.push(PassInfo {
                                        gs_id: gs.id.clone(),
                                        sat_id: satellite.id.clone(),
                                        aos: aos_time,
                                        los: los_time,
                                        max_elevation: max_elevation_deg,
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

    /// Get upcoming passes for all satellites visible from a specific ground station
    pub async fn get_ground_station_passes(
        &self,
        gs_id: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<PassInfo>, ServiceError> {
        // Get ground station
        let ground_station = self
            .ground_station_repository
            .get_ground_station(gs_id)
            .await?
            .ok_or_else(|| ServiceError::NotFound(format!("Ground station {} not found", gs_id)))?;

        // Get all satellites
        let satellites = self.satellite_repository.get_all_satellites().await?;

        if satellites.is_empty() {
            return Ok(Vec::new());
        }

        // Calculate passes for each satellite
        let mut all_passes = Vec::new();
        let duration = end.signed_duration_since(start);
        let window = std::time::Duration::from_secs(duration.num_seconds() as u64);
        let observer = Observer::new(
            ground_station.latitude,
            ground_station.longitude,
            ground_station.altitude as f64,
        );

        for satellite in satellites {
            // Parse TLE
            let tle_lines: Vec<&str> = satellite.tle.lines().collect();
            if tle_lines.len() < 3 {
                log::warn!(
                    "Invalid TLE format for satellite {}: expected 3 lines",
                    satellite.id
                );
                continue;
            }

            let sat_elements = match Elements::from_tle(
                Some(tle_lines[0].to_string()),
                tle_lines[1].as_bytes(),
                tle_lines[2].as_bytes(),
            ) {
                Ok(e) => e,
                Err(e) => {
                    log::error!("Error parsing TLE for satellite {}: {:?}", satellite.id, e);
                    continue;
                }
            };

            match Tracker::new(&observer, sat_elements) {
                Ok(tracker) => {
                    // Get all passes within the time window
                    if let Some(passes) = tracker.next_passes(start, window) {
                        for pass in passes.passes {
                            if let (Some(aos), Some(los), Some(max_elevation)) =
                                (pass.aos, pass.los, pass.max_elevation)
                            {
                                let aos_time =
                                    DateTime::from_timestamp(aos.time as i64, 0).unwrap_or(start);
                                let los_time =
                                    DateTime::from_timestamp(los.time as i64, 0).unwrap_or(start);

                                // Convert max_elevation from radians to degrees
                                let max_elevation_deg = max_elevation.to_degrees();

                                // Only include passes with elevation >= 10 degrees
                                if max_elevation_deg >= 10.0 {
                                    all_passes.push(PassInfo {
                                        gs_id: ground_station.id.clone(),
                                        sat_id: satellite.id.clone(),
                                        aos: aos_time,
                                        los: los_time,
                                        max_elevation: max_elevation_deg,
                                    });
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    log::error!(
                        "Error creating tracker for satellite {}: {:?}",
                        satellite.id,
                        e
                    );
                }
            }
        }

        // Sort by AOS time
        all_passes.sort_by(|a, b| a.aos.cmp(&b.aos));

        Ok(all_passes)
    }
}
