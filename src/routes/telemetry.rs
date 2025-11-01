use crate::models::responses::TelemetryResponse;
use crate::services::{errors::ServiceError, telemetry_service::TelemetryService};
use actix_web::{get, web, HttpResponse};
use std::sync::Arc;

#[utoipa::path(
    get,
    path = "/api/satellites/{id}/telemetry",
    params(
        ("id" = i64, Path, description = "ID of the satellite to fetch telemetry for")
    ),
    responses(
        (status = 200, description = "Telemetry data fetched successfully", body = [TelemetryResponse]),
        (status = 404, description = "Satellite not found or no telemetry available", body = String),
        (status = 422, description = "Invalid telemetry data format", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    ),
    tag = "Telemetry"
)]
#[get("/api/satellites/{id}/telemetry")]
pub async fn fetch_satellite_telemetry(
    id: web::Path<i64>,
    service: web::Data<Arc<TelemetryService>>,
) -> Result<HttpResponse, ServiceError> {
    let satellite_id = id.into_inner();

    let telemetry = service.get_telemetry_by_satellite_id(&satellite_id).await?;

    Ok(HttpResponse::Ok().json(telemetry))
}
