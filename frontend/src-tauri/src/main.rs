#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate log;

use rquickshare::channel::ChannelMessage;
use rquickshare::RQS;
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Define tauri async runtime to be tokio
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    // Define log level
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var(
            "RUST_LOG",
            "TRACE,mdns_sd=ERROR,polling=ERROR,neli=ERROR,bluez_async=ERROR",
        );
    }

    // Init logger/tracing
    tracing_subscriber::fmt::init();

    // Start the RQuickShare service
    let rqs = RQS::default();
    rqs.run().await?;

    let (sender, mut receiver) = rqs.channel;

    // Configure System Tray
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new().add_item(quit);
    let tray = SystemTray::new().with_menu(tray_menu);

    // Build and run Tauri app
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![js2rs])
        .setup(|app| {
            // Uncomment if you need to debug with devtools
            // #[cfg(debug_assertions)]
            // {
            //     let main_window = _app.get_window("main").unwrap();
            //     main_window.open_devtools();
            // }

            let app_handle = app.handle();
            tauri::async_runtime::spawn(async move {
                loop {
                    if let Ok(info) = receiver.recv().await {
                        rs2js(info, &app_handle);
                    } else {
                        error!("Error getting receiver message");
                        std::process::exit(0);
                    }
                }
            });

            Ok(())
        })
        .system_tray(tray)
        .on_system_tray_event(|_app, event| {
            if let SystemTrayEvent::MenuItemClick { id, .. } = event {
                if id.as_str() == "quit" {
                    std::process::exit(0);
                }
            }
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
    info!("rs2js: {:?}", message);
    manager.emit_all("rs2js", message).unwrap();
}

#[tauri::command]
async fn js2rs(message: String) -> Result<(), String> {
    let cmsg = serde_json::from_str::<ChannelMessage>(&message);
    info!("js2rs: {:?}", cmsg);
    Ok(())
}
