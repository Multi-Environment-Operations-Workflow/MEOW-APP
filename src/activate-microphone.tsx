import { invoke } from "@tauri-apps/api/core";

/**
 * Handles microphone recording logic for GUI.
 * 'invoke' calls rust funktion. in libs.rs ---> tauri::generate_handler![microphone_fn::start_mic_recording
 */
const handleMicTest = async () => {
  try {
    await invoke('start_mic_recording');
  } catch (e) {
    alert('Failed to start microphone test: ' + e);
  }
};

export default function ActivateMicrophone() {
  return (
    <main className="container">
      <button onClick={handleMicTest} style={{ margin: '1em', padding: '1em' }}>
        Start Microphone Test
      </button>
    </main>
  );
}