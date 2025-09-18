#[tauri::command]
pub fn generate_qr_code() -> String {
    "This is your code".to_string()
}