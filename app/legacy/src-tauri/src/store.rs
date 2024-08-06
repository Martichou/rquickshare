use std::path::PathBuf;

use rqs_lib::Visibility;
use tauri::{AppHandle, Manager};
use tauri_plugin_store::{with_store, JsonValue};

pub fn init_default(app_handle: &AppHandle) {
    let _ = with_store(
        app_handle.clone(),
        app_handle.state(),
        ".settings.json",
        |store| {
            if !store.has("autostart") {
                store
                    .insert("autostart".to_owned(), JsonValue::Bool(true))
                    .ok();
            }

            if !store.has("realclose") {
                store
                    .insert("realclose".to_owned(), JsonValue::Bool(false))
                    .ok();
            }

            if !store.has("visibility") {
                store
                    .insert(
                        "visibility".to_owned(),
                        JsonValue::Number((Visibility::Visible as u8).into()),
                    )
                    .ok();
            }

            if !store.has("startminimized") {
                store
                    .insert("startminimized".to_owned(), JsonValue::Bool(false))
                    .ok();
            }
            Ok(())
        },
    );
}

pub fn get_realclose(app_handle: &AppHandle) -> bool {
    with_store(
        app_handle.clone(),
        app_handle.state(),
        ".settings.json",
        |store| Ok(store.get("realclose").and_then(|json| json.as_bool())),
    )
    .unwrap_or_else(|e| {
        error!("get_realclose: error: {}", e);
        None
    })
    .unwrap_or(false)
}

pub fn get_port(app_handle: &AppHandle) -> Option<u32> {
    with_store(
        app_handle.clone(),
        app_handle.state(),
        ".settings.json",
        |store| Ok(store.get("port").and_then(|json| json.as_u64())),
    )
    .ok()
    .flatten()
    .map(|v| v as u32)
}

pub fn get_visibility(app_handle: &AppHandle) -> Visibility {
    with_store(
        app_handle.clone(),
        app_handle.state(),
        ".settings.json",
        |store| Ok(store.get("visibility").and_then(|json| json.as_u64())),
    )
    .unwrap_or_else(|e| {
        error!("get_visibility: error: {}", e);
        None
    })
    .map_or(Visibility::Visible, |v| Visibility::from_raw_value(v as u8))
}

pub fn set_visibility(app_handle: &AppHandle, v: Visibility) -> Result<(), anyhow::Error> {
    with_store(
        app_handle.clone(),
        app_handle.state(),
        ".settings.json",
        |store| store.insert("visibility".to_owned(), JsonValue::Number((v as u8).into())),
    )?;

    app_handle.emit_all("visibility_updated", ())?;
    Ok(())
}

pub fn get_download_path(app_handle: &AppHandle) -> Option<PathBuf> {
    with_store(
        app_handle.clone(),
        app_handle.state(),
        ".settings.json",
        |store| Ok(store.get("download_path").cloned()),
    )
    .ok()
    .flatten()
    .and_then(|v| v.as_str().map(PathBuf::from))
}

pub fn get_logging_level(app_handle: &AppHandle) -> Option<String> {
    with_store(
        app_handle.clone(),
        app_handle.state(),
        ".settings.json",
        |store| {
            Ok(store
                .get("debug_level")
                .and_then(|json| json.as_str().map(String::from)))
        },
    )
    .ok()
    .flatten()
}

pub fn get_startminimized(app_handle: &AppHandle) -> bool {
    with_store(
        app_handle.clone(),
        app_handle.state(),
        ".settings.json",
        |store| Ok(store.get("startminimized").and_then(|json| json.as_bool())),
    )
    .unwrap_or_else(|e| {
        error!("get_startminimized: error: {}", e);
        None
    })
    .unwrap_or(false)
}
