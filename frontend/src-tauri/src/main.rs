#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate log;

use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};
use tokio::sync::{mpsc, Mutex};

struct AsyncProcInputTx {
    inner: Mutex<mpsc::Sender<String>>,
}

#[tokio::main]
async fn main() {
    // Define log level
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var(
            "RUST_LOG",
            "TRACE,mdns_sd=ERROR,polling=ERROR,neli=ERROR,bluez_async=ERROR",
        );
    }

    // Init logger/tracing
    tracing_subscriber::fmt::init();

    // Define tauri async runtime to be tokio
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    // Configure System Tray
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new().add_item(quit);
    let tray = SystemTray::new().with_menu(tray_menu);

    let (async_proc_input_tx, async_proc_input_rx) = mpsc::channel(1);
    let (async_proc_output_tx, mut async_proc_output_rx) = mpsc::channel(1);

    // Build and run Tauri app
    tauri::Builder::default()
        .manage(AsyncProcInputTx {
            inner: Mutex::new(async_proc_input_tx),
        })
        .invoke_handler(tauri::generate_handler![js2rs])
        .setup(|app| {
            // Uncomment if you need to debug with devtools
            // #[cfg(debug_assertions)]
            // {
            //     let main_window = _app.get_window("main").unwrap();
            //     main_window.open_devtools();
            // }

            tauri::async_runtime::spawn(async move {
                async_process_model(async_proc_input_rx, async_proc_output_tx).await
            });

            let app_handle = app.handle();
            tauri::async_runtime::spawn(async move {
                loop {
                    if let Some(output) = async_proc_output_rx.recv().await {
                        rs2js(output, &app_handle);
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
}

fn rs2js<R: tauri::Runtime>(message: String, manager: &impl Manager<R>) {
    info!("rs2js: {}", message);
    manager
        .emit_all("rs2js", format!("rs: {}", message))
        .unwrap();
}

#[tauri::command]
async fn js2rs(message: String, state: tauri::State<'_, AsyncProcInputTx>) -> Result<(), String> {
    info!("js2rs: {}", message);
    let async_proc_input_tx = state.inner.lock().await;
    async_proc_input_tx
        .send(message)
        .await
        .map_err(|e| e.to_string())
}

async fn async_process_model(
    mut input_rx: mpsc::Receiver<String>,
    output_tx: mpsc::Sender<String>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    while let Some(input) = input_rx.recv().await {
        let output = input;
        output_tx.send(output).await?;
    }

    Ok(())
}
