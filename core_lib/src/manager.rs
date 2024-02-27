use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::Sender;
use tokio::sync::mpsc::Receiver;
use tokio_util::sync::CancellationToken;

use crate::channel::{ChannelDirection, ChannelMessage};
use crate::errors::AppError;
use crate::hdl::{InboundRequest, OutboundPayload, OutboundRequest, State};

const INNER_NAME: &str = "TcpServer";

pub struct SendInfo {
    addr: String,
    ob: OutboundPayload,
}

pub struct TcpServer {
    endpoint_id: [u8; 4],
    tcp_listener: TcpListener,
    sender: Sender<ChannelMessage>,
    connect_receiver: Receiver<SendInfo>,
}

impl TcpServer {
    pub fn new(
        endpoint_id: [u8; 4],
        tcp_listener: TcpListener,
        sender: Sender<ChannelMessage>,
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
                    if let Err(e) = self.connect(cctk, i.addr, i.ob).await {
                        error!("{INNER_NAME}: error sending: {:?}", e);
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
                                                let _ = esender.send(ChannelMessage {
                                                    id: remote_addr.to_string(),
                                                    direction: ChannelDirection::LibToFront,
                                                    state: Some(State::Disconnected),
                                                    ..Default::default()
                                                });
                                                error!("{INNER_NAME}: error while handling client: {e}");
                                                break;
                                            }
                                        },
                                    }
                                }
                            });
                        },
                        Err(err) => {
                            error!("{INNER_NAME}: error accepting: {}", err);
                            break;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// To be called inside a separate task if we want to handle concurrency
    pub async fn connect(
        &self,
        ctk: CancellationToken,
        addr: String,
        ob: OutboundPayload,
    ) -> Result<(), anyhow::Error> {
        let socket = TcpStream::connect(addr.clone()).await?;
        let mut or = OutboundRequest::new(
            self.endpoint_id,
            socket,
            addr.clone(),
            self.sender.clone(),
            ob,
        );

        // Send connection request
        or.send_connection_request().await?;
        // Send UKEY init
        or.send_ukey2_client_init().await?;

        loop {
            tokio::select! {
                _ = ctk.cancelled() => {
                    info!("{INNER_NAME}: tracker cancelled, breaking");
                    break;
                },
                r = or.handle() => {
                    if let Err(e) = r {
                        match e.downcast_ref() {
                            Some(AppError::NotAnError) => break,
                            None => {
                                let _ = self.sender.clone().send(ChannelMessage {
                                    id: addr,
                                    direction: ChannelDirection::LibToFront,
                                    state: Some(State::Disconnected),
                                    ..Default::default()
                                });
                                error!("{INNER_NAME}: error while handling client: {e}");
                                break;
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
