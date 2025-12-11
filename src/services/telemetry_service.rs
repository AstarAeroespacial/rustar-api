use crate::models::responses::TelemetryResponse;
use crate::repository::telemetry::TelemetryRepository;
use crate::services::errors::ServiceError;
use chrono::{DateTime, Utc};
use rustar_types::telemetry::TelemetryRecord;

pub struct TelemetryService {
    repository: TelemetryRepository,
}

impl TelemetryService {
    pub fn new(repository: TelemetryRepository) -> Self {
        Self { repository }
    }

    /// Add a new telemetry record to the database
    pub async fn add_telemetry(
        &self,
        timestamp: DateTime<Utc>,
        sat_id: &str,
        gs_id: &str,
        payload: Vec<u8>,
    ) -> Result<(), ServiceError> {
        // Insert into the database
        self.repository
            .insert_telemetry(timestamp, sat_id, gs_id, payload)
            .await?;

        Ok(())
    }

    /// Get telemetry for a specific satellite by its ID with optional pagination
    pub async fn get_telemetry_by_satellite_id(
        &self,
        satellite_id: &String,
        limit: Option<i64>,
        page: Option<i64>,
    ) -> Result<Vec<TelemetryResponse>, ServiceError> {
        let telemetry = self
            .repository
            .get_telemetry_by_satellite(satellite_id, limit, page)
            .await?;

        // Convert TelemetryDb to TelemetryResponse, filtering out invalid records
        let responses: Vec<TelemetryResponse> = telemetry
            .into_iter()
            .filter_map(|db_record| {
                // Decode the payload bytes to TelemetryRecord
                let telemetry_record = match db_record.payload {
                    Some(payload_bytes) => {
                        match serde_json::from_slice::<TelemetryRecord>(&payload_bytes) {
                            Ok(record) => record,
                            Err(e) => {
                                // Log the error but don't fail the entire request
                                eprintln!(
                                    "Warning: Failed to deserialize telemetry record {}: {}",
                                    db_record.id, e
                                );
                                return None;
                            }
                        }
                    }
                    None => {
                        eprintln!(
                            "Warning: Missing payload for telemetry record {}",
                            db_record.id
                        );
                        return None;
                    }
                };

                // Convert to TelemetryResponse
                Some(TelemetryResponse {
                    id: db_record.id,
                    timestamp: db_record.timestamp.timestamp(),
                    temperature: telemetry_record.temperature,
                    voltage: telemetry_record.voltage,
                    current: telemetry_record.current,
                    battery_level: telemetry_record.battery_level,
                })
            })
            .collect();

        Ok(responses)
    }
}
