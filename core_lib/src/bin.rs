#[macro_use]
extern crate log;

use rqs_lib::channel::{ChannelAction, ChannelDirection, ChannelMessage};
use rqs_lib::State;
use rqs_lib::RQS;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Define log level
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var(
            "RUST_LOG",
            "TRACE,mdns_sd=ERROR,polling=ERROR,neli=ERROR,bluez_async=ERROR",
        );
    }

    // Init logger/tracing
    tracing_subscriber::fmt::init();

    // Start the RQuickShare service
    let mut rqs = RQS::default();
    rqs.run().await?;

    // Daemon mode: subscribe to transfer consent requests and auto-accept them.
    // The Tauri desktop app normally handles this via a GUI notification; the
    // standalone daemon has no UI, so we accept every incoming transfer here.
    let auto_sender = rqs.message_sender.clone();
    let mut consent_rx = rqs.message_sender.subscribe();
    tokio::spawn(async move {
        loop {
            match consent_rx.recv().await {
                Ok(msg)
                    if msg.direction == ChannelDirection::LibToFront
                        && msg.state.as_ref() == Some(&State::WaitingForUserConsent) =>
                {
                    info!("Auto-accepting transfer: {:?}", msg.id);
                    let _ = auto_sender.send(ChannelMessage {
                        id: msg.id,
                        direction: ChannelDirection::FrontToLib,
                        action: Some(ChannelAction::AcceptTransfer),
                        ..Default::default()
                    });
                }
                Err(_) => break,
                _ => {}
            }
        }
    });

    let discovery_channel = broadcast::channel(10);
    rqs.discovery(discovery_channel.0)?;

    // Wait for CTRL+C and then stop RQS
    let _ = tokio::signal::ctrl_c().await;
    info!("Stopping service.");
    rqs.stop().await;

    Ok(())
}
