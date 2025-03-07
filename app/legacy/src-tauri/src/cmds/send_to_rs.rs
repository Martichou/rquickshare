use rqs_lib::channel::ChannelMessage;

use crate::AppState;

#[tauri::command]
pub fn send_to_rs(
    message: ChannelMessage,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    info!("send_to_rs: {:?}", &message);

    state
        .rqs_handle
        .send_message(message)
        .map_err(|e| format!("Couldn't perform: {}", e))
}
