#[tauri::command]
pub fn get_hostname() -> String {
    sys_metrics::host::get_hostname().unwrap_or(String::from("Unknown"))
}
