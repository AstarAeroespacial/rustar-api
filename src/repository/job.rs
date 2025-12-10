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
        let job_record = sqlx::query!(
            r#"
            INSERT INTO jobs (gs_id, sat_id, start, "end")
            VALUES ($1, $2, $3, $4)
            RETURNING id, gs_id, sat_id, start, "end"
            "#,
            gs_id,
            sat_id,
            start,
            end
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(RepositoryError::from)?;

        let job = Job {
            id: job_record.id,
            gs_id: job_record.gs_id,
            sat_id: job_record.sat_id,
            start: job_record.start,
            end: job_record.end,
            commands: Some(commands.clone()),
        };

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

    pub async fn get_all_jobs(&self) -> Result<Vec<Job>, RepositoryError> {
        let job_records = sqlx::query!(
            r#"
            SELECT id, gs_id, sat_id, start, "end"
            FROM jobs
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(RepositoryError::from)?;

        let mut jobs = Vec::new();

        for record in job_records {
            // Fetch commands for each job
            let command_records = sqlx::query!(
                r#"
                SELECT command
                FROM job_commands
                WHERE job_id = $1
                "#,
                record.id
            )
            .fetch_all(&self.pool)
            .await
            .map_err(RepositoryError::from)?;

            let commands: Vec<String> = command_records
                .into_iter()
                .map(|cmd_record| cmd_record.command)
                .collect();

            let job = Job {
                id: record.id,
                gs_id: record.gs_id,
                sat_id: record.sat_id,
                start: record.start,
                end: record.end,
                commands: Some(commands),
            };

            jobs.push(job);
        }

        Ok(jobs)
    }

    pub async fn get_job(&self, id: i64) -> Result<Option<Job>, RepositoryError> {
        let job_record = sqlx::query!(
            r#"
            SELECT id, gs_id, sat_id, start, "end"
            FROM jobs
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(RepositoryError::from)?;

        if let Some(record) = job_record {
            // Fetch commands for the job
            let command_records = sqlx::query!(
                r#"
                SELECT command
                FROM job_commands
                WHERE job_id = $1
                "#,
                record.id
            )
            .fetch_all(&self.pool)
            .await
            .map_err(RepositoryError::from)?;

            let commands: Vec<String> = command_records
                .into_iter()
                .map(|cmd_record| cmd_record.command)
                .collect();

            let job = Job {
                id: record.id,
                gs_id: record.gs_id,
                sat_id: record.sat_id,
                start: record.start,
                end: record.end,
                commands: Some(commands),
            };

            Ok(Some(job))
        } else {
            Ok(None)
        }
    }
}
