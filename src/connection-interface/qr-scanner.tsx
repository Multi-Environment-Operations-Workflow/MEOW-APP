import { useEffect, useState } from "react";
import {
  scan,
  Format,
  requestPermissions,
} from "@tauri-apps/plugin-barcode-scanner";

function QrScanner() {
  useEffect(() => {
    async function scanQr() {
      await requestPermissions();

      try {
        let result = await scan({ windowed: true, formats: [Format.QRCode] });
        console.log("scan result:", JSON.stringify(result));
        setQrInfo(result.content);
      } catch (e) {
        console.error("Scan error:", e);
      }
    }
    scanQr();
  }, []);

  return <div width="200px" height="200px"></div>;
}

export default QrScanner;
