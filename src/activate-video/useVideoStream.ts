import { useEffect, useRef, useState } from "react";

export function useVideoStream(onChunk: (chunk: Blob) => void) {
  const videoRef = useRef<HTMLVideoElement | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let recorder: MediaRecorder;

    navigator.mediaDevices.getUserMedia({ video: true, audio: true })
      .then((stream) => {
        if (videoRef.current) {
          videoRef.current.srcObject = stream;
        }

        recorder = new MediaRecorder(stream, {
          mimeType: "video/webm; codecs=vp8,opus",
        });

        recorder.ondataavailable = (event) => {
          if (event.data.size > 0) {
            onChunk(event.data);
          }
        };

        recorder.start(500);
      })
      .catch((err) => {
        console.error("Permission error:", err);
        setError("Camera or microphone access denied.");
        requestCameraPermission();
      });

    return () => {
      recorder?.stop();
    };
  }, [onChunk]);

  return { videoRef, error };
}

async function requestCameraPermission() {
  try {
    const status = await navigator.permissions.query({ name: "camera" as PermissionName });
    console.log("Camera permission status:", status.state);

    if (status.state === "denied") {
      alert("Camera access was blocked. Please enable it in your browser/Tauri settings.");
      return;
    }

    const stream = await navigator.mediaDevices.getUserMedia({ video: true, audio: true });
    return stream;
  } catch (err) {
    console.error("Permission request failed:", err);
  }
}
