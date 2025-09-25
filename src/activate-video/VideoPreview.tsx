import { useVideoStream } from "./useVideoStream";

const VideoPreview = () => {
  // Here we just ignore chunks (or log them),
  // since we only want live preview.
  const { videoRef } = useVideoStream((chunk) => {
    console.log("Got chunk:", chunk);
  });

  return (
    <div>
      <h2>Live Preview</h2>
      <video
        ref={videoRef}
        autoPlay
        muted
        style={{ width: "400px", borderRadius: "12px" }}
      />
    </div>
  );
};

export default VideoPreview;
