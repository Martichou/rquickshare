use mdns::MDnsServer;
use tokio_util::{sync::CancellationToken, task::TaskTracker};
use utils::DeviceType;

#[macro_use]
extern crate log;

mod mdns;
mod utils;

#[tokio::main]

async fn main() -> Result<(), anyhow::Error> {
    // Define log level
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "TRACE,mdns_sd=ERROR,polling=ERROR")
    }

    // Init logger/tracing
    tracing_subscriber::fmt::init();

    let tracker = TaskTracker::new();
    let token = CancellationToken::new();

    let service_port = 1234;

    let mdns = MDnsServer::new(service_port, DeviceType::Laptop)?;
    let ctk = token.clone();
    tracker.spawn(async move { mdns.run(ctk).await });

    tracker.close();
    tracker.wait().await;

    Ok(())
}
