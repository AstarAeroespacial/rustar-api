use crate::models::entities::JobStatus;
use crate::models::requests::JobCreateRequest;
use crate::services::{errors::ServiceError, job_service::JobService};
use actix_web::{get, post, web, HttpResponse};
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
        (status = 404, description = "Ground Station or Satellite not found", body = String),
        (status = 409, description = "Conflict", body = String),
        (status = 422, description = "Unprocessable Entity", body = String),
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

#[utoipa::path(
    get,
    path = "/api/jobs",
    responses(
        (status = 200, description = "List all jobs", body = [Job]),
        (status = 500, description = "Internal Server Error", body = String)
    ),
    tag = "Jobs"
)]
#[get("/api/jobs")]
pub async fn fetch_all_jobs(
    service: web::Data<Arc<JobService>>,
) -> Result<HttpResponse, ServiceError> {
    let jobs = service.get_all_jobs().await?;
    Ok(HttpResponse::Ok().json(jobs))
}

#[utoipa::path(
    get,
    path = "/api/jobs/{id}",
    params(
        ("id" = i64, Path, description = "ID of the job to fetch")
    ),
    responses(
        (status = 200, description = "Job fetched successfully", body = Jobs),
        (status = 404, description = "Job not found", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    ),
    tag = "Jobs"
)]
#[get("/api/jobs/{id}")]
pub async fn fetch_job(
    id: web::Path<i64>,
    service: web::Data<Arc<JobService>>,
) -> Result<HttpResponse, ServiceError> {
    let job_id = id.into_inner();
    let job = service
        .get_job(job_id)
        .await?
        .ok_or_else(|| ServiceError::NotFound(format!("Job {job_id} not found")))?;

    Ok(HttpResponse::Ok().json(job))
}
