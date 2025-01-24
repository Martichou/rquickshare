#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri_plugin_autostart::MacosLauncher;
use rquickshare_lib::{store::get_startminimized, kill_app, open_main_window};

#[macro_use]
extern crate log;

fn main() {
    rquickshare_lib::setup_tauri_app()
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            trace!("tauri_plugin_single_instance: instance already running");
            open_main_window(app);
        }))
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
        })
}