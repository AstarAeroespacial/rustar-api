use crate::models::requests::TelemetryQueryParams;
use crate::services::{errors::ServiceError, telemetry_service::TelemetryService};
use actix_web::{get, web, HttpResponse};
use std::sync::Arc;
use validator::Validate;

#[utoipa::path(
    get,
    path = "/api/satellites/{id}/telemetry",
    params(
        ("id" = String, Path, description = "ID of the satellite to fetch telemetry for"),
        TelemetryQueryParams
    ),
    responses(
        (status = 200, description = "Telemetry data fetched successfully. Returns paginated results if limit and page are provided, otherwise returns all telemetry records.", body = [TelemetryResponse]),
        (status = 400, description = "Invalid query parameters", body = String),
        (status = 404, description = "Satellite not found or no telemetry available", body = String),
        (status = 422, description = "Invalid telemetry data format", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    ),
    tag = "Telemetry"
)]
#[get("/api/satellites/{id}/telemetry")]
pub async fn fetch_satellite_telemetry(
    id: web::Path<String>,
    query: web::Query<TelemetryQueryParams>,
    service: web::Data<Arc<TelemetryService>>,
) -> Result<HttpResponse, ServiceError> {
    let satellite_id = id.into_inner();

    // Validate query params if provided
    query
        .validate()
        .map_err(|e| ServiceError::BadRequest(e.to_string()))?;

    let telemetry = service
        .get_telemetry_by_satellite_id(&satellite_id, query.limit, query.page)
        .await?;

    Ok(HttpResponse::Ok().json(telemetry))
}
