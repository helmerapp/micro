use crate::AppState;
use imgref::Img;
use rgb::RGBA;
use scap::frame::{BGRFrame, Frame, RGBFrame};
use serde::{Deserialize, Serialize};
use std::thread;
use tauri::{api::path::desktop_dir, AppHandle, Manager, WindowBuilder, WindowUrl};

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

fn transform_frame_bgr0(frame: &BGRFrame) -> Img<Vec<RGBA<u8>>> {
    let frame_data = &frame.data;
    let mut rgba_data: Vec<RGBA<u8>> = Vec::with_capacity(frame_data.len() / 4);

    for src in frame_data.chunks_exact(3) {
        rgba_data.push(RGBA::new(src[2], src[1], src[0], 255))
    }

    Img::new(rgba_data, frame.width as usize, frame.height as usize)
}

fn transform_frame_rgb(frame: &RGBFrame) -> Img<Vec<RGBA<u8>>> {
    let frame_data = &frame.data;
    let mut rgba_data: Vec<RGBA<u8>> = Vec::with_capacity(frame_data.len() / 4);

    for src in frame_data.chunks_exact(3) {
        rgba_data.push(RGBA::new(src[0], src[1], src[2], 255))
    }

    Img::new(rgba_data, frame.width as usize, frame.height as usize)
}

#[tauri::command]
pub async fn export_handler(options: ExportOptions, app_handle: AppHandle) {
    println!("TODO: export with options: {:?}", options);

    let state = app_handle.state::<AppState>();

    let mut settings = gifski::Settings::default();

    let width = options.size;
    let frame_start_time= options.range[0] as f64;
    let frame_end_time = options.range[1] as f64;

    match options.loop_gif {
        true => settings.repeat = gifski::Repeat::Infinite,
        false => settings.repeat = gifski::Repeat::Finite(0),
    }
    settings.width = Some(width);

    let mut no_progress = gifski::progress::NoProgress {};

    let (gif_encoder, gif_writer) = gifski::new(settings).unwrap();

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
    let mut base_ts;
    const TIMEBASE:f64 = 1000000000.0; // TODO: Verify for windows. This value may be different
    match &frames[0] {
        Frame::BGR0(f) => base_ts = f.display_time,
        Frame::RGB(f) => base_ts = f.display_time,
        _ => {
            base_ts = 0;
        }
    }

    println!("Encoding frames to GIF {}", frames.len());
    for frame in (*frames).iter_mut() {
        match frame {
            Frame::BGR0(bgr_frame) => {
                let img = transform_frame_bgr0(bgr_frame);
                gif_encoder
                    .add_frame_rgba(i, img,  (bgr_frame.display_time - base_ts) as f64/TIMEBASE)
                    .unwrap_or_else(|err| {
                        eprintln!("Error adding frame to encoder: {:?}", err);
                    });

                i += 1;
            }
            Frame::RGB(rgb_frame) => {
                let img = transform_frame_rgb(rgb_frame);
                let frame_pts = (rgb_frame.display_time - base_ts) as f64/TIMEBASE;

                if (frame_start_time > frame_pts || frame_pts > frame_end_time) {
                    println!("Ignoring frame {} with t {}", i, frame_pts);
                    continue;
                }

                gif_encoder
                    .add_frame_rgba(i, img, frame_pts)
                    .unwrap_or_else(|err| {
                        eprintln!("Error adding frame to encoder: {:?}", err);
                    });

                i += 1;
            }
            _ => {
                panic!("This frame type is not supported yet");
            }
        }
    }
    drop(gif_encoder);
    println!("GIF Encoded");

    handle.join().unwrap();
    println!("GIF Written to file");

    println!("Completed");
}
