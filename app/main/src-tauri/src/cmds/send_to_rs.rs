use rqs_lib::channel::ChannelMessage;

use crate::AppState;

#[tauri::command]
pub fn send_to_rs(
    message: ChannelMessage,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    info!("send_to_rs: {:?}", &message);

    match state.rqs_handle.send_message(message) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Couldn't perform: {}", e)),
    }
}
