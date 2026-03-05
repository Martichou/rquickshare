use std::sync::Arc;

use bluer::adv::{Advertisement, AdvertisementHandle};
use bluer::UuidExt;
use bytes::Bytes;
use tokio::sync::watch;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::hdl::mdns::Visibility;

const SERVICE_DATA: Bytes = Bytes::from_static(&[
    252, 18, 142, 1, 66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 191, 45, 91, 160, 225, 216, 117, 36, 202, 0,
]);

const INNER_NAME: &str = "BleAdvertiser";

#[derive(Debug, Clone)]
pub struct BleAdvertiser {
    adapter: Arc<bluer::Adapter>,
    visibility_receiver: watch::Receiver<Visibility>,
}

impl BleAdvertiser {
    pub async fn new(
        visibility_receiver: watch::Receiver<Visibility>,
    ) -> Result<Self, anyhow::Error> {
        let session = bluer::Session::new().await?;
        let adapter = session.default_adapter().await?;
        adapter.set_powered(true).await?;

        Ok(Self {
            adapter: Arc::new(adapter),
            visibility_receiver,
        })
    }

    pub async fn run(mut self, ctk: CancellationToken) -> Result<(), anyhow::Error> {
        info!(
            "{INNER_NAME}: advertising on Bluetooth adapter {} with address {}",
            self.adapter.name(),
            self.adapter.address().await?
        );

        let service_uuid = Uuid::from_u16(0xFE2C);
        let mut handle: Option<AdvertisementHandle> = None;

        // Start advertising if already visible
        let mut visibility = *self.visibility_receiver.borrow();
        if visibility == Visibility::Visible || visibility == Visibility::Temporarily {
            handle = Some(
                self.adapter
                    .advertise(self.get_advertisement(service_uuid, SERVICE_DATA))
                    .await?,
            );
            info!("{INNER_NAME}: started advertising (initial visibility: {visibility:?})");
        }

        loop {
            tokio::select! {
                _ = ctk.cancelled() => {
                    info!("{INNER_NAME}: tracker cancelled, returning");
                    break;
                }
                _ = self.visibility_receiver.changed() => {
                    visibility = *self.visibility_receiver.borrow_and_update();
                    debug!("{INNER_NAME}: visibility changed: {visibility:?}");

                    match visibility {
                        Visibility::Visible | Visibility::Temporarily => {
                            if handle.is_none() {
                                handle = Some(
                                    self.adapter
                                        .advertise(self.get_advertisement(service_uuid, SERVICE_DATA))
                                        .await?,
                                );
                                info!("{INNER_NAME}: started advertising");
                            }
                        }
                        Visibility::Invisible => {
                            if let Some(h) = handle.take() {
                                drop(h);
                                info!("{INNER_NAME}: stopped advertising");
                            }
                        }
                    }
                }
            }
        }

        drop(handle);
        Ok(())
    }

    fn get_advertisement(&self, service_uuid: Uuid, adv_data: Bytes) -> Advertisement {
        Advertisement {
            advertisement_type: bluer::adv::Type::Broadcast,
            service_data: [(service_uuid, adv_data.into())].into(),
            ..Default::default()
        }
    }
}
