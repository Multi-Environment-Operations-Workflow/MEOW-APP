use tauri::{async_runtime::Receiver, ipc::Channel};
use tokio_websocket_server::socket::{WebSocketMessage, WebsocketServer};

#[tauri::command]
pub async fn start_websocket_server(on_event: Channel<String>) {
    let hostname = "0.0.0.0";
    let port = "9001";
    let ws_server =  WebsocketServer::new(hostname.to_string(), port.to_string(), None, None);
    

    let ws_sender = ws_server.clone();
    let message_receiver = ws_server.start().await;

    tokio::spawn(async move {
        handle_websocket_messages(message_receiver, ws_sender).await;
    });

    let s = format!(
        "WebSocket server started on http://{host}:{port}",
        host = hostname,
        port = port
    );
    println!("{}", s);
    on_event.send(s);
}


async fn handle_websocket_messages(mut message_receiver: Receiver<(String, WebSocketMessage)>, ws_sender: WebsocketServer) {
    while let Some((client_id, message)) = message_receiver.recv().await {
        match message {
            WebSocketMessage::Text(text) => {
                println!("Received from {}: {}", client_id, text);
                
                // Echo the message back
                let response = WebSocketMessage::Text(format!("Echo: {}", text));
                ws_sender.send_to_client(client_id, response).await.unwrap();
            },
            // Handle other message types...
            _ => {}
        }
    }
}