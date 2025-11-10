use crate::models::entities::{Job, JobStatus, JobStatusUpdate};
use crate::repository::job::JobRepository;
use crate::repository::job_status_update::JobStatusUpdateRepository;
use crate::services::errors::ServiceError;
use crate::services::message_service::MessageService;
use crate::services::satellite_service::SatelliteService;
use chrono::{DateTime, Utc};
use rustar_types::jobs::TleData;
use std::sync::Arc;

pub struct JobService {
    repository: JobRepository,
    status_repository: JobStatusUpdateRepository,
    satellite_service: Arc<SatelliteService>,
    message_service: Arc<MessageService>,
}

impl JobService {
    pub fn new(
        repository: JobRepository,
        status_repository: JobStatusUpdateRepository,
        satellite_service: Arc<SatelliteService>,
        message_service: Arc<MessageService>,
    ) -> Self {
        Self {
            repository,
            status_repository,
            satellite_service,
            message_service,
        }
    }

    pub async fn create_job(
        &self,
        gs_id: &str,
        sat_id: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        commands: &Vec<String>,
    ) -> Result<Job, ServiceError> {
        let job = self
            .repository
            .create_job(gs_id, sat_id, start, end, commands)
            .await?;
        Ok(job)
    }

    pub async fn add_job_status(
        &self,
        job_id: i64,
        status: JobStatus,
        timestamp: DateTime<Utc>,
    ) -> Result<(), ServiceError> {
        let status_update = JobStatusUpdate {
            job_id,
            timestamp,
            status,
        };

        self.status_repository
            .create_status_update(&status_update)
            .await?;

        Ok(())
    }

    pub async fn send_job(&self, job: &Job) -> Result<(), ServiceError> {
        let satellite = self
            .satellite_service
            .get_satellite(&job.sat_id)
            .await
            .map_err(|e| ServiceError::Internal(format!("Failed to get TLE: {}", e)))?;

        let satellite = match satellite {
            Some(sat) => sat,
            None => {
                return Err(ServiceError::NotFound(format!(
                    "Satellite with ID {} not found",
                    &job.sat_id
                )))
            }
        };

        let tle = TleData::try_from(satellite.tle)
            .map_err(|e| ServiceError::BadRequest(format!("Invalid TLE: {:?}", e)))?;

        let commands = match &job.commands {
            Some(cmds) if !cmds.is_empty() => {
                let serialized = serde_json::to_vec(cmds).map_err(|e| {
                    ServiceError::Internal(format!("Failed to serialize commands: {}", e))
                })?;
                Some(serialized)
            }
            _ => None,
        };

        let mqtt_job = rustar_types::jobs::Job {
            id: job.id as u64,

            satellite_id: job.sat_id.clone(),
            start: job.start,

            end: job.end,
            tle,
            rx_frequency: satellite.downlink_frequency,
            tx_frequency: satellite.uplink_frequency,
            uplink: commands,
        };

        self.message_service
            .send_job(job.gs_id.clone(), mqtt_job)
            .await?;

        Ok(())
    }
}
