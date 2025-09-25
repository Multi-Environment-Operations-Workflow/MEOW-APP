use chrono::Local;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    FromSample, Sample, Stream,
};
use hound::{SampleFormat, WavSpec, WavWriter};
use std::{
    fs::{create_dir_all, File},
    io::BufWriter,
};
use std::{
    path::PathBuf,
    sync::atomic::{AtomicBool, Ordering},
    sync::{Arc, LazyLock, Mutex},
    time::Duration,
};
use tauri::{command, AppHandle, Manager, Runtime};

type WavWriterHandle = Arc<Mutex<Option<WavWriter<BufWriter<File>>>>>;

struct SafeStream(Stream);

unsafe impl Send for SafeStream {}
unsafe impl Sync for SafeStream {}

struct State {
    is_recording: Arc<AtomicBool>,
    save_path: Arc<Mutex<Option<PathBuf>>>,
    writer: WavWriterHandle,
    stream: Arc<Mutex<Option<SafeStream>>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            is_recording: Arc::new(AtomicBool::new(false)),
            save_path: Arc::new(Mutex::new(None)),
            writer: Arc::new(Mutex::new(None)),
            stream: Arc::new(Mutex::new(None)),
        }
    }
}

static STATE: LazyLock<Arc<Mutex<State>>> = LazyLock::new(|| Arc::new(Mutex::new(State::new())));

#[command]
pub async fn start_recording_with_timeout<R: Runtime>(
    app_handle: AppHandle<R>,
) -> Result<(), String> {
    // Start the recording
    start_recording(app_handle.clone()).await?;

    // Spawn a thread that will stop the recording after 5 seconds
    tauri::async_runtime::spawn(async move {
        tokio::time::sleep(Duration::from_secs(5)).await;
        let _ = stop_recording().await;
    });

    Ok(())
}

#[command]
pub async fn start_recording<R: Runtime>(app_handle: AppHandle<R>) -> Result<(), String> {
    let mut state = STATE.lock().map_err(|err| err.to_string())?;
    if state.is_recording.load(Ordering::SeqCst) {
        return Err("Recording is already in progress.".to_string());
    }
    state.is_recording.store(true, Ordering::SeqCst);

    let host = cpal::default_host();

    let device = host
        .default_input_device()
        .ok_or("No default input device avaialable")?;

    let config = device
        .default_input_config()
        .map_err(|err| err.to_string())?;

    let save_path = get_save_path(&app_handle)?;
    println!("{:?}", save_path);
    let spec = wav_spec_from_config(&config);
    let writer = WavWriter::create(&save_path, spec).map_err(|err| err.to_string())?;
    let writer = Arc::new(Mutex::new(Some(writer)));

    let writer_clone = writer.clone();
    let err_fn = move |err: cpal::StreamError| {
        eprintln!("an error occurred on stream: {}", err);
    };

    let stream = match config.sample_format() {
        cpal::SampleFormat::I8 => device
            .build_input_stream(
                &config.into(),
                move |data, _: &_| write_input_data::<i8, i8>(data, &writer_clone),
                err_fn,
                None,
            )
            .map_err(|err| err.to_string())?,
        cpal::SampleFormat::I16 => device
            .build_input_stream(
                &config.into(),
                move |data, _: &_| write_input_data::<i16, i16>(data, &writer_clone),
                err_fn,
                None,
            )
            .map_err(|err| err.to_string())?,
        cpal::SampleFormat::I32 => device
            .build_input_stream(
                &config.into(),
                move |data, _: &_| write_input_data::<i32, i32>(data, &writer_clone),
                err_fn,
                None,
            )
            .map_err(|err| err.to_string())?,
        cpal::SampleFormat::F32 => device
            .build_input_stream(
                &config.into(),
                move |data, _: &_| write_input_data::<f32, f32>(data, &writer_clone),
                err_fn,
                None,
            )
            .map_err(|err| err.to_string())?,
        _ => return Err("Unsupported sample format".to_string()),
    };

    stream.play().map_err(|err| err.to_string())?;

    *state.save_path.lock().map_err(|err| err.to_string())? = Some(save_path);
    state.writer = writer;
    *state.stream.lock().map_err(|err| err.to_string())? = Some(SafeStream(stream));

    Ok(())
}

#[command]
pub async fn stop_recording() -> Result<PathBuf, String> {
    let state = STATE.lock().map_err(|err| err.to_string())?;
    if !state.is_recording.load(Ordering::SeqCst) {
        return Err("No recording in progress.".to_string());
    }
    state.is_recording.store(false, Ordering::SeqCst);

    if let Some(stream) = state.stream.lock().map_err(|err| err.to_string())?.take() {
        drop(stream.0);
    }

    if let Some(writer) = state.writer.lock().map_err(|err| err.to_string())?.take() {
        writer.finalize().map_err(|err| err.to_string())?;
    }

    let save_path = state
        .save_path
        .lock()
        .map_err(|err| err.to_string())?
        .take()
        .ok_or("No recording in progress or save path not set.".to_string())?;

    Ok(save_path)
}

fn get_save_path<R: Runtime>(app_handle: &AppHandle<R>) -> Result<PathBuf, String> {
    let save_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|err| err.to_string())?
        .join("tauri-plugin-mic-recorder");

    create_dir_all(&save_dir).map_err(|err| err.to_string())?;

    let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();
    let save_path = save_dir.join(format!("{timestamp}.wav"));

    Ok(save_path)
}

fn sample_format(format: cpal::SampleFormat) -> SampleFormat {
    if format.is_float() {
        SampleFormat::Float
    } else {
        SampleFormat::Int
    }
}

fn wav_spec_from_config(config: &cpal::SupportedStreamConfig) -> WavSpec {
    WavSpec {
        channels: config.channels() as _,
        sample_rate: config.sample_rate().0 as _,
        bits_per_sample: (config.sample_format().sample_size() * 8) as _,
        sample_format: sample_format(config.sample_format()),
    }
}

fn write_input_data<T, U>(input: &[T], writer: &WavWriterHandle)
where
    T: Sample,
    U: Sample + hound::Sample + FromSample<T>,
{
    if let Ok(mut guard) = writer.try_lock() {
        if let Some(writer) = guard.as_mut() {
            for &sample in input.iter() {
                let sample: U = U::from_sample(sample);
                writer.write_sample(sample).ok();
            }
        }
    }
}

