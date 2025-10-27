use chrono::{Duration, Utc};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenvy::dotenv();
    let database_url = std::env::var("API_DATABASE_URL").expect("DATABASE_URL must be set");

    // Connect to database
    let pool = PgPool::connect(&database_url).await?;

    println!("Seeding database with test telemetry data...");

    // Generate some test data for the last 24 hours
    let now = Utc::now();
    let mut current_time = now - Duration::hours(24);

    for i in 0..100 {
        // Create a simple payload (in real scenario this would be encoded telemetry data)
        let payload = format!(
            "temp:{},volt:{},curr:{},batt:{}",
            20.0 + (i as f32 * 0.1),  // Varying temperature
            12.0 + (i as f32 * 0.01), // Varying voltage
            1.0 + (i as f32 * 0.005), // Varying current
            50 + (i % 20)             // Varying battery level
        )
        .into_bytes();

        sqlx::query!(
            r#"
            INSERT INTO telemetry (id, timestamp, sat_id, gs_id, payload)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            i as i64, // Use sequential IDs
            current_time,
            1i64, // Default satellite ID
            1i64, // Default ground station ID
            &payload
        )
        .execute(&pool)
        .await?;

        current_time += Duration::minutes(15); // 15-minute intervals
    }

    println!("Successfully seeded database with 100 telemetry records!");
    Ok(())
}
