use tokio::net::TcpListener;
use tokio::sync::broadcast::Sender;
use tokio_util::sync::CancellationToken;

use crate::channel::ChannelMessage;
use crate::errors::AppError;
use crate::hdl::InboundRequest;

const INNER_NAME: &str = "TcpServer";

pub struct TcpServer {
    tcp_listener: TcpListener,
    sender: Sender<ChannelMessage>,
}

impl TcpServer {
    pub fn new(
        tcp_listener: TcpListener,
        sender: Sender<ChannelMessage>,
    ) -> Result<Self, anyhow::Error> {
        Ok(Self {
            tcp_listener,
            sender,
        })
    }

    pub async fn run(&mut self, ctk: CancellationToken) -> Result<(), anyhow::Error> {
        info!("{INNER_NAME}: service starting");

        loop {
            // let cctk = ctk.clone();

            tokio::select! {
                _ = ctk.cancelled() => {
                    info!("{INNER_NAME}: tracker cancelled, breaking");
                    break;
                }
                r = self.tcp_listener.accept() => {
                    match r {
                        Ok((socket, remote_addr)) => {
                            trace!("{INNER_NAME}: new client: {remote_addr}");
                            let csender = self.sender.clone();

                            tokio::spawn(async move {
                                let mut ir = InboundRequest::new(socket, remote_addr.to_string(), csender);

                                loop {
                                    match ir.handle().await {
                                        Ok(_) => {},
                                        Err(e) => match e.downcast_ref() {
                                            Some(AppError::NotAnError) => break,
                                            None => {
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
}
