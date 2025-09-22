import React, { useEffect, useState } from "react";

const Offer: React.FC = () => {
  const [offer, setOffer] = useState<string>("");      // our generated offer
  const [answer, setAnswer] = useState<string>("");    // pasted answer from phone
  const [messages, setMessages] = useState<string[]>([]);
  const [pc, setPc] = useState<RTCPeerConnection | null>(null);
  const [dataChannel, setDataChannel] = useState<RTCDataChannel | null>(null);
  const [inputMessage, setInputMessage] = useState<string>("");

  useEffect(() => {
    // Create the WebRTC connection when component mounts
    const init = async () => {
      const connection = new RTCPeerConnection({
        iceServers: [{ urls: "stun:stun.l.google.com:19302" }],
      });

      // Create data channel for sending messages
      const channel = connection.createDataChannel("data");
      channel.onmessage = (e) =>
        setMessages((prev) => [...prev, `Phone: ${e.data}`]);
      setDataChannel(channel);

      // ICE candidate log (you’d send these to phone if using full signaling)
      connection.onicecandidate = (event) => {
        if (event.candidate) {
          console.log("Desktop ICE candidate:", event.candidate);
        }
      };

      // Generate the offer SDP
      const offerDesc = await connection.createOffer();
      await connection.setLocalDescription(offerDesc);

      setOffer(JSON.stringify(offerDesc));
      setPc(connection);
    };

    init();
  }, []);

  const handleSetAnswer = async () => {
    if (!pc) return;
    const answerDesc = new RTCSessionDescription(JSON.parse(answer));
    await pc.setRemoteDescription(answerDesc);
    alert("✅ Answer set, connection established!");
  };

  const handleSendMessage = () => {
    if (dataChannel && inputMessage.trim() !== "") {
      dataChannel.send(inputMessage);
      setMessages((prev) => [...prev, `You: ${inputMessage}`]);
      setInputMessage("");
    }
  };

  return (
    <div style={{ padding: "1rem" }}>
      <h2>Desktop Offer</h2>
      <p>Copy this offer JSON and send to phone:</p>
      <textarea
        value={offer}
        readOnly
        rows={8}
        style={{ width: "100%", fontSize: "0.8rem" }}
      />

      <h3>Paste Answer from Phone:</h3>
      <textarea
        value={answer}
        onChange={(e) => setAnswer(e.target.value)}
        rows={8}
        style={{ width: "100%", fontSize: "0.8rem" }}
      />
      <button onClick={handleSetAnswer}>Set Answer</button>

      <h3>Messages</h3>
      <div
        style={{
          background: "#eee",
          padding: "1rem",
          height: "150px",
          overflowY: "auto",
        }}
      >
        {messages.map((msg, i) => (
          <div key={i}>{msg}</div>
        ))}
      </div>

      <input
        type="text"
        value={inputMessage}
        onChange={(e) => setInputMessage(e.target.value)}
        placeholder="Type message..."
      />
      <button onClick={handleSendMessage}>Send</button>
    </div>
  );
};

export default Offer;
