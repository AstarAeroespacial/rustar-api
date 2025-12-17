use crate::{
    models::entities::JobStatus,
    services::{job_service::JobService, telemetry_service::TelemetryService},
};
use rumqttc::{
    tokio_rustls::{self, rustls::ClientConfig},
    AsyncClient,
    Event::{self, Incoming, Outgoing},
    EventLoop, MqttOptions,
    Packet::Publish,
    QoS, Transport,
};
use std::{sync::Arc, time::Duration};
use tokio::sync::oneshot;
use uuid::Uuid;

pub struct MqttReceiver {
    client: AsyncClient,
    eventloop: EventLoop,
    telemetry_service: Arc<TelemetryService>,
    job_service: Arc<JobService>,
}

impl MqttReceiver {
    #[allow(dead_code)]
    pub fn new(
        host: &str,
        port: u16,
        keep_alive: Duration,
        telemetry_service: Arc<TelemetryService>,
        job_service: Arc<JobService>,
    ) -> Self {
        let client_id = format!("rustar-api-{}", Uuid::new_v4());
        let mut options = MqttOptions::new(client_id, host, port);
        options.set_keep_alive(keep_alive);
        println!("connecting to broker {}:{}", host, port);

        let mut root_cert_store = tokio_rustls::rustls::RootCertStore::empty();
        root_cert_store.add_parsable_certificates(
            rustls_native_certs::load_native_certs().expect("could not load platform certs"),
        );

        let client_config = ClientConfig::builder()
            .with_root_certificates(root_cert_store)
            .with_no_client_auth();

        options.set_transport(Transport::tls_with_config(client_config.into()));
        options.set_credentials("admin", "Admin123");

        let (client, eventloop) = AsyncClient::new(options, 10);

        Self {
            client,
            eventloop,
            telemetry_service,
            job_service,
        }
    }

    pub fn from_client(
        client: AsyncClient,
        eventloop: EventLoop,
        telemetry_service: Arc<TelemetryService>,
        job_service: Arc<JobService>,
    ) -> Self {
        Self {
            client,
            eventloop,
            telemetry_service,
            job_service,
        }
    }

    #[allow(dead_code)]
    pub fn client(&self) -> AsyncClient {
        self.client.clone()
    }

    pub async fn run(&mut self, mut shutdown: oneshot::Receiver<()>) {
        // TODO: handle subscribe and unsubscribe from job topics dinamically
        // when we send a job, we should subscribe to its topic and
        // when a job is complete we should disconnect from its topic

        self.client
            .subscribe("satellite/+/telemetry", QoS::AtLeastOnce)
            .await
            .unwrap();
        self.client
            .subscribe("job/+", QoS::AtLeastOnce)
            .await
            .unwrap();

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
        // println!("Notif: {:?}", event);

        match event {
            Incoming(pk) => {
                // println!("Received incoming event: {:?}", pk);

                if let Publish(msg) = pk {
                    let parts: Vec<_> = msg.topic.split('/').collect();

                    match parts[0] {
                        "satellite" => {
                            let sat_id = parts[1];

                            match parts[2] {
                                "telemetry" => {
                                    let telemetry: rustar_types::mqtt::telemetry::TelemetryMessage =
                                        serde_json::from_slice(&msg.payload).unwrap();

                                    let _ = self
                                        .telemetry_service
                                        .add_telemetry(
                                            telemetry.timestamp,
                                            sat_id,
                                            &telemetry.ground_station_id,
                                            telemetry.payload,
                                        )
                                        .await;
                                }
                                _ => unimplemented!("not expecting anything here tbh"),
                            }
                        }
                        "job" => {
                            let job_id = parts[1].parse().unwrap();

                            dbg!(&msg.payload);

                            if let Ok(status_update) = serde_json::from_slice::<
                                rustar_types::jobs::JobStatusUpdate,
                            >(&msg.payload)
                            {
                                let _ = self
                                    .job_service
                                    .add_job_status(
                                        job_id,
                                        status_update.status.into(),
                                        status_update.timestamp,
                                    )
                                    .await;
                            } else {
                                eprintln!("Failed to parse job status update for job {}", job_id);
                                eprintln!("Payload: {}", String::from_utf8_lossy(&msg.payload));
                            }
                        }
                        topic => {
                            todo!("{}", format!("{} topic handling not yet supported", topic))
                        }
                    }
                } else {
                    // println!("Incoming event: {:?}", pk)
                }
            }
            Outgoing(ev) => {
                // println!("Outgoing event: {:?}", ev)
            }
        }

        Ok(())
    }
}

impl From<rustar_types::jobs::JobStatus> for JobStatus {
    fn from(value: rustar_types::jobs::JobStatus) -> Self {
        match value {
            rustar_types::jobs::JobStatus::Received => JobStatus::Received,
            rustar_types::jobs::JobStatus::Scheduled => JobStatus::Scheduled,
            rustar_types::jobs::JobStatus::Started => JobStatus::Started,
            rustar_types::jobs::JobStatus::Completed => JobStatus::Completed,
            rustar_types::jobs::JobStatus::Error => JobStatus::Error,
        }
    }
}
