use crate::{cropper, AppState, Status};
use helmer_media::encoder;
use rand::Rng;
use scap::{
    capturer::{CGPoint, CGRect, CGSize, Capturer, Options, Resolution},
    frame::FrameType,
};
use tauri::{AppHandle, Event, Manager};
use tempfile::NamedTempFile;

const FRAME_TYPE: FrameType = FrameType::BGRAFrame;

fn get_random_id() -> String {
    let random_number: u64 = rand::thread_rng().gen();
    let id = format!("{:x}", random_number);
    id.chars().take(13).collect()
}

#[tauri::command]
pub async fn start_capture(app_handle: AppHandle) {
    let cropper_win = app_handle.get_window("cropper").unwrap();
    cropper_win.set_ignore_cursor_events(true).unwrap();
    cropper_win.emit("capture-started", ()).unwrap();
    // // tokio sleep
    // tokio::time::sleep(std::time::Duration::from_secs(20)).await;
    // stop_capture(app_handle).await;
    // Update state to recording
    let state = app_handle.state::<AppState>();
    let mut status = state.status.lock().await;
    *status = Status::Recording;
    drop(status);

    // TODO: Calculate capture area
    println!("Cropped Area: {:?}", state.cropped_area);

    // area is of the form [x1, y1, x2, y2]
    // we need it of the form [x1, y1, x2-x1, y2-y1]
    let area = state.cropped_area.lock().await.clone();
    let crop_area = vec![
        area[0] as f64,
        area[1] as f64,
        area[2] as f64 - area[0] as f64,
        area[3] as f64 - area[1] as f64,
    ];

    // Initialize scap
    let options = Options {
        fps: 60,
        targets: Vec::new(),
        show_cursor: true,
        show_highlight: true,
        excluded_targets: None,
        output_type: FRAME_TYPE,
        output_resolution: Resolution::_1080p, // TODO: doesn't respect aspect ratio yet
        source_rect: Some(CGRect {
            origin: CGPoint {
                x: crop_area[0],
                y: crop_area[1],
            },
            size: CGSize {
                width: crop_area[2],
                height: crop_area[3],
            },
        }),
        ..Default::default()
    };

    let mut recorder = state.recorder.lock().await;
    *recorder = Some(Capturer::new(options));
    (*recorder).as_mut().unwrap().start_capture();
    drop(recorder);

    // Start capturing frames
    println!("Capturing frames...");
    let mut frames = state.frames.lock().await;

    // Reset frames to empty array to allow user to
    // record multiple gifs without restarting the app
    *frames = Vec::new();

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
}

#[tauri::command]
pub async fn stop_capture(app_handle: AppHandle) {
    // Update app state to editing
    let state = app_handle.state::<AppState>();
    let mut status = state.status.lock().await;
    *status = Status::Editing;
    drop(status);

    // Create file in temp directory
    let preview_file = format!("HM-{}.mp4", get_random_id());
    let mut preview_path = state.preview_path.lock().await;
    *preview_path = Some(
        NamedTempFile::new()
            .unwrap()
            .into_temp_path()
            .with_file_name(&preview_file),
    );

    println!("Preview path: {:?}", preview_path);

    // Hide cropper, create editor
    crate::cropper::toggle_cropper(&app_handle);
    crate::toolbar::toggle_toolbar(&app_handle);
    crate::editor::init_editor(
        &app_handle,
        preview_path.as_ref().unwrap().to_str().unwrap().to_string(),
    );

    // let cropper_win = app_handle.get_window("cropper").unwrap();
    // cropper_win.set_ignore_cursor_events(false).unwrap();
    // cropper_win.emit("capture-stopped", ()).unwrap();
    // crate::cropper::toggle_cropper(&app_handle);

    // Stop capturing frames and drop recorder
    let mut recorder = state.recorder.lock().await;
    (*recorder).as_mut().unwrap().stop_capture();
    let [output_width, output_height] = (*recorder).as_mut().unwrap().get_output_frame_size();
    recorder.take();
    drop(recorder);

    println!("All frames captured");

    // Create Encoder
    let mut encoder = encoder::Encoder::new(encoder::Options {
        output: encoder::Output::FileOutput(encoder::FileOutput {
            output_filename: preview_path.as_ref().unwrap().to_str().unwrap().to_string(),
        }),
        input: encoder::InputOptions {
            width: output_width as usize,
            height: output_height as usize,
            frame_type: FRAME_TYPE,
            base_timestamp: None,
        },
    });

    // print output_width and height
    println!("output_width: {}", output_width);
    println!("output_height: {}", output_height);

    let mut frames = state.frames.lock().await;

    let time_base = helmer_media::TimeBase::new(1, 25);
    let mut frame_idx = 0;
    let mut frame_timestamp = helmer_media::Timestamp::new(frame_idx, time_base);
    println!("Encoding preview...");
    for frame in (*frames).iter_mut() {
        encoder.ingest_next_video_frame(frame);

        frame_idx += 1;
        frame_timestamp = helmer_media::Timestamp::new(frame_idx, time_base);
    }
    encoder.done();
    drop(encoder);
    println!("Preview encoding complete");

    let editor_win = app_handle.get_window("editor").unwrap();
    editor_win.emit("preview-ready", ()).unwrap();
}
