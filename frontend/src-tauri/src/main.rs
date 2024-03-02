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
use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::tray::TrayIconBuilder;
use tauri::{AppHandle, Icon, Manager};
use tauri_plugin_autostart::MacosLauncher;
use tokio::sync::{broadcast, mpsc};

#[cfg(not(target_os = "linux"))]
use tauri_plugin_notification::NotificationExt;

pub struct AppState {
    pub sender: broadcast::Sender<ChannelMessage>,
    pub sender_file: mpsc::Sender<SendInfo>,
    pub rqs: Mutex<RQS>,
    pub dch_sender: mpsc::Sender<EndpointInfo>,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Define tauri async runtime to be tokio
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    // Define log level
    let default_level = if cfg!(debug_assertions) {
        "TRACE"
    } else {
        "INFO"
    };

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var(
            "RUST_LOG",
            format!("{default_level},mdns_sd=ERROR,polling=ERROR,neli=ERROR,bluez_async=ERROR,bluer=ERROR,async_io=ERROR"),
        );
    }

    // Init logger/tracing
    tracing_subscriber::fmt::init();

    // Start the RQuickShare service
    let rqs = RQS::default();
    let sender_file = rqs.run().await?;

    let (dch_sender, mut dch_receiver) = mpsc::channel(10);
    let (sender, mut receiver) = (rqs.channel.0.clone(), rqs.channel.1.resubscribe());

    // Build and run Tauri app
    tauri::Builder::default()
        .manage(AppState {
            sender: sender,
            sender_file: sender_file,
            rqs: Mutex::new(rqs),
            dch_sender: dch_sender,
        })
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_notification::init())
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
            debug!("Starting setup of Tauri app");
            // Setup system tray
            let show = MenuItemBuilder::with_id("show", "Show").build(app)?;
            let quit = MenuItemBuilder::with_id("quit", "Quit").build(app)?;
            let menu = MenuBuilder::new(app).items(&[&show, &quit]).build()?;
            let _tray = TrayIconBuilder::new()
                .icon(Icon::Raw(include_bytes!("../icons/icon.png").to_vec()))
                .menu(&menu)
                .on_menu_event(move |app, event| match event.id().as_ref() {
                    "show" => {
                        let app = app.app_handle();
                        if let Some(webview_window) = app.get_webview_window("main") {
                            let _ = webview_window.show();
                            let _ = webview_window.set_focus();
                        }
                    }
                    "quit" => {
                        tokio::task::block_in_place(|| {
                            tauri::async_runtime::block_on(async move {
                                let state: tauri::State<'_, AppState> = app.state();
                                let _ = state.rqs.lock().unwrap().stop().await;

                                std::process::exit(0);
                            });
                        });
                    }
                    _ => (),
                })
                .build(app)?;

            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
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
                loop {
                    match dch_receiver.recv().await {
                        Some(info) => rs2js_discovery(info, &app_handle),
                        None => {
                            error!("Error getting dch_receiver message");
                        }
                    }
                }
            });

            Ok(())
        })
        .on_window_event(|w, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                w.hide().unwrap();
                api.prevent_close();
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
    manager.emit("rs2js", &message).unwrap();
}

fn rs2js_discovery<R: tauri::Runtime>(message: EndpointInfo, manager: &impl Manager<R>) {
    info!("rs2js_discovery: {:?}", &message);
    manager.emit("rs2js_discovery", &message).unwrap();
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
    let _ = app_handle
        .notification()
        .builder()
        .title("RQuickShare")
        .body(&body)
        .show();
}
