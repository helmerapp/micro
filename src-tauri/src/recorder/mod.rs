use tauri::{AppHandle, WindowBuilder, WindowUrl};

pub fn init_recorder(app: &AppHandle) {
    let recorder_win = WindowBuilder::new(app, "recorder", WindowUrl::App("/recorder".into()))
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
