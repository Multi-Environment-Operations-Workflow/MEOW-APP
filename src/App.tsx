import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import Bridge from "./bridge/bridge.tsx";
import { BridgeContextProvider } from "./bridge/bridgeContext.tsx";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <main className="container">
      <BridgeContextProvider>
        <Bridge />
      </BridgeContextProvider>
    </main>
  );
}

export default App;
