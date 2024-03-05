use rqs_lib::Visibility;
use tauri::{AppHandle, Manager};
use tauri_plugin_store::with_store;

pub fn get_realclose(app_handle: &AppHandle) -> bool {
    with_store(
        app_handle.clone(),
        app_handle.state(),
        ".settings.json",
        |store| {
            return Ok(store
                .get("realclose")
                .and_then(|json| json.as_bool())
                .unwrap_or(false));
        },
    )
    .unwrap_or(false)
}

pub fn get_visibility(app_handle: &AppHandle) -> Visibility {
    Visibility::from_raw_value(
        with_store(
            app_handle.clone(),
            app_handle.state(),
            ".settings.json",
            |store| {
                return Ok(store
                    .get("visibility")
                    .and_then(|json| json.as_u64())
                    .unwrap_or(0));
            },
        )
        .unwrap_or(0) as u8,
    )
}
