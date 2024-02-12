use mdns_sd::{ServiceDaemon, ServiceInfo};
use rand::Rng;
use tokio_util::sync::CancellationToken;

use crate::utils::{gen_mdns_endpoint_info, gen_mdns_name, DeviceType};

const INNER_NAME: &str = "MDnsServer";

pub struct MDnsServer {
    daemon: ServiceDaemon,
    fullname: String,
}

impl MDnsServer {
    pub fn new(service_port: u16, device_type: DeviceType) -> Result<Self, anyhow::Error> {
        let daemon = ServiceDaemon::new()?;

        let endpoint_id = rand::thread_rng().gen::<[u8; 4]>();
        let name = gen_mdns_name(endpoint_id);
        let endpoint_info = gen_mdns_endpoint_info(device_type as u8, "rtin");

        let properties = [("n", endpoint_info)];
        let si = ServiceInfo::new(
            "_FC9F5ED42C8A._tcp.local.",
            &name,
            &sys_metrics::host::get_hostname()?,
            "",
            service_port,
            &properties[..],
        )?
        .enable_addr_auto();
        let fullname = si.get_fullname().to_owned();

        daemon.register(si)?;

        Ok(Self { daemon, fullname })
    }

    pub async fn run(self, ctk: CancellationToken) -> Result<(), anyhow::Error> {
        info!("{INNER_NAME}: service starting");
        let monitor = self.daemon.monitor()?;
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
                }
            }
        }

        // Unregister the mDNS service - we're shutting down
        let receiver = self.daemon.unregister(&self.fullname)?;
        if let Ok(event) = receiver.recv() {
            info!("MDnsServer: service unregistered: {:?}", &event);
        }

        Ok(())
    }
}
