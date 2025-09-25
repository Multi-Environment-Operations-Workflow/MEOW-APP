mod host_websocket_interface;
mod client_websocket_interface;
mod microphone;
mod qr_service;

use crate::host_websocket_interface::start_websocket_server;
use crate::client_websocket_interface::connect_to_websocket;
use crate::microphone::start_recording_with_timeout;
use crate::qr_service::generate_qr_code;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            generate_qr_code,
            start_websocket_server,
            start_recording_with_timeout,
            connect_to_websocket
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
