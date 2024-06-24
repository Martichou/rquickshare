use notify_rust::Notification;
#[cfg(not(target_os = "macos"))]
use rqs_lib::{
    channel::{ChannelAction, ChannelDirection, ChannelMessage},
    Visibility,
};
use tauri::AppHandle;
#[cfg(not(target_os = "macos"))]
use tauri::Manager;

#[cfg(not(target_os = "linux"))]
#[cfg(not(target_os = "macos"))]
use tauri_plugin_notification::NotificationExt;

#[cfg(not(target_os = "macos"))]
use crate::cmds;

pub fn send_request_notification(name: String, id: String, app_handle: &AppHandle) {
    let body = format!("{name} want to initiate a transfer");
    match Notification::new()
        .summary("RQuickShare")
        .body(&body)
        .action("accept", "Accept")
        .action("reject", "Reject")
        .show()
    {
        Ok(n) => {
            #[cfg(not(target_os = "macos"))]
            let capp_handle: AppHandle = app_handle.clone();
            // TODO - Meh, untracked, unwaited tasks...
            #[cfg(not(target_os = "macos"))]
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
            #[cfg(target_os = "macos")]
            debug!("Notification shown on macOS, but no action handle {:?} and app_handle {:?}, id {}.", n, app_handle, id);
        }
        Err(e) => {
            error!("Couldn't show notification: {}", e);
        }
    }

    #[cfg(not(target_os = "linux"))]
    #[cfg(not(target_os = "macos"))]
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
            #[cfg(not(target_os = "macos"))]
            let capp_handle = app_handle.clone();
            // TODO - Meh, untracked, unwaited tasks...
            #[cfg(not(target_os = "macos"))]
            tokio::task::spawn(async move {
                n.wait_for_action(|action| match action {
                    "visible" => {
                        cmds::change_visibility(Visibility::Temporarily, capp_handle.state());
                    }
                    "ignore" => {}
                    _ => (),
                });
            });
            #[cfg(target_os = "macos")]
            debug!("Notification shown on macOS, but no action handle {:?} and app_handle {:?}.", n, app_handle);
        }
        Err(e) => {
            error!("Couldn't show notification: {}", e);
        }
    }
}
