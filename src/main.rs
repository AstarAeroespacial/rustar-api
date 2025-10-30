use actix_web::{middleware::Logger, web, App, HttpServer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod config;
mod database;
mod messaging;
mod models;
mod repository;
mod routes;
mod services;

use config::{Config, DatabaseConfig, MessageBrokerConfig, ServerConfig};
use database::create_pool;
use messaging::{broker::MqttBroker, receiver::MqttReceiver};
use models::{
    commands::TestMessage,
    requests::{
        GroundStationCreateRequest, HistoricTelemetryRequest, LatestTelemetryRequest,
        SatelliteCreateRequest, TleUpdateRequest,
    },
    responses::*,
};
use repository::{
    ground_station::GroundStationRepository, job::JobRepository, satellite::SatelliteRepository,
    telemetry::TelemetryRepository,
};
use routes::{
    config::get_config,
    control::send_command,
    ground_stations::{
        create_ground_station, fetch_all_ground_stations, fetch_ground_station,
        set_tle_for_ground_station,
    },
    jobs::create_job,
    satellites::{
        create_satellite, delete_satellite, fetch_all_satellites, fetch_satellite,
        update_satellite_tle,
    },
    telemetry::{get_historic_telemetry, get_latest_telemetry},
};
use services::{
    ground_station_service::GroundStationService, job_service::JobService,
    message_service::MessageService, satellite_service::SatelliteService,
    telemetry_service::TelemetryService,
};
use std::sync::Arc;
use tokio::signal;
use tokio::sync::oneshot;

#[derive(OpenApi)]
#[openapi(
    paths(
        // Ground Stations
        routes::ground_stations::create_ground_station,
        routes::ground_stations::fetch_all_ground_stations,
        routes::ground_stations::fetch_ground_station,
        routes::ground_stations::set_tle_for_ground_station,
        // Telemetry
        routes::telemetry::get_latest_telemetry,
        routes::telemetry::get_historic_telemetry,
        // Config & Control
        routes::config::get_config,
        routes::control::send_command,
        // Jobs
        routes::jobs::create_job,
        // Satellites
        routes::satellites::fetch_all_satellites,
        routes::satellites::fetch_satellite,
        routes::satellites::create_satellite,
        routes::satellites::update_satellite_tle,
        routes::satellites::delete_satellite,
    ),
    components(schemas(
        TelemetryResponse,
        ConfigResponse,
        HistoricTelemetryRequest,
        LatestTelemetryRequest,
        ServerConfig,
        DatabaseConfig,
        MessageBrokerConfig,
        TestMessage,
        GroundStationCreateRequest,
        SatelliteCreateRequest,
        TleUpdateRequest
    )),
    tags(
        (name = "Telemetry", description = "Telemetry endpoints"),
        (name = "Config", description = "Configuration endpoints"),
        (name = "Ground Stations", description = "Ground station management"),
        (name = "Jobs", description = "Job management"),
        (name = "Satellites", description = "Satellite management endpoints")
    ),
    info(
        title = "Rust API with Utoipa",
        version = "1.0.0",
        description = "A Rust API with OpenAPI documentation using Utoipa"
    )
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // std::env::set_var("RUST_LOG", "debug");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Load configuration
    let config = Config::load().expect("Failed to load configuration");
    let shared_config = Arc::new(config);
    let server_address = shared_config.server_address();

    // Create database pool
    println!("Creating database pool...");
    println!("Database URL: {}", &shared_config.database.url);

    let pool = create_pool(&shared_config.database.url)
        .await
        .expect("Failed to create database pool");

    // Initialize repositories & services
    let telemetry_repository = TelemetryRepository::new(pool.clone());
    let telemetry_service = Arc::new(TelemetryService::new(telemetry_repository));

    let ground_station_repository = GroundStationRepository::new(pool.clone());
    let ground_station_service = Arc::new(GroundStationService::new(ground_station_repository));

    let job_repository = JobRepository::new(pool.clone());
    let job_service = Arc::new(JobService::new(job_repository));

    let satellite_repository = SatelliteRepository::new(pool.clone());
    let satellite_service = Arc::new(SatelliteService::new(satellite_repository));

    // Setup MQTT broker & receiver
    let keepalive = std::time::Duration::from_secs(shared_config.message_broker.keep_alive as u64);
    let (broker, eventloop) = MqttBroker::new(
        &shared_config.message_broker.host,
        shared_config.message_broker.port,
        keepalive,
    );
    let client = broker.client();
    let messaging_service = Arc::new(MessageService::new(broker));

    // Start MQTT event loop in background
    let mut recv = MqttReceiver::from_client(client, eventloop, telemetry_service.clone());

    println!("============= API SERVER STARTING =============");
    println!("Available endpoints:");
    println!("  - GET    /api/telemetry/latest");
    println!("  - GET    /api/telemetry/history");
    println!("  - GET    /api/config");
    println!("  - POST   /api/control");
    println!("  - POST   /api/jobs");
    println!("  - GET    /api/ground-stations");
    println!("  - GET    /api/ground-stations/{{id}}");
    println!("  - POST   /api/ground-stations");
    println!("  - PUT    /api/ground-stations/{{id}}/tle");
    println!("  - GET    /api/satellites");
    println!("  - GET    /api/satellites/{{id}}");
    println!("  - POST   /api/satellites");
    println!("  - PUT    /api/satellites/{{id}}/tle");
    println!("  - DELETE /api/satellites/{{id}}");
    println!("  - GET    /swagger-ui/");
    println!("Server running at: {}", server_address);
    println!("==============================================");

    let server = HttpServer::new(move || {
        App::new()
            // Shared data
            .app_data(web::Data::new(shared_config.clone()))
            .app_data(web::Data::new(telemetry_service.clone()))
            .app_data(web::Data::new(messaging_service.clone()))
            .app_data(web::Data::new(ground_station_service.clone()))
            .app_data(web::Data::new(job_service.clone()))
            .app_data(web::Data::new(satellite_service.clone()))
            // Telemetry
            .service(get_latest_telemetry)
            .service(get_historic_telemetry)
            // Config & Control
            .service(get_config)
            .service(send_command)
            // Ground Stations
            .service(create_ground_station)
            .service(fetch_all_ground_stations)
            .service(fetch_ground_station)
            .service(set_tle_for_ground_station)
            // Jobs
            .service(create_job)
            // Satellites
            .service(fetch_all_satellites)
            .service(fetch_satellite)
            .service(create_satellite)
            .service(update_satellite_tle)
            .service(delete_satellite)
            // Middleware & Docs
            .wrap(Logger::new("%r - %U | %s (%T)"))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind(server_address)?;

    // Create shutdown channel for MQTT receiver
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

    // Start HTTP server and obtain handle
    let server_handle = server.run();
    let handle = server_handle.handle();

    // Spawn MQTT receiver task
    let recv_task = tokio::task::spawn_blocking(move || {
        let rt =
            tokio::runtime::Runtime::new().expect("Failed to create runtime for MQTT receiver");
        rt.block_on(recv.run(shutdown_rx));
    });

    // Graceful shutdown on Ctrl+C
    tokio::select! {
        _ = signal::ctrl_c() => {
            println!("SIGINT received: shutting down server and MQTT receiver...");
            let _ = shutdown_tx.send(());
            handle.stop(true).await;
        }
        res = server_handle => {
            if let Err(e) = res {
                eprintln!("HTTP server error: {:?}", e);
            }
            let _ = shutdown_tx.send(());
        }
    }

    // Wait for MQTT receiver to finish
    let _ = recv_task.await;

    Ok(())
}
