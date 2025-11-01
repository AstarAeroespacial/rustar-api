use crate::models::entities::GroundStation;
use crate::models::requests::GroundStationCreateRequest;
use crate::services::{errors::ServiceError, ground_station_service::GroundStationService};
use actix_web::{delete, get, post, web, HttpResponse};
use std::sync::Arc;
use validator::Validate;

#[utoipa::path(
    get,
    path = "/api/ground-stations",
    responses(
        (status = 200, description = "List all ground stations", body = [GroundStation]),
        (status = 500, description = "Internal Server Error", body = String)
    ),
    tag = "Ground Stations"
)]
#[get("/api/ground-stations")]
pub async fn fetch_all_ground_stations(
    service: web::Data<Arc<GroundStationService>>,
) -> Result<HttpResponse, ServiceError> {
    let ground_stations = service.get_all_ground_stations().await?;
    Ok(HttpResponse::Ok().json(ground_stations))
}

#[utoipa::path(
    get,
    path = "/api/ground-stations/{id}",
    params(
        ("id" = i64, Path, description = "ID of the ground station to fetch")
    ),
    responses(
        (status = 200, description = "Ground station fetched successfully", body = GroundStation),
        (status = 404, description = "Ground station not found", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    ),
    tag = "Ground Stations"
)]
#[get("/api/ground-stations/{id}")]
pub async fn fetch_ground_station(
    id: web::Path<i64>,
    service: web::Data<Arc<GroundStationService>>,
) -> Result<HttpResponse, ServiceError> {
    let id = id.into_inner();
    let gs = service
        .get_ground_station(&id)
        .await?
        .ok_or_else(|| ServiceError::NotFound(format!("Ground station {id} not found")))?;

    Ok(HttpResponse::Ok().json(gs))
}

#[utoipa::path(
    post,
    path = "/api/ground-stations",
    request_body(
        content = GroundStationCreateRequest,
        example = json!({
            "name": "Ground Station Buenos Aires",
            "location": {
                "latitude": -34.6037,
                "longitude": -58.3816
            },
            "description": "Main UBA station for NOAA reception"
        })
    ),
    responses(
        (status = 201, description = "Ground station created successfully", body = GroundStation),
        (status = 400, description = "Bad Request", body = String),
        (status = 409, description = "Conflict", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    ),
    tag = "Ground Stations"
)]
#[post("/api/ground-stations")]
pub async fn create_ground_station(
    req_body: web::Json<GroundStationCreateRequest>,
    service: web::Data<Arc<GroundStationService>>,
) -> Result<HttpResponse, ServiceError> {
    let req = req_body.into_inner();

    req.validate()
        .map_err(|e| ServiceError::BadRequest(e.to_string()))?;

    let gs = GroundStation::from_request(req);
    let created = service.create_ground_station(&gs).await?;

    Ok(HttpResponse::Created().json(created))
}

#[utoipa::path(
    delete,
    path = "/api/ground-stations/{id}",
    params(
        ("id" = i64, Path, description = "ID of the ground station to delete")
    ),
    responses(
        (status = 204, description = "Ground station deleted successfully, no content returned"),
        (status = 404, description = "Ground station not found", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    ),
    tag = "Ground Stations"
)]
#[delete("/api/ground-stations/{id}")]
pub async fn delete_ground_station(
    id: web::Path<i64>,
    service: web::Data<Arc<GroundStationService>>,
) -> Result<HttpResponse, ServiceError> {
    let id = id.into_inner();
    let deleted = service.delete_ground_station(&id).await?;

    if deleted {
        Ok(HttpResponse::NoContent().finish()) // 204 No Content
    } else {
        Err(ServiceError::NotFound(format!(
            "Ground station {id} not found"
        )))
    }
}
