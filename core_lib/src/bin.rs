#[macro_use]
extern crate log;

use rquickshare::RQS;
use tokio::sync::mpsc;

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

    // Start the RQuickShare service
    let mut rqs = RQS::default();
    rqs.run().await?;

    let discovery_channel = mpsc::channel(10);

    rqs.discovery(discovery_channel.0)?;

    // Wait for CTRL+C and then stop RQS
    let _ = tokio::signal::ctrl_c().await;
    info!("Stopping service.");
    rqs.stop().await;

    Ok(())
}
