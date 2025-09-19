import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { scan, Format, requestPermissions } from '@tauri-apps/plugin-barcode-scanner';

function ConnectionInterface() {
    const [qrBase64, setQrBase64] = useState("");

    useEffect(() => {
        async function fetchQr() {
            const base64 = await invoke("generate_qr_code");
            setQrBase64(base64);
        }

        async function scanQr() {
            await requestPermissions();

            try {
                let result = await scan({ windowed: true, formats: [Format.QRCode] });
                console.log("scan result:", result);
            } catch (e) {
                console.error("Scan error:", e);
            }
        }
        scanQr();
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
