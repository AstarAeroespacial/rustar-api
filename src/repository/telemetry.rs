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

    /// Fetch telemetry records for a specific satellite with optional pagination
    pub async fn get_telemetry_by_satellite(
        &self,
        satellite_id: &i64,
        limit: Option<i64>,
        page: Option<i64>,
    ) -> Result<Vec<TelemetryDb>, RepositoryError> {
        let limit_value = limit.unwrap_or(i64::MAX); // No limit if not provided
        let page_value = page.unwrap_or(1); // Default to page 1
        let offset = (page_value - 1) * limit_value;

        // Build query with LIMIT and OFFSET
        // If limit is not provided (i64::MAX), effectively no limit
        let query = if limit.is_some() {
            r#"
            SELECT id, timestamp, sat_id, gs_id, payload
            FROM telemetry
            WHERE sat_id = $1
            ORDER BY timestamp DESC
            LIMIT $2 OFFSET $3
            "#
        } else if page.is_some() {
            // If only page is provided without limit, use a reasonable default limit
            r#"
            SELECT id, timestamp, sat_id, gs_id, payload
            FROM telemetry
            WHERE sat_id = $1
            ORDER BY timestamp DESC
            LIMIT 50 OFFSET $3
            "#
        } else {
            // No pagination at all
            r#"
            SELECT id, timestamp, sat_id, gs_id, payload
            FROM telemetry
            WHERE sat_id = $1
            ORDER BY timestamp DESC
            "#
        };

        let mut query_builder = sqlx::query_as::<_, TelemetryDb>(query).bind(satellite_id);

        if limit.is_some() {
            query_builder = query_builder.bind(limit_value).bind(offset);
        } else if page.is_some() {
            query_builder = query_builder.bind((page_value - 1) * 50);
        }

        let telemetry = query_builder
            .fetch_all(&self.pool)
            .await
            .map_err(RepositoryError::from)?;

        Ok(telemetry)
    }
}
