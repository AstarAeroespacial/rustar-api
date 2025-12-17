use crate::messaging::broker::MqttBroker;
use crate::services::errors::ServiceError;
use rustar_types::jobs::Job;

pub struct MessageService {
    pub message_broker: MqttBroker,
}

impl MessageService {
    pub fn new(message_broker: MqttBroker) -> Self {
        Self { message_broker }
    }

    pub async fn send_message(&self, topic: &str, payload: &str) -> Result<(), ServiceError> {
        self.message_broker.publish(topic, payload).await?;
        Ok(())
    }

    pub async fn send_job(&self, gs_id: String, job: Job) -> Result<(), ServiceError> {
        let topic = format!("gs/{}/jobs", gs_id);

        let data = serde_json::to_string(&job)
            .map_err(|e| ServiceError::Internal(format!("Failed to serialize job: {}", e)))?;

        self.message_broker
            .publish(&topic, &data)
            .await
            .map_err(|e| ServiceError::Internal(format!("Failed to publish to MQTT: {:?}", e)))?;

        Ok(())
    }
}
