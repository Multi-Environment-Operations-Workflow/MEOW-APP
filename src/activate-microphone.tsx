import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

function ActivateMicrophone() {
  const [message, setMessage] = useState("");

  const handleButtonClick = async () => {
    await invoke("microphone_access");
    console.log("Microphone function invoked!");
    setMessage("Microphone activated successfully!");
  };

  return (
    <div>
      <h1>Activate Microphone</h1>
      <button onClick={handleButtonClick}>Activate Microphone</button>
      {message && <p>{message}</p>}
    </div>
  );
}

export default ActivateMicrophone;