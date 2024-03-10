use notify_rust::Notification;
use rqs_lib::channel::{ChannelAction, ChannelDirection, ChannelMessage};
use rqs_lib::Visibility;
use tauri::{AppHandle, Manager};
#[cfg(not(target_os = "linux"))]
use tauri_plugin_notification::NotificationExt;

use crate::cmds;

pub fn send_request_notification(name: String, id: String, app_handle: &AppHandle) {
    let body = format!("{name} want to initiate a transfer");

    #[cfg(target_os = "linux")]
    match Notification::new()
        .summary("RQuickShare")
        .body(&body)
        .action("accept", "Accept")
        .action("reject", "Reject")
        .show()
    {
        Ok(n) => {
            let capp_handle = app_handle.clone();
            // TODO - Meh, untracked, unwaited tasks...
            tokio::task::spawn(async move {
                n.wait_for_action(|action| match action {
                    "accept" => {
                        let _ = cmds::send_to_rs(
                            ChannelMessage {
                                id,
                                direction: ChannelDirection::FrontToLib,
                                action: Some(ChannelAction::AcceptTransfer),
                                ..Default::default()
                            },
                            capp_handle.state(),
                        );
                    }
                    "reject" => {
                        let _ = cmds::send_to_rs(
                            ChannelMessage {
                                id,
                                direction: ChannelDirection::FrontToLib,
                                action: Some(ChannelAction::RejectTransfer),
                                ..Default::default()
                            },
                            capp_handle.state(),
                        );
                    }
                    _ => (),
                });
            });
        }
        Err(e) => {
            error!("Couldn't show notification: {}", e);
        }
    }

    #[cfg(not(target_os = "linux"))]
    let _ = app_handle
        .notification()
        .builder()
        .title("RQuickShare")
        .body(&body)
        .show();
}

pub fn send_temporarily_notification(app_handle: &AppHandle) {
    match Notification::new()
        .summary("RQuickShare")
        .body("A device is sharing nearby")
        .action("visible", "Be visible (1m)")
        .action("ignore", "Ignore")
        .id(1919)
        .show()
    {
        Ok(n) => {
            let capp_handle = app_handle.clone();
            // TODO - Meh, untracked, unwaited tasks...
            tokio::task::spawn(async move {
                n.wait_for_action(|action| match action {
                    "visible" => {
                        cmds::change_visibility(Visibility::Temporarily, capp_handle.state());
                    }
                    "ignore" => {}
                    _ => (),
                });
            });
        }
        Err(e) => {
            error!("Couldn't show notification: {}", e);
        }
    }
}
