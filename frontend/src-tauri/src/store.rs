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
                let _ = store.insert("autostart".to_owned(), JsonValue::Bool(true));
            }
            if !store.has("minimizeonstartup") {
                let _ = store.insert("minimizeonstartup".to_owned(), JsonValue::Bool(true));
            }
            if !store.has("realclose") {
                let _ = store.insert("realclose".to_owned(), JsonValue::Bool(false));
            }

            if !store.has("visibility") {
                let _ = store.insert(
                    "visibility".to_owned(),
                    JsonValue::Number((Visibility::Visible as u8).into()),
                );
            }
            Ok(())
        },
    );
}

pub fn get_realclose(app_handle: &AppHandle) -> bool {
    let realclose = with_store(
        app_handle.clone(),
        app_handle.state(),
        ".settings.json",
        |store| {
            return Ok(store.get("realclose").and_then(|json| json.as_bool()));
        },
    );

    match realclose {
        Ok(r) => r.unwrap_or(false),
        Err(e) => {
            error!("get_realclose: error: {}", e);
            false
        }
    }
}
pub fn get_minimizeonstartup(app_handle: &AppHandle) -> bool {
    let minimizeonstartup = with_store(
        app_handle.clone(),
        app_handle.state(),
        ".settings.json",
        |store| {
            return Ok(store
                .get("minimizeonstartup")
                .and_then(|json| json.as_bool()));
        },
    );

    match minimizeonstartup {
        Ok(r) => r.unwrap_or(false),
        Err(e) => {
            error!("get_minimizeonstartup: error: {}", e);
            false
        }
    }
}

pub fn get_port(app_handle: &AppHandle) -> Option<u32> {
    let visibility = with_store(
        app_handle.clone(),
        app_handle.state(),
        ".settings.json",
        |store| {
            return Ok(store.get("port").and_then(|json| json.as_u64()));
        },
    );

    match visibility {
        Ok(v) => match v {
            Some(vv) => Some(vv as u32),
            None => None,
        },
        Err(_) => None,
    }
}

pub fn get_visibility(app_handle: &AppHandle) -> Visibility {
    let visibility = with_store(
        app_handle.clone(),
        app_handle.state(),
        ".settings.json",
        |store| {
            return Ok(store.get("visibility").and_then(|json| json.as_u64()));
        },
    );

    match visibility {
        Ok(v) => Visibility::from_raw_value(v.unwrap_or(0) as u8),
        Err(e) => {
            error!("get_visibility: error: {}", e);
            Visibility::Visible
        }
    }
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
