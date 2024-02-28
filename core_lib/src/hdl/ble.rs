use std::sync::Arc;
use std::time::{Duration, SystemTime};

use anyhow::anyhow;
use bluer::adv::{Advertisement, Feature, SecondaryChannel};
use btleplug::api::{Central, CentralEvent, Manager as _, ScanFilter};
use btleplug::platform::{Adapter, Manager};
use bytes::Bytes;
use futures::stream::StreamExt;
use tokio::sync::broadcast::Sender;
use tokio_util::sync::CancellationToken;
use uuid::uuid;
use uuid::Uuid;

const SERVICE_UUID_SHARING: Uuid = uuid!("0000fe2c-0000-1000-8000-00805f9b34fb");
const SERVICE_DATA: Bytes = Bytes::from_static(&[
    252, 18, 142, 1, 66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
]);

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

#[derive(Debug, Clone)]
pub struct BleAdvertiser {
    adapter: Arc<bluer::Adapter>,
}

impl BleAdvertiser {
    pub async fn new() -> Result<Self, anyhow::Error> {
        let session = bluer::Session::new().await?;
        let adapter = session.default_adapter().await?;
        adapter.set_powered(true).await?;

        Ok(Self {
            adapter: Arc::new(adapter),
        })
    }

    pub async fn run(&self, ctk: CancellationToken) -> Result<(), anyhow::Error> {
        debug!(
            "BleAdvertiser: advertising on Bluetooth adapter {} with address {}",
            self.adapter.name(),
            self.adapter.address().await?
        );

        let handle = self
            .adapter
            .advertise(self.get_advertisment(SERVICE_UUID_SHARING, SERVICE_DATA))
            .await?;
        ctk.cancelled().await;
        info!("BleAdvertiser: tracker cancelled, returning");
        drop(handle);

        Ok(())
    }

    fn get_advertisment(&self, service_uuid: Uuid, adv_data: Bytes) -> Advertisement {
        Advertisement {
            advertisement_type: bluer::adv::Type::Broadcast,
            service_uuids: vec![service_uuid].into_iter().collect(),
            service_data: [(service_uuid, adv_data.into())].into(),
            secondary_channel: Some(SecondaryChannel::OneM),
            system_includes: [Feature::TxPower].into(),
            tx_power: Some(20),
            ..Default::default()
        }
    }
}
