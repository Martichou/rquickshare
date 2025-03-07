#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate log;

use rqs_lib::channel::{ChannelDirection, ChannelMessage};
use rqs_lib::{EndpointInfo, RqsConfig, RqsEvent, RqsHandle, State, Visibility, RQS};
use store::get_startminimized;
#[cfg(target_os = "macos")]
use tauri::image::Image;
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::TrayIconBuilder,
    AppHandle, Emitter, Manager, Window, WindowEvent,
};
use tauri_plugin_autostart::MacosLauncher;
use tokio::sync::broadcast;

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
    pub rqs_handle: RqsHandle,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Define tauri async runtime to be tokio
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    // Build and run Tauri app
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            trace!("tauri_plugin_single_instance: instance already running");
            open_main_window(app);
        }))
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            cmds::change_download_path,
            cmds::change_visibility,
            cmds::start_discovery,
            cmds::stop_discovery,
            cmds::get_hostname,
            cmds::send_payload,
            cmds::send_to_rs,
        ])
        .setup(|app| {
            // Setting up logging inside file for the app
            set_up_logging(app.app_handle())?;

            debug!("Starting setup of RQuickShare app");

            // Initialize default values for the store
            init_default(app.app_handle());

            // Initialize system Tray
            let name = MenuItemBuilder::new("RQuickShare")
                .enabled(false)
                .build(app)?;
            let show = MenuItemBuilder::with_id("show", "Show").build(app)?;
            let quit = MenuItemBuilder::with_id("quit", "Quit").build(app)?;
            let menu = MenuBuilder::new(app)
                .item(&name)
                .separator()
                .items(&[&show, &quit])
                .build()?;

            #[cfg(target_os = "macos")]
            let icon = Image::from_bytes(include_bytes!("../icons/tray.png")).unwrap();
            #[cfg(not(target_os = "macos"))]
            let icon = app.default_window_icon().unwrap().clone();

            let tray = TrayIconBuilder::new()
                .icon(icon)
                .menu(&menu)
                .on_menu_event(move |app, event| match event.id().as_ref() {
                    "show" => {
                        trace!("tray_show");
                        open_main_window(app);
                    }
                    "quit" => {
                        trace!("tray_quit");
                        kill_app(app.app_handle());
                    }
                    _ => (),
                })
                .build(app)?;

            let _ = tray.set_icon_as_template(true);

            // Fetch initial configuration values
            let visibility = get_visibility(app.app_handle());
            let port_number = get_port(app.app_handle());
            let download_path = get_download_path(app.app_handle());

            // Create RQS configuration
            let config = RqsConfig {
                visibility,
                port_number,
                download_path,
            };

            let app_handle = app.app_handle().clone();
            // This is not optimal, but until I find a better way to init log
            // (inside file and stdout) before starting the lib, I'll keep it as
            // is. This allow me to get the whole log :)
            tokio::task::block_in_place(|| {
                tauri::async_runtime::block_on(async move {
                    trace!("Beginning of RQS start");
                    // Start the RQuickShare service
                    let mut rqs = RQS::new(config.clone());
                    let rqs_handle = rqs.start(&config).await.unwrap();

                    // Define state for tauri app
                    app_handle.manage(AppState { rqs_handle });

                    // Subscribe to RQS events
                    spawn_event_handler(app_handle, rqs.subscribe());
                });
            });

            Ok(())
        })
        .on_window_event(handle_window_event)
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app_handle, event| match event {
            tauri::RunEvent::Ready { .. } => {
                trace!("RunEvent::Ready");
                if get_startminimized(app_handle) {
                    #[cfg(not(target_os = "macos"))]
                    app_handle
                        .get_webview_window("main")
                        .unwrap()
                        .hide()
                        .unwrap();
                    #[cfg(target_os = "macos")]
                    app_handle.hide().unwrap();
                }
            }
            tauri::RunEvent::ExitRequested { code, .. } => {
                trace!("RunEvent::ExitRequested");
                if code != Some(-1) {
                    kill_app(app_handle);
                }
            }
            #[cfg(target_os = "macos")]
            tauri::RunEvent::Reopen { .. } => {
                trace!("RunEvent::Reopen");
                open_main_window(app_handle);
            }
            _ => {}
        });

    info!("Application stopped");
    Ok(())
}

fn spawn_event_handler(app_handle: AppHandle, mut event_receiver: broadcast::Receiver<RqsEvent>) {
    tauri::async_runtime::spawn(async move {
        // Track the last time we sent a temporary notification
        let mut last_notification_time =
            std::time::Instant::now() - std::time::Duration::from_secs(120);

        loop {
            match event_receiver.recv().await {
                Ok(event) => {
                    match event {
                        RqsEvent::Message(message) => {
                            if message.state.as_ref().unwrap_or(&State::Initial)
                                == &State::WaitingForUserConsent
                            {
                                let name = message
                                    .meta
                                    .as_ref()
                                    .and_then(|meta| meta.source.as_ref())
                                    .map(|source| source.name.clone())
                                    .unwrap_or_else(|| "Unknown".to_string());
                                send_request_notification(name, message.id.clone(), &app_handle);
                            }
                            rs2js_channelmessage(message, &app_handle);
                        }
                        RqsEvent::DeviceDiscovered(endpoint) => {
                            rs2js_endpointinfo(endpoint, &app_handle);
                        }
                        RqsEvent::VisibilityChanged(visibility) => {
                            let _ = set_visibility(&app_handle, visibility);
                        }
                        RqsEvent::NearbyDeviceSharing => {
                            // Handle BLE notification if needed
                            let visibility = get_visibility(&app_handle);
                            trace!("Tauri: ble received: {:?}", visibility);

                            // If visibility is invisible and enough time has passed since the last notification
                            if visibility == Visibility::Invisible
                                && last_notification_time.elapsed()
                                    >= std::time::Duration::from_secs(120)
                            {
                                send_temporarily_notification(&app_handle);
                                last_notification_time = std::time::Instant::now();
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("Error receiving RQS event: {}", e);
                    break;
                }
            }
        }
    });
}

fn handle_window_event(w: &Window, event: &WindowEvent) {
    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
        if get_realclose(w.app_handle()) {
            trace!("handle_window_event: real close");
            return;
        }

        trace!("handle_window_event: prevent close");
        w.hide().unwrap();
        api.prevent_close();
    }
}

fn rs2js_channelmessage(message: ChannelMessage, manager: &AppHandle) {
    if message.direction == ChannelDirection::FrontToLib {
        return;
    }

    info!("rs2js_channelmessage: {:?}", &message);
    manager.emit("rs2js_channelmessage", &message).unwrap();
}

fn rs2js_endpointinfo(message: EndpointInfo, manager: &AppHandle) {
    info!("rs2js_endpointinfo: {:?}", &message);
    manager.emit("rs2js_endpointinfo", &message).unwrap();
}

fn open_main_window(app_handle: &AppHandle) {
    if let Some(webview_window) = app_handle.get_webview_window("main") {
        let _ = webview_window.show();
        let _ = webview_window.set_focus();
        return;
    }

    warn!("open_main_window: no main window found");
}

fn kill_app(app_handle: &AppHandle) {
    // Shutdown RQS service
    let state: tauri::State<AppState> = app_handle.state();
    let rqs_handle = state.rqs_handle.clone();

    tauri::async_runtime::spawn(async move {
        if let Err(e) = rqs_handle.shutdown().await {
            error!("Error shutting down RQS: {}", e);
        }
    });

    app_handle.exit(-1);
}
