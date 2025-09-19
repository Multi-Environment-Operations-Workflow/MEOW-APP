use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, StreamConfig};
use hound;
use std::sync::{Arc, Mutex};
use tauri::command;

/// # What it does
/// Starts simple recorder for 5 secounds.
/// Intern buffer is `f32` i [-1.0, 1.0] no matter device-sampleformat.
///
/// # How to use
/// Call from frontend via `invoke("start_mic_recording")`.
#[allow(dead_code)] // rust invoker does not count as usage, so avoid warning.
#[command] // This attribute makes the function callable from the frontend.
pub fn start_mic_recording() -> Result<(), String> {
    // returns a stream we can use. Drop it to stop recording.
    let (input_stream, samples, config) = match new_live_bitstream() {
        Ok(triple) => {
            println!("Bitstream: OK");
            triple
        }
        Err(e) => {
            eprintln!("Bitstream error: {e}");
            return Err(e);
        }
    };
    println!("Recording... (demo: 5 seconds)");
    // DEMO: records 5 sec.
    std::thread::sleep(std::time::Duration::from_secs(5));
    // Stop via drop.
    drop(input_stream);
    // Save buffer as WAV (16-bit PCM)
    let buf = samples
        .lock()
        .map_err(|_| "Buffer lock failed".to_string())?;

    let spec = hound::WavSpec {
        channels: config.channels,
        sample_rate: config.sample_rate.0,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create("recorded.wav", spec).map_err(|e| e.to_string())?;
    for &sample in buf.iter() {
        // Clamp to [-1,1] and scale to i16
        let val = (sample.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
        writer.write_sample(val).map_err(|e| e.to_string())?;
    }

    writer.finalize().map_err(|e| e.to_string())?;
    println!("Saved recorded.wav with {} samples", buf.len());
    Ok(())
}




/// # How to use
///  Call to get a live input stream and a shared buffer.
/// Returns: (stream, delt f32-buffer, config).
pub fn new_live_bitstream() -> Result<(cpal::Stream, Arc<Mutex<Vec<f32>>>, StreamConfig), String> {
    //Choses default input device and config
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .ok_or("No input device available")?;
    let input_config = device
        .default_input_config()
        .map_err(|e| e.to_string())?;
    let sample_format = input_config.sample_format();
    let config: StreamConfig = input_config.config();

    let sink = Arc::new(Mutex::new(Vec::<f32>::new()));

    // Builds a steam that converts any input sample format to f32 in [-1, 1] (Helperfunction below)
    let stream = build_input_stream_to_f32(&device, &config, sample_format, sink.clone())?;

    // Start recording
    stream.play().map_err(|e| e.to_string())?;

    Ok((stream, sink, config))
}



/// Helperfunction to build a stream that converts any input sample format to f32 in [-1, 1].
/// Because computers can have different sample formats, we need to handle them all.
fn build_input_stream_to_f32(
    device: &cpal::Device,
    config: &StreamConfig,
    sample_format: SampleFormat,
    sink: Arc<Mutex<Vec<f32>>>,
) -> Result<cpal::Stream, String> {
    let err_fn = |err| eprintln!("Input error: {:?}", err);
    match sample_format {
        SampleFormat::F32 => device
            .build_input_stream(
                config,
                {
                    let sink = sink.clone();
                    move |data: &[f32], _| {
                        if let Ok(mut buf) = sink.lock() {
                            buf.extend_from_slice(data);
                        }
                    }
                },
                err_fn,
                None,
            )
            .map_err(|e| e.to_string()),

        SampleFormat::I16 => device
            .build_input_stream(
                config,
                {
                    let sink = sink.clone();
                    move |data: &[i16], _| {
                        if let Ok(mut buf) = sink.lock() {
                            // i16 [-32768, 32767] -> f32 ~[-1, 1)
                            buf.extend(data.iter().map(|&s| (s as f32) / 32768.0));
                        }
                    }
                },
                err_fn,
                None,
            )
            .map_err(|e| e.to_string()),

        SampleFormat::U16 => device
            .build_input_stream(
                config,
                {
                    let sink = sink.clone();
                    move |data: &[u16], _| {
                        if let Ok(mut buf) = sink.lock() {
                            // u16 [0, 65535] -> centrer -> f32 [-1, 1]
                            buf.extend(data.iter().map(|&s| (s as f32 - 32768.0) / 32768.0));
                        }
                    }
                },
                err_fn,
                None,
            )
            .map_err(|e| e.to_string()),

        _ => Err("Unsupported sample format".into()),
    }
}
