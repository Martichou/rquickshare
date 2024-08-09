#[tauri::command]
pub fn open_url(message: String) -> Result<(), String> {
    info!("open_url: {:?}", &message);

    match open::that_detached(message) {
        Ok(_) => {
            trace!("open_url: success");

            Ok(())
        }
        Err(e) => {
            error!("open_url error: {}", e);

            Err(format!("Coudln't open: {}", e))
        }
    }
}
