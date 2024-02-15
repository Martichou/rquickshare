use local_ip_address::linux::local_ip;
use mdns_sd::{ServiceDaemon, ServiceInfo};
use rand::Rng;
use tokio::sync::broadcast::Receiver;
use tokio_util::sync::CancellationToken;

use crate::utils::{gen_mdns_endpoint_info, gen_mdns_name, DeviceType};

const INNER_NAME: &str = "MDnsServer";

pub struct MDnsServer {
    daemon: ServiceDaemon,
    fullname: String,
    receiver: Receiver<()>,
}

impl MDnsServer {
    fn build_service(
        service_port: u16,
        device_type: DeviceType,
    ) -> Result<ServiceInfo, anyhow::Error> {
        let endpoint_id = rand::thread_rng().gen::<[u8; 4]>();
        let name = gen_mdns_name(endpoint_id);
        let hostname = sys_metrics::host::get_hostname()?;
        info!("Broadcasting with: {hostname}");
        let endpoint_info = gen_mdns_endpoint_info(device_type as u8, &hostname);

        let local_ip = local_ip().unwrap();
        let properties = [("n", endpoint_info)];
        let si = ServiceInfo::new(
            "_FC9F5ED42C8A._tcp.local.",
            &name,
            &hostname,
            local_ip,
            service_port,
            &properties[..],
        )?;

        Ok(si)
    }

    pub fn new(
        service_port: u16,
        device_type: DeviceType,
        receiver: Receiver<()>,
    ) -> Result<Self, anyhow::Error> {
        let service_info = Self::build_service(service_port, device_type)?;
        let fullname = service_info.get_fullname().to_owned();

        let daemon = ServiceDaemon::new()?;
        daemon.register(service_info)?;

        Ok(Self {
            daemon,
            fullname,
            receiver,
        })
    }

    pub async fn run(self, ctk: CancellationToken) -> Result<(), anyhow::Error> {
        info!("{INNER_NAME}: service starting");
        let monitor = self.daemon.monitor()?;
        let mut receiver = self.receiver;

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
                // Can be used later on to change the visibility on the fly
                // by unregistering the service and registering it only when
                // a device nearby is sharing.
                _ = receiver.recv() => {
                    // Android can sometime not see the mDNS service if the service
                    // was running BEFORE Android started the Discovery phase for QuickShare.
                    // So resend a broadcast if there's a android device sending.
                    self.daemon.register_resend(&self.fullname)?;
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
