#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate log;

use std::sync::{Arc, Mutex};

use rqs_lib::channel::{ChannelDirection, ChannelMessage};
use rqs_lib::{EndpointInfo, SendInfo, State, Visibility, RQS};
use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};
use tauri_plugin_autostart::MacosLauncher;
use tokio::sync::{broadcast, mpsc, watch};

use crate::logger::set_up_logging;
use crate::notification::{send_request_notification, send_temporarily_notification};
use crate::store::{get_realclose, get_visibility, init_default, set_visibility};

mod cmds;
mod logger;
mod notification;
mod store;

pub struct AppState {
    pub sender: broadcast::Sender<ChannelMessage>,
    pub dch_sender: broadcast::Sender<EndpointInfo>,
    pub sender_file: mpsc::Sender<SendInfo>,
    pub ble_receiver: broadcast::Receiver<()>,
    pub visibility_sender: Arc<Mutex<watch::Sender<Visibility>>>,
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
            cmds::change_visibility,
            cmds::start_discovery,
            cmds::stop_discovery,
            cmds::get_hostname,
            cmds::open_url,
            cmds::send_payload,
            cmds::send_to_rs,
            sanity_check,
        ])
        .setup(|app| {
            set_up_logging(&app.app_handle())?;
            debug!("Starting setup of RQuickShare app");

            // Initialize default value for the store
            init_default(&app.app_handle());

            // Fetch default or previously saved visibility
            let visibility = get_visibility(&app.app_handle());

            let app_handle = app.app_handle().clone();
            // This is not optimal, but until I find a better way to init log
            // (inside file and stdout) before starting the lib, I'll keep it as
            // is. This allow me to get the whole log :)
            tokio::task::block_in_place(|| {
                tauri::async_runtime::block_on(async move {
                    trace!("Begining of RQS start");
                    // Start the RQuickShare service
                    let mut rqs = RQS::new(visibility);
                    // Need to be waited, but blocked on
                    let (sender_file, ble_receiver) = rqs.run().await.unwrap();

                    // Init the channels for use
                    let (dch_sender, _) = broadcast::channel(10);
                    let sender = rqs.message_sender.clone();

                    let visibility_sender = rqs.visibility_sender.clone();

                    // Define state for tauri app
                    app_handle.manage(AppState {
                        sender,
                        dch_sender,
                        sender_file,
                        ble_receiver,
                        visibility_sender,
                        rqs: Mutex::new(rqs),
                    });
                });
            });

            let app_handle = app.app_handle().clone();
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

                            rs2js_channelmessage(info, &app_handle);
                        }
                        Err(e) => {
                            error!("Error getting receiver message: {}", e);
                        }
                    }
                }
            });

            let app_handle = app.app_handle().clone();
            tauri::async_runtime::spawn(async move {
                let state: tauri::State<'_, AppState> = app_handle.state();
                let mut dch_receiver = state.dch_sender.subscribe();

                loop {
                    match dch_receiver.recv().await {
                        Ok(info) => rs2js_endpointinfo(info, &app_handle),
                        Err(e) => {
                            error!("Error getting dch_receiver message: {}", e);
                        }
                    }
                }
            });

            // Sync visibility with the store (mainly used for Temporarily)
            let app_handle = app.app_handle().clone();
            tauri::async_runtime::spawn(async move {
                let state: tauri::State<'_, AppState> = app_handle.state();
                let mut visibility_receiver = state.visibility_sender.lock().unwrap().subscribe();

                loop {
                    match visibility_receiver.changed().await {
                        Ok(_) => {
                            let v = visibility_receiver.borrow_and_update();
                            let _ = set_visibility(&app_handle, *v);
                        }
                        Err(e) => {
                            error!("Error getting visibility_receiver update: {}", e);
                        }
                    }
                }
            });

            // React to device sharing nearby
            let app_handle = app.app_handle().clone();
            tauri::async_runtime::spawn(async move {
                let state: tauri::State<'_, AppState> = app_handle.state();
                let mut ble_receiver = state.ble_receiver.resubscribe();

                loop {
                    match ble_receiver.recv().await {
                        Ok(_) => {
                            let v = get_visibility(&app_handle);
                            trace!("Tauri: ble received: {:?}", v);

                            if v == Visibility::Invisible {
                                // Show notification to pass to temporarily mdns mode
                                send_temporarily_notification(&app_handle);
                            }
                        }
                        Err(e) => {
                            error!("Error getting ble_receiver message: {}", e);
                        }
                    }
                }
            });

            Ok(())
        })
        .system_tray(SystemTray::new().with_menu(tray_menu))
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
                let realclose = get_realclose(&event.window().app_handle());

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

fn rs2js_channelmessage<R: tauri::Runtime>(message: ChannelMessage, manager: &impl Manager<R>) {
    if message.direction == ChannelDirection::FrontToLib {
        return;
    }

    info!("rs2js_channelmessage: {:?}", &message);
    manager.emit_all("rs2js_channelmessage", &message).unwrap();
}

fn rs2js_endpointinfo<R: tauri::Runtime>(message: EndpointInfo, manager: &impl Manager<R>) {
    info!("rs2js_endpointinfo: {:?}", &message);
    manager.emit_all("rs2js_endpointinfo", &message).unwrap();
}

#[tauri::command]
fn sanity_check() {
    info!("sanity_check");
}
