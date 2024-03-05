#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate log;

use std::sync::Mutex;

use notify_rust::Notification;
use rqs_lib::channel::{ChannelAction, ChannelDirection, ChannelMessage};
use rqs_lib::{EndpointInfo, SendInfo, State, RQS};
use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};
use tauri_plugin_autostart::MacosLauncher;
#[cfg(not(target_os = "linux"))]
#[cfg(not(target_os = "macos"))]
use tauri_plugin_notification::NotificationExt;
use tauri_plugin_store::with_store;
use tokio::sync::{broadcast, mpsc};

use crate::logger::set_up_logging;

mod logger;

pub struct AppState {
    pub sender: broadcast::Sender<ChannelMessage>,
    pub dch_sender: broadcast::Sender<EndpointInfo>,
    pub sender_file: mpsc::Sender<SendInfo>,
    pub rqs: Mutex<RQS>,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Define tauri async runtime to be tokio
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    // Configure System Tray
    let show = CustomMenuItem::new("show".to_string(), "Show");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("rqs", "RQuickShare").disabled())
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(show)
        .add_item(quit);
    let tray = SystemTray::new().with_menu(tray_menu);

    // Build and run Tauri app
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            println!("{}, {argv:?}, {cwd}", app.package_info().name);
        }))
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            js2rs,
            open,
            send_payload,
            start_discovery,
            stop_discovery,
            get_hostname,
            sanity_check
        ])
        .setup(|app| {
            set_up_logging(&app.app_handle())?;
            debug!("Starting setup of RQuickShare app");

            let app_handle = app.handle().clone();
            // This is not optimal, but until I find a better way to init log
            // (inside file and stdout) before starting the lib, I'll keep it as
            // is. This allow me to get the whole log :)
            tokio::task::block_in_place(|| {
                tauri::async_runtime::block_on(async move {
                    trace!("Begining of RQS start");
                    // Start the RQuickShare service
                    let rqs = RQS::default();
                    // Need to be waited, but blocked on
                    let sender_file = rqs.run().await.unwrap();
                    trace!("Ran RQS run");

                    // Init the channels for use
                    let (dch_sender, _) = broadcast::channel(10);
                    let sender = rqs.channel.0.clone();

                    // Define state for tauri app
                    app_handle.manage(AppState {
                        sender: sender,
                        dch_sender: dch_sender,
                        sender_file: sender_file,
                        rqs: Mutex::new(rqs),
                    });
                });
            });

            debug!("Done starting RQS lib");

            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let state: tauri::State<'_, AppState> = app_handle.state();
                let mut receiver = state.sender.subscribe();

                loop {
                    match receiver.recv().await {
                        Ok(info) => {
                            if info.state.as_ref().unwrap_or(&State::Initial)
                                == &State::WaitingForUserConsent
                            {
                                let name = info
                                    .meta
                                    .as_ref()
                                    .and_then(|meta| meta.source.as_ref())
                                    .map(|source| source.name.clone())
                                    .unwrap_or_else(|| "Unknown".to_string());

                                send_request_notification(name, info.id.clone(), &app_handle);
                            }

                            rs2js(info, &app_handle);
                        }
                        Err(e) => {
                            error!("Error getting receiver message: {}", e);
                        }
                    }
                }
            });

            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let state: tauri::State<'_, AppState> = app_handle.state();
                let mut dch_receiver = state.dch_sender.subscribe();

                loop {
                    match dch_receiver.recv().await {
                        Ok(info) => rs2js_discovery(info, &app_handle),
                        Err(e) => {
                            error!("Error getting dch_receiver message: {}", e);
                        }
                    }
                }
            });

            Ok(())
        })
        .system_tray(tray)
        .on_system_tray_event(|app, event| {
            if let SystemTrayEvent::MenuItemClick { id, .. } = event {
                match id.as_str() {
                    "show" => {
                        let app = app.app_handle();
                        if let Some(webview_window) = app.get_window("main") {
                            let _ = webview_window.show();
                            let _ = webview_window.set_focus();
                        }
                    }
                    "quit" => {
                        tokio::task::block_in_place(|| {
                            tauri::async_runtime::block_on(async move {
                                let state: tauri::State<'_, AppState> = app.state();
                                let _ = state.rqs.lock().unwrap().stop().await;

                                app.app_handle().exit(0);
                            });
                        });
                    }
                    _ => {}
                }
            }
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                let app_handle = event.window().app_handle();
                // This can never be an Err. We check for the realclose key, then convert the json to a bool
                // and handle default to be "false".
                let realclose = with_store(
                    app_handle.clone(),
                    app_handle.state(),
                    ".settings.json",
                    |store| {
                        return Ok(store
                            .get("realclose")
                            .and_then(|json| json.as_bool())
                            .unwrap_or(false));
                    },
                )
                .unwrap();

                if !realclose {
                    trace!("Prevent close");
                    event.window().hide().unwrap();
                    api.prevent_close();
                } else {
                    trace!("Real close");
                    tokio::task::block_in_place(|| {
                        tauri::async_runtime::block_on(async move {
                            let app_handle = event.window().app_handle();
                            let state: tauri::State<'_, AppState> = app_handle.state();
                            let _ = state.rqs.lock().unwrap().stop().await;

                            app_handle.exit(0);
                        });
                    });
                }
            }
            _ => {}
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, event| {
            if let tauri::RunEvent::ExitRequested { api, .. } = event {
                api.prevent_exit();
            }
        });

    info!("Application stopped");
    Ok(())
}

fn rs2js<R: tauri::Runtime>(message: ChannelMessage, manager: &impl Manager<R>) {
    if message.direction == ChannelDirection::FrontToLib {
        return;
    }

    info!("rs2js: {:?}", &message);
    manager.emit_all("rs2js", &message).unwrap();
}

fn rs2js_discovery<R: tauri::Runtime>(message: EndpointInfo, manager: &impl Manager<R>) {
    info!("rs2js_discovery: {:?}", &message);
    manager.emit_all("rs2js_discovery", &message).unwrap();
}

#[tauri::command]
async fn send_payload(message: SendInfo, state: tauri::State<'_, AppState>) -> Result<(), String> {
    info!("send_payload: {:?}", &message);

    state
        .sender_file
        .send(message)
        .await
        .map_err(|e| format!("couldn't send payload: {e}"))
}

#[tauri::command]
fn open(message: String) -> Result<(), String> {
    info!("open: {:?}", &message);

    match open::that(message) {
        Ok(_) => Ok(()),
        Err(e) => return Err(format!("Coudln't open: {}", e)),
    }
}

#[tauri::command]
fn js2rs(message: ChannelMessage, state: tauri::State<'_, AppState>) -> Result<(), String> {
    info!("js2rs: {:?}", &message);

    match state.sender.send(message) {
        Ok(_) => Ok(()),
        Err(e) => return Err(format!("Coudln't perform: {}", e)),
    }
}

#[tauri::command]
async fn start_discovery(state: tauri::State<'_, AppState>) -> Result<(), String> {
    info!("start_discovery");

    state
        .rqs
        .lock()
        .unwrap()
        .discovery(state.dch_sender.clone())
        .map_err(|e| format!("unable to start discovery: {}", e))
}

#[tauri::command]
fn stop_discovery(state: tauri::State<'_, AppState>) {
    info!("stop_discovery");

    state.rqs.lock().unwrap().stop_discovery();
}

#[tauri::command]
fn get_hostname() -> String {
    return sys_metrics::host::get_hostname().unwrap_or(String::from("Unknown"));
}

#[tauri::command]
fn sanity_check() {
    info!("sanity_check");
}

fn send_request_notification(name: String, id: String, app_handle: &AppHandle) {
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
                        let _ = js2rs(
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
                        let _ = js2rs(
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
    #[cfg(not(target_os = "macos"))]
    let _ = app_handle
        .notification()
        .builder()
        .title("RQuickShare")
        .body(&body)
        .show();
}
