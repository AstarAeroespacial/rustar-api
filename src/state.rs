use crate::{
    config::Config,
    services::{
        ground_station_service::GroundStationService, job_service::JobService,
        pass_service::PassService, satellite_service::SatelliteService,
        telemetry_service::TelemetryService,
    },
};
use std::sync::Arc;

/// Application state shared across all handlers
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub telemetry_service: Arc<TelemetryService>,
    pub ground_station_service: Arc<GroundStationService>,
    pub job_service: Arc<JobService>,
    pub satellite_service: Arc<SatelliteService>,
    pub pass_service: Arc<PassService>,
}

impl AppState {
    pub fn new(
        config: Arc<Config>,
        telemetry_service: Arc<TelemetryService>,
        ground_station_service: Arc<GroundStationService>,
        job_service: Arc<JobService>,
        satellite_service: Arc<SatelliteService>,
        pass_service: Arc<PassService>,
    ) -> Self {
        Self {
            config,
            telemetry_service,
            ground_station_service,
            job_service,
            satellite_service,
            pass_service,
        }
    }
}
