import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

function ConnectionInterface() {
    const [qrBase64, setQrBase64] = useState("");

    useEffect(() => {
        async function fetchQr() {
            const base64 = await invoke("generate_qr_code");
            setQrBase64(base64);
        }
        fetchQr();
    }, []);

    return (
        <main className="container">
            <h1>Hej</h1>
            {qrBase64 && (
                <img width="200px" src={`data:image/png;base64,${qrBase64}`} alt="QR Code" />
            )}
        </main>
    );
}

export default ConnectionInterface;
