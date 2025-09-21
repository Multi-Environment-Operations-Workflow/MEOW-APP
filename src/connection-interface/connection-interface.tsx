import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import {
  scan,
  cancel,
  Format,
  requestPermissions,
} from "@tauri-apps/plugin-barcode-scanner";

function ConnectionInterface() {
  const [qrBase64, setQrBase64] = useState("");
  const [qrInfo, setQrInfo] = useState("");
  const [scanning, setScanning] = useState(false);

  useEffect(() => {
    async function fetchQr() {
      const base64 = await invoke("generate_qr_code");
      setQrBase64(base64);
    }
    fetchQr();
  }, []);

  async function scanQr() {
    await requestPermissions();
    setScanning(true);

    try {
      let result = await scan({ windowed: true, formats: [Format.QRCode] });
      console.log("scan result:", JSON.stringify(result));
      setQrInfo(result.content);
    } catch (e) {
      console.error("Scan error:", e);
    } finally {
      setScanning(false);
    }
  }

  async function cancleScan() {
    await cancel();
    setScanning(false);
  }

  return (
    <main className="container">
      <h1>Hej</h1>
      {qrBase64 && (
        <img
          width="200px"
          src={`data:image/png;base64,${qrBase64}`}
          alt="QR Code"
        />
      )}
      {qrInfo && (
        <div>
          <h3>Scanned Content:</h3>
          <p>{qrInfo}</p>
        </div>
      )}

      <button onClick={scanQr}>Start Scan</button>

      {/* Scanner overlay */}
      {scanning && (
        <div className="absolute inset-0 bg-black/70 flex flex-col items-center justify-center z-50">
          {/* Cutout effect */}
          <div
            className="absolute inset-0"
            style={{
              clipPath: `polygon(
                0 0, 0 100%, 100% 100%, 100% 0,
                0 0,
                20% 30%, 80% 30%, 80% 70%, 20% 70%, 20% 30%
              )`,
              backgroundColor: "rgba(0,0,0,0.7)",
            }}
          />

          {/* Border around scanning area */}
          <div className="absolute top-[30%] left-[20%] w-[60%] h-[40%] border-4 border-green-500 rounded-lg pointer-events-none" />

          {/* Cancel button */}
          <button
            onClick={cancleScan}
            className="mt-[60%] px-4 py-2 bg-red-600 text-white rounded-lg shadow relative z-10"
          >
            Cancel
          </button>
        </div>
      )}
    </main>
  );
}

export default ConnectionInterface;
