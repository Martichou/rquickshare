#[cfg(target_os = "macos")]
use mac_notification_sys::*;

#[cfg(target_os = "linux")]
use notify_rust::Notification;

use crate::cmds;
use rqs_lib::{
    channel::{ChannelAction, ChannelDirection, ChannelMessage},
    Visibility,
};
use tauri::{AppHandle, Manager};

#[cfg(target_os = "macos")]
fn send_confirmation_notification<F: FnOnce(Option<bool>) -> ()>(
    title: &str,
    body: String,
    ok_label: &str,
    close_label: &str,
    id: Option<u32>,
    on_action: F,
) {
    let _ = id;
    let bundle = get_bundle_identifier_or_default("dev.mandre.rquickshare");

    if let Err(e) = set_application(&bundle) {
        warn!("Cannot set application: {}", e);
    };

    let mut opts = Notification::new();

    opts.main_button(MainButton::SingleAction(ok_label));
    opts.close_button(close_label);

    match send_notification(title, None, body.as_str(), Some(&opts)) {
        Err(e) => error!("Couldn't show notification: {}", e),

        Ok(NotificationResponse::ActionButton(_)) => on_action(Some(true)),
        Ok(NotificationResponse::CloseButton(_)) => on_action(Some(false)),
        _ => on_action(None),
    }
}

#[cfg(target_os = "linux")]
fn send_confirmation_notification<F: FnOnce(Option<bool>) -> ()>(
    title: &str,
    body: String,
    ok_label: &str,
    close_label: &str,
    id: Option<u32>,
    on_action: F,
) {
    let mut notification = Notification::new()
        .summary(title)
        .body(&body)
        .action("ok", ok_label)
        .action("close", close_label);

    if let Some(id) = id {
        notification.id(id);
    }

    match notification.show() {
        Ok(n) => {
            // TODO - Meh, untracked, unwaited tasks...
            tokio::task::spawn(async move {
                n.wait_for_action(|action| match action {
                    "ok" => on_action(Some(true)),
                    "close" => on_action(Some(false)),
                    _ => on_action(None),
                });
            });
        }
        Err(e) => {
            error!("Couldn't show notification: {}", e);
        }
    }
}

pub fn send_request_notification(name: String, id: String, app_handle: &AppHandle) {
    send_confirmation_notification(
        "RQuickShare",
        format!("{name} want to initiate a transfer"),
        "Accept",
        "Reject",
        None,
        |confirmed| match confirmed {
            Some(true) => {
                let _ = cmds::send_to_rs(
                    ChannelMessage {
                        id,
                        direction: ChannelDirection::FrontToLib,
                        action: Some(ChannelAction::AcceptTransfer),
                        ..Default::default()
                    },
                    app_handle.state(),
                );
            }
            Some(false) => {
                let _ = cmds::send_to_rs(
                    ChannelMessage {
                        id,
                        direction: ChannelDirection::FrontToLib,
                        action: Some(ChannelAction::RejectTransfer),
                        ..Default::default()
                    },
                    app_handle.state(),
                );
            }
            _ => (),
        },
    );
}

pub fn send_temporarily_notification(app_handle: &AppHandle) {
    send_confirmation_notification(
        "RQuickShare",
        "RQuickShare is temporarily hidden".to_string(),
        "Be visible (1m)",
        "Ignore",
        #[cfg(target_os = "linux")]
        Some(1919),
        #[cfg(not(target_os = "linux"))]
        None,
        |confirmed| match confirmed {
            Some(true) => cmds::change_visibility(Visibility::Temporarily, app_handle.state()),
            _ => (),
        },
    );
}
