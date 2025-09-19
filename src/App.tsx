import { BrowserRouter as Router, Routes, Route, Link } from "react-router-dom";
import Home from "./home/home.tsx";
import ConnectionInterface from "./connection-interface/connection-interface.tsx";

export default function App() {
    return (
        <main>
        <Router>
            <nav>
                <Link to="/">Home</Link> | <Link to="/connect">About</Link>
            </nav>
            <Routes>
                <Route path="/" element={<Home />} />
                <Route path="/connect" element={<ConnectionInterface />} />
            </Routes>
        </Router>
        </main>
    );
}
