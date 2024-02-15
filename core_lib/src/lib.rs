#[macro_use]
extern crate log;

use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;

use crate::hdl::{BleListener, MDnsServer};
use crate::manager::TcpServer;

mod errors;
mod hdl;
mod manager;
mod utils;

pub use utils::DeviceType;

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

#[derive(Debug)]
pub struct RQS {
    tracker: TaskTracker,
    ctoken: CancellationToken,
}

impl Default for RQS {
    fn default() -> Self {
        Self::new()
    }
}

impl RQS {
    fn new() -> Self {
        let tracker = TaskTracker::new();
        let ctoken = CancellationToken::new();

        Self { tracker, ctoken }
    }

    pub async fn run(&self) -> Result<(), anyhow::Error> {
        let tcp_listener = TcpListener::bind("0.0.0.0:0").await?;
        let binded_addr = tcp_listener.local_addr()?;
        info!("TcpListener on: {}", binded_addr);

        // Start TcpServer in own "task"
        let server = TcpServer::new(tcp_listener)?;
        let ctk = self.ctoken.clone();
        self.tracker.spawn(async move { server.run(ctk).await });

        // Start BleListener in own "task"
        let ble_channel = broadcast::channel(10);
        // Don't threat BleListener error as fatal, it's a nice to have.
        if let Ok(ble) = BleListener::new(ble_channel.0).await {
            let ctk = self.ctoken.clone();
            self.tracker.spawn(async move { ble.run(ctk).await });
        }

        // Start MDnsServer in own "task"
        let mdns = MDnsServer::new(binded_addr.port(), DeviceType::Laptop, ble_channel.1)?;
        let ctk = self.ctoken.clone();
        self.tracker.spawn(async move { mdns.run(ctk).await });

        self.tracker.close();

        Ok(())
    }

    pub async fn stop(&self) {
        self.ctoken.cancel();
        self.tracker.wait().await;
    }
}
