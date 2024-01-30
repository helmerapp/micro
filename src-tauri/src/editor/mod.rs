use serde::{Deserialize, Serialize};
use tauri::{AppHandle, WindowBuilder, WindowUrl};

pub fn init_editor(app: &AppHandle) {
    let editor_win = WindowBuilder::new(app, "editor", WindowUrl::App("/editor".into()))
        .title("Helmer Micro")
        .accept_first_mouse(true)
        .inner_size(800.0, 800.0)
        .skip_taskbar(true)
        .always_on_top(true)
        .decorations(true)
        .resizable(false)
        .visible(true)
        .focused(true)
        .center()
        .build();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportOptions {
    size: String,
    fps: String,
    speed: f32,
    loop_gif: bool,
    bounce: bool,
}

#[tauri::command]
pub async fn export_handler(options: ExportOptions, app_handle: AppHandle) {
    println!("TODO: export with options: {:?}", options);

    // TODO: use the options to export GIF with Gifski
}
