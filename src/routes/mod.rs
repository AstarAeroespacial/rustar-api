pub mod config;
pub mod control;
pub mod ground_stations;
pub mod jobs;
pub mod satellites;
pub mod telemetry;

use actix_web::web;

use self::{
    config::get_config,
    control::send_command,
    ground_stations::{
        create_ground_station, delete_ground_station, fetch_all_ground_stations,
        fetch_ground_station,
    },
    jobs::{create_job, fetch_all_jobs, fetch_job, fetch_job_status},
    satellites::{
        create_satellite, delete_satellite, fetch_all_satellites, fetch_satellite,
        update_satellite_tle,
    },
    telemetry::fetch_satellite_telemetry,
};

/// Configure all application routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
        // Telemetry
        .service(fetch_satellite_telemetry)
        // Config & Control
        .service(get_config)
        .service(send_command)
        // Ground Stations
        .service(create_ground_station)
        .service(fetch_all_ground_stations)
        .service(fetch_ground_station)
        .service(delete_ground_station)
        // Jobs
        .service(create_job)
        .service(fetch_all_jobs)
        .service(fetch_job)
        .service(fetch_job_status)
        // Satellites
        .service(fetch_all_satellites)
        .service(fetch_satellite)
        .service(create_satellite)
        .service(update_satellite_tle)
        .service(delete_satellite);
}
