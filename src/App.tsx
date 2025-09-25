import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { BrowserRouter as Router, Routes, Route, Link } from "react-router-dom";
import Home from "./home/home.tsx";
import ConnectionInterface from "./connection-interface/connection-interface.tsx";
import Bridge from "./bridge/bridge.tsx";
import ActivateMicrophone from "./activate-microphone/activate-microphone.tsx";
import VideoPreview from "./activate-video/VideoPreview.tsx";

function App() {
    const [_greetMsg, setGreetMsg] = useState("");
    const [name, _setName] = useState("");

    async function greet() {
        setGreetMsg(await invoke("greet", { name }));
    }

    return (
        <main>
            <Router>
                <nav>
                    <Link to="/">Home</Link> | <Link to="/connect">About</Link> |{" "}
                    <Link to="/bridge">bridge</Link> |{" "}
                    <Link to="/microphone">microphone</Link> |{" "}
                    <Link to="/video">video</Link>
                </nav>
                <Routes>
                    <Route path="/" element={<Home />} />
                    <Route path="/connect" element={<ConnectionInterface />} />
                    <Route path="/bridge" element={<Bridge />} />
                    <Route path="/microphone" element={<ActivateMicrophone />} />
                    <Route path="/video" element={<VideoPreview />}></Route>
                </Routes>
            </Router>
        </main>
    );
}

export default App;
