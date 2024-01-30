use crate::{constants, editor};
use gifski::{self, progress::NoProgress, Repeat, Settings};
use imgref::{Img, ImgVec};
use rgb::{RGBA, RGBA8};
use scap::{
    capturer::{self, CGPoint, CGRect, CGSize, Capturer},
    frame::{BGRFrame, RGBxFrame},
};
use std::sync::atomic::{AtomicBool, AtomicU8};
use std::time::Duration;
use std::time::Instant;
use std::{sync::Arc, thread};
use tauri::{AppHandle, Manager};
use tokio::sync::Mutex;

static FPS: AtomicU8 = AtomicU8::new(20);
static QUALITY: AtomicU8 = AtomicU8::new(100);

fn make_img(dim: usize, phase: bool) -> ImgVec<RGBA8> {
    let black = RGBA8 {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };
    let red = RGBA8 {
        r: 255,
        g: 0,
        b: 0,
        a: 255,
    };
    let pixels = vec![black; dim * dim];
    let mut img = Img::new(pixels, dim, dim);
    let mut make_rect = |x0, y0| {
        let mut region = img.sub_image_mut(x0, y0, dim / 2, dim / 2);
        for p in region.pixels_mut() {
            *p = red;
        }
    };
    match phase {
        false => {
            make_rect(0, 0);
            make_rect(dim / 2, dim / 2);
        }
        true => {
            make_rect(dim / 2, 0);
            make_rect(0, dim / 2);
        }
    }

    img
}

fn transform_frame(frame: RGBxFrame) -> Img<Vec<RGBA<u8>>> {
    let frame_data = frame.data;
    let width = frame_data.len();
    let mut rgb_data: Vec<RGBA<u8>> = vec![];

    for src in frame_data.chunks_exact(4) {
        rgb_data.push(RGBA::new(src[0], src[1], src[2], src[3]))
    }

    println!(
        "length of buffer: {} | Length of data: {}",
        frame_data.len(),
        rgb_data.len()
    );

    Img::new(rgb_data, frame.width as usize, frame.height as usize)
}

fn transform_frame_bgr0(frame: BGRFrame) -> Img<Vec<RGBA<u8>>> {
    let frame_data = frame.data;
    let mut rgba_data: Vec<RGBA<u8>> = Vec::with_capacity(frame_data.len() / 4);

    for src in frame_data.chunks_exact(4) {
        rgba_data.push(RGBA::new(src[0], src[1], src[2], src[3]))
    }
    println!(
        "length of buffer: {} | Length of data: {}",
        frame_data.len(),
        rgba_data.len()
    );

    Img::new(rgba_data, frame.width as usize, frame.height as usize)
}

#[tauri::command]
pub async fn start_capture(area: Vec<u32>, app_handle: AppHandle) {
    println!("Capturing Area: {:?}", area);
    let mut settings = Settings::default();
    settings.repeat = Repeat::Infinite;
    let (encoder, writer) = gifski::new(settings).unwrap();

    let options = capturer::Options {
        fps: 60,
        targets: vec![],
        show_cursor: true,
        show_highlight: true,
        excluded_targets: None,
        output_type: constants::CAPTURER_OUTPUT_TYPE,
        source_rect: Some(CGRect {
            origin: CGPoint { x: 0.0, y: 0.0 },
            size: CGSize {
                width: 1500.0,
                height: 1500.0,
            },
        }),
        ..Default::default()
    };

    let gif = match std::fs::File::create("C:\\Users\\Rohan\\OneDrive\\Desktop\\flasher.gif") {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error creating GIF file: {:?}", err);
            return;
        }
    };
    let mut recorder = Capturer::new(options);
    let mut vector_frames: Vec<RGBxFrame> = Vec::new();
    app_handle
        .tray_handle()
        .set_icon(tauri::Icon::Raw(
            include_bytes!("..\\..\\icons\\RecordingIcon.png").to_vec(),
        ))
        .unwrap();
    let mut no_progress = NoProgress {};
    thread::spawn(move || {
        recorder.start_capture();
        let mut total_time = Duration::new(0, 0);
        for i in 0..180 {
            let start_time = Instant::now();

            let frame = match recorder.get_next_frame() {
                Ok(frame) => frame,
                Err(err) => {
                    eprintln!("Error getting frame: {:?}", err);
                    continue;
                }
            };

            let img_data: RGBxFrame = match frame {
                scap::frame::Frame::RGBx(bgr_frame) => bgr_frame,
                _ => {
                    eprintln!("Unsupported frame format");
                    continue;
                }
            };
            vector_frames.push(img_data);

            // let img = transform_frame(img_data);
            // encoder.add_frame_rgba(i, img, i as f64 * 0.5).unwrap_or_else(|err| {
            //     eprintln!("Error adding frame to encoder: {:?}", err);
            // });
            let end_time = Instant::now();
            let iteration_time = end_time - start_time;
            total_time += iteration_time;

            println!("Time for iteration {}: {:?}", i, iteration_time);
        }

        let average_time = total_time / 180;
        println!("Average time: {:?}", average_time);
        recorder.stop_capture();
        stop_capture(vector_frames, app_handle);
        drop(encoder);
    });

    // thread::spawn(move || {
    // println!("Writing GIF file");
    // let write_result = writer.write(gif, &mut no_progress);
    // if let Err(err) = write_result {
    //     eprintln!("Error writing GIF file: {:?}", err);
    // }
    // println!("Finished writing");
    // stop_capture(app_handle);
    // });
}

#[tauri::command]
pub fn stop_capture(frames: Vec<RGBxFrame>, app_handle: AppHandle) {
    println!("Capture stopped");

    // TODO: fire event
    // TODO: stop capturing with scap and cleanup

    app_handle
        .tray_handle()
        .set_icon(tauri::Icon::Raw(
            include_bytes!("..\\..\\icons\\128x128.png").to_vec(),
        ))
        .unwrap();
    // send frames to editor window

    // Hide cropper, create editor
    crate::cropper::toggle_cropper(&app_handle);
    crate::editor::init_editor(&app_handle);
    // let editor_window = app_handle.get_window("editor").unwrap();
    println!("Waiting for editor to be ready");
    thread::sleep(Duration::from_millis(5000));
    println!("Sending frames to editor");
    app_handle.emit_all("captured_frames", &frames).unwrap();
}

pub struct Recorder {}

pub fn new() -> Recorder {
    return Recorder {};
}

pub async fn start(capturer: &Arc<Mutex<Recorder>>) {
    let mut capturer = capturer.lock().await;
    println!("Starting recorder");
}

pub async fn stop(capturer: &Arc<Mutex<Recorder>>) -> String {
    let mut capturer = capturer.lock().await;

    println!("Stopping recorder");

    "".into()
}
