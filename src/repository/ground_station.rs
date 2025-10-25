use crate::models::entities::GroundStation;
use sqlx::{Pool, Postgres};

pub struct GroundStationRepository {
    pool: Pool<Postgres>,
}

impl GroundStationRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn create_ground_station(
        &self,
        ground_station: &GroundStation,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        sqlx::query!(
            r#"
            INSERT INTO ground_stations (id, name, latitude, longitude, altitude, satellite)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            ground_station.id,
            ground_station.name,
            ground_station.latitude,
            ground_station.longitude,
            ground_station.altitude,
            ground_station.tle
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_all_ground_stations(
        &self,
    ) -> Result<Vec<GroundStation>, Box<dyn std::error::Error + Send + Sync>> {
        let gss = sqlx::query_as!(
            GroundStation,
            r#"SELECT id, name, latitude, longitude, altitude, satellite AS tle FROM ground_stations"#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(gss)
    }

    pub async fn get_ground_station(
        &self,
        id: &String,
    ) -> Result<Option<GroundStation>, Box<dyn std::error::Error + Send + Sync>> {
        let query_result = sqlx::query_as!(
            GroundStation,
            r#"SELECT id, name, latitude, longitude, altitude, satellite AS tle FROM ground_stations WHERE id = $1"#,
            id
        )
        .fetch_one(&self.pool)
        .await;
        match query_result {
            Ok(gs) => Ok(Some(gs)),
            Err(e) => {
                if let sqlx::Error::RowNotFound = e {
                    Ok(None)
                } else {
                    Err(e.into())
                }
            }
        }
    }

    pub async fn set_tle_for_ground_station(
        &self,
        id: &String,
        tle: &String,
    ) -> Result<Option<()>, Box<dyn std::error::Error + Send + Sync>> {
        let query_result: Result<sqlx::postgres::PgQueryResult, sqlx::Error> = sqlx::query!(
            r#"UPDATE ground_stations SET satellite = $1 WHERE id = $2"#,
            tle,
            id
        )
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
}
