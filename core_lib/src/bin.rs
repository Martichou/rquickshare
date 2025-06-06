#[macro_use]
extern crate log;

use rqs_lib::RQS;
use tokio::sync::broadcast;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Define log level

    // Init logger/tracing
    tracing_subscriber::fmt()
        .with_env_filter(if std::env::var("RUST_LOG").is_ok() {
            EnvFilter::builder().from_env_lossy()
        } else {
            EnvFilter::builder()
                .parse_lossy("TRACE,mdns_sd=ERROR,polling=ERROR,neli=ERROR,bluez_async=ERROR")
        })
        .init();

    // Start the RQuickShare service
    let mut rqs = RQS::default();
    rqs.run().await?;

    let discovery_channel = broadcast::channel(10);
    rqs.discovery(discovery_channel.0)?;

    // Wait for CTRL+C and then stop RQS
    let _ = tokio::signal::ctrl_c().await;
    info!("Stopping service.");
    rqs.stop().await;

    Ok(())
}
