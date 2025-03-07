use std::collections::HashMap;

use mdns_sd::{ServiceDaemon, ServiceEvent};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;
use ts_rs::TS;

use crate::utils::{is_not_self_ip, parse_mdns_endpoint_info};
use crate::{DeviceType, RqsEvent};

#[derive(Debug, Clone, Default, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct EndpointInfo {
    pub fullname: String,
    pub id: String,
    pub name: Option<String>,
    pub ip: Option<String>,
    pub port: Option<String>,
    pub rtype: Option<DeviceType>,
    pub present: Option<bool>,
}

pub struct MDnsDiscovery {
    daemon: ServiceDaemon,
    sender: broadcast::Sender<RqsEvent>,
}

impl MDnsDiscovery {
    pub fn new(sender: broadcast::Sender<RqsEvent>) -> Result<Self, anyhow::Error> {
        let daemon = ServiceDaemon::new()?;

        Ok(Self { daemon, sender })
    }

    pub async fn run(self, ctk: CancellationToken) -> Result<(), anyhow::Error> {
        info!("MDnsDiscovery: service starting");

        let service_type = "_FC9F5ED42C8A._tcp.local.";
        let receiver = self.daemon.browse(service_type)?;

        // Map with fullname as key and EndpointInfo as value
        let mut cache: HashMap<String, EndpointInfo> = HashMap::new();

        loop {
            tokio::select! {
                _ = ctk.cancelled() => {
                    info!("MDnsDiscovery: tracker cancelled, breaking");
                    break;
                }
                Ok(event) = receiver.recv_async() => {
                    match event {
                        ServiceEvent::ServiceResolved(info) => {
                            let fullname = info.get_fullname().to_string();
                            let addresses = info.get_addresses_v4();
                            if addresses.is_empty() {
                                continue;
                            }

                            let ip = addresses
                                .iter()
                                .find(|ip| is_not_self_ip(ip))
                                .map(|ip| ip.to_string());

                            if ip.is_none() {
                                continue;
                            }

                            let port = info.get_port().to_string();

                            // Extract the "n" property which contains the encoded endpoint info
                            let n_property = info.get_properties()
                                .iter()
                                .find(|prop| prop.key() == "n")
                                .map(|prop| prop.val_str());

                            if let Some(n_value) = n_property {
                                match parse_mdns_endpoint_info(n_value) {
                                    Ok((device_type, device_name)) => {
                                        let endpoint_info = EndpointInfo {
                                            fullname: fullname.clone(),
                                            id: format!("{}:{}", ip.as_ref().unwrap(), port),
                                            name: Some(device_name),
                                            ip,
                                            port: Some(port),
                                            rtype: Some(device_type),
                                            present: Some(true),
                                        };

                                        cache.insert(fullname.clone(), endpoint_info.clone());
                                        let _ = self.sender.send(RqsEvent::DeviceDiscovered(endpoint_info));
                                    },
                                    Err(e) => {
                                        error!("Failed to parse endpoint info: {}", e);
                                    }
                                }
                            }
                        }
                        ServiceEvent::ServiceRemoved(_, fullname) => {
                            if let Some(mut endpoint_info) = cache.remove(&fullname) {
                                endpoint_info.present = Some(false);
                                let _ = self.sender.send(RqsEvent::DeviceDiscovered(endpoint_info));
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        info!("MDnsDiscovery: service stopped");
        Ok(())
    }
}
