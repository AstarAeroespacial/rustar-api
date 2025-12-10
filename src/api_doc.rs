use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::config::{BrokerConfig, DatabaseConfig, ServerConfig};
use crate::models::{
    commands::TestMessage,
    entities::{GroundStation, Job, JobStatusUpdate, Satellite},
    requests::{
        GroundStationCreateRequest, JobCreateRequest, SatelliteCreateRequest, TelemetryQueryParams,
        TleUpdateRequest,
    },
    responses::*,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        // Ground Stations
        crate::routes::ground_stations::create_ground_station,
        crate::routes::ground_stations::fetch_all_ground_stations,
        crate::routes::ground_stations::fetch_ground_station,
        crate::routes::ground_stations::delete_ground_station,
        // Telemetry
        crate::routes::telemetry::fetch_satellite_telemetry,
        // Config & Control
        crate::routes::config::get_config,
        crate::routes::control::send_command,
        // Jobs
        crate::routes::jobs::create_job,
        crate::routes::jobs::fetch_all_jobs,
        crate::routes::jobs::fetch_job,
        crate::routes::jobs::fetch_job_status,
        // Satellites
        crate::routes::satellites::fetch_all_satellites,
        crate::routes::satellites::fetch_satellite,
        crate::routes::satellites::create_satellite,
        crate::routes::satellites::update_satellite_tle,
        crate::routes::satellites::delete_satellite,
    ),
    components(schemas(
        TelemetryResponse,
        ConfigResponse,
        TelemetryQueryParams,
        ServerConfig,
        DatabaseConfig,
        BrokerConfig,
        TestMessage,
        GroundStation,
        GroundStationCreateRequest,
        Satellite,
        SatelliteCreateRequest,
        TleUpdateRequest,
        Job,
        JobCreateRequest,
        JobStatusUpdate
    )),
    tags(
        (name = "Telemetry", description = "Telemetry endpoints"),
        (name = "Config", description = "Configuration endpoints"),
        (name = "Ground Stations", description = "Ground station management"),
        (name = "Jobs", description = "Job management"),
        (name = "Satellites", description = "Satellite management endpoints")
    ),
    info(
        title = "Rustar API",
        version = "1.0.0",
        description = "All-in-one solution for ground station and satellite management"
    )
)]
pub struct ApiDoc;

/// Configure Swagger UI
pub fn configure_swagger(cfg: &mut web::ServiceConfig) {
    cfg.service(
        SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
    );
}
