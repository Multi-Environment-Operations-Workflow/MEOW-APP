import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

function ConnectionInterface() {
    const [qrPath, setQrPath] = useState("");

    useEffect(() => {
        async function fetchQr() {
            const path = await invoke("generate_qr_code");
            setQrPath(path);
        }
        fetchQr();
    }, []);

    return (
        <main className="container">
            <h1>Hej</h1>
            {qrPath && <img src={qrPath} alt="QR Code" />}
        </main>
    );
}

export default ConnectionInterface;
