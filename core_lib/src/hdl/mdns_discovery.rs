use mdns_sd::{ServiceDaemon, ServiceEvent};
use serde::{Deserialize, Serialize};
use tokio::{net::TcpStream, sync::broadcast};
use tokio_util::sync::CancellationToken;
use ts_rs::TS;

use crate::{
    utils::{is_not_self_ip, parse_mdns_endpoint_info},
    DeviceType,
};

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
    sender: broadcast::Sender<EndpointInfo>,
}

impl MDnsDiscovery {
    pub fn new(sender: broadcast::Sender<EndpointInfo>) -> Result<Self, anyhow::Error> {
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
                                    if ip_hash.is_empty() {
                                        continue;
                                    }

                                    let ip = match ip_hash.iter().next() {
                                        Some(i) => i,
                                        None => continue,
                                    };

                                    // Check that the IP is not a "self IP"
                                    if !is_not_self_ip(ip) {
                                        continue;
                                    }

                                    // Decode the "n" text properties
                                    let n = match info.get_property("n") {
                                        Some(_n) => _n,
                                        None => continue,
                                    };

                                    // Parse the endpoint info
                                    let (dt, dn) = match parse_mdns_endpoint_info(n.val_str()) {
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
                                        let _ = self.sender.send(ei);
                                    }
                                }
                                ServiceEvent::ServiceRemoved(_, fullname) => {
                                    info!("Remove a previous service: {}", fullname);
                                    let _ = self.sender.send(EndpointInfo {
                                        id: fullname,
                                        ..Default::default()
                                    });
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
