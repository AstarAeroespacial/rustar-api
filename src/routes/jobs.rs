use crate::models::requests::JobCreateRequest;
use crate::services::job_service::JobService;
use actix_web::{post, web, Responder, Result};
use log::error;
use std::sync::Arc;

#[utoipa::path(
    post,
    path = "/api/jobs",
    request_body = JobCreateRequest,
    responses(
        (status = 201, description = "Created", body = Job),
        (status = 400, description = "Bad Request", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    ),
    tag = "Jobs"
)]
#[post("/api/jobs")]
pub async fn create_job(
    req_body: web::Json<JobCreateRequest>,
    service: web::Data<Arc<JobService>>,
) -> Result<impl Responder> {
    println!("req_body: {:?}", req_body);
    let req = req_body.into_inner();
    match service
        .create_job(&req.gs_id, &req.sat_id, &req.commands)
        .await
    {
        Ok(job) => Ok(actix_web::web::Json(job)),
        Err(e) => {
            error!("Error creating job: {}", e);
            Err(actix_web::error::ErrorInternalServerError(
                "Failed to create job",
            ))
        }
    }
}
