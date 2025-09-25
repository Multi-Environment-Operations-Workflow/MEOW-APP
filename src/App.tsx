import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import ConnectionInterface from "./connection-interface/connection-interface.tsx";
import Bridge from "./bridge/bridge.tsx";
import ActivateMicrophone from "./activate-microphone/activate-microphone.tsx";

function App() {
    const [greetMsg, setGreetMsg] = useState("");
    const [name, setName] = useState("");

    async function greet() {
        setGreetMsg(await invoke("greet", { name }));
    }

    return (
        <main className="container">
      
                 <Router>
            <nav>
                <Link to="/">Home</Link> | <Link to="/connect">About</Link>
            </nav>
            <Routes>
                <Route path="/" element={<Home />} />
                <Route path="/connect" element={<ConnectionInterface />} />
            </Routes>
        </Router>
              {HostStartSocket()}
            {ClientConnectSocket()}
            {ActivateMicrophone()}
            {ConnectionInterface()}
        </main>
    );
}

export default App;
