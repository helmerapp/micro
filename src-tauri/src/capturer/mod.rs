use std::{sync::mpsc, thread};

use crate::{AppState, Status};

use rand::Rng;
use tauri::{AppHandle, Manager};
use tempfile::NamedTempFile;
use tokio::time::{sleep, Duration};

use scap::frame::{Frame, FrameType};
pub const FRAME_TYPE: FrameType = FrameType::BGRAFrame;

mod utils;
use utils::start_recorder;

use self::encoder::FileOutput;

mod encoder;

fn get_random_id() -> String {
    let random_number: u64 = rand::thread_rng().gen();
    let id = format!("{:x}", random_number);
    id.chars().take(13).collect()
}

fn process_data(rx: mpsc::Receiver<Frame>, width: usize, height: usize, file_path: FileOutput) {
    let mut encoder = encoder::Encoder::new(encoder::Options {
        output: encoder::Output::FileOutput(file_path),
        input: encoder::InputOptions {
            width: width,
            height: height,
            frame_type: FRAME_TYPE,
            base_timestamp: None,
        },
    });
    // Process data until the channel is closed
    while let Ok(data) = rx.recv() {
        // Process the received data
        let _ = encoder.ingest_next_video_frame(&data);
    }
    println!("Recording stopped");
    let x = encoder.done();
    match x {
        Ok(_) => {
            println!("Encoding complete");
        }
        Err(e) => println!("Error: {:?}", e),
    }
    drop(encoder);
    println!("Processing thread terminated.");
}

#[tauri::command]
pub async fn start_capture(app_handle: AppHandle) {
    let app_handle_clone = app_handle.clone();
    start_recorder(app_handle_clone).await;

    let state = app_handle.state::<AppState>();

    // Start capturing frames
    println!("Capturing frames...");
    let mut frames = state.frames.lock().await;

    // Reset frames to empty array to allow user to
    // record multiple gifs without restarting the app
    *frames = Vec::new();

    // Create file in temp directory
    let preview_file = format!("HM-{}.mp4", get_random_id());
    let mut preview_path = state.preview_path.lock().await;
    *preview_path = Some(
        NamedTempFile::new()
            .unwrap()
            .into_temp_path()
            .with_file_name(&preview_file),
    );

    let mut recorder = state.recorder.lock().await;
    let [output_width, output_height] = (*recorder).as_mut().unwrap().get_output_frame_size();
    drop(recorder);

    let file_output = encoder::FileOutput {
        output_filename: preview_path.as_ref().unwrap().to_str().unwrap().to_string(),
    };

    let mut i = 0;

    let (tx, rx) = mpsc::channel();

    // Spawn a processing thread

    let process_thread = thread::spawn(move || {
        process_data(
            rx,
            output_width as usize,
            output_height as usize,
            file_output,
        );
    });

    loop {
        let mut recorder = state.recorder.lock().await;

        if recorder.is_none() {
            println!("Exiting encoding loop");
            break;
        }

        let frame = (*recorder)
            .as_mut()
            .unwrap()
            .get_next_frame()
            .expect("Error");

        let frame_for_preview = frame.clone();
        tx.send(frame_for_preview).unwrap();

        (*frames).push(frame);

        println!("Frame captured {}", i);
        i += 1;
        drop(recorder);
    }

    drop(tx);
    process_thread.join().expect("Processing thread panicked.");

    println!("Creating Editor Window");
    println!("Preview path: {:?}", preview_path);
    println!("Preview dimensions: {}x{}", output_width, output_height);

    crate::editor::init_editor(
        &app_handle,
        preview_path.as_ref().unwrap().to_str().unwrap().to_string(),
    );

    let editor_win = app_handle
        .get_webview_window("editor")
        .expect("couldn't get editor window");

    // this sleep is needed because frontend is not ready
    // to receive the preview-ready event immediately
    sleep(Duration::from_secs(1)).await;

    // TODO:
    // if it is guaranteed that preview encoding is fully complete
    // at the time of editor window creation, we can remove this
    // event entirely from BE + FE

    editor_win
        .emit("preview-ready", ())
        .expect("couldn't emit preview-ready");
}

#[tauri::command]
pub async fn stop_capture(app_handle: AppHandle) {
    // Hide cropper and toolbar
    crate::cropper::toggle_cropper(&app_handle);
    crate::toolbar::toggle_toolbar(&app_handle);

    // Stop capturing frames and drop recorder
    let state = app_handle.state::<AppState>();
    let mut recorder = state.recorder.lock().await;
    (*recorder).as_mut().unwrap().stop_capture();
    recorder.take();
    drop(recorder);

    // Update app state to editing
    let mut status = state.status.lock().await;
    *status = Status::Editing;
    drop(status);
}
