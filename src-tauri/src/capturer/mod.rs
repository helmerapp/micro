// use scap::capturer::{Capturer, Options};
use crate::{AppState, Status};
use std::sync::Arc;
use helmer_media::encoder;
use tauri::{AppHandle, Manager};
use tokio::sync::Mutex;

#[tauri::command]
pub async fn start_capture(area: Vec<u32>, app_handle: AppHandle) {
    app_handle.emit_all("capture-started", false).unwrap();

    // TODO: initialize scap and start capturing
    println!("Cropped Area: {:?}", area);

    const FRAME_TYPE:scap::frame::FrameType = scap::frame::FrameType::BGR0;
    let options = scap::capturer::Options {
        fps: 60,
        targets: Vec::new(),
        show_cursor: true,
        show_highlight: true,
        excluded_targets: None,
        output_type: FRAME_TYPE,
        output_resolution: scap::capturer::Resolution::_480p,
        source_rect: Some(scap::capturer::CGRect {
            origin: scap::capturer::CGPoint { x: 0.0, y: 0.0 },
            size: scap::capturer::CGSize {
                width: 600.0,
                height: 400.0,
            },
        }),
        ..Default::default()
    };
    let mut recorder = scap::capturer::Capturer::new(options);
    recorder.start_capture();

    println!("Capturing frames...");
    let mut frames: Vec<scap::frame::Frame> = Vec::new();
    for _ in 0..200 {
        let frame = recorder.get_next_frame().expect("Error");
        frames.push(frame);
    }
    recorder.start_capture();
    println!("All frames captured");

    let [output_width, output_height] = recorder.get_output_frame_size();

    // Create Encoder
    let mut encoder = encoder::Encoder::new(encoder::Options {
        output: encoder::Output::FileOutput(encoder::FileOutput {
            output_filename: "/Users/pranav2612000/Desktop/dummy.mp4".to_owned(),
        }),
        input: encoder::InputOptions {
            width: output_width as usize,
            height: output_height as usize,
            frame_type: FRAME_TYPE,
        },
    });

    let time_base = helmer_media::TimeBase::new(1, 25);
    let mut frame_idx = 0;
    let mut frame_timestamp = helmer_media::Timestamp::new(frame_idx, time_base);
    println!("Encoding frames...");
    for frame in frames {
        encoder.ingest_next_video_frame(frame, frame_timestamp);

        frame_idx += 1;
        frame_timestamp = helmer_media::Timestamp::new(frame_idx, time_base);
    }
    encoder.done();
    println!("Encoding completed");

    println!("complete"); // this never runs
}

#[tauri::command]
pub async fn stop_capture(app_handle: AppHandle) {
    println!("Capture stopped");
    app_handle.emit_all("capture-stopped", false).unwrap();

    // TODO: stop capturing with scap

    // Hide cropper, create editor
    crate::cropper::toggle_cropper(&app_handle);
    crate::editor::init_editor(&app_handle);

    // Update app state
    let state_mutex = app_handle.state::<Mutex<AppState>>();
    let mut state = state_mutex.lock().await; // TODO: this line is not running
    println!("Updating app state");

    state.status = Status::Editing;

    println!("Status: {:?}", state.status);
}
