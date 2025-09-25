use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use tauri::ipc::Channel;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tauri::command]
pub async fn start_websocket_server(on_event: Channel<String>) -> String {
    let addr = "0.0.0.0:9001";

    let listener = match TcpListener::bind(addr).await {
        Ok(l) => l,
        Err(e) => return format!("Failed to bind to {}: {}", addr, e),
    };

    let on_event = Arc::new(Mutex::new(on_event));

    tokio::spawn(async move {
        println!("WebSocket server listening on {}", addr);

        while let Ok((stream, _)) = listener.accept().await {
            let on_event = Arc::clone(&on_event);

            tokio::spawn(async move {
                let ws_stream = match accept_async(stream).await {
                    Ok(ws) => ws,
                    Err(e) => {
                        eprintln!("WebSocket handshake failed: {}", e);
                        return;
                    }
                };

                println!("New WebSocket connection established");

                let (mut write, mut read) = ws_stream.split();

                while let Some(msg) = read.next().await {
                    match msg {
                        Ok(msg) => {
                            if msg.is_text() || msg.is_binary() {
                                let newmsg = msg.clone();
                                if let Err(e) = write.send(msg).await {
                                    eprintln!("Failed to send message: {}", e);
                                    break;
                                }

                                let text = newmsg.to_text().unwrap_or("").to_string();
                                let on_event = on_event.lock().await;
                                on_event.send(text);
                            }
                        }
                        Err(e) => {
                            eprintln!("Error reading message: {}", e);
                            break;
                        }
                    }
                }

                println!("WebSocket connection closed");
            });
        }
    });

    format!("WebSocket server started on {}", addr)
}
