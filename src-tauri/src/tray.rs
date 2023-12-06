use crate::cropper::toggle_cropper;
use opener::open;
use tauri::{
    AppHandle, CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};

pub fn build() -> SystemTray {
    // id, label
    let menu_items = vec![
        ("record", "Record"),
        ("separator", ""),
        ("feedback", "Give Feedback"),
        ("about", "About Helmer"),
        ("quit", "Quit"),
    ];

    let mut tray_menu = SystemTrayMenu::new();

    for item in menu_items {
        match item.0 {
            "separator" => {
                tray_menu = tray_menu.add_native_item(SystemTrayMenuItem::Separator);
            }
            _ => {
                let mut menu_item = CustomMenuItem::new(item.0, item.1);

                match item.0 {
                    "record" => {
                        menu_item = menu_item.accelerator("CommandOrControl+Shift+2");
                    }
                    "about" => {
                        menu_item = menu_item.accelerator("CommandOrControl+I");
                    }
                    _ => {}
                }

                tray_menu = tray_menu.add_item(menu_item);
            }
        }
    }

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
