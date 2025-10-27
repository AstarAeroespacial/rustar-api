use chrono::{DateTime, Utc};
use rustar_types::telemetry::TelemetryRecord;
use sqlx::{Pool, Postgres};

#[derive(sqlx::FromRow)]
struct TelemetryDb {
    id: i64,
    timestamp: DateTime<Utc>,
    sat_id: i64,
    gs_id: i64,
    payload: Vec<u8>,
}

pub struct TelemetryRepository {
    pool: Pool<Postgres>,
}

impl TelemetryRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn get_latest(
        &self,
        _sat_name: String,
        limit: i32,
    ) -> Result<Vec<TelemetryRecord>, Box<dyn std::error::Error + Send + Sync>> {
        // Note: This needs to be adapted based on TelemetryRecord structure
        // The database stores raw payload bytes, but TelemetryRecord expects decoded fields
        // TODO: Implement proper payload decoding
        let _records = sqlx::query_as::<_, TelemetryDb>(
            r#"
            SELECT id, timestamp, sat_id, gs_id, payload
            FROM telemetry
            ORDER BY timestamp DESC
            LIMIT $1
            "#,
        )
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await?;

        // For now, return empty vec since we need payload decoding logic
        // TODO: Decode payload bytes into temperature, voltage, current, battery_level
        Ok(vec![])
    }

    pub async fn get_historic(
        &self,
        _sat_name: String,
        start_time: Option<i64>,
        end_time: Option<i64>,
    ) -> Result<Vec<TelemetryRecord>, Box<dyn std::error::Error + Send + Sync>> {
        let start_ts = start_time
            .map(|ts| DateTime::from_timestamp(ts, 0).unwrap_or_default())
            .unwrap_or(DateTime::UNIX_EPOCH);
        let end_ts = end_time
            .map(|ts| DateTime::from_timestamp(ts, 0).unwrap_or_default())
            .unwrap_or_else(|| Utc::now());

        let _records = sqlx::query_as::<_, TelemetryDb>(
            r#"
            SELECT id, timestamp, sat_id, gs_id, payload
            FROM telemetry
            WHERE timestamp >= $1 AND timestamp <= $2
            ORDER BY timestamp DESC
            "#,
        )
        .bind(start_ts)
        .bind(end_ts)
        .fetch_all(&self.pool)
        .await?;

        // For now, return empty vec since we need payload decoding logic
        // TODO: Decode payload bytes into temperature, voltage, current, battery_level
        Ok(vec![])
    }

    pub async fn save(
        &self,
        _telemetry: TelemetryRecord,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // TODO: This method needs to be completely rewritten to match the actual DB schema
        // The DB uses: (id: i64, timestamp: DateTime<Utc>, sat_id: i64, gs_id: i64, payload: bytes)
        // But TelemetryRecord (from rustar-types) uses different fields
        //
        // Required steps:
        // 1. Encode TelemetryRecord fields into payload bytes
        // 2. Get sat_id and gs_id from context
        // 3. Convert timestamp to DateTime<Utc>
        // 4. Parse/convert id to i64

        // For now, this is a no-op stub
        Ok(())
    }
}
