use rqs_lib::Visibility;

use crate::AppState;

#[tauri::command]
pub fn change_visibility(
    message: Visibility,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    info!("change_visibility: {message:?}");

    state
        .rqs_handle
        .change_visibility(message)
        .map_err(|e| format!("Failed to change visibility: {}", e))
}
