import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

interface CameraDeviceInfo {
  id: string;
  name: string;
  description: string;
  is_available: boolean;
  supports_formats: CameraFormat[];
}

interface CameraFormat {
  width: number;
  height: number;
  fps: number;
  format_type: string;
}

interface CameraFrame {
  data: number[]; // raw byte array
  width: number;
  height: number;
  format: string;
  timestamp: string;
}

export default function CameraApp() {
  const [cameras, setCameras] = useState<CameraDeviceInfo[]>([]);
  const [photo, setPhoto] = useState<string | null>(null);

  useEffect(() => {
    async function initCamera() {
      await invoke("initialize_camera_system");
      const cams = await invoke<CameraDeviceInfo[]>("get_available_cameras");
      console.log("Available cameras:", cams);
      setCameras(cams);
    }
    initCamera();
  }, []);

  const handleTakePhoto = async () => {
    if (cameras.length === 0) return;

    const format = await invoke<CameraFormat>("get_recommended_format");
    const frame = await invoke<CameraFrame>("capture_single_photo", {
      deviceId: cameras[0].id,
      format: format,
    });

    // Convert raw bytes (Vec<u8>) to base64 for <img>
    const byteArray = new Uint8Array(frame.data);
    const base64 = btoa(String.fromCharCode(...byteArray));
    setPhoto(`data:image/jpeg;base64,${base64}`);
  };

  return (
    <main className="container">
      <h1>CrabCamera Demo</h1>
      <button onClick={handleTakePhoto}>Take Photo</button>

      {photo && (
        <div>
          <h2>Captured Image:</h2>
          <img src={photo} alt="Captured" />
        </div>
      )}
    </main>
  );
}
