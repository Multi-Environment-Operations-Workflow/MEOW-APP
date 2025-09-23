use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::StreamExt;
use std::net::SocketAddr;

pub async fn start_ws_server() {
    let app = Router::new().route("/ws", get(ws_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Starting WS server at ws://{}", addr);

    // Run until process exits (or server is dropped)
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.next().await {
        match msg {
            Message::Text(t) => {
                if socket
                    .send(Message::Text(format!("Echo: {}", t).into()))
                    .await
                    .is_err()
                {
                    return;
                }
            }
            Message::Close(_) => return,
            _ => {}
        }
    }
}

#[tauri::command]
pub async fn start_server() {
    tauri::async_runtime::spawn(async {
        start_ws_server().await;
    });
}

use tokio::io::*;
use web_socket::*;

async fn example<IO>(mut ws: WebSocket<IO>) -> Result<()>
where
    IO: Unpin + AsyncRead + AsyncWrite,
{
    for _ in 0..3 {
        ws.send("Copy Cat!").await?;

        match ws.recv_event().await? {
            Event::Data { ty, data } => {
                assert!(matches!(ty, DataType::Complete(MessageType::Text)));
                assert_eq!(&*data, b"Copy Cat!");
            }
            Event::Ping(data) => ws.send_pong(data).await?,
            Event::Pong(..) => {}
            Event::Error(..) => return ws.close(CloseCode::ProtocolError).await,
            Event::Close { .. } => return ws.close(()).await,
        }
    }
    ws.close("bye!").await
}