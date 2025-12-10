use crate::{
    repository::{ground_station::GroundStationRepository, satellite::SatelliteRepository},
    services::errors::ServiceError,
};

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
}
