use crate::AppState;

#[tauri::command]
pub fn change_device_name(message: Option<String>, state: tauri::State<'_, AppState>) {
    info!("device_name: {message:?}");

    state
        .rqs
        .lock()
        .unwrap()
        .set_device_name(message);
}
