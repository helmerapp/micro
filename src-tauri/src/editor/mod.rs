use crate::AppState;
use scap::frame::Frame;
use serde::{Deserialize, Serialize};
use std::{sync::Arc, thread};
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_dialog::DialogExt;



use tauri_plugin_decorum::WebviewWindowExt;

mod frame_encoder;
use frame_encoder::FrameEncoder;

pub fn init_editor(app: &AppHandle, video_file: String, size: (u32, u32)) {
    let (width, height) = size;
    let editor_url = format!(
        "/editor?file={}&width={}&height={}",
        video_file, width, height
    );

    const EDITOR_WIDTH: u32 = 600;
    const TOOLS_HEIGHT: u32 = 280;

    let preview_height_adjusted = EDITOR_WIDTH * height / width;
    let editor_win_height = preview_height_adjusted + TOOLS_HEIGHT;

    let mut editor_win =
        WebviewWindowBuilder::new(app, "editor", WebviewUrl::App(editor_url.into()))
            .title("Helmer Micro")
            .accept_first_mouse(true)
            .max_inner_size(800.0, 1000.0)
            .min_inner_size(600.0, 400.0)
            .inner_size(EDITOR_WIDTH.into(), editor_win_height.into())
            .resizable(false)
            .visible(false)
            .focused(false)
            .center();

    #[cfg(target_os = "macos")]
    {
        editor_win = editor_win
            .title_bar_style(tauri::TitleBarStyle::Overlay)
            .hidden_title(true);
    }

    let editor_win = editor_win.build().expect("Failed to build editor window");
    editor_win.create_overlay_titlebar().unwrap();

    #[cfg(target_os = "macos")]
    {
        editor_win.clone()
        .with_webview(move |wv| {
            use cocoa::{base::nil, foundation::NSString};
            use objc::{class, msg_send, sel, sel_impl};

            let id = wv.inner();
            let no: cocoa::base::id = unsafe { msg_send![class!(NSNumber), numberWithBool:0] };
            let _: cocoa::base::id = unsafe { msg_send![id, setValue:no forKey: NSString::alloc(nil).init_str("drawsBackground")] };
         })
        .expect("Failed to get webview");
    }

    editor_win.show().unwrap();
    editor_win.set_focus().unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportOptions {
    range: Vec<f64>,
    size: u32,
    fps: u32,
    speed: f32,
    loop_gif: bool,
}

#[tauri::command]
pub async fn export_gif(options: ExportOptions, app_handle: AppHandle) {
    // let time = std::time::SystemTime::now();
    println!("Export options: {:?}", options);

    let mut settings = gifski::Settings::default();
    settings.fast = true;

    let fps = options.fps;
    let width = options.size;
    let speed = options.speed;
    let frame_start_time = options.range[0] as f64;
    let frame_end_time = options.range[1] as f64;

    match options.loop_gif {
        true => settings.repeat = gifski::Repeat::Infinite,
        false => settings.repeat = gifski::Repeat::Finite(0),
    }
    settings.width = Some(width);

    let mut no_progress = gifski::progress::NoProgress {};

    let (gif_encoder, gif_writer) = gifski::new(settings).unwrap();

    let gif_encoder = Arc::new(gif_encoder);

    let gif_name = chrono::Local::now()
        .format("GIF %Y-%m-%d at %I-%M-%S %p")
        .to_string();
    let gif_path = match app_handle
    .dialog()
    .file()
    .add_filter("GIF Files", &["gif"])
    .set_file_name(&gif_name)  // Set the default filename
    .blocking_save_file()
    {
        Some(path) => path,
        None => {
            eprintln!("Save dialog closed or cancelled.");
            return; // Early return if dialog is closed or cancelled
        }
    };

    // Create a new file at the chosen path
    let gif_file = match std::fs::File::create(&gif_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error creating GIF file: {:?}", err);
            return;
        }
    };

    let handle = thread::spawn(move || {
        println!("Writing to a GIF file");
        let write_result = gif_writer.write(gif_file, &mut no_progress);
        if let Err(err) = write_result {
            eprintln!("Error writing GIF file: {:?}", err);
        }
        println!("Finished writing");
    });

    let mut i = 0;

    // Get AppState from AppHandle
    let state = app_handle.state::<AppState>();
    //  Get frames from app state
    let frames = state.frames.lock().await;
    let frames = frames.iter().collect::<Vec<&Frame>>();

    // Get the timestamp of the first frame
    let base_ts;
    match &frames[0] {
        Frame::BGR0(f) => base_ts = f.display_time,
        Frame::BGRA(f) => base_ts = f.display_time,
        Frame::RGB(f) => base_ts = f.display_time,
        _ => {
            base_ts = 0;
        }
    }

    let step = ((60.0 * speed) / fps as f32).floor() as usize;
    // println!("Encoding {} frames to GIF by step {}", frames.len(), step);

    for frame in frames.iter().step_by(step) {
        let gif_encoder_clone = gif_encoder.clone();

        // Remove the `frame` argument
        unit_frame_handler(
            &frame,
            gif_encoder_clone,
            i,
            base_ts,
            frame_start_time,
            frame_end_time,
            speed,
        );

        // if i % 5 === 0 then log time elapsed
        // if (i % 5) == 0 {
        //     // log time elapsed since start
        //     let time_elapsed = time.elapsed().unwrap();
        //     println!("Time elapsed: {:?}", time_elapsed);
        // }

        i += 1;
    }

    drop(gif_encoder);
    // println!("GIF Encoded");

    handle.join().unwrap();
    // println!("GIF Written to file");

    // let time_elapsed = time.elapsed().unwrap();
    // println!("Completed in {:?} seconds", time_elapsed.as_secs());
}

pub fn unit_frame_handler(
    frame: &Frame,
    gif_encoder: Arc<gifski::Collector>,
    index: usize,
    base_ts: u64,
    start_ts: f64,
    end_ts: f64,
    speed: f32,
) {
    let frame_encoder = FrameEncoder::new(gif_encoder.clone(), index, base_ts);
    match frame {
        Frame::BGR0(bgr_frame) => frame_encoder.encode_bgr(bgr_frame),
        Frame::BGRA(bgra_frame) => frame_encoder.encode_bgra(bgra_frame, speed, start_ts, end_ts),
        Frame::RGB(rgb_frame) => frame_encoder.encode_rgb(rgb_frame, speed, start_ts, end_ts),
        _ => {
            panic!("This frame type is not supported yet");
        }
    }
    println!("Frame {} Encoded", index)
}

