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
    AppHandle, CustomMenuItem, GlobalWindowEvent, Manager, SystemTray, SystemTrayEvent,
    SystemTrayMenu, SystemTrayMenuItem,
};
use tauri_plugin_autostart::MacosLauncher;
use tokio::sync::{broadcast, mpsc, watch};

use crate::logger::set_up_logging;
use crate::notification::{send_request_notification, send_temporarily_notification};
use crate::store::{
    get_download_path, get_port, get_realclose, get_visibility, init_default, set_visibility,
};

mod cmds;
mod logger;
mod notification;
mod store;

pub struct AppState {
    pub message_sender: broadcast::Sender<ChannelMessage>,
    pub dch_sender: broadcast::Sender<EndpointInfo>,
    pub visibility_sender: Arc<Mutex<watch::Sender<Visibility>>>,
    pub sender_file: mpsc::Sender<SendInfo>,
    pub ble_receiver: broadcast::Receiver<()>,
    pub rqs: Mutex<RQS>,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Define tauri async runtime to be tokio
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    // Configure System Tray
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("rqs", "RQuickShare").disabled())
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("show".to_string(), "Show"))
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit"));

    // Build and run Tauri app
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            trace!("tauri_plugin_single_instance: instance already running");
            open_main_window(app);
        }))
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            cmds::change_download_path,
            cmds::change_visibility,
            cmds::start_discovery,
            cmds::stop_discovery,
            cmds::get_hostname,
            cmds::open_url,
            cmds::send_payload,
            cmds::send_to_rs,
        ])
        .setup(|app| {
            // Setting up logging inside file for the app
            set_up_logging(&app.app_handle())?;

            debug!("Starting setup of RQuickShare app");

            // Initialize default values for the store
            init_default(&app.app_handle());

            // Fetch initial configuration values
            let visibility = get_visibility(&app.app_handle());
            let port_number = get_port(&app.app_handle());
            let download_path = get_download_path(&app.app_handle());

            let app_handle = app.app_handle().clone();
            // This is not optimal, but until I find a better way to init log
            // (inside file and stdout) before starting the lib, I'll keep it as
            // is. This allow me to get the whole log :)
            tokio::task::block_in_place(|| {
                tauri::async_runtime::block_on(async move {
                    trace!("Beginning of RQS start");
                    // Start the RQuickShare service
                    let mut rqs = RQS::new(visibility, port_number, download_path);
                    let (sender_file, ble_receiver) = rqs.run().await.unwrap();

                    // Define state for tauri app
                    app_handle.manage(AppState {
                        message_sender: rqs.message_sender.clone(),
                        dch_sender: broadcast::channel(10).0,
                        visibility_sender: rqs.visibility_sender.clone(),
                        sender_file,
                        ble_receiver,
                        rqs: Mutex::new(rqs),
                    });
                });
            });

            spawn_receiver_tasks(&app.app_handle());
            Ok(())
        })
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .on_system_tray_event(handle_system_tray_event)
        .on_window_event(handle_window_event)
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app_handle, event| {
            if let tauri::RunEvent::ExitRequested { .. } = event {
                trace!("RunEvent::ExitRequested");
                kill_app(app_handle);
            }
        });

    info!("Application stopped");
    Ok(())
}

fn spawn_receiver_tasks(app_handle: &AppHandle) {
    let capp_handle = app_handle.clone();
    tauri::async_runtime::spawn(async move {
        let state: tauri::State<'_, AppState> = capp_handle.state();
        let mut receiver = state.message_sender.subscribe();

        loop {
            let rinfo = receiver.recv().await;

            match rinfo {
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
                        send_request_notification(name, info.id.clone(), &capp_handle);
                    }
                    rs2js_channelmessage(info, &capp_handle);
                }
                Err(e) => {
                    error!("RecvError: message_sender: {e}");
                }
            }
        }
    });

    let capp_handle = app_handle.clone();
    tauri::async_runtime::spawn(async move {
        let state: tauri::State<'_, AppState> = capp_handle.state();
        let mut dch_receiver = state.dch_sender.subscribe();

        loop {
            let rinfo = dch_receiver.recv().await;

            match rinfo {
                Ok(info) => rs2js_endpointinfo(info, &capp_handle),
                Err(e) => {
                    error!("RecvError: dch_sender: {e}");
                }
            }
        }
    });

    let capp_handle = app_handle.clone();
    tauri::async_runtime::spawn(async move {
        let state: tauri::State<'_, AppState> = capp_handle.state();
        let mut visibility_receiver = state.visibility_sender.lock().unwrap().subscribe();

        loop {
            let rinfo = visibility_receiver.changed().await;

            match rinfo {
                Ok(_) => {
                    let v = visibility_receiver.borrow_and_update();
                    let _ = set_visibility(&capp_handle, *v);
                }
                Err(e) => {
                    error!("RecvError: visibility_receiver: {e}");
                }
            }
        }
    });

    let capp_handle = app_handle.clone();
    tauri::async_runtime::spawn(async move {
        let state: tauri::State<'_, AppState> = capp_handle.state();
        let mut ble_receiver = state.ble_receiver.resubscribe();

        loop {
            let rinfo = ble_receiver.recv().await;

            match rinfo {
                Ok(_) => {
                    let v = get_visibility(&capp_handle);
                    trace!("Tauri: ble received: {:?}", v);

                    if v == Visibility::Invisible {
                        send_temporarily_notification(&capp_handle);
                    }
                }
                Err(e) => {
                    error!("RecvError: ble_receiver: {e}");
                }
            }
        }
    });
}

fn handle_system_tray_event(app: &tauri::AppHandle, event: SystemTrayEvent) {
    if let SystemTrayEvent::MenuItemClick { id, .. } = event {
        match id.as_str() {
            "show" => {
                trace!("tray_show");
                open_main_window(app);
            }
            "quit" => {
                trace!("tray_quit");
                kill_app(&app.app_handle());
            }
            _ => {}
        }
    }
}

fn handle_window_event(event: GlobalWindowEvent) {
    if let tauri::WindowEvent::CloseRequested { api, .. } = event.event() {
        if get_realclose(&event.window().app_handle()) {
            trace!("handle_window_event: real close");
            return;
        }

        trace!("handle_window_event: prevent close");
        event.window().hide().unwrap();
        api.prevent_close();
    }
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

fn open_main_window(app_handle: &AppHandle) {
    if let Some(webview_window) = app_handle.get_window("main") {
        let _ = webview_window.show();
        let _ = webview_window.set_focus();
        return;
    }

    warn!("open_main_window: no main window found");
}

fn kill_app(app_handle: &AppHandle) {
    let state: tauri::State<'_, AppState> = app_handle.state();

    tokio::task::block_in_place(|| {
        #[allow(clippy::await_holding_lock)]
        tauri::async_runtime::block_on(async move {
            let _ = state.rqs.lock().unwrap().stop().await;
        });
    });

    app_handle.exit(-1);
}
