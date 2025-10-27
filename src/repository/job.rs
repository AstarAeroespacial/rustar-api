use crate::models::entities::Job;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};

pub struct JobRepository {
    pool: Pool<Postgres>,
}

impl JobRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn create_job(
        &self,
        job: &Job,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Convert timestamps from i32 to DateTime
        let start_dt =
            DateTime::from_timestamp(job.start_time as i64, 0).unwrap_or_else(|| Utc::now());
        let end_dt = DateTime::from_timestamp(job.end_time as i64, 0).unwrap_or_else(|| Utc::now());

        // Note: DB schema doesn't have 'commands' column
        // Commands are stored elsewhere or not persisted
        sqlx::query!(
            r#"
            INSERT INTO jobs (id, sat_id, gs_id, start, "end")
            VALUES ($1, $2, $3, $4, $5)
            "#,
            job.id,
            job.sat_id,
            job.gs_id,
            start_dt,
            end_dt
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
