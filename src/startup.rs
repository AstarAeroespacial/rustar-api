use crate::{
    config::Config,
    database::create_pool,
    messaging::{broker::MqttBroker, receiver::MqttReceiver},
    repository::{
        ground_station::GroundStationRepository, job::JobRepository,
        job_status_update::JobStatusUpdateRepository, satellite::SatelliteRepository,
        telemetry::TelemetryRepository,
    },
    services::{
        ground_station_service::GroundStationService, job_service::JobService,
        message_service::MessageService, satellite_service::SatelliteService,
        telemetry_service::TelemetryService,
    },
    state::AppState,
};
use std::sync::Arc;

/// Initialize database pool
pub async fn init_database(database_url: &str) -> sqlx::PgPool {
    log::info!("Creating database pool...");
    log::info!("Database URL: {}", database_url);

    create_pool(database_url)
        .await
        .expect("Failed to create database pool")
}

/// Initialize all services and application state
pub async fn init_app_state(config: Arc<Config>) -> AppState {
    let pool = init_database(&config.database.url).await;

    // Initialize repositories
    let telemetry_repository = TelemetryRepository::new(pool.clone());
    let ground_station_repository = GroundStationRepository::new(pool.clone());
    let job_repository = JobRepository::new(pool.clone());
    let job_status_repository = JobStatusUpdateRepository::new(pool.clone());
    let satellite_repository = SatelliteRepository::new(pool.clone());

    // Initialize services
    let telemetry_service = Arc::new(TelemetryService::new(telemetry_repository));
    let ground_station_service = Arc::new(GroundStationService::new(ground_station_repository));
    let satellite_service = Arc::new(SatelliteService::new(satellite_repository));

    // Setup MQTT broker & messaging service
    let keepalive = std::time::Duration::from_secs(config.broker.keep_alive as u64);
    let (broker, _) = MqttBroker::new(&config.broker.host, config.broker.port, keepalive);
    let messaging_service = Arc::new(MessageService::new(broker));

    // Initialize job service with all dependencies
    let job_service = Arc::new(JobService::new(
        job_repository,
        job_status_repository,
        satellite_service.clone(),
        messaging_service,
    ));

    AppState::new(
        config,
        telemetry_service,
        ground_station_service,
        job_service,
        satellite_service,
    )
}

/// Initialize MQTT receiver
pub fn init_mqtt_receiver(
    config: &Config,
    telemetry_service: Arc<TelemetryService>,
    job_service: Arc<JobService>,
) -> MqttReceiver {
    let keepalive = std::time::Duration::from_secs(config.broker.keep_alive as u64);
    let (broker, eventloop) = MqttBroker::new(&config.broker.host, config.broker.port, keepalive);
    let client = broker.client();

    MqttReceiver::from_client(client, eventloop, telemetry_service, job_service)
}
