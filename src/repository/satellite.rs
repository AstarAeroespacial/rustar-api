use crate::models::entities::Satellite;
use sqlx::{Pool, Postgres};

pub struct SatelliteRepository {
    pool: Pool<Postgres>,
}

impl SatelliteRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn create_satellite(
        &self,
        satellite: &Satellite,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        sqlx::query!(
            r#"
            INSERT INTO satellites (id, name, tle, downlink_frequency, uplink_frequency)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            satellite.id,
            satellite.name,
            satellite.tle,
            satellite.downlink_frequency,
            satellite.uplink_frequency
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_all_satellites(
        &self,
    ) -> Result<Vec<Satellite>, Box<dyn std::error::Error + Send + Sync>> {
        let satellites = sqlx::query_as!(
            Satellite,
            r#"
            SELECT id, name, tle, downlink_frequency, uplink_frequency
            FROM satellites
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(satellites)
    }

    pub async fn get_satellite(
        &self,
        id: &i64,
    ) -> Result<Option<Satellite>, Box<dyn std::error::Error + Send + Sync>> {
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
        .await?;

        Ok(satellite)
    }

    pub async fn update_tle(
        &self,
        id: &i64,
        tle: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        sqlx::query!(
            r#"
            UPDATE satellites
            SET tle = $2
            WHERE id = $1
            "#,
            id,
            tle
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn update_frequencies(
        &self,
        id: &i64,
        downlink_frequency: f64,
        uplink_frequency: f64,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        sqlx::query!(
            r#"
            UPDATE satellites
            SET downlink_frequency = $2, uplink_frequency = $3
            WHERE id = $1
            "#,
            id,
            downlink_frequency,
            uplink_frequency
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete_satellite(
        &self,
        id: &i64,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        sqlx::query!(
            r#"
            DELETE FROM satellites
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
