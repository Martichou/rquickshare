use std::pin::Pin;
use std::time::{Duration, SystemTime};

use anyhow::anyhow;
use btleplug::api::{Central, CentralEvent, Manager as _, ScanFilter};
use btleplug::platform::{Adapter, Manager};
use futures::stream::StreamExt;
use tokio::sync::broadcast::Sender;
use tokio::sync::watch;
use tokio::time::{sleep, Instant, Sleep};
use tokio_util::sync::CancellationToken;
use uuid::{uuid, Uuid};

const SERVICE_UUID_SHARING: Uuid = uuid!("0000fe2c-0000-1000-8000-00805f9b34fb");

const INNER_NAME: &str = "BleListener";

// Duty-cycle timings used when the app is backgrounded (in tray) on Linux.
// We scan for a short window, then release the adapter for a longer gap so
// BlueZ isn't held in continuous discovery for the whole app lifetime.
// See: https://github.com/Martichou/rquickshare/issues/429
const DUTY_CYCLE_SCAN: Duration = Duration::from_secs(5);
const DUTY_CYCLE_IDLE: Duration = Duration::from_secs(25);

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

    /// Whether we should duty-cycle scanning rather than scan continuously.
    ///
    /// Continuous BlueZ discovery monopolizes the adapter on Linux, so we only
    /// duty-cycle there, and only while the app is backgrounded (in tray).
    /// On other platforms the OS multiplexes scanning, so we keep the original
    /// continuous behaviour regardless of foreground state.
    fn should_duty_cycle(foreground: bool) -> bool {
        cfg!(target_os = "linux") && !foreground
    }

    async fn start_scan(&self, scanning: &mut bool) -> Result<(), anyhow::Error> {
        if !*scanning {
            // Filter on the NearbyShare/QuickShare services UUID
            self.adapter
                .start_scan(ScanFilter {
                    services: vec![SERVICE_UUID_SHARING],
                })
                .await?;
            *scanning = true;
            debug!("{INNER_NAME}: scan started");
        }

        Ok(())
    }

    async fn stop_scan(&self, scanning: &mut bool) -> Result<(), anyhow::Error> {
        if *scanning {
            self.adapter.stop_scan().await?;
            *scanning = false;
            debug!("{INNER_NAME}: scan stopped");
        }

        Ok(())
    }

    /// Apply the scan mode implied by the current foreground state, (re)arming
    /// the duty-cycle timer when needed.
    async fn apply_mode(
        &self,
        scanning: &mut bool,
        foreground: bool,
        mut duty_timer: Pin<&mut Sleep>,
    ) -> Result<(), anyhow::Error> {
        // Both modes begin by scanning; the difference is whether we let the
        // duty-cycle timer subsequently stop the scan during idle gaps.
        self.start_scan(scanning).await?;

        if Self::should_duty_cycle(foreground) {
            duty_timer.as_mut().reset(Instant::now() + DUTY_CYCLE_SCAN);
        }

        Ok(())
    }

    pub async fn run(
        self,
        ctk: CancellationToken,
        mut foreground_rx: watch::Receiver<bool>,
    ) -> Result<(), anyhow::Error> {
        info!("{INNER_NAME}: service starting");

        let mut events = self.adapter.events().await?;
        let mut last_alert = SystemTime::UNIX_EPOCH;

        // Tracks whether a BlueZ discovery session is currently open.
        let mut scanning = false;
        // Current app foreground state (true = main window visible).
        let mut foreground = *foreground_rx.borrow_and_update();

        // Timer driving the duty-cycle while backgrounded on Linux. Its select!
        // branch is gated on `should_duty_cycle()`, so it is never polled while
        // we are scanning continuously.
        let duty_timer = sleep(Duration::from_secs(0));
        tokio::pin!(duty_timer);

        // Apply the initial scan mode.
        self.apply_mode(&mut scanning, foreground, duty_timer.as_mut())
            .await?;

        loop {
            let duty = Self::should_duty_cycle(foreground);

            tokio::select! {
                _ = ctk.cancelled() => {
                    info!("{INNER_NAME}: tracker cancelled, breaking");
                    break;
                }
                changed = foreground_rx.changed() => {
                    if changed.is_err() {
                        // Sender dropped (app shutting down). Stop reacting to
                        // foreground changes and just wait for cancellation to
                        // avoid busy-looping on the closed channel.
                        debug!("{INNER_NAME}: foreground sender dropped, parking until cancel");
                        ctk.cancelled().await;
                        break;
                    }

                    foreground = *foreground_rx.borrow_and_update();
                    debug!("{INNER_NAME}: foreground changed: {foreground}");
                    self.apply_mode(&mut scanning, foreground, duty_timer.as_mut()).await?;
                }
                _ = &mut duty_timer, if duty => {
                    // Toggle the scan window: scan for DUTY_CYCLE_SCAN, then idle
                    // for DUTY_CYCLE_IDLE, letting BlueZ free the adapter between.
                    if scanning {
                        self.stop_scan(&mut scanning).await?;
                        duty_timer.as_mut().reset(Instant::now() + DUTY_CYCLE_IDLE);
                    } else {
                        self.start_scan(&mut scanning).await?;
                        duty_timer.as_mut().reset(Instant::now() + DUTY_CYCLE_SCAN);
                    }
                }
                Some(e) = events.next() => {
                    // Ignore any event delivered during an idle gap.
                    if !scanning {
                        continue;
                    }

                    match e {
                        CentralEvent::ServiceDataAdvertisement { id, service_data } => {
                            // Sanity check as per: https://github.com/Martichou/rquickshare/issues/74
                            // Seems like the filtering is not enough, so we'll add a check before
                            // proceeding with the service_data.
                            if !service_data.contains_key(&SERVICE_UUID_SHARING) {
                                continue;
                            }

                            let now = SystemTime::now();
                            // Don't spam, max once per 30s
                            if now.duration_since(last_alert)? <= Duration::from_secs(30) {
                                continue;
                            }

                            debug!("{INNER_NAME}: A device ({id}) is sharing ({service_data:?}) nearby");
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

        // Release the adapter on shutdown.
        if scanning {
            let _ = self.adapter.stop_scan().await;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_duty_cycle_only_when_backgrounded_on_linux() {
        // Foreground (window visible): always scan continuously, every platform.
        assert!(!BleListener::should_duty_cycle(true));

        // Backgrounded (in tray): duty-cycle only on Linux/BlueZ, where
        // continuous discovery monopolizes the adapter. Other platforms keep
        // scanning continuously because the OS multiplexes scanning.
        assert_eq!(
            BleListener::should_duty_cycle(false),
            cfg!(target_os = "linux")
        );
    }
}
