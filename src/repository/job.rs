use crate::models::entities::Job;
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
        sqlx::query!(
            r#"
            INSERT INTO jobs (id, gs_id, sat_id, start_time, end_time, commands)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            job.id,
            job.gs_id,
            job.sat_id,
            job.start_time,
            job.end_time,
            &job.commands
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
