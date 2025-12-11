use crate::models::requests::PassesQueryParams;
use crate::models::responses::{PassInfo, SatellitePassesResponse};
use crate::services::{errors::ServiceError, pass_service::PassService};
use actix_web::{get, web, HttpResponse};
use serde::Serialize;
use std::sync::Arc;
use utoipa::ToSchema;

#[utoipa::path(
    get,
    path = "/api/satellites/{id}/passes",
    params(
        ("id" = String, Path, description = "ID of the satellite"),
        PassesQueryParams
    ),
    responses(
        (status = 200, description = "List of upcoming passes for all ground stations", body = SatellitePassesResponse),
        (status = 404, description = "Satellite not found", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    ),
    tag = "Passes"
)]
#[get("/api/satellites/{id}/passes")]
pub async fn get_satellite_passes(
    sat_id: web::Path<String>,
    query: web::Query<PassesQueryParams>,
    service: web::Data<Arc<PassService>>,
) -> Result<HttpResponse, ServiceError> {
    let sat_id = sat_id.into_inner();

    let passes = service
        .get_satellite_passes(&sat_id, query.start, query.end)
        .await?;

    Ok(HttpResponse::Ok().json(SatellitePassesResponse { passes }))
}

#[derive(ToSchema, Debug, Serialize)]
pub struct GroundStationPassesResponse {
    pub passes: Vec<PassInfo>,
}

#[utoipa::path(
    get,
    path = "/api/ground-stations/{id}/passes",
    params(
        ("id" = String, Path, description = "ID of the ground station"),
        PassesQueryParams
    ),
    responses(
        (status = 200, description = "List of upcoming satellites visible from this ground station", body = GroundStationPassesResponse),
        (status = 404, description = "Ground station not found", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    ),
    tag = "Passes"
)]
#[get("/api/ground-stations/{id}/passes")]
pub async fn get_ground_station_passes(
    gs_id: web::Path<String>,
    query: web::Query<PassesQueryParams>,
    service: web::Data<Arc<PassService>>,
) -> Result<HttpResponse, ServiceError> {
    let gs_id = gs_id.into_inner();

    let passes = service
        .get_ground_station_passes(&gs_id, query.start, query.end)
        .await?;

    Ok(HttpResponse::Ok().json(GroundStationPassesResponse { passes }))
}
