use tauri::command;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::SampleFormat;
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};

use mp3lame_encoder::{Builder, InterleavedPcm, Bitrate, Quality, EncoderFlush, FlushNoGap};
use cpal::Sample; // for i16::from_sample

#[command]
pub fn microphone_access() {
    // 1) Input device/config
    let host = cpal::default_host();
    let device = host.default_input_device().expect("no input device");
    let supported = device.default_input_config().expect("no default input config");
    let config: cpal::StreamConfig = supported.clone().into();

    println!("Input device: {}", device.name().unwrap_or_default());
    println!("Default input config: {:?}", supported);

    // 2) Capture interleaved i16 samples
    let buf = Arc::new(Mutex::new(Vec::<i16>::new()));
    let buf_cb = buf.clone();

    let stream = match supported.sample_format() {
        SampleFormat::F32 => device.build_input_stream(
            &config,
            move |data: &[f32], _| buf_cb.lock().unwrap().extend(data.iter().map(|&x| i16::from_sample(x))),
            move |e| eprintln!("stream error: {e}"),
            None,
        ),
        SampleFormat::I16 => device.build_input_stream(
            &config,
            move |data: &[i16], _| buf_cb.lock().unwrap().extend_from_slice(data),
            move |e| eprintln!("stream error: {e}"),
            None,
        ),
        SampleFormat::U16 => device.build_input_stream(
            &config,
            move |data: &[u16], _| buf_cb.lock().unwrap().extend(data.iter().map(|&x| i16::from_sample(x))),
            move |e| eprintln!("stream error: {e}"),
            None,
        ),
        _ => panic!("unsupported format"),
    }.expect("build stream");

    stream.play().expect("play");
    std::thread::sleep(std::time::Duration::from_secs(10));
    drop(stream);
    println!("Stopped recording.");

    let pcm = buf.lock().unwrap();
    if pcm.is_empty() {
        eprintln!("No audio captured");
        return;
    }

    // 3) Build MP3 encoder (uses bundled LAME)
    let mut b = Builder::new().expect("builder");
    b.set_num_channels(config.channels as u8).expect("channels");
    b.set_sample_rate(config.sample_rate.0).expect("samplerate");
    b.set_brate(Bitrate::Kbps192).expect("bitrate");
    b.set_quality(Quality::Best).expect("quality");
    let mut enc = b.build().expect("init lame"); // docs example shows this usage

    // 4) Encode interleaved PCM directly
    let input = InterleavedPcm(&pcm); // accepts i16, f32, etc.
    let mut mp3 = Vec::<u8>::new();
    mp3.reserve(mp3lame_encoder::max_required_buffer_size(pcm.len() / config.channels as usize));

    // write frames
    let written = enc.encode(input, mp3.spare_capacity_mut()).expect("encode");
    unsafe { mp3.set_len(mp3.len() + written); }

    // flush
    let flushed = enc.flush::<FlushNoGap>(mp3.spare_capacity_mut()).expect("flush");
    unsafe { mp3.set_len(mp3.len() + flushed); }

    // 5) Save
    let mut f = File::create("recorded_audio.mp3").expect("create");
    f.write_all(&mp3).expect("write");
    println!("Saved 'recorded_audio.mp3'");
}
