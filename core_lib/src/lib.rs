#[macro_use]
extern crate log;

use std::sync::{Arc, Mutex};

use anyhow::anyhow;
use channel::ChannelMessage;
#[cfg(all(feature = "experimental", not(target_os = "macos")))]  
use hdl::BleAdvertiser;
use hdl::MDnsDiscovery;
use rand::{distributions, Rng};
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio::sync::{mpsc, watch};
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;
#[cfg(not(target_os = "macos"))]
use crate::hdl::{BleListener};
use crate::hdl::{MDnsServer};
use crate::manager::TcpServer;

pub mod channel;
mod errors;
mod hdl;
mod manager;
mod utils;

pub use hdl::{EndpointInfo, OutboundPayload, State, Visibility};
pub use manager::SendInfo;
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
    tracker: Option<TaskTracker>,
    ctoken: Option<CancellationToken>,
    // Discovery token is different than ctoken because he is on his own
    // - can be cancelled while the ctoken is still active
    discovery_ctk: Option<CancellationToken>,

    // Used to trigger a change in the mDNS visibility (and later on, BLE)
    pub visibility_sender: Arc<Mutex<watch::Sender<Visibility>>>,
    visibility_receiver: watch::Receiver<Visibility>,

    // Only used to send the info "a nearby device is sharing"
    ble_sender: broadcast::Sender<()>,

    pub message_sender: broadcast::Sender<ChannelMessage>,
}

impl Default for RQS {
    fn default() -> Self {
        Self::new(Visibility::Visible)
    }
}

impl RQS {
    pub fn new(visibility: Visibility) -> Self {
        let (message_sender, _) = broadcast::channel(50);
        let (ble_sender, _) = broadcast::channel(5);

        // Define default visibility as per the args inside the new()
        let (visibility_sender, visibility_receiver) = watch::channel(Visibility::Invisible);
        let _ = visibility_sender.send(visibility);

        Self {
            tracker: None,
            ctoken: None,
            discovery_ctk: None,
            visibility_sender: Arc::new(Mutex::new(visibility_sender)),
            visibility_receiver,
            ble_sender,
            message_sender,
        }
    }

    pub async fn run(
        &mut self,
    ) -> Result<(mpsc::Sender<SendInfo>, broadcast::Receiver<()>), anyhow::Error> {
        let tracker = TaskTracker::new();
        let ctoken = CancellationToken::new();
        self.tracker = Some(tracker.clone());
        self.ctoken = Some(ctoken.clone());

        let endpoint_id: Vec<u8> = rand::thread_rng()
            .sample_iter(distributions::Alphanumeric)
            .take(4)
            .map(u8::from)
            .collect();
        let tcp_listener = TcpListener::bind("0.0.0.0:0").await?;
        let binded_addr = tcp_listener.local_addr()?;
        info!("TcpListener on: {}", binded_addr);

        // MPSC for the TcpServer
        let send_channel = mpsc::channel(10);
        // Start TcpServer in own "task"
        let mut server = TcpServer::new(
            endpoint_id[..4].try_into()?,
            tcp_listener,
            self.message_sender.clone(),
            send_channel.1,
        )?;
        let ctk = ctoken.clone();
        tracker.spawn(async move { server.run(ctk).await });

        // Don't threat BleListener error as fatal, it's a nice to have.
        if let Ok(ble) = BleListener::new(self.ble_sender.clone()).await {
            let ctk = ctoken.clone();
            tracker.spawn(async move { ble.run(ctk).await });
        #[cfg(not(target_os = "macos"))]
        if let Ok(ble) = BleListener::new(ble_channel.0).await {
            let ctk = self.ctoken.clone();
            self.tracker.spawn(async move { ble.run(ctk).await });
        }

        // Start MDnsServer in own "task"
        let mut mdns = MDnsServer::new(
            endpoint_id[..4].try_into()?,
            binded_addr.port(),
            self.ble_sender.subscribe(),
            self.visibility_sender.clone(),
            self.visibility_receiver.clone(),
        )?;
        let ctk = ctoken.clone();
        tracker.spawn(async move { mdns.run(ctk).await });

        tracker.close();

        Ok((send_channel.0, self.ble_sender.subscribe()))
    }

    pub fn discovery(
        &mut self,
        sender: broadcast::Sender<EndpointInfo>,
    ) -> Result<(), anyhow::Error> {
        let tracker = match &self.tracker {
            Some(t) => t,
            None => return Err(anyhow!("The service wasn't first started")),
        };

        let ctk = CancellationToken::new();
        self.discovery_ctk = Some(ctk.clone());

        #[cfg(all(feature = "experimental", not(target_os = "macos")))]  
        self.tracker.spawn(async move {
            let blea = match BleAdvertiser::new().await {
                Ok(b) => b,
                Err(e) => {
                    error!("Couldn't init BleAdvertiser: {}", e);
                    return;
                }
            };

                if let Err(e) = blea.run(ctk_blea).await {
                    error!("Couldn't start BleAdvertiser: {}", e);
                }
            });
        }

        let discovery = MDnsDiscovery::new(sender)?;
        tracker.spawn(async move { discovery.run(ctk.clone()).await });

        Ok(())
    }

    pub fn stop_discovery(&mut self) {
        if let Some(discovert_ctk) = &self.discovery_ctk {
            discovert_ctk.cancel();
            self.discovery_ctk = None;
        }
    }

    pub fn change_visibility(&mut self, nv: Visibility) {
        self.visibility_sender
            .lock()
            .unwrap()
            .send_modify(|state| *state = nv);
    }

    pub async fn stop(&mut self) {
        self.stop_discovery();

        if let Some(ctoken) = &self.ctoken {
            ctoken.cancel();
        }

        if let Some(tracker) = &self.tracker {
            tracker.wait().await;
        }

        self.ctoken = None;
        self.tracker = None;
    }
}
