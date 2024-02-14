use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;

use crate::errors::AppError;
use crate::inbound::InboundRequest;

const INNER_NAME: &str = "TcpServer";

pub struct TcpServer {
    tcp_listener: TcpListener,
}

impl TcpServer {
    pub fn new(tcp_listener: TcpListener) -> Result<Self, anyhow::Error> {
        Ok(Self { tcp_listener })
    }

    pub async fn run(&self, ctk: CancellationToken) -> Result<(), anyhow::Error> {
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
                            trace!("New client: {remote_addr}");

                            tokio::spawn(async move {
                                let mut ir = InboundRequest::new(socket, remote_addr.to_string());

                                loop {
                                    match ir.handle().await {
                                        Ok(_) => {},
                                        Err(e) => match e.downcast_ref() {
                                            Some(AppError::NotAnError) => break,
                                            None => {
                                                error!("Error while handling client: {e}");
                                                break;
                                            }
                                        },
                                    }
                                }
                            });
                        },
                        Err(err) => {
                            error!("TcpListener: error accepting: {}", err);
                            break;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
