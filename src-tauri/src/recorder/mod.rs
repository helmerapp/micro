use std::{sync::mpsc, thread};

use crate::{open_onboarding, AppState, Status};

use tauri::{AppHandle, Manager};
use tempfile::NamedTempFile;

mod utils;
use utils::{get_random_id, start_frame_capture};

use henx::{VideoEncoder, VideoEncoderOptions};

#[tauri::command]
pub async fn start_recording(app_handle: AppHandle) {
    // If no permissions, open onboarding screen
    if !scap::has_permission() {
        eprintln!("no permission to record screen");
        open_onboarding(&app_handle);
        return;
    }

    let app_handle_clone = app_handle.clone();
    start_frame_capture(app_handle_clone).await;

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

    let output_path = preview_path.as_ref().unwrap().to_str().unwrap().to_string();

    let (tx, rx) = mpsc::channel();

    // Spawn a processing thread
    let preview_encoding_thread = thread::spawn(move || {
        let mut encoder = VideoEncoder::new(VideoEncoderOptions {
            width: output_width as usize,
            height: output_height as usize,
            path: output_path.clone(),
        });

        // Process data until the channel is closed
        while let Ok(data) = rx.recv() {
            encoder
                .ingest_next_frame(&data)
                .expect("failed to send frame");
        }

        match encoder.finish() {
            Ok(_) => {
                println!("Encoding complete");
            }
            Err(e) => println!("Error: {:?}", e),
        }
        println!("Processing thread terminated.");
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
        drop(recorder);
    }

    // if there exists an editor, close it here
    // TODO: ideal would be to use the existing editor itself
    // update it's preview path and animate to the right dimensions
    let existing_editor_win = app_handle.get_webview_window("editor");
    if let Some(existing_editor_win) = existing_editor_win {
        existing_editor_win
            .close()
            .expect("couldn't destory and close existing editor");
    }

    // drop the sender to close the channel
    drop(tx);
    // wait for the encoding thread to finish
    preview_encoding_thread
        .join()
        .expect("Processing thread panicked.");

    println!("Creating Editor Window");
    println!("Preview path: {:?}", preview_path);
    println!("Preview dimensions: {}x{}", output_width, output_height);

    // sleep 1 second
    std::thread::sleep(std::time::Duration::from_secs(1));

    // initialise the editor with the file path of encoded video
    let preview_path_string = preview_path.as_ref().unwrap().to_str().unwrap().to_string();

    crate::editor::init_editor(
        &app_handle,
        preview_path_string,
        (output_width, output_height),
    );
}

#[tauri::command]
pub async fn stop_recording(app_handle: AppHandle) {
    // Hide and reset cropper (toolbar also handled)
    crate::cropper::toggle_cropper(&app_handle);
    let cropper_win = app_handle.get_webview_window("cropper").unwrap();
    cropper_win.emit("capture-stopped", ()).unwrap();
    cropper_win.set_ignore_cursor_events(false).unwrap();

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
