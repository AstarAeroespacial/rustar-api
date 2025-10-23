use crate::models::entities::GroundStation;
use sqlx::{Pool, Postgres};

pub struct GroundStationRepository {
    pool: Pool<Postgres>,
}

impl GroundStationRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn create_ground_station(
        &self,
        ground_station: GroundStation,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        sqlx::query!(
            r#"
            INSERT INTO ground_stations (id, name, latitude, longitude, altitude)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            ground_station.id,
            ground_station.name,
            ground_station.latitude,
            ground_station.longitude,
            ground_station.altitude
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
