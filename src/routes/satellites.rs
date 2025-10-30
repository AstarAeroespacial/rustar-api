use crate::models::entities::Satellite;
use crate::models::requests::{SatelliteCreateRequest, TleUpdateRequest};
use crate::services::{errors::ServiceError, satellite_service::SatelliteService};
use actix_web::{delete, get, post, put, web, HttpResponse};
use std::sync::Arc;
use validator::Validate;

#[utoipa::path(
    get,
    path = "/api/satellites",
    responses(
        (status = 200, description = "List all satellites", body = [Satellite]),
        (status = 500, description = "Internal Server Error", body = String)
    ),
    tag = "Satellites"
)]
#[get("/api/satellites")]
pub async fn fetch_all_satellites(
    service: web::Data<Arc<SatelliteService>>,
) -> Result<HttpResponse, ServiceError> {
    let satellites = service.get_all_satellites().await?;
    Ok(HttpResponse::Ok().json(satellites))
}

#[utoipa::path(
    get,
    path = "/api/satellites/{id}",
    params(
        ("id" = i64, Path, description = "ID of the satellite to fetch")
    ),
    responses(
        (status = 200, description = "Satellite fetched successfully", body = Satellite),
        (status = 404, description = "Satellite not found", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    ),
    tag = "Satellites"
)]
#[get("/api/satellites/{id}")]
pub async fn fetch_satellite(
    id: web::Path<i64>,
    service: web::Data<Arc<SatelliteService>>,
) -> Result<HttpResponse, ServiceError> {
    let id = id.into_inner();
    let sat = service
        .get_satellite(&id)
        .await?
        .ok_or_else(|| ServiceError::NotFound(format!("Satellite {id} not found")))?;

    Ok(HttpResponse::Ok().json(sat))
}

#[utoipa::path(
    put,
    path = "/api/satellites/{id}/tle",
    params(
        ("id" = i64, Path, description = "ID of the satellite whose TLE is being updated")
    ),
    request_body(
        content = TleUpdateRequest,
        example = json!({
            "tle": "1 33591U 09005A   24305.51234567  .00000020  00000-0  12000-4 0  9993\n2 33591  99.1234 123.4567 0012345 123.4567 234.5678 14.12345678901234"
        })
    ),
    responses(
        (status = 200, description = "TLE updated successfully", body = Satellite),
        (status = 400, description = "Bad Request", body = String),
        (status = 404, description = "Satellite not found", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    ),
    tag = "Satellites"
)]
#[put("/api/satellites/{id}/tle")]
pub async fn update_satellite_tle(
    id: web::Path<i64>,
    req_body: web::Json<TleUpdateRequest>,
    service: web::Data<Arc<SatelliteService>>,
) -> Result<HttpResponse, ServiceError> {
    let id = id.into_inner();
    let req = req_body.into_inner();

    req.validate()
        .map_err(|e| ServiceError::BadRequest(e.to_string()))?;

    let sat = service
        .update_satellite_tle(&id, req.tle)
        .await?
        .ok_or_else(|| ServiceError::NotFound(format!("Satellite {id} not found")))?;

    Ok(HttpResponse::Ok().json(sat))
}

#[utoipa::path(
    post,
    path = "/api/satellites",
    request_body(
        content = SatelliteCreateRequest,
        example = json!({
            "name": "NOAA 19",
            "tle": "1 33591U 09005A   24304.41234567  .00000023  00000-0  12345-4 0  9992\n2 33591  99.1234 123.4567 0012345 123.4567 234.5678 14.12345678901234",
            "downlinkFrequency": 137.1,
            "uplinkFrequency": 145.8
        })
    ),
    responses(
        (status = 201, description = "Satellite created successfully", body = Satellite),
        (status = 400, description = "Bad Request", body = String),
        (status = 409, description = "Conflict", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    ),
    tag = "Satellites"
)]
#[post("/api/satellites")]
pub async fn create_satellite(
    req_body: web::Json<SatelliteCreateRequest>,
    service: web::Data<Arc<SatelliteService>>,
) -> Result<HttpResponse, ServiceError> {
    let req = req_body.into_inner();

    req.validate()
        .map_err(|e| ServiceError::BadRequest(e.to_string()))?;

    let sat = Satellite::from_request(req);
    let created = service.create_satellite(&sat).await?;

    Ok(HttpResponse::Created().json(created))
}

#[utoipa::path(
    delete,
    path = "/api/satellites/{id}",
    params(
        ("id" = i64, Path, description = "ID of the satellite to delete")
    ),
    responses(
        (status = 204, description = "Satellite deleted successfully, no content returned"),
        (status = 404, description = "Satellite not found", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    ),
    tag = "Satellites"
)]
#[delete("/api/satellites/{id}")]
pub async fn delete_satellite(
    id: web::Path<i64>,
    service: web::Data<Arc<SatelliteService>>,
) -> Result<HttpResponse, ServiceError> {
    let id = id.into_inner();

    service.delete_satellite(&id).await?;

    Ok(HttpResponse::NoContent().finish())
}
