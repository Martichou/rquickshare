use std::path::PathBuf;

use crate::AppState;

#[tauri::command]
pub fn change_download_path(message: Option<String>, state: tauri::State<'_, AppState>) {
    info!("change_download_path: {message:?}");

    state
        .rqs_handle
        .set_download_path(message.map(PathBuf::from));
}
