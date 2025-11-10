use crate::models::entities::JobStatus;
use crate::models::requests::JobCreateRequest;
use crate::services::{errors::ServiceError, job_service::JobService};
use actix_web::{post, web, HttpResponse};
use chrono::Utc;
use std::sync::Arc;
use validator::Validate;

#[utoipa::path(
    post,
    path = "/api/jobs",
    request_body = JobCreateRequest,
    responses(
        (status = 201, description = "Job created successfully", body = Job),
        (status = 400, description = "Bad Request", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    ),
    tag = "Jobs"
)]
#[post("/api/jobs")]
pub async fn create_job(
    req_body: web::Json<JobCreateRequest>,
    service: web::Data<Arc<JobService>>,
) -> Result<HttpResponse, ServiceError> {
    let req = req_body.into_inner();

    req.validate()
        .map_err(|e| ServiceError::BadRequest(e.to_string()))?;

    let commands = if let Some(commands) = req.commands {
        commands
    } else {
        Vec::new()
    };

    let job = service
        .create_job(
            &req.gs_id,
            &req.sat_id,
            req.start_time,
            req.end_time,
            &commands,
        )
        .await?;

    service.send_job(&job).await?;

    service
        .add_job_status(job.id, JobStatus::Sent, Utc::now())
        .await?;

    Ok(HttpResponse::Created().json(job))
}
