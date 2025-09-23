import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { createContext, useContext, useEffect, useState } from "react";

type BridgeContextProviderProps = {
  children: React.ReactNode;
};

type BridgeContextType = {
  startBroker: () => Promise<void>;
  handshake: () => Promise<void>;
  url: string | null;
  pc: RTCPeerConnection;
};


export const BridgeContext = createContext({} as BridgeContextType);
export const useBridge = () => useContext(BridgeContext);

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
    //ToDo Look at whether ip is encoded in the offer
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
    <BridgeContext.Provider value={{ startBroker, handshake, url, pc }}>
      {children}
    </BridgeContext.Provider>
  );
};
