use crate::AppState;
use imgref::Img;
use rgb::RGBA;
use scap::frame::{BGRFrame, Frame, RGBFrame};
use serde::{Deserialize, Serialize};
use std::thread;
use tauri::{api::path::desktop_dir, AppHandle, Manager, WindowBuilder, WindowUrl};
use tokio::sync::Mutex;

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
        editor_win = editor_win
            .title_bar_style(tauri::TitleBarStyle::Overlay)
            .hidden_title(true);
    }

    editor_win.build().expect("Failed to build editor window");
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportOptions {
    range: Vec<u32>,
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
    println!("TOFIX: export not working with YUV frames yet");

    let state = app_handle.state::<AppState>();

    // TODO: use the options to export GIF with Gifski
    // Starting GIF creation
    println!("Starting Gif creation");

    let mut settings = gifski::Settings::default();
    settings.repeat = gifski::Repeat::Infinite;

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
    println!("Encoding frames to gif");
    for frame in (*frames).iter_mut() {
        match frame {
            Frame::BGR0(bgr_frame) => {
                let img = transform_frame_bgr0(bgr_frame);
                gif_encoder
                    .add_frame_rgba(i, img, i as f64 * 0.5)
                    .unwrap_or_else(|err| {
                        eprintln!("Error adding frame to encoder: {:?}", err);
                    });

                i += 1;
            }
            Frame::RGB(rgb_frame) => {
                let img = transform_frame_rgb(rgb_frame);
                gif_encoder
                    .add_frame_rgba(i, img, i as f64 / 60.0)
                    .unwrap_or_else(|err| {
                        eprintln!("Error adding frame to encoder: {:?}", err);
                    });

                i += 1;
            }
            _ => {
                panic!("This frame type is not supported on BGR0 yet");
            }
        }
    }
    drop(gif_encoder);
    println!("Encoding to gif completed");

    handle.join();
    println!("writing to a GIF completed");

    println!("complete");
}
