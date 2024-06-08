#[macro_use]
extern crate log;

use std::hash::{DefaultHasher, Hash, Hasher};

use directories::ProjectDirs;
use logger::set_up_logging;
use notify_rust::{Notification, NotificationHandle};
use rqs_lib::channel::{ChannelAction, ChannelDirection, ChannelMessage};
use rqs_lib::{State, RQS};

mod config;
mod logger;

static PROGRAM_NAME: &str = "RQuickShare";

fn hash_to_u32(s: &str) -> u32 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);

    hasher.finish() as u32
}

fn to_capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn send_notification(
    id: u32,
    body: &str,
    actions: &[&str],
) -> Result<NotificationHandle, notify_rust::error::Error> {
    let mut notification = Notification::new();
    notification.id(id);
    notification.summary(PROGRAM_NAME);
    notification.body(body);

    for action in actions {
        notification.action(action, &to_capitalize(action));
    }

    notification.show()
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let proj_dirs =
        ProjectDirs::from("dev", "mandre", "rquickshare.ng").expect("Failed to load project dirs");

    // Configure the logging with log file rotation in the data_dir.
    set_up_logging(proj_dirs.data_dir())?;

    // Construct the config
    let config = config::Config::new(proj_dirs.config_dir())?;

    // Start the RQuickShare service
    let mut rqs = RQS::new(config.visibility, config.port_number, config.download_path);

    // TODO - Add back the ability to send a file
    let (_sender_file, mut ble_receiver) = rqs.run().await?;

    // Setting up the listener for notification, thus user approval
    let message_sender = rqs.message_sender.clone();
    let mut message_receiver = message_sender.subscribe();
    let notification_receiver = tokio::spawn(async move {
        loop {
            match message_receiver.recv().await {
                Ok(info) => {
                    match info.state.as_ref().unwrap_or(&State::Initial) {
                        State::WaitingForUserConsent => {
                            trace!("notification: waiting for user approval");

                            let id = info.id;
                            let name = info
                                .meta
                                .as_ref()
                                .and_then(|meta| meta.source.as_ref())
                                .map(|source| source.name.clone())
                                .unwrap_or_else(|| "Unknown".to_string());

                            // Send notification
                            match send_notification(
                                hash_to_u32(&id),
                                &format!("{name} want to initiate a transfer"),
                                &["accept", "reject"],
                            ) {
                                Ok(n) => {
                                    let local_ms = message_sender.clone();
                                    // TODO - Meh, untracked, unwaited tasks...
                                    tokio::spawn(async move {
                                        n.wait_for_action(|action| match action {
											"accept" => {
												if let Err(e) = local_ms.send(ChannelMessage {
													id,
													direction: ChannelDirection::FrontToLib,
													action: Some(ChannelAction::AcceptTransfer),
													..Default::default()
												}) {
													error!("notification: couldn't perform accept: {e}");
												}
											}
											"reject" => {
												if let Err(e) = local_ms.send(ChannelMessage {
													id,
													direction: ChannelDirection::FrontToLib,
													action: Some(ChannelAction::RejectTransfer),
													..Default::default()
												}) {
													error!("notification: couldn't perform reject: {e}");
												}
											},
											_ => ()
										})
                                    });
                                }
                                Err(e) => error!("notification: couldn't show WaitingForUserConsent notification: {}", e),
                            }
                        }
                        State::Disconnected | State::Rejected | State::Cancelled => {
                            // TODO - Transfer aborted, show error message
                        }
                        State::Finished => {
                            // TODO - Transfer finished, show to open file if we received
                        }
                        _ => {}
                    }
                }
                Err(e) => error!("notification: error getting receiver message: {e}"),
            }
        }
    });

    // React to device sharing nearby
    let nearby_listener = tokio::spawn(async move {
        loop {
            match ble_receiver.recv().await {
                Ok(_) => {
                    trace!("ble: a device nearby is sharing");

                    // Send notification telling a device nearby is sharing
                    // if the current visibility is Invisible
                    match send_notification(
                        1919,
                        "A device is sharing nearby",
                        &["visible", "ignore"],
                    ) {
                        Ok(n) => {
                            // TODO - Meh, untracked, unwaited tasks...
                            tokio::spawn(async move {
                                n.wait_for_action(|action| match action {
                                    "accept" => {
                                        // TODO: change visibility
                                    }
                                    "ignore" => {}
                                    _ => (),
                                })
                            });
                        }
                        Err(e) => error!("notification: couldn't show notification: {}", e),
                    }
                }
                Err(e) => error!("ble: error getting ble_receiver message: {}", e),
            }
        }
    });

    let _ = tokio::signal::ctrl_c().await;
    info!("Stopping service.");
    notification_receiver.abort();
    nearby_listener.abort();
    rqs.stop().await;

    Ok(())
}
