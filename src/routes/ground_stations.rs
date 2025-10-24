use crate::models::{entities::GroundStation, requests::GroundStationCreateRequest};
use crate::services::ground_station_service::GroundStationService;
use actix_web::{get, post, web, Responder, Result};
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
    service: web::Data<Arc<GroundStationService>>
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
