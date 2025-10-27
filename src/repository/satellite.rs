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
            INSERT INTO satellites (id, name, tle)
            VALUES ($1, $2, $3)
            "#,
            satellite.id,
            satellite.name,
            satellite.tle
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_all_satellites(
        &self,
    ) -> Result<Vec<Satellite>, Box<dyn std::error::Error + Send + Sync>> {
        let satellites = sqlx::query_as!(Satellite, r#"SELECT id, name, tle FROM satellites"#)
            .fetch_all(&self.pool)
            .await?;
        Ok(satellites)
    }

    pub async fn get_satellite(
        &self,
        id: &i64,
    ) -> Result<Option<Satellite>, Box<dyn std::error::Error + Send + Sync>> {
        let query_result = sqlx::query_as!(
            Satellite,
            r#"SELECT id, name, tle FROM satellites WHERE id = $1"#,
            id
        )
        .fetch_one(&self.pool)
        .await;

        match query_result {
            Ok(sat) => Ok(Some(sat)),
            Err(e) => {
                if let sqlx::Error::RowNotFound = e {
                    Ok(None)
                } else {
                    Err(e.into())
                }
            }
        }
    }

    pub async fn update_tle(
        &self,
        id: &i64,
        tle: &String,
    ) -> Result<Option<()>, Box<dyn std::error::Error + Send + Sync>> {
        let query_result = sqlx::query!(r#"UPDATE satellites SET tle = $1 WHERE id = $2"#, tle, id)
            .execute(&self.pool)
            .await;

        match query_result {
            Ok(r) => {
                if r.rows_affected() == 0 {
                    Ok(None)
                } else {
                    Ok(Some(()))
                }
            }
            Err(e) => Err(e.into()),
        }
    }

    pub async fn delete_satellite(
        &self,
        id: &i64,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let result = sqlx::query!(r#"DELETE FROM satellites WHERE id = $1"#, id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}
