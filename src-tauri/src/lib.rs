mod host_websocket_interface;
mod client_websocket_interface;
mod microphone;
mod qr_service;

use tokio::sync::Mutex;
use futures_util::stream::{SplitSink, SplitStream};
use tauri::Manager;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::tungstenite::handshake::client::Response;
use tokio_tungstenite::tungstenite::Message;
use crate::host_websocket_interface::start_websocket_server;
use crate::client_websocket_interface::connect_to_websocket;
use crate::microphone::start_recording_with_timeout;
use crate::qr_service::generate_qr_code;
use client_websocket_interface::WebSocketClientState;
use crate::client_websocket_interface::handle_file_message;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

struct AppState {
    pub client_data_stream_writer: Option<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>,
    pub client_response: Option<Response>
}

impl AppState {
    pub fn new() -> Self{
        AppState {client_data_stream_writer: None, client_response: None}
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(mobile)]
            {
                let _ = app.handle().plugin(tauri_plugin_barcode_scanner::init());
            }
            app.manage(Mutex::new(AppState::new()));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .manage(WebSocketClientState::default())
        .invoke_handler(tauri::generate_handler![
            greet,
            generate_qr_code,
            start_websocket_server,
            start_recording_with_timeout,
            connect_to_websocket,
            handle_file_message
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
