let pc: RTCPeerConnection;

function initPeer() {
  pc = new RTCPeerConnection();

  // Handle data channel
  pc.ondatachannel = (event) => {
    const channel = event.channel;
    channel.onmessage = (e) => console.log("Got:", e.data);
  };

  // Handle remote media
  pc.ontrack = (event) => {
    const video = document.getElementById("remoteVideo") as HTMLVideoElement;
    video.srcObject = event.streams[0];
  };

  return pc;
}

async function createOfferQR() {
  const pc = initPeer();

  // Add local media
  const stream = await navigator.mediaDevices.getUserMedia({
    video: true,
    audio: true,
  });
  stream.getTracks().forEach((track) => pc.addTrack(track, stream));

  // Create & set offer
  const offer = await pc.createOffer();
  await pc.setLocalDescription(offer);

  // Wait for ICE candidates to gather fully
  await waitForIceGatheringComplete(pc);

  // Encode to QR
  const offerString = btoa(JSON.stringify(pc.localDescription));
  return offerString; // pass to QR generator
}

function waitForIceGatheringComplete(pc: RTCPeerConnection) {
  return new Promise<void>((resolve) => {
    if (pc.iceGatheringState === "complete") resolve();
    else
      pc.onicegatheringstatechange = () => {
        if (pc.iceGatheringState === "complete") resolve();
      };
  });
}

async function handleOffer(offerString: string) {
  const pc = initPeer();

  const offer = JSON.parse(atob(offerString));
  await pc.setRemoteDescription(offer);

  const answer = await pc.createAnswer();
  await pc.setLocalDescription(answer);

  await waitForIceGatheringComplete(pc);

  // Encode as QR
  const answerString = btoa(JSON.stringify(pc.localDescription));
  return answerString; // show as QR for Device A
}
