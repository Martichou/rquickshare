use std::collections::HashMap;
use std::time::{Duration, SystemTime};

use anyhow::anyhow;
use btleplug::api::{Central, CentralEvent, Manager as _, ScanFilter};
use btleplug::platform::{Adapter, Manager};
use futures::stream::StreamExt;
use tokio::sync::broadcast::Sender;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

const INNER_NAME: &str = "BleListener";

pub struct BleListener {
    adapter: Adapter,
    sender: Sender<()>,
}

impl BleListener {
    pub async fn new(sender: Sender<()>) -> Result<Self, anyhow::Error> {
        let manager = Manager::new().await?;
        let adapters = manager.adapters().await.unwrap();
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
        // TODO - Determine if it's common for everyone.
        self.adapter
            .start_scan(ScanFilter {
                services: vec![Uuid::parse_str("0000fe2c-0000-1000-8000-00805f9b34fb").unwrap()],
            })
            .await?;

        let mut already_alerted = HashMap::new();

        loop {
            tokio::select! {
                _ = ctk.cancelled() => {
                    info!("{INNER_NAME}: tracker cancelled, breaking");
                    break;
                }
                Some(e) = events.next() => {
                    match e {
                        CentralEvent::ServiceDataAdvertisement { id, service_data } => {
                            let _ = service_data;
                            let now = SystemTime::now();
                            let when = already_alerted.get(&id);

                            // Don't spam, max once per 60s
                            if let Some(t) = when {
                                if now.duration_since(*t)? <= Duration::from_secs(60) {
                                    continue;
                                }
                            }

                            debug!("{INNER_NAME}: A device is sharing nearby");
                            self.sender.send(())?;
                            already_alerted.insert(id, now);
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
