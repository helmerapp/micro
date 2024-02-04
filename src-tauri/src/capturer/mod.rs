// use scap::capturer::{Capturer, Options};
use crate::{AppState, Status};
use std::thread;
use helmer_media::encoder;
use imgref::Img;
use rgb::RGBA;
use scap::frame;
use tauri::{AppHandle, Manager};
use tokio::sync::Mutex;

fn transform_frame_bgr0(frame: frame::BGRFrame) -> Img<Vec<RGBA<u8>>> {
    let frame_data = frame.data;
    let mut rgba_data: Vec<RGBA<u8>> = Vec::with_capacity(frame_data.len() / 4);

    for src in frame_data.chunks_exact(3) {
        rgba_data.push(RGBA::new(src[0], src[1], src[2], 255))
    }

    Img::new(rgba_data, frame.width as usize, frame.height as usize)
}

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
    for frame in frames.iter_mut() {
        encoder.ingest_next_video_frame(frame, frame_timestamp);

        frame_idx += 1;
        frame_timestamp = helmer_media::Timestamp::new(frame_idx, time_base);
    }
    encoder.done();
    drop(encoder);
    println!("Encoding completed");

    // Starting GIF creation
    println!("Starting Gif creation");

    let mut settings = gifski::Settings::default();
    settings.repeat = gifski::Repeat::Infinite;

    let mut no_progress = gifski::progress::NoProgress {};

    let (gif_encoder, gif_writer) = gifski::new(settings).unwrap();

    let gif = match std::fs::File::create("/Users/pranav2612000/Desktop/final.gif") {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error creating GIF file: {:?}", err);
            return;
        }
    };

    let handle = thread::spawn(move || {
        println!("Writing to a GIF file");
        let write_result = gif_writer.write(gif, &mut no_progress);
        if let Err(err) = write_result {
            eprintln!("Error writing GIF file: {:?}", err);
        }
        println!("Finished writing");
    });

    let mut i = 0;
    println!("Encoding frames to gif");
    for frame in frames {
        match frame {
            scap::frame::Frame::BGR0(bgr_frame) => {
                let img = transform_frame_bgr0(bgr_frame);
                gif_encoder
                .add_frame_rgba(i, img, i as f64 * 0.5)
                .unwrap_or_else(|err| {
                    eprintln!("Error adding frame to encoder: {:?}", err);
                });

                i += 1;
            },
            _ => {
                panic!("This frame type is not supported on BGR0 yet");
            }
        }
    }
    drop(gif_encoder);
    println!("Encoding to gif completed");

    handle.join();
    println!("writing to a GIF completed");

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
