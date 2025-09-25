use meow_app_lib::bridge::start_websocket_server;
#[cfg(test)]
mod tests {
    use super::*;
    use futures_util::{SinkExt, StreamExt};
    use tauri::ipc::Channel;
    use tokio::sync::mpsc;
    use tokio_tungstenite::connect_async;
    use url::Url;

    // Helper to create a fake tauri::ipc::Channel using an mpsc channel
    fn mock_channel() -> (Channel<String>, mpsc::Receiver<String>) {
        let (tx, rx) = mpsc::channel::<String>(10);
        (Channel::new(tx), rx)
    }

    #[tokio::test]
    async fn test_websocket_echo() {
        let (channel, mut rx) = mock_channel();

        // Start server
        let _ = start_websocket_server(channel).await;

        // Connect client
        let url = Url::parse("ws://127.0.0.1:9001").unwrap();
        let (mut ws_stream, _) = connect_async(url).await.expect("Failed to connect");

        // Send message
        let msg = "hello server";
        ws_stream.send(tokio_tungstenite::tungstenite::Message::Text(msg.to_string()))
            .await
            .unwrap();

        // Expect echo back
        if let Some(Ok(tokio_tungstenite::tungstenite::Message::Text(reply))) = ws_stream.next().await {
            assert_eq!(reply, msg);
        } else {
            panic!("Did not receive echo");
        }

        // Expect event via Channel
        if let Some(event) = rx.recv().await {
            assert_eq!(event, msg);
        } else {
            panic!("Did not receive event on Channel");
        }
    }
}
