use crate::models::{entities::GroundStation, requests::GroundStationCreateRequest};
use crate::services::ground_station_service::GroundStationService;
use actix_web::{post, web, Responder, Result};
use log::error;
use std::sync::Arc;

#[utoipa::path(
    post,
    path = "/api/ground-stations",
    request_body = GroundStationCreateRequest,
    responses(
        (status = 200, description = "Success", body = Vec<TelemetryResponse>),
        (status = 400, description = "Bad Request", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    ),
    tag = "API"
)]
#[post("/api/control/command")]
pub async fn create_ground_station(
    req_body: web::Json<GroundStationCreateRequest>,
    service: web::Data<Arc<GroundStationService>>,
) -> Result<impl Responder> {
    println!("req_body: {:?}", req_body);
    let gs = GroundStation::from_request(req_body.into_inner());
    match service.create_ground_station(gs).await {
        Ok(_) => Ok(actix_web::HttpResponse::Ok().body("Ground station created successfully")),
        Err(e) => {
            error!("Error creating ground station: {}", e);
            Err(actix_web::error::ErrorInternalServerError(
                "Failed to create ground station",
            ))
        }
    }
}
