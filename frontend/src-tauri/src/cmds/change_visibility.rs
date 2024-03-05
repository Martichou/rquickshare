use rqs_lib::Visibility;

use crate::AppState;

#[tauri::command]
pub fn change_visibility(message: Visibility, state: tauri::State<'_, AppState>) {
    info!("change_visibility: {message:?}");

    state.rqs.lock().unwrap().change_visibility(message);
}
