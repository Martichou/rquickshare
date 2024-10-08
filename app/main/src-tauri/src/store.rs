use std::{path::PathBuf, time::Duration};

use rqs_lib::Visibility;
use tauri::{AppHandle, Emitter, Wry};
use tauri_plugin_store::{Store, StoreExt};

fn _get_store(app_handle: &AppHandle) -> Store<Wry> {
    app_handle
        .store_builder(".settings.json")
        .auto_save(Duration::from_millis(100))
        .build()
}

pub fn init_default(app_handle: &AppHandle) {
    let store = _get_store(app_handle);

    if !store.has("autostart") {
        store.set("autostart", true);
    }

    if !store.has("realclose") {
        store.set("realclose", false);
    }

    if !store.has("visibility") {
        store.set("visibility", Visibility::Visible as u8);
    }

    if !store.has("startminimized") {
        store.set("startminimized", false);
    }
}

pub fn get_realclose(app_handle: &AppHandle) -> bool {
    let store = _get_store(app_handle);

    store
        .get("realclose")
        .and_then(|json| json.as_bool())
        .unwrap_or_default()
}

pub fn get_port(app_handle: &AppHandle) -> Option<u32> {
    let store = _get_store(app_handle);

    store
        .get("port")
        .and_then(|json| json.as_u64().map(|v| v as u32))
}

pub fn get_visibility(app_handle: &AppHandle) -> Visibility {
    let store = _get_store(app_handle);

    store
        .get("visibility")
        .and_then(|json| json.as_u64().map(Visibility::from_raw_value))
        .unwrap_or(Visibility::Visible)
}

pub fn set_visibility(app_handle: &AppHandle, v: Visibility) -> Result<(), anyhow::Error> {
    let store = _get_store(app_handle);

    store.set("visibility", v as u8);
    app_handle.emit("visibility_updated", ())?;

    Ok(())
}

pub fn get_download_path(app_handle: &AppHandle) -> Option<PathBuf> {
    let store = _get_store(app_handle);

    store
        .get("download_path")
        .and_then(|json| json.as_str().map(PathBuf::from))
}

pub fn get_logging_level(app_handle: &AppHandle) -> Option<String> {
    let store = _get_store(app_handle);

    store
        .get("debug_level")
        .and_then(|json| json.as_str().map(String::from))
}

pub fn get_startminimized(app_handle: &AppHandle) -> bool {
    let store = _get_store(app_handle);

    store
        .get("startminimized")
        .and_then(|json| json.as_bool())
        .unwrap_or_default()
}
