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
        // Start a transaction
        let mut tx = self.pool.begin().await.map_err(RepositoryError::from)?;

        // Insert the job
        let job = sqlx::query_as!(
            Job,
            r#"
            INSERT INTO jobs (gs_id, sat_id, start, "end")
            VALUES ($1, $2, $3, $4)
            RETURNING id, gs_id, sat_id, start, "end", NULL::text[] as "commands: Option<Vec<String>>"
            "#,
            gs_id,
            sat_id,
            start,
            end
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(RepositoryError::from)?;

        // Insert commands
        for command in commands.iter() {
            sqlx::query!(
                r#"
                INSERT INTO job_commands (job_id, command)
                VALUES ($1, $2)
                "#,
                job.id,
                command
            )
            .execute(&mut *tx)
            .await
            .map_err(RepositoryError::from)?;
        }

        // Commit transaction
        tx.commit().await.map_err(RepositoryError::from)?;

        Ok(job)
    }
}
