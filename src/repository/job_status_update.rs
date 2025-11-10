use crate::models::entities::{JobStatus, JobStatusUpdate};
use crate::repository::errors::RepositoryError;
use sqlx::{Pool, Postgres};

pub struct JobStatusUpdateRepository {
    pool: Pool<Postgres>,
}

impl JobStatusUpdateRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn create_status_update(
        &self,
        update: &JobStatusUpdate,
    ) -> Result<(), RepositoryError> {
        sqlx::query!(
            r#"
            INSERT INTO jobs_status_updates (job_id, timestamp, status)
            VALUES ($1, $2, $3)
            "#,
            update.job_id,
            update.timestamp,
            update.status as JobStatus
        )
        .execute(&self.pool)
        .await
        .map_err(RepositoryError::from)?;

        Ok(())
    }

    pub async fn get_job_status_updates(
        &self,
        job_id: &i64,
    ) -> Result<Vec<JobStatusUpdate>, RepositoryError> {
        let updates = sqlx::query_as!(
            JobStatusUpdate,
            r#"
            SELECT job_id, timestamp, status as "status: JobStatus"
            FROM jobs_status_updates
            WHERE job_id = $1
            ORDER BY timestamp ASC
            "#,
            job_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(RepositoryError::from)?;

        Ok(updates)
    }

    pub async fn get_latest_status(
        &self,
        job_id: &i64,
    ) -> Result<Option<JobStatusUpdate>, RepositoryError> {
        let query_result = sqlx::query_as!(
            JobStatusUpdate,
            r#"
            SELECT job_id, timestamp, status as "status: JobStatus"
            FROM jobs_status_updates
            WHERE job_id = $1
            ORDER BY timestamp DESC
            LIMIT 1
            "#,
            job_id
        )
        .fetch_one(&self.pool)
        .await;

        match query_result {
            Ok(update) => Ok(Some(update)),
            Err(e) => {
                if let sqlx::Error::RowNotFound = e {
                    Ok(None)
                } else {
                    Err(RepositoryError::from(e))
                }
            }
        }
    }
}
