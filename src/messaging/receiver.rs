use crate::services::telemetry_service::TelemetryService;
use rumqttc::{
    AsyncClient,
    Event::{self, Incoming, Outgoing},
    EventLoop, MqttOptions,
    Packet::Publish,
    QoS,
};
use std::{sync::Arc, time::Duration};
use tokio::sync::oneshot;
use uuid::Uuid;

pub struct MqttReceiver {
    client: AsyncClient,
    eventloop: EventLoop,
    telemetry_service: Arc<TelemetryService>,
}

// TODO: somehow we should have listeners or handlers or something like the
// actix/axum handlers, to callback on received topics

impl MqttReceiver {
    #[allow(dead_code)]
    pub fn new(
        host: &str,
        port: u16,
        keep_alive: Duration,
        telemetry_service: Arc<TelemetryService>,
    ) -> Self {
        let client_id = format!("rustar-api-{}", Uuid::new_v4());
        let mut options = MqttOptions::new(client_id, host, port);
        options.set_keep_alive(keep_alive);
        println!("connecting to broker {}:{}", host, port);

        let (client, eventloop) = AsyncClient::new(options, 10);

        Self {
            client,
            eventloop,
            telemetry_service,
        }
    }

    pub fn from_client(
        client: AsyncClient,
        eventloop: EventLoop,
        telemetry_service: Arc<TelemetryService>,
    ) -> Self {
        Self {
            client,
            eventloop,
            telemetry_service,
        }
    }

    #[allow(dead_code)]
    pub fn client(&self) -> AsyncClient {
        self.client.clone()
    }

    pub async fn run(&mut self, mut shutdown: oneshot::Receiver<()>) {
        if let Err(e) = self.client.subscribe("test-topic", QoS::AtLeastOnce).await {
            eprintln!("Error subscribing to topic: {:?}", e)
        } else {
            println!("Subscribed to topic: test-topic")
        }

        loop {
            tokio::select! {
                _ = &mut shutdown => {
                    println!("MqttReceiver: shutdown signal received");
                    break;
                }
                event = self.eventloop.poll() => {
                    match event {
                        Ok(notif) => {
                            if let Err(e) = self.handle_event(notif).await {
                                eprintln!("Error handling event: {:?}", e);
                            }
                        },
                        Err(e) => eprintln!("Connection error in recv: {:?}", e)
                    }
                }
            }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }

        if let Err(e) = self.client.disconnect().await {
            eprintln!("Error disconnecting MQTT client: {:?}", e);
        }
    }

    fn subscribe(&self, topic: impl Into<String>, qos: QoS) {
        todo!()
    }

    fn unsubscribe(&self, topic: impl Into<String>) {
        todo!()
    }

    // TODO: modify this function to handle the arrival of stuff from the ground station
    async fn handle_event(
        &self,
        event: Event,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Notif: {:?}", event);

        match event {
            Incoming(pk) => {
                println!("Received incoming event: {:?}", pk);

                if let Publish(msg) = pk {
                    let msg_text = String::from_utf8(msg.payload.to_vec());

                    // find the topic
                    let parts: Vec<_> = msg.topic.split('/').collect();

                    // TODO: make safe with .get()
                    match parts[0] {
                        "gs" => {
                            let gs_id = parts[1];

                            todo!("handle whatever the gs send. metrics, logs?")
                        }
                        "job" => {
                            let job_id = parts[1];

                            // TODO: bring the status type from rustar-gs
                            let status = serde_json::from_slice(&msg.payload).unwrap();

                            todo!("handle the status");
                        }
                        "satellite" => {
                            let sat_id = parts[1];

                            match parts[2] {
                                "telemetry" => {
                                    // TODO: bring telemetry message type from rustar-gs <- or rustar-types
                                    let frame = serde_json::from_slice(&msg.payload).unwrap();

                                    todo!("handle the frame");
                                }
                                _ => {
                                    unreachable!("wasnt expecting anything else tbh");
                                }
                            }
                        }
                        _ => {
                            unreachable!("wasnt expecting something here ¿?");
                        }
                    }
                } else {
                    println!("Incoming event: {:?}", pk)
                }
            }
            Outgoing(ev) => println!("Outgoing event: {:?}", ev),
        }

        Ok(())
    }
}
