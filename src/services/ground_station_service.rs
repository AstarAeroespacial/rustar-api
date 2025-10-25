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

    pub async fn get_all_ground_stations(
        &self,
    ) -> Result<Vec<GroundStation>, Box<dyn std::error::Error + Send + Sync>> {
        self.repository.get_all_ground_stations().await
    }

    pub async fn get_ground_station(
        &self,
        id: &String,
    ) -> Result<Option<GroundStation>, Box<dyn std::error::Error + Send + Sync>> {
        self.repository.get_ground_station(id).await
    }

    pub async fn set_tle_for_ground_station(
        &self,
        id: &String,
        tle: &String,
    ) -> Result<Option<()>, Box<dyn std::error::Error + Send + Sync>> {
        self.repository.set_tle_for_ground_station(id, tle).await
    }
}
