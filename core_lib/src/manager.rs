use serde::{Deserialize, Serialize};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{self, Sender};
use tokio::sync::mpsc::Receiver;
use tokio_util::sync::CancellationToken;
use ts_rs::TS;

use crate::channel::{ChannelDirection, ChannelMessage};
use crate::errors::AppError;
use crate::hdl::{InboundRequest, OutboundPayload, OutboundRequest, State};
use crate::utils::RemoteDeviceInfo;
use crate::RqsEvent;

const INNER_NAME: &str = "TcpServer";

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct SendInfo {
    pub id: String,
    pub name: String,
    pub addr: String,
    pub ob: OutboundPayload,
}

pub struct TcpServer {
    endpoint_id: [u8; 4],
    tcp_listener: TcpListener,
    sender: Sender<RqsEvent>,
    connect_receiver: Receiver<SendInfo>,
}

impl TcpServer {
    pub fn new(
        endpoint_id: [u8; 4],
        tcp_listener: TcpListener,
        sender: Sender<RqsEvent>,
        connect_receiver: Receiver<SendInfo>,
    ) -> Result<Self, anyhow::Error> {
        Ok(Self {
            endpoint_id,
            tcp_listener,
            sender,
            connect_receiver,
        })
    }

    pub async fn run(&mut self, ctk: CancellationToken) -> Result<(), anyhow::Error> {
        info!("{INNER_NAME}: service starting");

        loop {
            let cctk = ctk.clone();

            tokio::select! {
                _ = ctk.cancelled() => {
                    info!("{INNER_NAME}: tracker cancelled, breaking");
                    break;
                }
                Some(i) = self.connect_receiver.recv() => {
                    info!("{INNER_NAME}: connect_receiver: got {:?}", i);
                    if let Err(e) = self.connect(cctk, i).await {
                        error!("{INNER_NAME}: error sending: {}", e.to_string());
                    }
                }
                r = self.tcp_listener.accept() => {
                    match r {
                        Ok((socket, remote_addr)) => {
                            trace!("{INNER_NAME}: new client: {remote_addr}");
                            let esender = self.sender.clone();
                            let csender = self.sender.clone();

                            tokio::spawn(async move {
                                let mut ir = InboundRequest::new(socket, remote_addr.to_string(), csender);

                                loop {
                                    match ir.handle().await {
                                        Ok(_) => {},
                                        Err(e) => match e.downcast_ref() {
                                            Some(AppError::NotAnError) => break,
                                            None => {
                                                if ir.state.state == State::Initial {
                                                    break;
                                                }

                                                if ir.state.state != State::Finished {
                                                    let _ = esender.send(RqsEvent::Message(ChannelMessage {
                                                        id: remote_addr.to_string(),
                                                        direction: ChannelDirection::LibToFront,
                                                        state: Some(State::Disconnected),
                                                        ..Default::default()
                                                    }));
                                                }
                                                error!("{INNER_NAME}: error while handling client: {e} ({:?})", ir.state.state);
                                                break;
                                            }
                                        },
                                    }
                                }
                            });
                        },
                        Err(err) => {
                            error!("{INNER_NAME}: error accepting: {}", err);
                        }
                    }
                }
            }
        }

        info!("{INNER_NAME}: service stopped");
        Ok(())
    }

    /// To be called inside a separate task if we want to handle concurrency
    pub async fn connect(&self, ctk: CancellationToken, si: SendInfo) -> Result<(), anyhow::Error> {
        info!("{INNER_NAME}: connecting to {}", si.addr);

        let socket = match TcpStream::connect(&si.addr).await {
            Ok(s) => s,
            Err(e) => {
                error!("{INNER_NAME}: couldn't connect to {}: {}", si.addr, e);
                return Err(e.into());
            }
        };

        // Create a dedicated channel for this connection
        let (channel_sender, _) = broadcast::channel::<ChannelMessage>(10);

        // Forward messages from the unified event channel to this connection's channel
        let event_sender = self.sender.clone();
        let id = si.id.clone();
        let channel_sender_clone = channel_sender.clone();

        tokio::spawn(async move {
            let mut event_receiver = event_sender.subscribe();
            while let Ok(event) = event_receiver.recv().await {
                if let RqsEvent::Message(msg) = event {
                    if msg.id == id {
                        let _ = channel_sender_clone.send(msg);
                    }
                }
            }
        });

        let mut or = OutboundRequest::new(
            self.endpoint_id,
            socket,
            si.id.clone(),
            channel_sender,
            si.ob,
            RemoteDeviceInfo {
                device_type: crate::DeviceType::Unknown,
                name: si.name,
            },
        );

        let cctk = ctk.clone();
        let event_sender = self.sender.clone();
        let addr = si.addr.clone();

        tokio::spawn(async move {
            // Instead of calling run, use the individual methods
            if let Err(e) = or.send_connection_request().await {
                handle_error(&or, &event_sender, &addr, e);
                return;
            }

            if let Err(e) = or.send_ukey2_client_init().await {
                handle_error(&or, &event_sender, &addr, e);
                return;
            }

            // Main loop to handle the connection
            loop {
                if cctk.is_cancelled() {
                    break;
                }

                match or.handle().await {
                    Ok(_) => {}
                    Err(e) => match e.downcast_ref() {
                        Some(AppError::NotAnError) => break,
                        None => {
                            handle_error(&or, &event_sender, &addr, e);
                            break;
                        }
                    },
                }
            }
        });

        Ok(())
    }
}

// Helper function to handle errors
fn handle_error(
    or: &OutboundRequest,
    event_sender: &Sender<RqsEvent>,
    addr: &str,
    e: anyhow::Error,
) {
    if or.state.state != State::Finished && or.state.state != State::Cancelled {
        let _ = event_sender.send(RqsEvent::Message(ChannelMessage {
            id: addr.to_string(),
            direction: ChannelDirection::LibToFront,
            state: Some(State::Disconnected),
            ..Default::default()
        }));
    }
    error!(
        "{INNER_NAME}: error while handling client: {e} ({:?})",
        or.state.state
    );
}
