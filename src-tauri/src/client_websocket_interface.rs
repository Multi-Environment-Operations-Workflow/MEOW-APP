use tauri::ipc::Channel;
use tokio_tungstenite::{connect_async, tungstenite::{client::IntoClientRequest, Message}};
use futures_util::{SinkExt, StreamExt};

#[tauri::command]
pub async fn connect_to_websocket(connection_string: String, mut on_event: Channel<String>) {
    let request = connection_string.into_client_request().unwrap();
    let (mut stream, response) = connect_async(request).await.unwrap();

    let message = "Hello from client!";


    // Convert String -> Utf8Bytes and send
    if let Err(e) = stream.send(Message::Text(message.to_string().into())).await {
        eprintln!("Failed to send message: {}", e);
        return;
    }
    
    println!("Sent: {}", message);

    while let Some(msg) = stream.next().await {
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
}
