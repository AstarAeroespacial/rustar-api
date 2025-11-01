use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};

use crate::repository::errors::RepositoryError;

#[derive(sqlx::FromRow)]
pub struct TelemetryDb {
    pub id: i64,
    pub timestamp: DateTime<Utc>,
    pub sat_id: i64,
    pub gs_id: i64,
    pub payload: Option<Vec<u8>>,
}

pub struct TelemetryRepository {
    pool: Pool<Postgres>,
}

impl TelemetryRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    /// Fetch all telemetry records for a specific satellite
    pub async fn get_telemetry_by_satellite(
        &self,
        satellite_id: &i64,
    ) -> Result<Vec<TelemetryDb>, RepositoryError> {
        let telemetry = sqlx::query_as!(
            TelemetryDb,
            r#"
            SELECT id, timestamp, sat_id, gs_id, payload
            FROM telemetry
            WHERE sat_id = $1
            ORDER BY timestamp DESC
            "#,
            satellite_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(RepositoryError::from)?;

        Ok(telemetry)
    }
}
