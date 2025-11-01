use crate::models::responses::TelemetryResponse;
use crate::repository::telemetry::TelemetryRepository;
use crate::services::errors::ServiceError;
use rustar_types::telemetry::TelemetryRecord;

pub struct TelemetryService {
    repository: TelemetryRepository,
}

impl TelemetryService {
    pub fn new(repository: TelemetryRepository) -> Self {
        Self { repository }
    }

    /// Get all telemetry for a specific satellite by its ID
    pub async fn get_telemetry_by_satellite_id(
        &self,
        satellite_id: &i64,
    ) -> Result<Vec<TelemetryResponse>, ServiceError> {
        let telemetry = self
            .repository
            .get_telemetry_by_satellite(satellite_id)
            .await?;

        // Convert TelemetryDb to TelemetryResponse
        let responses: Result<Vec<TelemetryResponse>, ServiceError> = telemetry
            .into_iter()
            .map(|db_record| {
                // Decode the payload bytes to TelemetryRecord
                let telemetry_record = if let Some(payload_bytes) = db_record.payload {
                    serde_json::from_slice::<TelemetryRecord>(&payload_bytes)
                        .map_err(|e| ServiceError::DeserializationError(e.to_string()))?
                } else {
                    return Err(ServiceError::DeserializationError(
                        "Missing telemetry payload".to_string(),
                    ));
                };

                // Convert to TelemetryResponse
                Ok(TelemetryResponse {
                    timestamp: db_record.timestamp.timestamp(),
                    temperature: telemetry_record.temperature,
                    voltage: telemetry_record.voltage,
                    current: telemetry_record.current,
                    battery_level: telemetry_record.battery_level,
                })
            })
            .collect();

        responses
    }
}
