import { useMemo, useState } from "react";
import {Channel, invoke} from "@tauri-apps/api/core";

function ClientConnectSocket() {
    const [websocketData, setWebsocketData] = useState("");
    const [connectionString, setConnectionString] = useState("");

    const [selectedFile, setSelectedFile] = useState<File | null>(null);
    const [isConnected, setIsConnected] = useState<boolean | null>(null);

    async function handleFilePicker(event: React.ChangeEvent<HTMLInputElement>) {
        const files = event.target.files;
        if (!files || files.length === 0) {
            console.log("Not correct file format.");
            return null;
        }
        const file = files[0];
        setSelectedFile(file);
    }

    async function getDataInBase64(file: File): Promise<string> {
        return new Promise((resolve) => {
            const reader = new FileReader();
            reader.onload = () => {
                const result = reader.result as string;
                const [, base64 = ""] = result.split(",");
                resolve(base64);
            }
            reader.readAsDataURL(file);
        });
    }
    async function handleFileUpload() {
        if (selectedFile == null) {
            console.log("Proper file not selected.");
            return;
        }
        const fileData = await getDataInBase64(selectedFile);
        const data = {
            fileName: selectedFile.name,
            contentType: selectedFile.type || "application/octet-stream",
            fileData,
            sizeByte: selectedFile.size,
        };
        await invoke("handle_file_message", { fileMsg: data });
        console.log("File sent to backend:", selectedFile.name);
    }

    const onEvent = useMemo(() => {
        const channel = new Channel(String);
        channel.onmessage = (message) => {
            console.log("Received from backend:", message);
            setWebsocketData(message);
            if(message.includes("connected")) {
                setIsConnected(true);
            } else if (message.includes("disconnected")) {
                setIsConnected(false);
            }
        };
        return channel;
    }, []);

    async function startWebsocket(input: string) {
        const connectionString = `${input}`;
        console.log("Connecting to:", connectionString);
        await invoke("connect_to_websocket", { connectionString, onEvent });
    }

    return (
        <div>
            <h1>Peer A - Signaling Server</h1>

            <div>
                <label>
                    Websocket URL
                </label>
                <input
                    id="connection-input"
                    type="text"
                    value={connectionString}
                    onChange={(event) => setConnectionString(event.target.value)}
                    placeholder="ws://0.0.0.0:9001"
                />
                <button
                    onClick={() => {startWebsocket(connectionString)}}
                    className=""
                    disabled={!connectionString.trim()}
                >
                    Connect
                </button>
            </div>
            <div>
                <input
                disabled = {!isConnected || selectedFile != null}
                type="file" 
                onChange={handleFilePicker}/>
            </div>
            <div>
                <button onClick={handleFileUpload}>Send</button>
            </div>
            {websocketData && (
                <div>
                    <h2>Latest message</h2>
                    <p>{websocketData}</p>
                </div>
            )}
        </div>
    );
}

export default ClientConnectSocket;