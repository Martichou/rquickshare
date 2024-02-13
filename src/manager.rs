use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use prost::Message;
use tokio::net::{TcpListener, TcpStream};
use tokio_util::sync::CancellationToken;

use crate::location_nearby_connections::OfflineFrame;
use crate::utils::stream_read_exact;

const INNER_NAME: &str = "TcpServer";

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum State {
    Initial,
    ReceivedConnectionRequest,
    SentUkeyServerInit,
    ReceivedUkeyClientFinish,
    SentConnectionResponse,
    SentPairedKeyResult,
    ReceivedPairedKeyResult,
    WaitingForUserConsent,
    ReceivingFiles,
    Disconnected,
}

type ArcState = Arc<Mutex<HashMap<SocketAddr, State>>>;

pub struct TcpServer {
    tcp_listener: TcpListener,
    states: ArcState,
}

impl TcpServer {
    pub fn new(tcp_listener: TcpListener) -> Result<Self, anyhow::Error> {
        Ok(Self {
            tcp_listener,
            states: Arc::default(),
        })
    }

    pub async fn run(&self, ctk: CancellationToken) -> Result<(), anyhow::Error> {
        info!("{INNER_NAME}: service starting");

        loop {
            let cctk = ctk.clone();
            let states = self.states.clone();

            tokio::select! {
                _ = ctk.cancelled() => {
                    info!("{INNER_NAME}: tracker cancelled, breaking");
                    break;
                }
                r = self.tcp_listener.accept() => {
                    match r {
                        Ok((socket, remote_addr)) => {
                            trace!("New client: {remote_addr}");

                            tokio::spawn(async move {
                                let copied_state;
                                { // Ensure the lock guard is dropped pretty fast
                                    let mut states_lock = states.lock().unwrap();
                                    copied_state = states_lock.entry(remote_addr).or_insert(State::Initial).clone();
                                }

                                // info!("State is: {:?}", state);
                                let r = Self::handle_new_client(socket, cctk, copied_state).await;
                                match r {
                                    Ok(new_state) => {
                                        let mut states_lock = states.lock().unwrap();
                                        states_lock.entry(remote_addr).and_modify(|e| { *e = new_state });
                                    }
                                    Err(e) => {
                                        error!("Error while handling client: {e}");
                                        let mut states_lock = states.lock().unwrap();
                                        states_lock.remove(&remote_addr);
                                    }
                                }
                            });
                        },
                        Err(err) => {
                            error!("TcpListener: error accepting: {}", err);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn handle_new_client(
        mut socket: TcpStream,
        _ctk: CancellationToken,
        _state: State,
    ) -> Result<State, anyhow::Error> {
        // Buffer for the 4-byte length
        let mut length_buf = [0u8; 4];
        stream_read_exact(&mut socket, &mut length_buf).await?;

        let msg_length = u32::from_be_bytes(length_buf) as usize;
        // Ensure the message length is not unreasonably big to avoid allocation attacks
        if msg_length > 5 * 1024 * 1024 {
            error!("Message length too big");
            return Err(anyhow::Error::msg("value"));
        }

        // Allocate buffer for the actual message and read it
        let mut msg_buf = vec![0u8; msg_length];
        stream_read_exact(&mut socket, &mut msg_buf).await?;

        info!("Received message of length: {}", msg_length);
        let offline_frame = OfflineFrame::decode(&*msg_buf);
        let offline_frame = offline_frame.unwrap();

        match offline_frame.version() {
            crate::location_nearby_connections::offline_frame::Version::UnknownVersion => {
                todo!()
            }
            crate::location_nearby_connections::offline_frame::Version::V1 => {
                let v1_frame = offline_frame.v1.unwrap();
                match v1_frame.r#type() {
					crate::location_nearby_connections::v1_frame::FrameType::UnknownFrameType => todo!(),
					crate::location_nearby_connections::v1_frame::FrameType::ConnectionRequest => todo!(),
					crate::location_nearby_connections::v1_frame::FrameType::ConnectionResponse => todo!(),
					crate::location_nearby_connections::v1_frame::FrameType::PayloadTransfer => todo!(),
					crate::location_nearby_connections::v1_frame::FrameType::BandwidthUpgradeNegotiation => todo!(),
					crate::location_nearby_connections::v1_frame::FrameType::KeepAlive => todo!(),
					crate::location_nearby_connections::v1_frame::FrameType::Disconnection => todo!(),
					crate::location_nearby_connections::v1_frame::FrameType::PairedKeyEncryption => todo!(),
				}
            }
        }
    }
}
