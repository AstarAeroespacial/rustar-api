use crate::services::telemetry_service::TelemetryService;
use rumqttc::{
    AsyncClient,
    Event::{self, Incoming, Outgoing},
    EventLoop, MqttOptions,
    Packet::Publish,
    QoS,
};
use rustar_types::mqtt::telemetry::TelemetryMessage;
use std::{sync::Arc, time::Duration};
use tokio::sync::oneshot;
use uuid::Uuid;

pub struct MqttReceiver {
    client: AsyncClient,
    eventloop: EventLoop,
    telemetry_service: Arc<TelemetryService>,
}

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

    async fn handle_event(
        &self,
        event: Event,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Notif: {:?}", event);

        match event {
            Incoming(pk) => {
                println!("Received incoming event: {:?}", pk);

                if let Publish(msg) = pk {
                    let parts: Vec<_> = msg.topic.split('/').collect();

                    match parts[0] {
                        "satellite" => {
                            let sat_id = parts[1];

                            match parts[2] {
                                "telemetry" => {
                                    let telemetry: TelemetryMessage =
                                        serde_json::from_slice(&msg.payload).unwrap();

                                    self.telemetry_service
                                        .add_telemetry(
                                            telemetry.timestamp,
                                            sat_id,
                                            &telemetry.ground_station_id,
                                            telemetry.payload,
                                        )
                                        .await
                                        .unwrap();
                                }
                                _ => unimplemented!("not expecting anything here tbh"),
                            }
                        }
                        topic => {
                            todo!("{}", format!("{} topic handling not yet supported", topic))
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
