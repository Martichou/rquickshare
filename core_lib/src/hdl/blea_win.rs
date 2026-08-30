use tokio_util::sync::CancellationToken;

const INNER_NAME: &str = "BleAdvertiser";

/// Windows stub implementation of the BLE advertiser.
///
/// This is a placeholder that returns an error indicating Windows BLE advertising
/// is not yet implemented. The full implementation will use WinRT's
/// `BluetoothLEAdvertisementPublisher` API to broadcast 0xFE2C service data.
///
/// See: deferred track `windows-impl_20260406` — Issue: "Implement BLE advertiser for automatic Android discovery"
#[derive(Debug, Clone)]
pub struct BleAdvertiser;

impl BleAdvertiser {
    pub async fn new() -> Result<Self, anyhow::Error> {
        Err(anyhow::anyhow!(
            "Windows BLE advertiser is not yet implemented. \
             Android devices will require manual receive UI activation."
        ))
    }

    pub async fn run(&self, ctk: CancellationToken) -> Result<(), anyhow::Error> {
        warn!("{INNER_NAME}: Windows BLE advertiser not implemented, waiting for cancellation");
        ctk.cancelled().await;
        info!("{INNER_NAME}: tracker cancelled, returning");
        Ok(())
    }
}
