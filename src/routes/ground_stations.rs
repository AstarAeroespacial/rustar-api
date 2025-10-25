use crate::models::{entities::GroundStation, requests::GroundStationCreateRequest};
use crate::services::ground_station_service::GroundStationService;
use actix_web::{get, post, put, web, Responder, Result};
use log::error;
use std::sync::Arc;

#[utoipa::path(
    post,
    path = "/api/ground-stations",
    request_body = GroundStationCreateRequest,
    responses(
        (status = 201, description = "Created", body = GroundStation),
        (status = 400, description = "Bad Request", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    ),
    tag = "Ground Stations"
)]
#[post("/api/ground-stations")]
pub async fn create_ground_station(
    req_body: web::Json<GroundStationCreateRequest>,
    service: web::Data<Arc<GroundStationService>>,
) -> Result<impl Responder> {
    println!("req_body: {:?}", req_body);
    let gs = GroundStation::from_request(req_body.into_inner());
    match service.create_ground_station(&gs).await {
        Ok(_) => Ok(actix_web::web::Json(gs)),
        Err(e) => {
            error!("Error creating ground station: {}", e);
            Err(actix_web::error::ErrorInternalServerError(
                "Failed to create ground station",
            ))
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/ground-stations",
    responses(
        (status = 200, description = "Success", body = Vec<GroundStation>),
        (status = 500, description = "Internal Server Error", body = String)
    ),
    tag = "Ground Stations"
)]
#[get("/api/ground-stations")]
pub async fn fetch_all_ground_stations(
    service: web::Data<Arc<GroundStationService>>,
) -> Result<impl Responder> {
    println!("Fetch ground stations");
    match service.get_all_ground_stations().await {
        Ok(gss) => Ok(actix_web::web::Json(gss)),
        Err(e) => {
            error!("Error fetching historic telemetry: {}", e);
            Err(actix_web::error::ErrorInternalServerError(
                "Failed to fetch telemetry data",
            ))
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/ground-stations/{id}",
    params(
        ("id" = String, Path, description = "ID of ground station to fetch"),
    ),
    responses(
        (status = 200, description = "Success", body = GroundStation),
        (status = 404, description = "Not Found", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    ),
    tag = "Ground Stations"
)]
#[get("/api/ground-stations/{id}")]
pub async fn fetch_ground_station(
    id: web::Path<String>,
    service: web::Data<Arc<GroundStationService>>,
) -> Result<impl Responder> {
    let id = id.into_inner();
    println!("Fetch ground station {}", id);
    match service.get_ground_station(&id).await {
        Ok(gs) => match gs {
            Some(gs) => Ok(actix_web::web::Json(gs)),
            None => Err(actix_web::error::ErrorNotFound(format!(
                "Ground station {} not found",
                id
            ))),
        },
        Err(e) => {
            error!("Error fetching historic telemetry: {}", e);
            Err(actix_web::error::ErrorInternalServerError(
                "Failed to fetch telemetry data",
            ))
        }
    }
}

#[utoipa::path(
    put,
    path = "/api/ground-stations/{id}/satellite",
    params(
        ("id" = String, Path, description = "ID of ground station to set satellite for"),
    ),
    request_body = String,
    responses(
        (status = 200, description = "Success", body = String),
        (status = 400, description = "Bad Request", body = String),
        (status = 404, description = "Not Found", body = String),
        (status = 500, description = "Internal Server Error", body = String),
    ),
    tag = "Ground Stations"
)]
#[put("/api/ground-stations/{id}/satellite")]
pub async fn set_tle_for_ground_station(
    id: web::Path<String>,
    req_body: String,
    service: web::Data<Arc<GroundStationService>>,
) -> Result<impl Responder> {
    let id = id.into_inner();
    let tle = req_body;
    println!("Set TLE for ground station {} to {}", id, tle);
    match service.set_tle_for_ground_station(&id, &tle).await {
        Ok(r) => match r {
            Some(_) => Ok(String::from("TLE set successfully")),
            None => Err(actix_web::error::ErrorNotFound(format!(
                "Ground station {} not found",
                id
            ))),
        },
        Err(e) => {
            error!("Error setting TLE for ground station {}: {}", id, e);
            Err(actix_web::error::ErrorInternalServerError(
                "Failed to set TLE for ground station",
            ))
        }
    }
}
