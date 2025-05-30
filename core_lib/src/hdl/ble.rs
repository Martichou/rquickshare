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

        // Not using the ScanFilter here to filter out advertisements
        // not matching the Nearby Share service UUID, it seems to
        // exclude Nearby Share advertisements despite its UUID being
        // in the filter.
        //
        // Perhaps broken?
        self.adapter.start_scan(ScanFilter::default()).await?;

        let mut last_alert: SystemTime = SystemTime::UNIX_EPOCH;

        loop {
            tokio::select! {
                _ = ctk.cancelled() => {
                    info!("{INNER_NAME}: tracker cancelled, breaking");
                    break;
                }
                Some(e) = events.next() => {
                    match e {
                        CentralEvent::ServiceDataAdvertisement { id, service_data } => {
                            // Sanity check as per: https://github.com/Martichou/rquickshare/issues/74
                            // Seems like the filtering is not enough, so we'll add a check before
                            // proceeding with the service_data.
                            //
                            // ...The filtering is being done only here now.
                            if let Some(service_data) = service_data.get(&SERVICE_UUID_SHARING) {
                                let now = SystemTime::now();
                                // Quick Share seems to emit LE advert every 10 seconds...
                                // Doesn't really seem much of a spam, so we wait for 10s now
                                // just in case some implementation is sending it under that
                                // time period
                                if now.duration_since(last_alert)? <= Duration::from_secs(10) {
                                    // debug!("{INNER_NAME}: Received LE advert but last alert was {}s ago", now.duration_since(last_alert)?.as_secs());
                                    continue;
                                }

                                debug!("{INNER_NAME}: A device ({id}) is sharing ({}) nearby", hex::encode(service_data));
                                self.sender.send(())?;
                                last_alert = now;
                            }
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
