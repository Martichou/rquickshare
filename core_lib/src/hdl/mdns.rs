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
    endpoint_id: [u8; 4],
    service_port: u16,
    service_info: ServiceInfo,
    ble_receiver: Receiver<()>,
    visibility_sender: Arc<Mutex<watch::Sender<Visibility>>>,
    visibility_receiver: watch::Receiver<Visibility>,
    device_name_receiver: watch::Receiver<String>,
}

impl MDnsServer {
    pub fn new(
        endpoint_id: [u8; 4],
        service_port: u16,
        ble_receiver: Receiver<()>,
        visibility_sender: Arc<Mutex<watch::Sender<Visibility>>>,
        visibility_receiver: watch::Receiver<Visibility>,
        device_name_receiver: watch::Receiver<String>,
    ) -> Result<Self, anyhow::Error> {
        let device_name = device_name_receiver.borrow().clone();
        let service_info = Self::build_service(endpoint_id, service_port, DeviceType::Laptop, &device_name)?;

        Ok(Self {
            daemon: ServiceDaemon::new()?,
            endpoint_id,
            service_port,
            service_info,
            ble_receiver,
            visibility_sender,
            visibility_receiver,
            device_name_receiver,
        })
    }

    pub async fn run(&mut self, ctk: CancellationToken) -> Result<(), anyhow::Error> {
        info!("{INNER_NAME}: service starting");
        let monitor = self.daemon.monitor()?;
        let mut visibility = *self.visibility_receiver.borrow();
        let mut interval = interval_at(Instant::now() + TICK_INTERVAL, TICK_INTERVAL);

        loop {
            tokio::select! {
                _ = ctk.cancelled() => {
                    info!("{INNER_NAME}: tracker cancelled, breaking");
                    break;
                }
                r = monitor.recv_async() => {
                    match r {
                        Ok(_) => continue,
                        Err(err) => return Err(err.into()),
                    }
                },
                _ = self.visibility_receiver.changed() => {
                    visibility = *self.visibility_receiver.borrow_and_update();

                    debug!("{INNER_NAME}: visibility changed: {visibility:?}");
                    if visibility == Visibility::Visible {
                        self.daemon.register(self.service_info.clone())?;
                    } else if visibility == Visibility::Invisible {
                        let receiver = self.daemon.unregister(self.service_info.get_fullname())?;
                        let _ = receiver.recv();
                    } else if visibility == Visibility::Temporarily {
                        self.daemon.register(self.service_info.clone())?;
                        interval.reset();
                    }
                }
                _ = self.device_name_receiver.changed() => {
                    let new_name = self.device_name_receiver.borrow_and_update().clone();
                    debug!("{INNER_NAME}: device name changed to: {new_name}");

                    // Unregister old service
                    let receiver = self.daemon.unregister(self.service_info.get_fullname())?;
                    let _ = receiver.recv();

                    // Rebuild service with new name
                    self.rebuild_service()?;

                    // Re-register if visible
                    if visibility == Visibility::Visible || visibility == Visibility::Temporarily {
                        self.daemon.register(self.service_info.clone())?;
                    }
                }
                _ = self.ble_receiver.recv() => {
                    if visibility == Visibility::Invisible {
                        continue;
                    }

                    debug!("{INNER_NAME}: ble_receiver: got event");
                    if visibility == Visibility::Visible || visibility == Visibility::Temporarily {
                        // Android can sometime not see the mDNS service if the service
                        // was running BEFORE Android started the Discovery phase for QuickShare.
                        // So resend a broadcast if there's a android device sending.
                        self.daemon.register_resend(self.service_info.get_fullname())?;
                    } else {
                        self.daemon.register(self.service_info.clone())?;
                    }
                },
                _ = interval.tick() => {
                    if visibility != Visibility::Temporarily {
                        continue;
                    }

                    let receiver = self.daemon.unregister(self.service_info.get_fullname())?;
                    let _ = receiver.recv();
                    let _ = self.visibility_sender.lock().unwrap().send(Visibility::Invisible);
                }
            }
        }

        // Unregister the mDNS service - we're shutting down
        let receiver = self.daemon.unregister(self.service_info.get_fullname())?;
        if let Ok(event) = receiver.recv() {
            info!("MDnsServer: service unregistered: {:?}", &event);
        }

        Ok(())
    }

    fn build_service(
        endpoint_id: [u8; 4],
        service_port: u16,
        device_type: DeviceType,
        device_name: &str,
    ) -> Result<ServiceInfo, anyhow::Error> {
        let name = gen_mdns_name(endpoint_id);
        // Use system hostname for mDNS (must be DNS-compatible)
        // The device_name goes in endpoint_info which is what Quick Share displays
        let hostname = sys_metrics::host::get_hostname()?;
        info!("Broadcasting as '{device_name}' (mDNS hostname: {hostname})");
        let endpoint_info = gen_mdns_endpoint_info(device_type as u8, device_name);

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

    fn rebuild_service(&mut self) -> Result<(), anyhow::Error> {
        let device_name = self.device_name_receiver.borrow().clone();
        self.service_info = Self::build_service(
            self.endpoint_id,
            self.service_port,
            DeviceType::Laptop,
            &device_name,
        )?;
        Ok(())
    }
}
