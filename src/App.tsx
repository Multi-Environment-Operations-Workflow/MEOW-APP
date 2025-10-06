import { BrowserRouter as Router, Routes, Route, Link } from "react-router-dom";
import ConnectionInterface from "./connection-interface/connection-interface.tsx";
import Bridge from "./bridge/bridge.tsx";
import ActivateMicrophone from "./activate-microphone/activate-microphone.tsx";

function App() {
    return (
        <main>
            <Router>
                <nav>
                    <Link to="/">Home</Link>|{" "}
                    <Link to="/connect">About</Link> |{" "}
                    <Link to="/bridge">bridge</Link> |{" "}
                    <Link to="/microphone">microphone</Link>
                </nav>
                <Routes>
                    <Route path="/" element={<h1>Home page. Brug nav bar til at komme til din component.</h1>} />
                    <Route path="/connect" element={<ConnectionInterface />} />
                    <Route path="/bridge" element={<Bridge />} />
                    <Route path="/microphone" element={<ActivateMicrophone />} />
                </Routes>
            </Router>
        </main>
    );
}

export default App;
