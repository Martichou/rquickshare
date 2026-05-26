use image::{ImageBuffer, Rgba};
use tauri_plugin_clipboard_manager::ClipboardExt;

#[tauri::command]
pub async fn save_clipboard_image(app: tauri::AppHandle) -> Result<String, String> {
    let img_data = app
        .clipboard()
        .read_image()
        .map_err(|_| "Clipboard does not contain an image".to_string())?;

    let width = img_data.width();
    let height = img_data.height();
    let rgba = img_data.rgba().to_vec();

    let img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(width, height, rgba)
        .ok_or_else(|| "Invalid image dimensions".to_string())?;

    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_millis();

    let temp_path = std::env::temp_dir().join(format!("rqs_clipboard_{ts}.png"));

    img.save(&temp_path).map_err(|e| e.to_string())?;

    Ok(temp_path.to_string_lossy().to_string())
}
