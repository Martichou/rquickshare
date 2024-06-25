#[tauri::command]
pub fn open_url(message: String) -> Result<(), String> {
    info!("open: {:?}", &message);

    match open::that(message) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Coudln't open: {}", e)),
    }
}
