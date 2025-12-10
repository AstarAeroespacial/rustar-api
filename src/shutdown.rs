use crate::messaging::receiver::MqttReceiver;
use tokio::{signal, sync::oneshot, task::JoinHandle};

/// Spawn MQTT receiver in a background task
pub fn spawn_mqtt_receiver(
    mut receiver: MqttReceiver,
    shutdown_rx: oneshot::Receiver<()>,
) -> JoinHandle<()> {
    tokio::task::spawn_blocking(move || {
        let rt =
            tokio::runtime::Runtime::new().expect("Failed to create runtime for MQTT receiver");
        rt.block_on(receiver.run(shutdown_rx));
    })
}

/// Handle graceful shutdown
pub async fn handle_shutdown(
    server_handle: actix_web::dev::Server,
    shutdown_tx: oneshot::Sender<()>,
    mqtt_task: JoinHandle<()>,
) {
    let handle = server_handle.handle();

    tokio::select! {
        _ = signal::ctrl_c() => {
            log::info!("SIGINT received: shutting down server and MQTT receiver...");
            let _ = shutdown_tx.send(());
            handle.stop(true).await;
        }
        res = server_handle => {
            if let Err(e) = res {
                log::error!("HTTP server error: {:?}", e);
            }
            let _ = shutdown_tx.send(());
        }
    }

    // Wait for MQTT receiver to finish
    let _ = mqtt_task.await;
}
