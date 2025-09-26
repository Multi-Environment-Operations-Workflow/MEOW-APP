use meow_app_lib::bridge::start_websocket_server;
#[cfg(test)]
mod tests {
    use super::*;
    use futures_util::{SinkExt, StreamExt};
    use tauri::ipc::Channel;
    use tokio::sync::mpsc;
    use tokio_tungstenite::connect_async;
    use std::time::Duration;
    use tokio::time::sleep;

    // Simple test that just focuses on the WebSocket echo functionality
    #[tokio::test]  
    async fn test_websocket_echo() {
        // Create a dummy channel that we won't actually check
        // This is just to satisfy the function signature
        let (_dummy_tx, _dummy_rx) = mpsc::unbounded_channel::<String>();
        let dummy_channel = Channel::new(move |_response| -> tauri::Result<()> {
            Ok(()) // Do nothing with the channel events for this test
        });

        // Start the WebSocket server
        let _ = start_websocket_server(dummy_channel).await;

        // Give the server time to start listening
        sleep(Duration::from_millis(500)).await;

        // Connect client
        let url = "ws://127.0.0.1:9001";
        let (mut ws_stream, _) = connect_async(url).await.expect("Failed to connect");

        // Send message
        let msg = "hello server";
        ws_stream.send(tokio_tungstenite::tungstenite::Message::Text(msg.to_string().into()))
            .await
            .unwrap();

        // Expect echo back with timeout
        let echo_result = tokio::time::timeout(Duration::from_secs(2), ws_stream.next()).await;
        match echo_result {
            Ok(Some(Ok(tokio_tungstenite::tungstenite::Message::Text(reply)))) => {
                assert_eq!(reply.as_str(), msg);
                println!(" Received echo: {}", reply);
            }
            _ =>{panic! (" Did not receive expected echo message");}
        }
    }
}
