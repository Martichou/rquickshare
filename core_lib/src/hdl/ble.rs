use std::time::{Duration, SystemTime};

use anyhow::anyhow;
use btleplug::api::{Central, CentralEvent, Manager as _, ScanFilter};
use btleplug::platform::{Adapter, Manager};
use futures::stream::StreamExt;
use tokio::sync::broadcast::Sender;
use tokio_util::sync::CancellationToken;
use uuid::{uuid, Uuid};

const SERVICE_UUID_SHARING: Uuid = uuid!("0000fe2c-0000-1000-8000-00805f9b34fb");

const INNER_NAME: &str = "BleListener";

pub struct BleListener {
    adapter: Adapter,
    sender: Sender<()>,
}

impl BleListener {
    pub async fn new(sender: Sender<()>) -> Result<Self, anyhow::Error> {
        let manager = Manager::new().await?;
        let adapters = manager.adapters().await?;
        if adapters.is_empty() {
            return Err(anyhow!("no bluetooth adapter"));
        }

        Ok(Self {
            adapter: adapters[0].clone(),
            sender,
        })
    }

    pub async fn run(self, ctk: CancellationToken) -> Result<(), anyhow::Error> {
        info!("{INNER_NAME}: service starting");

        let mut events = self.adapter.events().await?;
        // Filter on the NearyShare/QuickShare services UUID
        self.adapter
            .start_scan(ScanFilter {
                services: vec![SERVICE_UUID_SHARING],
            })
            .await?;

        let mut last_alert = SystemTime::now();

        loop {
            tokio::select! {
                _ = ctk.cancelled() => {
                    info!("{INNER_NAME}: tracker cancelled, breaking");
                    break;
                }
                Some(e) = events.next() => {
                    match e {
                        CentralEvent::ServiceDataAdvertisement { id, service_data } => {
                            let _ = id;
                            let _ = service_data;
                            let now = SystemTime::now();

                            // Don't spam, max once per 15s
                            if now.duration_since(last_alert)? <= Duration::from_secs(15) {
                                continue;
                            }

                            debug!("{INNER_NAME}: A device is sharing nearby");
                            self.sender.send(())?;
                            last_alert = now;
                        },
                        // Not interesting for us
                        _ => {
                            // trace!("{INNER_NAME}: Another CentralEvent got the same services: {:?}", e);
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
