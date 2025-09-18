import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./connection-interface.css";

async function gen_qr() {
    await invoke("generate_qr_code")
}

function ConnectionInterface() {
    const  [generate_qr_code] = useState("");
    //const [name, setName] = useState("");

    return (
        <main className="container">
            Hej {gen_qr()}
        </main>
    );
}

export default ConnectionInterface;