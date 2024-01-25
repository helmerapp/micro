use tauri::{AppHandle, WindowBuilder, WindowUrl};

pub fn init_editor(app: &AppHandle) {
    let mut editor_win = WindowBuilder::new(app, "editor", WindowUrl::App("/editor".into()))
        .title("Helmer Micro")
        .accept_first_mouse(true)
        .inner_size(800.0, 600.0)
        .skip_taskbar(true)
        .always_on_top(true)
        .decorations(true)
        .resizable(false)
        .visible(true)
        .focused(true)
        .center()
        .build();
}
