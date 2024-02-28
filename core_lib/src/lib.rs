#[macro_use]
extern crate log;

use channel::ChannelMessage;
use hdl::BleAdvertiser;
use mdns_sd::{ServiceDaemon, ServiceEvent};
use rand::{distributions, Rng};
use serde::{Deserialize, Serialize};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{self, Receiver};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;
use ts_rs::TS;

use crate::hdl::{BleListener, MDnsServer};
use crate::manager::TcpServer;
use crate::utils::{is_not_self_ip, parse_mdns_endpoint_info};

pub mod channel;
mod errors;
mod hdl;
mod manager;
mod utils;

pub use hdl::OutboundPayload;
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
    tracker: TaskTracker,
    ctoken: CancellationToken,
    discovery_ctk: Option<CancellationToken>,
    blea_ctk: Option<CancellationToken>,
    pub channel: (broadcast::Sender<ChannelMessage>, Receiver<ChannelMessage>),
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
        let channel = broadcast::channel(10);

        Self {
            tracker,
            ctoken,
            channel,
            discovery_ctk: None,
            blea_ctk: None,
        }
    }

    pub async fn run(&self) -> Result<mpsc::Sender<SendInfo>, anyhow::Error> {
        let endpoint_id: Vec<u8> = rand::thread_rng()
            .sample_iter(distributions::Alphanumeric)
            .take(4)
            .map(u8::from)
            .collect();
        let tcp_listener = TcpListener::bind("0.0.0.0:0").await?;
        let binded_addr = tcp_listener.local_addr()?;
        info!("TcpListener on: {}", binded_addr);

        // Sender for the TcpServer
        let sender = self.channel.0.clone();

        let send_channel = mpsc::channel(10);

        // Start TcpServer in own "task"
        let mut server = TcpServer::new(
            endpoint_id[..4].try_into()?,
            tcp_listener,
            sender,
            send_channel.1,
        )?;
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
        let mdns = MDnsServer::new(
            endpoint_id[..4].try_into()?,
            binded_addr.port(),
            DeviceType::Laptop,
            ble_channel.1,
        )?;
        let ctk = self.ctoken.clone();
        self.tracker.spawn(async move { mdns.run(ctk).await });

        self.tracker.close();

        Ok(send_channel.0)
    }

    pub fn discovery(&mut self, sender: mpsc::Sender<EndpointInfo>) -> Result<(), anyhow::Error> {
        let ctk = CancellationToken::new();
        self.discovery_ctk = Some(ctk.clone());

        let blea_ctk = CancellationToken::new();
        self.blea_ctk = Some(blea_ctk.clone());

        let discovery = MDnsDiscovery::new(sender)?;
        self.tracker.spawn(async move { discovery.run(ctk).await });

        self.tracker.spawn(async move {
            let blea = match BleAdvertiser::new().await {
                Ok(b) => b,
                Err(e) => {
                    error!("Couldn't init BleAdvertiser: {}", e);
                    return;
                }
            };

            if let Err(e) = blea.run(blea_ctk).await {
                error!("Couldn't start BleAdvertiser: {}", e);
            }
        });

        Ok(())
    }

    pub fn stop_discovery(&mut self) {
        if let Some(discovert_ctk) = &self.discovery_ctk {
            discovert_ctk.cancel();
            self.discovery_ctk = None;
        }

        if let Some(blea_ctk) = &self.blea_ctk {
            blea_ctk.cancel();
            self.blea_ctk = None;
        }
    }

    pub async fn stop(&mut self) {
        self.stop_discovery();
        self.ctoken.cancel();
        self.tracker.wait().await;
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct EndpointInfo {
    pub id: String,
    pub name: Option<String>,
    pub ip: Option<String>,
    pub port: Option<String>,
    pub rtype: Option<DeviceType>,
    pub present: Option<bool>,
}

pub struct MDnsDiscovery {
    daemon: ServiceDaemon,
    sender: mpsc::Sender<EndpointInfo>,
}

impl MDnsDiscovery {
    pub fn new(sender: mpsc::Sender<EndpointInfo>) -> Result<Self, anyhow::Error> {
        let daemon = ServiceDaemon::new()?;

        Ok(Self { daemon, sender })
    }

    pub async fn run(self, ctk: CancellationToken) -> Result<(), anyhow::Error> {
        info!("MDnsDiscovery: service starting");

        let service_type = "_FC9F5ED42C8A._tcp.local.";
        let receiver = self.daemon.browse(service_type)?;

        loop {
            tokio::select! {
                _ = ctk.cancelled() => {
                    info!("MDnsDiscovery: tracker cancelled, breaking");
                    break;
                }
                r = receiver.recv_async() => {
                    match r {
                        Ok(event) => {
                            match event {
                                ServiceEvent::ServiceResolved(info) => {
                                    let port = info.get_port();
                                    let ip_hash = info.get_addresses_v4();
                                    let ip = if !ip_hash.is_empty() {
                                        ip_hash.iter().next().unwrap()
                                    } else {
                                        continue;
                                    };

                                    // Check that the IP is not a "self IP"
                                    if !is_not_self_ip(ip) {
                                        continue;
                                    }

                                    // Decode the "n" text properties
                                    let n = info.get_property("n");
                                    if n.is_none() {
                                        continue;
                                    }
                                    let (dt, dn) = match parse_mdns_endpoint_info(n.unwrap().val_str()) {
                                        Ok(r) => r,
                                        Err(_) => continue
                                    };

                                    if TcpStream::connect(format!("{ip}:{port}")).await.is_ok() {
                                        let ei = EndpointInfo {
                                            id: info.get_fullname().to_string(),
                                            name: Some(dn),
                                            ip: Some(ip.to_string()),
                                            port: Some(port.to_string()),
                                            rtype: Some(dt),
                                            present: Some(true),
                                        };
                                        info!("Resolved a new service: {:?}", ei);
                                        let _ = self.sender.send(ei).await;
                                    }
                                }
                                ServiceEvent::ServiceRemoved(_, fullname) => {
                                    info!("Remove a previous service: {}", fullname);
                                    let _ = self.sender.send(EndpointInfo {
                                        id: fullname,
                                        ..Default::default()
                                    }).await;
                                }
                                ServiceEvent::SearchStarted(_) | ServiceEvent::SearchStopped(_) => {}
                                _ => {}
                            }
                        },
                        Err(err) => error!("MDnsDiscovery: error: {}", err),
                    }
                }
            }
        }

        Ok(())
    }
}
