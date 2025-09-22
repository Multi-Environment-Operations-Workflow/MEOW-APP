import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { createContext, useEffect, useState } from "react";

type BridgeContextProviderProps = {
  children: React.ReactNode;
};

type BridgeContextType = {};

export const BridgeContext = createContext({} as BridgeContextType);

export const BridgeContextProvider = ({
  children,
}: BridgeContextProviderProps) => {
  const [pc, _] = useState(new RTCPeerConnection());
  const [url, setUrl] = useState<string | null>(null);

  const startBroker = async () => {
    const offer = await pc.createOffer();
    pc.setLocalDescription(offer);

    const result = await invoke<string>("start_server", {
      offer: JSON.stringify(offer),
    });
    setUrl(result);

    listen("answer-ready", async (event) => {
      const answer = event.payload as RTCSessionDescriptionInit;
      await pc.setRemoteDescription(answer);
    });

    //ToDo unsubscribe after Event fired
  };
  const stopBroker = () => {
    //ToDo implement function
  };

  const handshake = async () => {
    if (url) {
      const offerRes = await fetch(url.concat("/init"), {
        method: "GET",
      });

      await pc.setRemoteDescription(await offerRes.json());

      const answer = await pc.createAnswer();
      await pc.setLocalDescription(answer);

      fetch(url.concat("/answer"), {
        method: "POST",
        headers: { "content-type": "application/plain;charset=UTF-8" },
        body: JSON.stringify(answer),
      });
    }
  };

  return (
    <BridgeContext.Provider value={{ startBroker, url, pc }}>
      {children}
    </BridgeContext.Provider>
  );
};
