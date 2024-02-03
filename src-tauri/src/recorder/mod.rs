use tauri::{AppHandle, WindowBuilder, WindowUrl};

pub fn init_recorder_btn(app: &AppHandle) {
    let recorder_btn = WindowBuilder::new(app, "recorder", WindowUrl::App("/recorder".into()))
        .title("Helmer Micro")
        .accept_first_mouse(true)
        .inner_size(20.0, 20.0)
        .skip_taskbar(true)
        .always_on_top(true)
        .decorations(false)
        .resizable(false)
        .transparent(true) // Set the window to be transparent
        .visible(true)
        .focused(true)
        .center()
        .build();
}

#[tauri::command]
pub async fn show_recorder_btn(coords: Vec<u32>, app_handle: AppHandle) {
    crate::recorder::init_recorder_btn(&app_handle);
}