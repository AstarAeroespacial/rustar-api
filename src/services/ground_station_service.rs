use crate::models::entities::GroundStation;
use crate::repository::ground_station::GroundStationRepository;

pub struct GroundStationService {
    repository: GroundStationRepository,
}

impl GroundStationService {
    pub fn new(repository: GroundStationRepository) -> Self {
        Self { repository }
    }

    pub async fn create_ground_station(
        &self,
        ground_station: &GroundStation,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.repository.create_ground_station(ground_station).await
    }
}
