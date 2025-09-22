import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";

export default function Bridge() {
  const [url, setUrl] = useState<string | null>(null);

  async function startServer() {

  }

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-100 p-6">
      <h1 className="text-2xl font-bold mb-6">Peer A - Signaling Server</h1>

      {!url ? (
        <button
          onClick={startServer}
          className="px-6 py-3 bg-blue-600 text-white rounded-2xl shadow-lg hover:bg-blue-700"
        >
          Start Signaling Server
        </button>
      ) : (
        <div className="flex flex-col items-center gap-4">
          <p className="text-lg text-gray-700">{url}</p>
        </div>
      )}
    </div>
  );
}
