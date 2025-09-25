import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { BrowserRouter as Router, Routes, Route, Link } from "react-router-dom";
import Home from "./home/home.tsx";
import ConnectionInterface from "./connection-interface/connection-interface.tsx";
import ActivateMicrophone from "./activate-microphone/activate-microphone.tsx";
import HostStartSocket from "./host-start-socktet/host-start-socket.tsx";
import ClientConnectSocket from "./host-client-socket/client-connect-socket.tsx";

function App() {
    const [_greetMsg, setGreetMsg] = useState("");
    const [name, _setName] = useState("");

    async function greet() {
        setGreetMsg(await invoke("greet", { name }));
    }

    return (
        <main className="container">
            {HostStartSocket()}
            {ClientConnectSocket()}
            {ActivateMicrophone()}
            {ConnectionInterface()}
            <Router>
                <nav>
                    <Link to="/">Home</Link> | <Link to="/connect">About</Link> |{" "}
                    <Link to="/host-socket">Host Socket</Link> |{" "}
                    <Link to="/connect-socket">Connect socket</Link>
                    <Link to="/microphone">microphone</Link>
                </nav>
                <Routes>
                    <Route path="/" element={<Home />} />
                    <Route path="/host-socket" element={<HostStartSocket />} />
                    <Route path="/connect-socket" element={<ClientConnectSocket />} />
                    <Route path="/microphone" element={<ActivateMicrophone />} />
                </Routes>
            </Router>
        </main>
    );
}

export default App;
