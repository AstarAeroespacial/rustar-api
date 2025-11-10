use crate::models::entities::Job;
use crate::repository::errors::RepositoryError;
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
        gs_id: &str,
        sat_id: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        commands: &Vec<String>,
    ) -> Result<Job, RepositoryError> {
        let job = sqlx::query_as!(
            Job,
            r#"
            INSERT INTO jobs (gs_id, sat_id, start, "end", commands)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, gs_id, sat_id, start as "start!", "end" as "end!", commands
            "#,
            gs_id,
            sat_id,
            start,
            end,
            commands
        )
        .fetch_one(&self.pool)
        .await
        .map_err(RepositoryError::from)?;

        Ok(job)
    }
}
