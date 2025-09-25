import { useVideoStream } from "./useVideoStream";

const VideoPreview = () => {
  const { videoRef } = useVideoStream((chunk) => {
    //Insert logic for handling video chunks here
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
