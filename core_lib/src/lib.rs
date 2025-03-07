#[macro_use]
extern crate log;

use std::cell::RefCell;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, RwLock};

use anyhow::{anyhow, Result};
use channel::ChannelMessage;
#[cfg(all(feature = "experimental", target_os = "linux"))]
use hdl::BleAdvertiser;
use hdl::MDnsDiscovery;
use once_cell::sync::Lazy;
use rand::distr::Alphanumeric;
use rand::Rng;
use tokio::net::TcpListener;
use tokio::sync::{broadcast, mpsc, watch};
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;

#[cfg(feature = "experimental")]
use crate::hdl::BleListener;
use crate::hdl::MDnsServer;
use crate::manager::TcpServer;

pub mod channel;
mod errors;
mod hdl;
mod manager;
mod utils;

pub use hdl::{EndpointInfo, OutboundPayload, State, Visibility};
pub use manager::SendInfo;
pub use utils::DeviceType;

pub mod sharing_nearby {
    include!(concat!(env!("OUT_DIR"), "/sharing.nearby.rs"));
}

pub mod securemessage {
    include!(concat!(env!("OUT_DIR"), "/securemessage.rs"));
}

pub mod securegcm {
    include!(concat!(env!("OUT_DIR"), "/securegcm.rs"));
}

pub mod location_nearby_connections {
    include!(concat!(env!("OUT_DIR"), "/location.nearby.connections.rs"));
}

static CUSTOM_DOWNLOAD: Lazy<RwLock<Option<PathBuf>>> = Lazy::new(|| RwLock::new(None));

// Thread-local storage for the discovery token
thread_local! {
    static DISCOVERY_TOKEN: RefCell<Option<Arc<CancellationToken>>> = RefCell::new(None);
}

/// Configuration for the RQuickShare service
#[derive(Debug, Clone)]
pub struct RqsConfig {
    /// Initial visibility setting
    pub visibility: Visibility,
    /// Optional port number to bind to (uses random port if None)
    pub port_number: Option<u32>,
    /// Optional custom download path
    pub download_path: Option<PathBuf>,
}

impl Default for RqsConfig {
    fn default() -> Self {
        Self {
            visibility: Visibility::Visible,
            port_number: None,
            download_path: None,
        }
    }
}

/// Event emitted by the RQuickShare service
#[derive(Debug, Clone)]
pub enum RqsEvent {
    /// A message related to a file transfer
    Message(ChannelMessage),
    /// A nearby device was discovered
    DeviceDiscovered(EndpointInfo),
    /// Visibility state changed
    VisibilityChanged(Visibility),
    /// A nearby device is sharing (BLE notification)
    NearbyDeviceSharing,
}

/// The main RQuickShare service
#[derive(Debug)]
pub struct RQS {
    /// Task tracker for spawned tasks
    tracker: TaskTracker,
    /// Main cancellation token for the service
    ctoken: CancellationToken,
    /// Unified event sender for all events
    event_sender: broadcast::Sender<RqsEvent>,
    /// Visibility sender (using watch channel for state synchronization)
    visibility_sender: Arc<Mutex<watch::Sender<Visibility>>>,
}

impl Default for RQS {
    fn default() -> Self {
        Self::new(RqsConfig::default())
    }
}

impl RQS {
    /// Create a new RQuickShare service with the given configuration
    pub fn new(config: RqsConfig) -> Self {
        // Set custom download path if provided
        if let Some(path) = &config.download_path {
            let mut guard = CUSTOM_DOWNLOAD.write().unwrap();
            *guard = Some(path.clone());
        }

        // Create unified event channel
        let (event_sender, _) = broadcast::channel(50);

        // Create visibility channel with initial value
        let (visibility_sender, _) = watch::channel(Visibility::Invisible);
        let _ = visibility_sender.send(config.visibility);

        // Create task tracker and cancellation token
        let tracker = TaskTracker::new();
        let ctoken = CancellationToken::new();

        Self {
            tracker,
            ctoken,
            event_sender,
            visibility_sender: Arc::new(Mutex::new(visibility_sender)),
        }
    }

    /// Start the RQuickShare service
    pub async fn start(&self, config: &RqsConfig) -> Result<RqsHandle> {
        // Generate random endpoint ID
        let endpoint_id: Vec<u8> = rand::rng()
            .sample_iter(Alphanumeric)
            .take(4)
            .map(u8::from)
            .collect();

        // Create TCP listener
        let tcp_listener =
            TcpListener::bind(format!("0.0.0.0:{}", config.port_number.unwrap_or(0))).await?;

        let binded_addr = tcp_listener.local_addr()?;
        info!("TcpListener on: {}", binded_addr);

        // Create channel for file transfers
        let (tx, rx) = mpsc::channel(10);

        // Create TcpServer
        let mut server = TcpServer::new(
            endpoint_id[..4].try_into()?,
            tcp_listener,
            self.event_sender.clone(),
            rx,
        )?;

        // Spawn TcpServer task
        let server_token = self.ctoken.child_token();
        self.tracker.spawn(async move {
            if let Err(e) = server.run(server_token).await {
                error!("TcpServer error: {}", e);
            }
        });

        // Spawn BLE listener task if experimental feature is enabled
        #[cfg(feature = "experimental")]
        {
            // Create a dedicated channel for BLE notifications
            let (ble_sender, _) = broadcast::channel::<()>(5);

            // Forward BLE events to the unified event channel
            let event_sender = self.event_sender.clone();
            let ble_receiver = ble_sender.subscribe();
            self.tracker.spawn(async move {
                let mut receiver = ble_receiver;
                while (receiver.recv().await).is_ok() {
                    let _ = event_sender.send(RqsEvent::NearbyDeviceSharing);
                }
            });

            // Use the dedicated channel for BLE listener
            if let Ok(ble) = BleListener::new(ble_sender).await {
                let ble_token = self.ctoken.child_token();
                self.tracker.spawn(async move {
                    if let Err(e) = ble.run(ble_token).await {
                        error!("BleListener error: {}", e);
                    }
                });
            }
        }

        // Create MDnsServer
        let mut mdns = MDnsServer::new(
            endpoint_id[..4].try_into()?,
            binded_addr.port(),
            self.event_sender.subscribe(),
            self.visibility_sender.clone(),
            self.visibility_sender.lock().unwrap().subscribe(),
        )?;

        // Spawn MDnsServer task
        let mdns_token = self.ctoken.child_token();
        self.tracker.spawn(async move {
            if let Err(e) = mdns.run(mdns_token).await {
                error!("MDnsServer error: {}", e);
            }
        });

        // Create and return handle
        Ok(RqsHandle {
            sender: tx,
            event_sender: self.event_sender.clone(),
            visibility_sender: self.visibility_sender.clone(),
            ctoken: self.ctoken.clone(),
        })
    }

    /// Subscribe to events from the service
    pub fn subscribe(&self) -> broadcast::Receiver<RqsEvent> {
        self.event_sender.subscribe()
    }

    /// Shutdown the service
    pub async fn shutdown(&self) {
        // Cancel the main token (which will cancel all child tokens)
        self.ctoken.cancel();

        // Wait for all tasks to complete
        self.tracker.wait().await;
    }
}

/// Handle to control the RQuickShare service
#[derive(Debug, Clone)]
pub struct RqsHandle {
    /// Sender for file transfer operations
    pub sender: mpsc::Sender<SendInfo>,
    /// Unified event sender
    event_sender: broadcast::Sender<RqsEvent>,
    /// Sender for visibility changes
    visibility_sender: Arc<Mutex<watch::Sender<Visibility>>>,
    /// Cancellation token for the service
    ctoken: CancellationToken,
}

impl RqsHandle {
    /// Start device discovery
    pub fn start_discovery(&self) -> Result<()> {
        // Create a new discovery token and store it in a shared location
        let discovery_token = Arc::new(self.ctoken.child_token());

        // Store the token in thread-local storage or another accessible location
        DISCOVERY_TOKEN.with(|cell| {
            *cell.borrow_mut() = Some(discovery_token.clone());
        });

        // Start MDnsDiscovery with the token
        let discovery = MDnsDiscovery::new(self.event_sender.clone())?;
        let discovery_token_clone = discovery_token.clone();

        tokio::spawn(async move {
            if let Err(e) = discovery.run((*discovery_token_clone).clone()).await {
                error!("Discovery error: {}", e);
            }
        });

        // For Linux with experimental features, start BLE advertiser with the same token
        #[cfg(all(feature = "experimental", target_os = "linux"))]
        {
            let adv_token = discovery_token.clone();

            tokio::spawn(async move {
                let blea = match BleAdvertiser::new().await {
                    Ok(b) => b,
                    Err(e) => {
                        error!("Couldn't init BleAdvertiser: {}", e);
                        return;
                    }
                };

                if let Err(e) = blea.run((*adv_token).clone()).await {
                    error!("Couldn't start BleAdvertiser: {}", e);
                }
            });
        }

        Ok(())
    }

    /// Stop discovery by cancelling the discovery token
    pub fn stop_discovery(&self) {
        // Retrieve and cancel the discovery token
        DISCOVERY_TOKEN.with(|cell| {
            if let Some(token) = cell.borrow().as_ref() {
                token.cancel();
                debug!("Discovery cancelled");
            } else {
                debug!("No discovery token to cancel");
            }
            *cell.borrow_mut() = None;
        });
    }

    /// Change visibility setting
    pub fn change_visibility(&self, visibility: Visibility) -> Result<()> {
        // Update the visibility state
        self.visibility_sender
            .lock()
            .map_err(|e| anyhow!("Failed to lock visibility sender: {}", e))?
            .send_modify(|state| *state = visibility);

        // Also send an event through the unified channel
        self.event_sender
            .send(RqsEvent::VisibilityChanged(visibility))
            .map_err(|e| anyhow!("Failed to send visibility event: {}", e))?;

        Ok(())
    }

    /// Set custom download path
    pub fn set_download_path(&self, path: Option<PathBuf>) -> Result<()> {
        debug!("Setting the download path to {:?}", path);
        let mut guard = CUSTOM_DOWNLOAD
            .write()
            .map_err(|e| anyhow!("Failed to acquire write lock: {}", e))?;
        *guard = path;
        Ok(())
    }

    /// Send a message to the service
    pub fn send_message(&self, message: ChannelMessage) -> Result<()> {
        self.event_sender
            .send(RqsEvent::Message(message))
            .map(|_| ())
            .map_err(|e| anyhow!("Failed to send message: {}", e))
    }

    /// Subscribe to events from the service
    pub fn subscribe(&self) -> broadcast::Receiver<RqsEvent> {
        self.event_sender.subscribe()
    }

    /// Shutdown the service
    pub async fn shutdown(self) -> Result<()> {
        // Stop discovery first
        self.stop_discovery();
        // Cancel the main token
        self.ctoken.cancel();
        Ok(())
    }
}
