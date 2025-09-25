import { useState } from "react";
import {Channel, invoke} from "@tauri-apps/api/core";

function ClientConnectSocket() {
    const [websocketData, setWebsocketData] = useState("");

    // Create the channel
    const onEvent = new Channel(String);
    const connectionString = "10"

    // Handle incoming messages
    onEvent.onmessage = (message) => {
        console.log("Received from backend:", message);
        setWebsocketData(message);
    };

    async function startWebsocket() {
        // Pass channel as `on_event`, matching the Rust parameter name
        await invoke("connect_to_websocket", { connectionString, onEvent });
    }

    return (
        <div className="flex flex-col items-center justify-center min-h-screen bg-gray-100 p-6">
            <h1 className="text-2xl font-bold mb-6">Peer A - Signaling Server</h1>
            {websocketData}
            {!websocketData ? (
                <button
                    onClick={startWebsocket}
                    className="px-6 py-3 bg-blue-600 text-white rounded-2xl shadow-lg hover:bg-blue-700"
                >
                    Start Signaling Server
                </button>
            ) : (
                <div className="flex flex-col items-center gap-4">
                    <p className="text-lg text-gray-700"></p>
                </div>
            )}
        </div>
    );
}

export default ClientConnectSocket;