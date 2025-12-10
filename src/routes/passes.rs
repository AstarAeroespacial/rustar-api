use crate::models::responses::SatellitePassesResponse;
use crate::services::{errors::ServiceError, pass_service::PassService};
use actix_web::{get, web, HttpResponse};
use std::sync::Arc;

/* #[utoipa::path(
    get,
    path = "/api/satellites/{id}/passes",
    params(
        ("id" = String, Path, description = "ID of the satellite")
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
    service: web::Data<Arc<PassService>>,
) -> Result<HttpResponse, ServiceError> {
    let sat_id = sat_id.into_inner();

    let passes = service.get_satellite_passes(&sat_id).await?;

    Ok(HttpResponse::Ok().json(SatellitePassesResponse { passes }))
} */
