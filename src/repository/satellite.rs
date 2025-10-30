use crate::models::entities::Satellite;
use crate::repository::errors::RepositoryError;
use sqlx::{Pool, Postgres};

pub struct SatelliteRepository {
    pool: Pool<Postgres>,
}

impl SatelliteRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    /// Create a new satellite and return the created record
    pub async fn create_satellite(
        &self,
        satellite: &Satellite,
    ) -> Result<Satellite, RepositoryError> {
        let satellite = sqlx::query_as!(
            Satellite,
            r#"
            INSERT INTO satellites (name, tle, downlink_frequency, uplink_frequency)
            VALUES ($1, $2, $3, $4)
            RETURNING id, name, tle, downlink_frequency, uplink_frequency
            "#,
            satellite.name,
            satellite.tle,
            satellite.downlink_frequency,
            satellite.uplink_frequency
        )
        .fetch_one(&self.pool)
        .await
        .map_err(RepositoryError::from)?;

        Ok(satellite)
    }

    /// Fetch all satellites
    pub async fn get_all_satellites(&self) -> Result<Vec<Satellite>, RepositoryError> {
        let satellites = sqlx::query_as!(
            Satellite,
            r#"
            SELECT id, name, tle, downlink_frequency, uplink_frequency
            FROM satellites
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(RepositoryError::from)?;

        Ok(satellites)
    }

    /// Fetch a single satellite by ID
    pub async fn get_satellite(&self, id: &i64) -> Result<Option<Satellite>, RepositoryError> {
        let satellite = sqlx::query_as!(
            Satellite,
            r#"
            SELECT id, name, tle, downlink_frequency, uplink_frequency
            FROM satellites
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(RepositoryError::from)?;

        Ok(satellite)
    }

    /// Update only the TLE of a satellite
    pub async fn update_tle(&self, id: &i64, tle: &str) -> Result<bool, RepositoryError> {
        let result = sqlx::query!(
            r#"
            UPDATE satellites
            SET tle = $2
            WHERE id = $1
            "#,
            id,
            tle
        )
        .execute(&self.pool)
        .await
        .map_err(RepositoryError::from)?;

        Ok(result.rows_affected() > 0)
    }

    /// Delete a satellite by ID
    pub async fn delete_satellite(&self, id: &i64) -> Result<bool, RepositoryError> {
        let result = sqlx::query!(
            r#"
            DELETE FROM satellites
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await
        .map_err(RepositoryError::from)?;

        Ok(result.rows_affected() > 0)
    }
}
