use crate::cropper::toggle_cropper;
use opener::open;
use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};

pub fn build() -> SystemTray {
    let tray_menu = SystemTrayMenu::new()
        .add_item(
            CustomMenuItem::new("record", "Start Recording")
                .accelerator("CommandOrControl+Shift+2"),
        )
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("show_cursor", "Show Mouse Cursor"))
        .add_item(CustomMenuItem::new("start_at_login", "Start at Login"))
        .add_item(CustomMenuItem::new("updates", "Check for Updates"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("feedback", "Give Feedback"))
        .add_item(CustomMenuItem::new("about", "About Helmer").accelerator("CommandOrControl+I"))
        .add_item(CustomMenuItem::new("quit", "Quit"));

    return SystemTray::new().with_menu(tray_menu);
}

pub fn events(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            // TODO: if not already recording
            toggle_cropper(app);
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "record" => {
                toggle_cropper(app);
            }
            "feedback" => {
                open("https://www.helmer.app/feedback").expect("Failed to open feedback link");
            }
            "about" => {
                open("https://www.helmer.app/micro").expect("failed to open about link");
            }
            "quit" => {
                // TODO: pre-exit cleanup? (e.g. stop recording)
                std::process::exit(0);
            }
            _ => {}
        },
        _ => {}
    }
}
