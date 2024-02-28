#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate log;

use std::sync::Mutex;

use rquickshare::channel::{ChannelDirection, ChannelMessage};
use rquickshare::{EndpointInfo, SendInfo, RQS};
use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};
use tokio::sync::broadcast;
use tokio::sync::mpsc;

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
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var(
            "RUST_LOG",
            "TRACE,mdns_sd=ERROR,polling=ERROR,neli=ERROR,bluez_async=ERROR,bluer=ERROR",
        );
    }

    // Init logger/tracing
    tracing_subscriber::fmt::init();

    // Start the RQuickShare service
    let rqs = RQS::default();
    let sender_file = rqs.run().await?;

    let (dch_sender, mut dch_receiver) = mpsc::channel(10);
    let (sender, mut receiver) = (rqs.channel.0.clone(), rqs.channel.1.resubscribe());

    // Configure System Tray
    let test = CustomMenuItem::new("test".to_string(), "Test");
    let show = CustomMenuItem::new("show".to_string(), "Show");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new()
        .add_item(show)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(test);
    let tray = SystemTray::new().with_menu(tray_menu);

    // Build and run Tauri app
    tauri::Builder::default()
        .manage(AppState {
            sender: sender,
            sender_file: sender_file,
            rqs: Mutex::new(rqs),
            dch_sender: dch_sender,
        })
        .invoke_handler(tauri::generate_handler![
            js2rs,
            open,
            send_payload,
            start_discovery,
            stop_discovery
        ])
        .setup(|app| {
            let app_handle = app.handle();
            tauri::async_runtime::spawn(async move {
                loop {
                    match receiver.recv().await {
                        Ok(info) => rs2js(info, &app_handle),
                        Err(e) => {
                            error!("Error getting receiver message: {}", e);
                        }
                    }
                }
            });

            let capp_handle = app.handle();
            tauri::async_runtime::spawn(async move {
                loop {
                    match dch_receiver.recv().await {
                        Some(info) => rs2js_discovery(info, &capp_handle),
                        None => {
                            error!("Error getting dch_receiver message");
                        }
                    }
                }
            });

            Ok(())
        })
        .system_tray(tray)
        .on_system_tray_event(|_app, event| {
            if let SystemTrayEvent::MenuItemClick { id, .. } = event {
                match id.as_str() {
                    "show" => {
                        let widow = _app.get_window("main").unwrap();
                        let _ = widow.show();
                        let _ = widow.set_focus();
                    }
                    "quit" => {
                        // TODO - Clean exit
                        std::process::exit(0);
                    }
                    _ => {}
                }
            }
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
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
