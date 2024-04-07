use crate::AppState;
use scap::frame::Frame;
use serde::{Deserialize, Serialize};
use std::{sync::Arc, thread};
use tauri::{api::path::desktop_dir, AppHandle, Manager, WindowBuilder, WindowUrl};

mod frame_encoder;
use frame_encoder::{bgr_frame_encoder, bgra_frame_encoder, rgb_frame_encoder};

pub fn init_editor(app: &AppHandle, video_file: String) {
    let editor_url = format!("/editor?file={}", video_file);

    let mut editor_win = WindowBuilder::new(app, "editor", WindowUrl::App(editor_url.into()))
        .title("Helmer Micro")
        .accept_first_mouse(true)
        .inner_size(800.0, 800.0)
        .always_on_top(true)
        .decorations(true)
        .resizable(false)
        .visible(true)
        .focused(true)
        .center();

    #[cfg(target_os = "macos")]
    {
        editor_win = editor_win.title_bar_style(tauri::TitleBarStyle::Overlay);
    }

    editor_win.build().expect("Failed to build editor window");
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportOptions {
    range: Vec<f64>,
    size: u32,
    fps: u32,
    speed: f32,
    loop_gif: bool,
    bounce: bool,
}

#[tauri::command]
pub async fn export_handler(options: ExportOptions, app_handle: AppHandle) {
    println!("TODO: export with options: {:?}", options);

    let state = app_handle.state::<AppState>();

    let mut settings = gifski::Settings::default();

    let width = options.size;
    let frame_start_time = options.range[0] as f64;
    let frame_end_time = options.range[1] as f64;
    let speed = options.speed;
    let fps = options.fps;

    match options.loop_gif {
        true => settings.repeat = gifski::Repeat::Infinite,
        false => settings.repeat = gifski::Repeat::Finite(0),
    }
    settings.width = Some(width);

    let mut no_progress = gifski::progress::NoProgress {};

    let (gif_encoder, gif_writer) = gifski::new(settings).unwrap();

    let gif_encoder = Arc::new(gif_encoder);

    let gif_name = chrono::Local::now().format("HM-%y%m%d-%I%M%p").to_string();
    let gif_path = desktop_dir().unwrap().join(format!("{}.gif", gif_name));

    let gif = match std::fs::File::create(gif_path) {
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

    //  Get frames from app state
    let mut frames = state.frames.lock().await;
    let mut i = 0;

    // Get the timestamp of the first frame
    let base_ts;
    match &frames[0] {
        Frame::BGR0(f) => base_ts = f.display_time,
        Frame::RGB(f) => base_ts = f.display_time,
        _ => {
            base_ts = 0;
        }
    }

    let step = ((60.0 * speed) / fps as f32).floor() as usize;
    println!("Encoding frames to GIF {} by step {}", frames.len(), step);
    for frame in (*frames).iter_mut().step_by(step) {
        unit_frame_handler(
            frame,
            gif_encoder.clone(),
            i,
            base_ts,
            frame_start_time,
            frame_end_time,
            speed,
        );
        i += 1;
    }
    drop(gif_encoder);
    println!("GIF Encoded");

    handle.join().unwrap();
    println!("GIF Written to file");

    println!("Completed");
}

pub fn unit_frame_handler(
    frame: &mut Frame,
    gif_encoder: Arc<gifski::Collector>,
    i: usize,
    base_ts: u64,
    frame_start_time: f64,
    frame_end_time: f64,
    speed: f32,
) {
    match frame {
        Frame::BGR0(bgr_frame) => bgr_frame_encoder(gif_encoder.clone(), i, bgr_frame, base_ts),
        Frame::BGRA(bgra_frame) => bgra_frame_encoder(gif_encoder.clone(), i, bgra_frame, base_ts),
        Frame::RGB(rgb_frame) => rgb_frame_encoder(
            gif_encoder.clone(),
            i,
            rgb_frame,
            base_ts,
            speed,
            frame_start_time,
            frame_end_time,
        ),
        _ => {
            panic!("This frame type is not supported yet");
        }
    }
}
