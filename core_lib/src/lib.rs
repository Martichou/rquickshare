#[macro_use]
extern crate log;

use std::path::PathBuf;
use std::sync::{Arc, Mutex, RwLock};

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

/// Handle to control the RQuickShare service
#[derive(Debug, Clone)]
pub struct RqsHandle {
    /// Sender for file transfer operations
    pub sender: mpsc::Sender<SendInfo>,
    /// Sender for messages to the service
    message_sender: broadcast::Sender<ChannelMessage>,
    /// Sender for visibility changes
    visibility_sender: Arc<Mutex<watch::Sender<Visibility>>>,
    /// Sender for discovery events
    discovery_sender: broadcast::Sender<EndpointInfo>,
    /// Cancellation token for the service
    ctoken: CancellationToken,
    /// Cancellation token for discovery
    discovery_token: CancellationToken,
}

impl RqsHandle {
    /// Start device discovery
    pub fn start_discovery(&self) -> Result<(), anyhow::Error> {
        // Cancel any existing discovery first
        self.discovery_token.cancel();

        // Create a new discovery token
        let discovery_token = CancellationToken::new();

        // Start discovery with the new token
        let discovery = MDnsDiscovery::new(self.discovery_sender.clone())?;

        tokio::spawn(async move {
            if let Err(e) = discovery.run(discovery_token.clone()).await {
                error!("Discovery error: {}", e);
            }
        });

        #[cfg(all(feature = "experimental", target_os = "linux"))]
        {
            let discovery_token = discovery_token.clone();
            tokio::spawn(async move {
                let blea = match BleAdvertiser::new().await {
                    Ok(b) => b,
                    Err(e) => {
                        error!("Couldn't init BleAdvertiser: {}", e);
                        return;
                    }
                };

                if let Err(e) = blea.run(discovery_token).await {
                    error!("Couldn't start BleAdvertiser: {}", e);
                }
            });
        }

        Ok(())
    }

    /// Stop device discovery
    pub fn stop_discovery(&self) {
        self.discovery_token.cancel();
    }

    /// Change visibility setting
    pub fn change_visibility(&self, visibility: Visibility) {
        self.visibility_sender
            .lock()
            .unwrap()
            .send_modify(|state| *state = visibility);
    }

    /// Set custom download path
    pub fn set_download_path(&self, path: Option<PathBuf>) {
        debug!("Setting the download path to {:?}", path);
        let mut guard = CUSTOM_DOWNLOAD.write().unwrap();
        *guard = path;
    }

    /// Send a message to the service
    pub fn send_message(
        &self,
        message: ChannelMessage,
    ) -> Result<(), Box<broadcast::error::SendError<ChannelMessage>>> {
        self.message_sender
            .send(message)
            .map(|_| ())
            .map_err(Box::new)
    }

    /// Shutdown the service
    pub async fn shutdown(self) {
        // Stop discovery first
        self.stop_discovery();

        // Cancel the main service
        self.ctoken.cancel();
    }
}

/// The main RQuickShare service
#[derive(Debug)]
pub struct RQS {
    /// Task tracker for spawned tasks
    tracker: TaskTracker,
    /// Cancellation token for the service
    ctoken: CancellationToken,
    /// Discovery cancellation token
    discovery_token: CancellationToken,
    /// Event sender
    event_sender: broadcast::Sender<RqsEvent>,
    /// Visibility sender
    visibility_sender: Arc<Mutex<watch::Sender<Visibility>>>,
    /// Message sender
    message_sender: broadcast::Sender<ChannelMessage>,
    /// Discovery sender
    discovery_sender: broadcast::Sender<EndpointInfo>,
    /// BLE sender
    ble_sender: broadcast::Sender<()>,
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

        // Create channels
        let (event_sender, _) = broadcast::channel(50);
        let (message_sender, _) = broadcast::channel(50);
        let (discovery_sender, _) = broadcast::channel(10);
        let (ble_sender, _) = broadcast::channel(5);

        // Create visibility channel with initial value
        let (visibility_sender, _visibility_receiver) = watch::channel(Visibility::Invisible);
        let _ = visibility_sender.send(config.visibility);

        // Create task tracker and cancellation tokens
        let tracker = TaskTracker::new();
        let ctoken = CancellationToken::new();
        let discovery_token = CancellationToken::new();

        Self {
            tracker,
            ctoken,
            discovery_token,
            event_sender,
            visibility_sender: Arc::new(Mutex::new(visibility_sender)),
            message_sender,
            discovery_sender,
            ble_sender,
        }
    }

    /// Start the RQuickShare service
    pub async fn start(&mut self, config: &RqsConfig) -> Result<RqsHandle, anyhow::Error> {
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

        // Create MPSC channel for the TcpServer
        let (sender, receiver) = mpsc::channel(10);

        // Create TcpServer
        let mut server = TcpServer::new(
            endpoint_id[..4].try_into()?,
            tcp_listener,
            self.message_sender.clone(),
            receiver,
        )?;

        // Spawn TcpServer task
        let ctoken = self.ctoken.clone();
        self.tracker.spawn(async move {
            if let Err(e) = server.run(ctoken).await {
                error!("TcpServer error: {}", e);
            }
        });

        // Spawn BLE listener task if experimental feature is enabled
        #[cfg(feature = "experimental")]
        {
            if let Ok(ble) = BleListener::new(self.ble_sender.clone()).await {
                let ctoken = self.ctoken.clone();
                self.tracker.spawn(async move {
                    if let Err(e) = ble.run(ctoken).await {
                        error!("BleListener error: {}", e);
                    }
                });
            }
        }

        // Create MDnsServer
        let mut mdns = MDnsServer::new(
            endpoint_id[..4].try_into()?,
            binded_addr.port(),
            self.ble_sender.subscribe(),
            self.visibility_sender.clone(),
            self.visibility_sender.lock().unwrap().subscribe(),
        )?;

        // Spawn MDnsServer task
        let ctoken = self.ctoken.clone();
        self.tracker.spawn(async move {
            if let Err(e) = mdns.run(ctoken).await {
                error!("MDnsServer error: {}", e);
            }
        });

        // Set up event forwarding
        self.setup_event_forwarding();

        // Create and return handle
        Ok(RqsHandle {
            sender,
            message_sender: self.message_sender.clone(),
            visibility_sender: self.visibility_sender.clone(),
            discovery_sender: self.discovery_sender.clone(),
            ctoken: self.ctoken.clone(),
            discovery_token: self.discovery_token.clone(),
        })
    }

    /// Subscribe to events from the service
    pub fn subscribe(&self) -> broadcast::Receiver<RqsEvent> {
        self.event_sender.subscribe()
    }

    /// Set up forwarding of events from various channels to the unified event channel
    fn setup_event_forwarding(&self) {
        // Forward message events
        let message_sender = self.message_sender.clone();
        let event_sender = self.event_sender.clone();
        self.tracker.spawn(async move {
            let mut receiver = message_sender.subscribe();
            while let Ok(message) = receiver.recv().await {
                let _ = event_sender.send(RqsEvent::Message(message));
            }
        });

        // Forward discovery events
        let discovery_sender = self.discovery_sender.clone();
        let event_sender = self.event_sender.clone();
        self.tracker.spawn(async move {
            let mut receiver = discovery_sender.subscribe();
            while let Ok(endpoint) = receiver.recv().await {
                let _ = event_sender.send(RqsEvent::DeviceDiscovered(endpoint));
            }
        });

        // Forward visibility events
        let visibility_sender = self.visibility_sender.clone();
        let event_sender = self.event_sender.clone();
        self.tracker.spawn(async move {
            let mut receiver = visibility_sender.lock().unwrap().subscribe();
            while (receiver.changed().await).is_ok() {
                let visibility = *receiver.borrow();
                let _ = event_sender.send(RqsEvent::VisibilityChanged(visibility));
            }
        });

        // Forward BLE events
        let ble_sender = self.ble_sender.clone();
        let event_sender = self.event_sender.clone();
        self.tracker.spawn(async move {
            let mut receiver = ble_sender.subscribe();
            while (receiver.recv().await).is_ok() {
                let _ = event_sender.send(RqsEvent::NearbyDeviceSharing);
            }
        });
    }

    /// Shutdown the service
    pub async fn shutdown(&self) {
        // Cancel discovery token
        self.discovery_token.cancel();

        // Cancel main token
        self.ctoken.cancel();

        // Wait for all tasks to complete
        self.tracker.wait().await;
    }
}
