use crate::models::entities::Job;
use crate::repository::job::JobRepository;

pub struct JobService {
    repository: JobRepository,
}

impl JobService {
    pub fn new(repository: JobRepository) -> Self {
        Self { repository }
    }

    pub async fn create_job(&self, gs_id: &String, sat_id: &String, commands: &Vec<String>) -> Result<Job, Box<dyn std::error::Error + Send + Sync>> {
        // TODO: calculate start - end
        let job = Job::new(gs_id, sat_id, commands);
        self.repository.create_job(&job).await?;
        Ok(job)
    }
}