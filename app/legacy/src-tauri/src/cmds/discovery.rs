use crate::AppState;

#[tauri::command]
pub async fn start_discovery(state: tauri::State<'_, AppState>) -> Result<(), String> {
    info!("start_discovery");

    state
        .rqs_handle
        .start_discovery()
        .map_err(|e| format!("unable to start discovery: {}", e))
}

#[tauri::command]
pub fn stop_discovery(state: tauri::State<'_, AppState>) {
    info!("stop_discovery");

    state.rqs_handle.stop_discovery();
}
