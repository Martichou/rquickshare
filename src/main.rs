#[macro_use]
extern crate log;

use manager::TcpServer;
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;
use utils::DeviceType;

use crate::backend::MDnsServer;

mod backend;
mod errors;
mod manager;
mod utils;

pub mod sharing_nearby {
    include!(concat!(env!("OUT_DIR"), "/sharing.nearby.rs"));
}

pub mod securemessage {
    include!(concat!(env!("OUT_DIR"), "/securemessage.rs"));
}

pub mod securegcm {
    include!(concat!(env!("OUT_DIR"), "/securegcm.rs"));
}

pub mod location_nearby_connections {
    include!(concat!(env!("OUT_DIR"), "/location.nearby.connections.rs"));
}

#[tokio::main]

async fn main() -> Result<(), anyhow::Error> {
    // Define log level
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "TRACE,mdns_sd=ERROR,polling=ERROR,neli=ERROR");
    }

    // Init logger/tracing
    tracing_subscriber::fmt::init();

    let tracker = TaskTracker::new();
    let token = CancellationToken::new();

    let tcp_listener = TcpListener::bind("0.0.0.0:0").await?;
    let binded_addr = tcp_listener.local_addr()?;
    info!("TcpListener on: {}", binded_addr);

    let server = TcpServer::new(tcp_listener)?;
    let ctk = token.clone();
    tracker.spawn(async move { server.run(ctk).await });

    let mdns = MDnsServer::new(binded_addr.port(), DeviceType::Laptop)?;
    let ctk = token.clone();
    tracker.spawn(async move { mdns.run(ctk).await });

    tracker.close();
    tracker.wait().await;

    Ok(())
}
