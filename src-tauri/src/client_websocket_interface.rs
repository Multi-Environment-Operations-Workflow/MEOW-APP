use std::sync::Arc;
use futures_util::{stream::SplitSink, SinkExt, StreamExt};
use serde::Deserialize;
use serde_json::json;
use tauri::ipc::Channel;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::{connect_async, 
    tungstenite::{client::IntoClientRequest, Message}, 
    MaybeTlsStream, WebSocketStream,};

pub struct WebSocketClientState {
    ws_writer: Arc<Mutex<Option<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>>,
}

impl Default for WebSocketClientState {
    fn default() -> Self {
        Self {
            ws_writer: Arc::new(Mutex::new(None)),
        }
    }
}

#[tauri::command]
pub async fn connect_to_websocket(connection_string: String, on_event: Channel<String>, ws_state: tauri::State<'_, WebSocketClientState>) -> Result<(), String> {
    let request = match connection_string.into_client_request() {
        Ok(req) => req,
        Err(e) => return Err(format!("Invalid WebSocket URL: {}", e)),
    };
    let (mut stream, _res) = match connect_async(request).await {
        Ok(res) => res,
        Err(e) => return Err(format!("Failed to connect: {}", e)),
    };

    let message = "Hello from client! You are connected.";

    // Convert String -> Utf8Bytes and send
    if let Err(e) = stream.send(Message::Text(message.to_string().into())).await {
        eprintln!("Failed to send message: {}", e);
        return Err(format!("Failed to send message: {}", e));
    }
    
    println!("Sent: {}", message);
    let (writer, mut reader) = stream.split();
    
    let mut ws_writer_lock = ws_state.ws_writer.lock().await;
    *ws_writer_lock = Some(writer);
    

    tauri::async_runtime::spawn(async move {
        while let Some(msg) = reader.next().await {
            match msg {
                Ok(msg) => {
                    println!("Received: {}", msg);
                    let _ = on_event.send(msg.to_string());
                }
                Err(e) => {
                    eprintln!("Error receiving message: {}", e);
                    break;
                }
            }
        }
    });
    Ok(())
}

#[derive(Deserialize)]
pub struct FileMessage {
    #[serde(rename = "fileName")]
    pub file_name: String,
    #[serde(rename = "contentType")]
    pub content_type: String,
    #[serde(rename = "fileData")]
    pub data_base64: String,
    #[serde(rename = "sizeByte")]
    pub size_byte: u64,
}

#[tauri::command]
pub async fn handle_file_message(file_msg: FileMessage, ws_state: tauri::State<'_, WebSocketClientState>) -> Result<(), String> {
    let mut ws_writer_lock = ws_state.ws_writer.lock().await;
    if let Some(writer) = &mut *ws_writer_lock {
        print!("Handling file: {}, size: {} bytes", file_msg.file_name, file_msg.size_byte);
        let msg_json = json!({
            "type": "file",
            "fileName": file_msg.file_name,
            "contentType": file_msg.content_type,
            "sizeByte": file_msg.size_byte,
            "dataBase64": file_msg.data_base64,
        });
        let msg_text = msg_json.to_string();
        if let Err(e) = writer.send(Message::Text(msg_text.into())).await {
            eprintln!("Failed to send file message: {}", e);
            return Err(format!("Failed to send file message: {}", e));
        }
        println!("Sent file message for: {}", file_msg.file_name);
        Ok(())
    } else { 
        Err("WebSocket is not connected.".into()) 
    }
}