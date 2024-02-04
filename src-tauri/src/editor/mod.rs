use serde::{Deserialize, Serialize};
use tauri::{AppHandle, WindowBuilder, WindowUrl};

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
    size: u32,
    fps: u32,
    speed: f32,
    loop_gif: bool,
    bounce: bool,
}

#[tauri::command]
pub async fn export_handler(options: ExportOptions, app_handle: AppHandle) {
    println!("TODO: export with options: {:?}", options);

    // TODO: use the options to export GIF with Gifski
}
