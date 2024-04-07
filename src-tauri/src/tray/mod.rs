use crate::cropper::toggle_cropper;
use opener::open;
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder, PredefinedMenuItem},
    tray::{ClickType, TrayIconBuilder},
    AppHandle, Manager,
};

pub fn build(app: &AppHandle) {
    println!("Building tray");
    let tray_menu = MenuBuilder::new(app)
        .items(&[
            &MenuItemBuilder::with_id("record", "Start Recording")
                .accelerator("CommandOrControl+Shift+2")
                .build(app)
                .expect(""),
            // PredefinedMenuItem::separator(app),
            &MenuItemBuilder::with_id("show_cursor", "Show Mouse Cursor")
                .build(app)
                .expect(""),
            &MenuItemBuilder::with_id("start_at_login", "Start at Login")
                .build(app)
                .expect(""),
            &MenuItemBuilder::with_id("updates", "Check for Updates")
                .build(app)
                .expect(""),
            // PredefinedMenuItem::separator(app),
            &MenuItemBuilder::with_id("feedback", "Give Feedback")
                .build(app)
                .expect(""),
            &MenuItemBuilder::with_id("about", "About Helmer")
                .accelerator("CommandOrControl+I")
                .build(app)
                .expect(""),
            // PredefinedMenuItem::quit(app, Some("Quit")),
        ])
        .build()
        .expect("Failed to build tray menu");

    TrayIconBuilder::new()
        .menu(&tray_menu)
        .on_menu_event(move |app, event| match event.id().as_ref() {
            "record" => {
                toggle_cropper(app);
            }
            "feedback" => {
                open("https://www.helmer.app/feedback").expect("Failed to open feedback link");
            }
            "about" => {
                open("https://www.helmer.app/micro").expect("failed to open about link");
            }
            // "quit" => {
            //     // TODO: pre-exit cleanup? (e.g. stop recording)
            //     std::process::exit(0);
            // }
            _ => (),
        })
        .on_tray_icon_event(|tray, event| {
            println!("Tray icon event: {:?}", event.click_type);
            if event.click_type == ClickType::Left {
                // TODO: if not already recording
                println!("Left click");
                let app = tray.app_handle();
                toggle_cropper(app);
            }
        })
        .build(app)
        .expect("Failed to build tray");
}
