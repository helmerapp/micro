use tauri::{
    AppHandle, CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};

use opener::open;

pub fn build() -> SystemTray {
    // id, label
    let menu_items = vec![
        ("record", "Record"),
        ("settings", "Preferences"),
        ("separator", ""),
        ("feedback", "Give Feedback"),
        ("changelog", "View Changelog"),
        ("separator", ""),
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
                    "settings" => {
                        menu_item = menu_item.accelerator("CommandOrControl+,");
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
            println!("TODO: open cropper");
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "record" => {
                println!("TODO: open cropper");
            }
            "settings" => {
                println!("TODO: open settings");
            }
            "feedback" => {
                open("https://www.helmer.app/feedback").expect("Failed to open feedback link");
            }
            "changelog" => {
                open("https://www.helmer.app/changelog").expect("Failed to open changelog link");
            }
            "about" => {
                open("https://www.helmer.app/").expect("failed to open about link");
            }
            "quit" => {
                // TODO: find out what kind of cleanup will be needed before exiting
                std::process::exit(0);
            }
            _ => {}
        },
        _ => {}
    }
}
