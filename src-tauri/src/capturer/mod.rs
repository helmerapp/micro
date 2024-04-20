use std::{sync::mpsc, thread};

use crate::{AppState, Status};

use rand::Rng;
use tauri::{AppHandle, Manager};
use tempfile::NamedTempFile;

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
    // // tokio sleep
    // tokio::time::sleep(std::time::Duration::from_secs(20)).await;
    // stop_capture(app_handle).await;
    // Update state to recording
    let state = app_handle.state::<AppState>();

    // TODO: Calculate capture area
    println!("Cropped Area: {:?}", state.cropped_area);

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

    crate::editor::init_editor(
        &app_handle,
        preview_path.as_ref().unwrap().to_str().unwrap().to_string(),
    );

    println!("Preview path: {:?}", preview_path);

    let mut recorder = state.recorder.lock().await;
    let [output_width, output_height] = (*recorder).as_mut().unwrap().get_output_frame_size();
    drop(recorder);

    let file_output = encoder::FileOutput {
        output_filename: preview_path.as_ref().unwrap().to_str().unwrap().to_string(),
    };

    let mut i = 0;

    // let mut tasks = vec![];

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
        let status = state.status.lock().await;
        if *status != Status::Recording {
            break;
        }
        drop(status);

        let mut recorder = state.recorder.lock().await;

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

    let editor_win = app_handle.get_webview_window("editor").unwrap();
    editor_win.emit("preview-ready", ()).unwrap();
}

#[tauri::command]
pub async fn stop_capture(app_handle: AppHandle) {
    // Update app state to editing
    let state = app_handle.state::<AppState>();
    let mut status = state.status.lock().await;
    *status = Status::Editing;
    drop(status);

    // Create file in temp directory
    // let preview_file = format!("HM-{}.mp4", get_random_id());
    // let mut preview_path = state.preview_path.lock().await;
    // *preview_path = Some(
    //     NamedTempFile::new()
    //         .unwrap()
    //         .into_temp_path()
    //         .with_file_name(&preview_file),
    // );

    // println!("Preview path: {:?}", preview_path);

    // Hide cropper, create editor
    crate::cropper::toggle_cropper(&app_handle);
    crate::toolbar::toggle_toolbar(&app_handle);
    // crate::editor::init_editor(
    //     &app_handle,
    //     preview_path.as_ref().unwrap().to_str().unwrap().to_string(),
    // );

    // Stop capturing frames and drop recorder
    let mut recorder = state.recorder.lock().await;
    (*recorder).as_mut().unwrap().stop_capture();
    // let [output_width, output_height] = (*recorder).as_mut().unwrap().get_output_frame_size();
    recorder.take();
    drop(recorder);

    println!("All frames captured");

    // Create Encoder
    // let mut encoder = encoder::Encoder::new(encoder::Options {
    //     output: encoder::Output::FileOutput(encoder::FileOutput {
    //         output_filename: preview_path.as_ref().unwrap().to_str().unwrap().to_string(),
    //     }),
    //     input: encoder::InputOptions {
    //         width: output_width as usize,
    //         height: output_height as usize,
    //         frame_type: FRAME_TYPE,
    //         base_timestamp: None,
    //     },
    // });

    // // print output_width and height
    // println!("output_width: {}", output_width);
    // println!("output_height: {}", output_height);

    // let mut frames = state.frames.lock().await;

    // let time_base = encoder::TimeBase::new(1, 25);
    // let mut frame_idx = 0;
    // let mut _frame_timestamp = encoder::Timestamp::new(frame_idx, time_base);
    // println!("Encoding preview...");

    // for frame in (*frames).iter_mut() {
    //     let _ = encoder.ingest_next_video_frame(frame);

    //     frame_idx += 1;
    //     _frame_timestamp = encoder::Timestamp::new(frame_idx, time_base);
    // }
    // let _ = encoder.done();
    // drop(encoder);
    // println!("Preview encoding complete");

    // let editor_win = app_handle.get_webview_window("editor").unwrap();
    // editor_win.emit("preview-ready", ()).unwrap();

    let editor_win = app_handle.get_webview_window("editor").unwrap();
    editor_win.emit("preview-ready", ()).unwrap();
}
