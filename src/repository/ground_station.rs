use crate::models::entities::GroundStation;
use crate::repository::errors::RepositoryError;
use sqlx::{Pool, Postgres};

pub struct GroundStationRepository {
    pool: Pool<Postgres>,
}

impl GroundStationRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    /// Create a new ground station and return the created record
    pub async fn create_ground_station(
        &self,
        ground_station: &GroundStation,
    ) -> Result<GroundStation, RepositoryError> {
        let ground_station = sqlx::query_as!(
            GroundStation,
            r#"
            INSERT INTO ground_stations (name, latitude, longitude, altitude)
            VALUES ($1, $2, $3, $4)
            RETURNING id, name, latitude, longitude, altitude
            "#,
            ground_station.name,
            ground_station.latitude,
            ground_station.longitude,
            ground_station.altitude
        )
        .fetch_one(&self.pool)
        .await
        .map_err(RepositoryError::from)?;

        Ok(ground_station)
    }

    /// Fetch all ground stations
    pub async fn get_all_ground_stations(&self) -> Result<Vec<GroundStation>, RepositoryError> {
        let ground_stations = sqlx::query_as!(
            GroundStation,
            r#"
            SELECT id, name, latitude, longitude, altitude
            FROM ground_stations
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(RepositoryError::from)?;

        Ok(ground_stations)
    }

    /// Fetch a single ground station by ID
    pub async fn get_ground_station(
        &self,
        id: &i64,
    ) -> Result<Option<GroundStation>, RepositoryError> {
        let ground_station = sqlx::query_as!(
            GroundStation,
            r#"
            SELECT id, name, latitude, longitude, altitude
            FROM ground_stations
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(RepositoryError::from)?;

        Ok(ground_station)
    }

    /// Delete a ground station by ID
    pub async fn delete_ground_station(&self, id: &i64) -> Result<bool, RepositoryError> {
        let result = sqlx::query!(
            r#"
            DELETE FROM ground_stations
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
