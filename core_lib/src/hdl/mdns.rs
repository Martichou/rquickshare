use std::sync::{Arc, Mutex};
use std::time::Duration;

use mdns_sd::{AddrType, ServiceDaemon, ServiceInfo};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast::Receiver;
use tokio::sync::watch;
use tokio::time::{interval_at, Instant};
use tokio_util::sync::CancellationToken;
use ts_rs::TS;

use crate::utils::{gen_mdns_endpoint_info, gen_mdns_name, DeviceType};
use crate::RqsEvent;

const INNER_NAME: &str = "MDnsServer";
const TICK_INTERVAL: Duration = Duration::from_secs(60);

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum Visibility {
    Visible = 0,
    Invisible = 1,
    Temporarily = 2,
}

#[allow(dead_code)]
impl Visibility {
    pub fn from_raw_value(value: u64) -> Self {
        match value {
            0 => Visibility::Visible,
            1 => Visibility::Invisible,
            2 => Visibility::Temporarily,
            _ => unreachable!(),
        }
    }
}

pub struct MDnsServer {
    daemon: ServiceDaemon,
    service_info: ServiceInfo,
    event_receiver: Receiver<RqsEvent>,
    visibility_sender: Arc<Mutex<watch::Sender<Visibility>>>,
    visibility_receiver: watch::Receiver<Visibility>,
}

impl MDnsServer {
    pub fn new(
        endpoint_id: [u8; 4],
        service_port: u16,
        event_receiver: Receiver<RqsEvent>,
        visibility_sender: Arc<Mutex<watch::Sender<Visibility>>>,
        visibility_receiver: watch::Receiver<Visibility>,
    ) -> Result<Self, anyhow::Error> {
        let daemon = ServiceDaemon::new()?;
        let service_info = Self::build_service(endpoint_id, service_port, DeviceType::Unknown)?;

        Ok(Self {
            daemon,
            service_info,
            event_receiver,
            visibility_sender,
            visibility_receiver,
        })
    }

    pub async fn run(&mut self, ctk: CancellationToken) -> Result<(), anyhow::Error> {
        info!("{INNER_NAME}: service starting");

        let mut current_visibility = *self.visibility_receiver.borrow();
        if current_visibility == Visibility::Visible {
            self.daemon.register(self.service_info.clone())?;
        }

        let mut interval = interval_at(Instant::now() + TICK_INTERVAL, TICK_INTERVAL);

        loop {
            tokio::select! {
                _ = ctk.cancelled() => {
                    info!("{INNER_NAME}: tracker cancelled, breaking");
                    break;
                }
                _ = interval.tick() => {
                    trace!("{INNER_NAME}: tick");
                }
                Ok(event) = self.event_receiver.recv() => {
                    if let RqsEvent::NearbyDeviceSharing = event {
                        trace!("{INNER_NAME}: ble received");
                        if current_visibility == Visibility::Invisible {
                            self.visibility_sender.lock().unwrap().send_modify(|state| *state = Visibility::Temporarily);
                        }
                    }
                }
                Ok(_) = self.visibility_receiver.changed() => {
                    let new_visibility = *self.visibility_receiver.borrow();
                    trace!("{INNER_NAME}: visibility changed: {:?}", new_visibility);

                    if new_visibility != current_visibility {
                        match new_visibility {
                            Visibility::Visible => {
                                self.daemon.register(self.service_info.clone())?;
                            }
                            Visibility::Invisible => {
                                self.daemon.unregister(self.service_info.get_fullname())?;
                            }
                            Visibility::Temporarily => {
                                self.daemon.register(self.service_info.clone())?;

                                let visibility_sender = self.visibility_sender.clone();
                                tokio::spawn(async move {
                                    tokio::time::sleep(Duration::from_secs(60)).await;
                                    visibility_sender.lock().unwrap().send_modify(|state| {
                                        if *state == Visibility::Temporarily {
                                            *state = Visibility::Invisible;
                                        }
                                    });
                                });
                            }
                        }

                        current_visibility = new_visibility;
                    }
                }
            }
        }

        if current_visibility != Visibility::Invisible {
            let _ = self.daemon.unregister(self.service_info.get_fullname());
        }

        info!("{INNER_NAME}: service stopped");
        Ok(())
    }

    fn build_service(
        endpoint_id: [u8; 4],
        service_port: u16,
        device_type: DeviceType,
    ) -> Result<ServiceInfo, anyhow::Error> {
        let name = gen_mdns_name(endpoint_id);
        let hostname = sys_metrics::host::get_hostname()?;
        info!("Broadcasting with: {hostname}");
        let endpoint_info = gen_mdns_endpoint_info(device_type as u8, &hostname);

        let properties = [("n", endpoint_info)];
        let si = ServiceInfo::new(
            "_FC9F5ED42C8A._tcp.local.",
            &name,
            &hostname,
            "",
            service_port,
            &properties[..],
        )?
        .enable_addr_auto(AddrType::V4);

        Ok(si)
    }
}
