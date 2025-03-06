#[macro_use]
extern crate log;

use rqs_lib::{RqsConfig, RQS};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Define log level
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var(
            "RUST_LOG",
            "TRACE,mdns_sd=ERROR,polling=ERROR,neli=ERROR,bluez_async=ERROR",
        );
    }

    // Init logger/tracing
    tracing_subscriber::fmt::init();

    // Create the RQuickShare service with default configuration
    let mut rqs = RQS::default();
    let config = RqsConfig::default();

    // Start the service and get the handle
    let handle = rqs.start(&config).await?;

    // Start discovery
    handle.start_discovery()?;

    // Subscribe to events if needed
    let mut event_receiver = rqs.subscribe();
    tokio::spawn(async move {
        while let Ok(event) = event_receiver.recv().await {
            info!("Received event: {:?}", event);
        }
    });

    // Wait for CTRL+C and then stop RQS
    let _ = tokio::signal::ctrl_c().await;
    info!("Stopping service.");

    // Shutdown the service
    handle.shutdown().await;
    rqs.shutdown().await;

    Ok(())
}
