use crate::{
    models::entities::Satellite, repository::satellite::SatelliteRepository,
    services::errors::ServiceError,
};

pub struct SatelliteService {
    repository: SatelliteRepository,
}

impl SatelliteService {
    pub fn new(repository: SatelliteRepository) -> Self {
        Self { repository }
    }

    /// Create a new satellite
    pub async fn create_satellite(&self, satellite: &Satellite) -> Result<Satellite, ServiceError> {
        let existing = self
            .repository
            .get_all_satellites()
            .await?
            .into_iter()
            .find(|s| s.name.eq_ignore_ascii_case(&satellite.name));

        if existing.is_some() {
            return Err(ServiceError::Conflict(format!(
                "Satellite with name '{}' already exists",
                satellite.name
            )));
        }

        if satellite.tle.trim().is_empty() {
            return Err(ServiceError::BadRequest("TLE cannot be empty".into()));
        }

        if satellite.downlink_frequency <= 0.0 || satellite.uplink_frequency <= 0.0 {
            return Err(ServiceError::BadRequest(
                "Frequencies must be positive numbers".into(),
            ));
        }

        self.repository
            .create_satellite(satellite)
            .await
            .map_err(ServiceError::from)
    }

    /// Get all satellites
    pub async fn get_all_satellites(&self) -> Result<Vec<Satellite>, ServiceError> {
        self.repository
            .get_all_satellites()
            .await
            .map_err(ServiceError::from)
    }

    /// Get one satellite by ID
    pub async fn get_satellite(&self, id: &i64) -> Result<Option<Satellite>, ServiceError> {
        self.repository
            .get_satellite(id)
            .await
            .map_err(ServiceError::from)
    }

    /// Update only the TLE of a satellite
    pub async fn update_satellite_tle(
        &self,
        id: &i64,
        tle: String,
    ) -> Result<Option<Satellite>, ServiceError> {
        if tle.trim().is_empty() {
            return Err(ServiceError::BadRequest("TLE cannot be empty".into()));
        }

        if let Some(mut sat) = self.repository.get_satellite(id).await? {
            if sat.tle == tle {
                // Nada que actualizar, devolver igual
                return Ok(Some(sat));
            }

            let updated = self.repository.update_tle(id, &tle).await?;

            if updated {
                sat.tle = tle;
                Ok(Some(sat))
            } else {
                Ok(None)
            }
        } else {
            Err(ServiceError::NotFound(format!(
                "Satellite with ID {} not found",
                id
            )))
        }
    }

    /// Delete a satellite by ID
    pub async fn delete_satellite(&self, id: &i64) -> Result<bool, ServiceError> {
        let result = self.repository.delete_satellite(id).await?;

        if result {
            Ok(true)
        } else {
            Err(ServiceError::NotFound(format!(
                "Satellite with ID {} not found",
                id
            )))
        }
    }
}
