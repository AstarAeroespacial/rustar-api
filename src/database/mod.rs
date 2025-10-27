use sqlx::{PgPool, Pool, Postgres};
use std::path::Path;

pub async fn create_pool(database_url: &str) -> Result<Pool<Postgres>, sqlx::Error> {
    // Create data directory if it doesn't exist
    if let Some(path) = Path::new(database_url).parent() {
        std::fs::create_dir_all(path).ok();
    }

    let mut retries = 0;
    let pool = loop {
        match PgPool::connect(database_url).await {
            Ok(pool) => break pool,
            Err(e) if retries < 3 => {
                retries += 1;
                println!(
                    "Failed to connect to database (attempt {}): {}. Retrying...",
                    retries, e
                );
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
            Err(e) => return Err(e),
        }
    };

    println!("pool connected");

    // Run migrations (skip if API_SKIP_MIGRATIONS=true)
    let skip_migrations = std::env::var("API_SKIP_MIGRATIONS")
        .unwrap_or_default()
        .to_lowercase();

    if skip_migrations == "true" {
        println!("Skipping migrations (API_SKIP_MIGRATIONS=true)");
    } else {
        println!("Running migrations...");
        sqlx::migrate!("./migrations").run(&pool).await?;
        println!("Migrations completed");
    }

    Ok(pool)
}
