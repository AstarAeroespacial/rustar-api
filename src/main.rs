mod api_doc;
mod config;
mod database;
mod messaging;
mod models;
mod repository;
mod routes;
mod server;
mod services;
mod shutdown;
mod startup;
mod state;

use config::Config;
use std::sync::Arc;
use tokio::sync::oneshot;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Load configuration
    let config = Config::load().expect("Failed to load configuration");
    let config = Arc::new(config);
    let server_address = config.server_address();

    // Initialize application state and services
    let app_state = startup::init_app_state(config.clone()).await;

    // Initialize MQTT receiver
    let mqtt_receiver = startup::init_mqtt_receiver(
        &config,
        app_state.telemetry_service.clone(),
        app_state.job_service.clone(),
    );

    // Print startup information
    server::print_startup_info(&server_address);

    // Create and start HTTP server
    let http_server = server::create_server(app_state, &server_address)?;

    // Setup shutdown coordination
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
    let mqtt_task = shutdown::spawn_mqtt_receiver(mqtt_receiver, shutdown_rx);

    // Handle graceful shutdown
    shutdown::handle_shutdown(http_server, shutdown_tx, mqtt_task).await;

    Ok(())
}
