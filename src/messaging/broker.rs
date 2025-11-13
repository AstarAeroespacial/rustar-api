use rumqttc::{
    tokio_rustls::{self, rustls::ClientConfig},
    AsyncClient, ClientError, EventLoop, MqttOptions, QoS, Transport,
};
use std::time::Duration;
use uuid::Uuid;

pub struct MqttBroker {
    client: AsyncClient,
}

impl MqttBroker {
    pub fn new(host: &str, port: u16, keep_alive: Duration) -> (Self, EventLoop) {
        // let client_id = format!("rustar-api-{}", Uuid::new_v4());
        // let mut options = MqttOptions::new(client_id, host, port);
        // options.set_keep_alive(keep_alive);
        // println!("connecting to broker {}:{}", host, port);

        // let (client, eventloop) = AsyncClient::new(options, 10);

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

        (Self { client }, eventloop)
    }

    #[allow(dead_code)]
    pub fn from_client(client: AsyncClient) -> Self {
        Self {
            client: client.clone(),
        }
    }

    pub fn client(&self) -> AsyncClient {
        self.client.clone()
    }

    pub async fn publish(&self, topic: &str, payload: &str) -> Result<(), ClientError> {
        self.client
            .publish(topic, QoS::AtLeastOnce, false, payload.as_bytes())
            .await?;
        println!("Published message {} to topic: {}", payload, topic);
        Ok(())
    }
}
