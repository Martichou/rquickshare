use tauri::{AppHandle, Manager};

use crate::{store, AppState};

#[derive(Debug, serde::Serialize)]
pub struct DeviceNameValidation {
    pub valid: bool,
    pub error: Option<String>,
}

/// Validates a device name for Quick Share compatibility
fn validate_device_name(name: &str) -> DeviceNameValidation {
    let trimmed = name.trim();

    // Check if empty
    if trimmed.is_empty() {
        return DeviceNameValidation {
            valid: false,
            error: Some("Device name cannot be empty".to_string()),
        };
    }

    // Quick Share protocol limit is 255 bytes
    let byte_len = trimmed.len();
    if byte_len > 255 {
        return DeviceNameValidation {
            valid: false,
            error: Some(format!("Device name too long ({byte_len} bytes, max 255)")),
        };
    }

    // No control characters (newlines, tabs, etc.)
    if trimmed.chars().any(|c| c.is_control()) {
        return DeviceNameValidation {
            valid: false,
            error: Some("Device name cannot contain control characters".to_string()),
        };
    }

    DeviceNameValidation {
        valid: true,
        error: None,
    }
}

#[tauri::command]
pub fn set_device_name(
    app_handle: AppHandle,
    name: Option<String>,
) -> Result<DeviceNameValidation, String> {
    // If name is None or empty after trim, reset to system hostname
    let name_to_set = match &name {
        Some(n) if !n.trim().is_empty() => Some(n.trim().to_string()),
        _ => None,
    };

    // Validate if we have a name
    if let Some(ref n) = name_to_set {
        let validation = validate_device_name(n);
        if !validation.valid {
            return Ok(validation);
        }
    }

    // Save the device name to persistent storage
    store::set_device_name(&app_handle, name_to_set.clone());

    // Update the RQS service to broadcast the new name
    let state: tauri::State<'_, AppState> = app_handle.state();
    state.rqs.lock().unwrap().set_device_name(name_to_set);

    Ok(DeviceNameValidation {
        valid: true,
        error: None,
    })
}

#[tauri::command]
pub fn validate_device_name_cmd(name: String) -> DeviceNameValidation {
    validate_device_name(&name)
}
