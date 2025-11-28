use std::{path::PathBuf, sync::Arc, time::Duration};

use rqs_lib::Visibility;
use tauri::{AppHandle, Emitter, Wry};
use tauri_plugin_store::{Store, StoreExt};

fn _get_store(app_handle: &AppHandle) -> Arc<Store<Wry>> {
    app_handle
        .store_builder(".settings.json")
        .auto_save(Duration::from_millis(100))
        .build()
        .unwrap()
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

pub fn is_device_trusted_and_valid(app_handle: &AppHandle, device_name: &str) -> bool {
    let store = _get_store(app_handle);

    // Get timeout in minutes (default 5 minutes)
    let timeout_minutes = store
        .get("auto_accept_timeout")
        .and_then(|json| json.as_u64())
        .unwrap_or(5);

    let timeout_ms = timeout_minutes * 60 * 1000;

    // Get trusted devices array
    let trusted_devices = match store.get("trusted_devices") {
        Some(json) => json,
        None => return false,
    };

    let devices = match trusted_devices.as_array() {
        Some(arr) => arr,
        None => return false,
    };

    // Find device by name and check if within timeout
    for device in devices {
        let name = device.get("name").and_then(|v| v.as_str());
        let last_transfer = device.get("lastTransfer").and_then(|v| v.as_u64());

        if let (Some(name), Some(last_transfer)) = (name, last_transfer) {
            if name == device_name {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_millis() as u64)
                    .unwrap_or(0);

                return (now - last_transfer) < timeout_ms;
            }
        }
    }

    false
}
