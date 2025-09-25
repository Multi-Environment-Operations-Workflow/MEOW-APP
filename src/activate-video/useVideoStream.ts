import { useEffect, useRef } from "react";

export function useVideoStream(onChunk: (chunk: Blob) => void) {
  const videoRef = useRef<HTMLVideoElement | null>(null);
  const mediaRecorderRef = useRef<MediaRecorder | null>(null);

  useEffect(() => {
    let recorder: MediaRecorder;

    navigator.mediaDevices.getUserMedia({ video: true, audio: true }).then((stream) => {
      if (videoRef.current) {
        videoRef.current.srcObject = stream; // preview
      }

      recorder = new MediaRecorder(stream, {
        mimeType: "video/webm; codecs=vp8,opus",
      });

      recorder.ondataavailable = (event) => {
        if (event.data.size > 0) {
          onChunk(event.data); // pass chunk to consumer
        }
      };

      recorder.start(500); // send ~2 chunks per second
      mediaRecorderRef.current = recorder;
    });

    return () => {
      recorder?.stop();
    };
  }, [onChunk]);

  return { videoRef };
}
