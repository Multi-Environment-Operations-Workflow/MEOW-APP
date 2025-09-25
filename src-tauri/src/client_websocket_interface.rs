use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use tauri::ipc::Channel;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tauri::command]
pub async fn connect_to_websocket(connection_string: String, on_event: Channel<String>) -> String {
    let url = Url::parse(connection_string).expect("Can't parse WebSocket URL");

    match connect_async(url).await {
        Ok((mut ws_stream, _)) => {
            println!("Connected to WebSocket server");

            // Send a message
            let message = "Hello from client!";
            if let Err(e) = ws_stream.send(tokio_tungstenite::tungstenite::Message::Text(message.to_string())).await {
                eprintln!("Failed to send message: {}", e);
                return;
            }
            

            println!("Sent: {}", message);

            // Read response
            while let Some(msg) = ws_stream.next().await {
                match msg {
                    Ok(msg) => {
                        println!("Received: {}", msg);
                    }
                    Err(e) => {
                        eprintln!("Error receiving message: {}", e);
                        break;
                    }
                }
            }

            println!("Connection closed");
        }
        Err(e) => {
            eprintln!("Failed to connect to WebSocket server: {}", e);
        }
    }
}



use tokio_tungstenite::connect_async;
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use url::Url;

#[tokio::main]
async fn main() {
    
}
