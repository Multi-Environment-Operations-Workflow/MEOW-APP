import { useMemo, useState } from "react";
import {Channel, invoke} from "@tauri-apps/api/core";

function ClientConnectSocket() {
    const [websocketData, setWebsocketData] = useState("");
    const [connectionString, setConnectionString] = useState("ws://127.0.0.1:9001");
    const [error, setError] = useState<string | null>(null);

    const onEvent = useMemo(() => {
        const channel = new Channel(String);
        channel.onmessage = (message) => {
            console.log("Received from backend:", message);
            setWebsocketData(message);
        };
        return channel;
    }, []);

    async function startWebsocket(input: string) {
        setError(null);
        const connectionString = `${input}`;
        if (!connectionString.startsWith("ws://") && !connectionString.startsWith("wss://")) {
            setError("Invalid WebSocket URL. It should start with ws:// or wss://");
            return;
        }
        console.log("Connecting to:", connectionString);
        // Pass channel as `on_event`, matching the Rust parameter name
        await invoke("connect_to_websocket", { connectionString, onEvent });
    }

    return (
        <div className="flex flex-col items-center justify-center min-h-screen bg-gray-100 p-6 gap-4">
            <h1 className="text-2xl font-bold">Peer A - Signaling Server</h1>

            <div className="flex flex-col items-stretch w-full max-w-md">
                <label className="mb-1 text-sm font-medium text-gray-700" htmlFor="connection-input">
                    Websocket URL
                </label>
                <input
                    id="connection-input"
                    type="text"
                    value={connectionString}
                    onChange={(event) => setConnectionString(event.target.value)}
                    placeholder="ws://0.0.0.0:9001"
                    className="mb-3 p-2 border border-gray-300 rounded"
                />
                <button
                    onClick={() => {startWebsocket(connectionString)}}
                    className="px-6 py-3 bg-blue-600 text-white rounded-2xl shadow-lg hover:bg-blue-700 disabled:bg-blue-300"
                    disabled={!connectionString.trim()}
                >
                    Connect
                </button>
            </div>

            {error && <p className="text-sm text-red-600">{error}</p>}

            {websocketData && (
                <div className="flex flex-col items-center gap-2 bg-white shadow rounded p-4 w-full max-w-md">
                    <h2 className="text-lg font-semibold text-gray-700">Latest message</h2>
                    <p className="text-gray-900 break-words w-full text-center">{websocketData}</p>
                </div>
            )}
        </div>
    );
}

export default ClientConnectSocket;