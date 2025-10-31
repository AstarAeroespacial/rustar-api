use crate::{
    models::entities::GroundStation, repository::ground_station::GroundStationRepository,
    services::errors::ServiceError,
};

pub struct GroundStationService {
    repository: GroundStationRepository,
}

impl GroundStationService {
    pub fn new(repository: GroundStationRepository) -> Self {
        Self { repository }
    }

    /// Create a new ground station
    pub async fn create_ground_station(
        &self,
        ground_station: &GroundStation,
    ) -> Result<GroundStation, ServiceError> {
        // Validation rules
        const LAT_MIN: f64 = -90.0;
        const LAT_MAX: f64 = 90.0;
        const LON_MIN: f64 = -180.0;
        const LON_MAX: f64 = 180.0;
        const ALT_MIN: i64 = -500; // allow slightly below sea level
        const ALT_MAX: i64 = 10000; // arbitrary upper bound for ground stations

        // Basic name checks
        let name_trimmed = ground_station.name.trim();
        if name_trimmed.is_empty() {
            return Err(ServiceError::BadRequest("Name cannot be empty".into()));
        }

        // Coordinate range checks
        if !(LAT_MIN..=LAT_MAX).contains(&ground_station.latitude) {
            return Err(ServiceError::BadRequest(format!(
                "Latitude must be between {} and {}",
                LAT_MIN, LAT_MAX
            )));
        }
        if !(LON_MIN..=LON_MAX).contains(&ground_station.longitude) {
            return Err(ServiceError::BadRequest(format!(
                "Longitude must be between {} and {}",
                LON_MIN, LON_MAX
            )));
        }

        // Altitude sanity
        if !(ALT_MIN..=ALT_MAX).contains(&ground_station.altitude) {
            return Err(ServiceError::BadRequest(format!(
                "Altitude must be between {} and {} meters",
                ALT_MIN, ALT_MAX
            )));
        }

        // Check for duplicates by name
        let all_gs = self.repository.get_all_ground_stations().await?;
        if all_gs
            .iter()
            .any(|gs| gs.name.eq_ignore_ascii_case(name_trimmed))
        {
            return Err(ServiceError::Conflict(format!(
                "Ground station with name '{}' already exists",
                name_trimmed
            )));
        }

        // Passed validations; create record
        self.repository
            .create_ground_station(ground_station)
            .await
            .map_err(ServiceError::from)
    }

    /// Get all ground stations
    pub async fn get_all_ground_stations(&self) -> Result<Vec<GroundStation>, ServiceError> {
        self.repository
            .get_all_ground_stations()
            .await
            .map_err(ServiceError::from)
    }

    /// Get a ground station by ID
    pub async fn get_ground_station(
        &self,
        id: &i64,
    ) -> Result<Option<GroundStation>, ServiceError> {
        self.repository
            .get_ground_station(id)
            .await
            .map_err(ServiceError::from)
    }

    /// Delete a ground station by id
    pub async fn delete_ground_station(&self, id: &i64) -> Result<bool, ServiceError> {
        let deleted = self
            .repository
            .delete_ground_station(id)
            .await
            .map_err(ServiceError::from)?;

        if deleted {
            Ok(true)
        } else {
            Err(ServiceError::NotFound(format!(
                "Ground station {} not found",
                id
            )))
        }
    }
}
