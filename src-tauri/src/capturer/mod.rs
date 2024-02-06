// use scap::capturer::{Capturer, Options};
use crate::{AppState, Status};
use helmer_media::encoder;
use tauri::{AppHandle, Manager};
use tokio::sync::Mutex;

const FRAME_TYPE: scap::frame::FrameType = scap::frame::FrameType::BGR0;

#[tauri::command]
pub async fn start_capture(area: Vec<u32>, app_handle: AppHandle) {
    app_handle.emit_all("capture-started", false).unwrap();

    // Update app state
    let state = app_handle.state::<AppState>();

    let mut status = state.status.lock().await;
    *status = Status::Recording;
    drop(status);

    // TODO: initialize scap and start capturing
    println!("Cropped Area: {:?}", area);

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
                width: 1280.0,
                height: 720.0,
            },
        }),
        ..Default::default()
    };

    let mut recorder = state.recorder.lock().await;
    *recorder = Some(scap::capturer::Capturer::new(options));
    // let mut recorder = scap::capturer::Capturer::new(options);
    (*recorder).as_mut().unwrap().start_capture();
    drop(recorder);

    println!("Capturing frames...");
    let mut frames = state.frames.lock().await;
    // let
    // let mut frames: Vec<scap::frame::Frame> = Vec::new();

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
        (*frames).push(frame);
        println!("Frame captured");
        drop(recorder);
    }

    println!("Recording stopped");
    // for _ in 0..200 {
    //     let frame = recorder.get_next_frame().expect("Error");
    //     frames.push(frame);
    // }
}

#[tauri::command]
pub async fn stop_capture(app_handle: AppHandle) {
    println!("Capture stopped");
    app_handle.emit_all("capture-stopped", false).unwrap();

    // Update app state
    let state = app_handle.state::<AppState>();

    // TODO: stop capturing with scap
    let mut status = state.status.lock().await;
    *status = Status::Editing;
    drop(status);

    let mut recorder = state.recorder.lock().await;
    (*recorder).as_mut().unwrap().stop_capture();
    println!("All frames captured");

    let [output_width, output_height] = (*recorder).as_mut().unwrap().get_output_frame_size();
    drop(recorder);

    // Create Encoder
    let mut encoder = encoder::Encoder::new(encoder::Options {
        output: encoder::Output::FileOutput(encoder::FileOutput {
            output_filename: "/Users/siddharth/Desktop/dummy.mp4".to_owned(),
        }),
        input: encoder::InputOptions {
            width: output_width as usize,
            height: output_height as usize,
            frame_type: FRAME_TYPE,
        },
    });

    let mut frames = state.frames.lock().await;

    let time_base = helmer_media::TimeBase::new(1, 25);
    let mut frame_idx = 0;
    let mut frame_timestamp = helmer_media::Timestamp::new(frame_idx, time_base);
    println!("Encoding frames...");
    for frame in (*frames).iter_mut() {
        encoder.ingest_next_video_frame(frame, frame_timestamp);

        frame_idx += 1;
        frame_timestamp = helmer_media::Timestamp::new(frame_idx, time_base);
    }
    encoder.done();
    drop(encoder);
    println!("Encoding completed");

    // Hide cropper, create editor
    crate::cropper::toggle_cropper(&app_handle);
    crate::editor::init_editor(&app_handle);
}
