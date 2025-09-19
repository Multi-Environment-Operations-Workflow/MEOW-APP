// Learn more about Tauri commands at https://tauri.app/
mod microphone;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            microphone::start_mic_recording,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
