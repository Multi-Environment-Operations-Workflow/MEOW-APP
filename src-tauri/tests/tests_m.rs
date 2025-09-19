// IMPORTANT: give the module a DIFFERENT name than this file to avoid circular inclusion.
// Point it to the implementation file in src/.
#[path = "../src/microphone.rs"]
mod microphone_impl;

use std::time::{Duration, Instant};

// Because the file is compiled as part of THIS test crate,
use microphone_impl::new_live_bitstream;
#[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows", target_os = "android"))]
#[test]
fn test_new_live_bitstream_creates_stream() {
    // Optional guard to only run locally when you want:
    if std::env::var("AUDIO_TEST").ok().as_deref() != Some("1") {
        eprintln!("Skipping (set AUDIO_TEST=1 to enable).");
        return;
    }

    let (stream, buffer, cfg) = match new_live_bitstream() {
        Ok(v) => v,
        Err(e) => { eprintln!("Skipped: {e}"); return; }
    };

    // Wait for a small chunk of samples (≈50ms) up to 3s
    let target = (cfg.sample_rate.0 as usize * cfg.channels as usize) / 20;
    let start = Instant::now();
    while start.elapsed() < Duration::from_secs(3) {
        if buffer.lock().unwrap().len() >= target {
            drop(stream);
            return; // success
        }
        std::thread::sleep(Duration::from_millis(50));
    }

    drop(stream);
    panic!("no audio captured within timeout");
}
