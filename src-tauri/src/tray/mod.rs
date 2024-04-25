mod updater;

use crate::cropper::toggle_cropper;
use opener::open;
use tauri::{
    image::Image,
    menu::{CheckMenuItemBuilder, MenuBuilder, MenuItemBuilder, PredefinedMenuItem},
    tray::{ClickType, TrayIconBuilder},
    AppHandle,
};

pub fn build(app: &AppHandle) {
    let tray_menu = MenuBuilder::new(app)
        .items(&[
            &MenuItemBuilder::with_id("record", "Start Recording")
                .accelerator("CommandOrControl+Shift+2")
                .build(app)
                .expect(""),
            &PredefinedMenuItem::separator(app).expect(""),
            &CheckMenuItemBuilder::with_id("record_cursor", "Record Mouse Cursor")
                .enabled(true)
                .build(app)
                .expect(""),
            &CheckMenuItemBuilder::with_id("start_at_login", "Start at Login")
                .build(app)
                .expect(""),
            &CheckMenuItemBuilder::with_id("share_usage_data", "Share Usage Data")
                .build(app)
                .expect(""),
            &PredefinedMenuItem::separator(app).expect(""),
            &MenuItemBuilder::with_id("updates", "Check for Updates...")
                .build(app)
                .expect(""),
            &MenuItemBuilder::with_id("feedback", "Give Feedback")
                .build(app)
                .expect(""),
            &MenuItemBuilder::with_id("about", "About Helmer")
                .accelerator("CommandOrControl+I")
                .build(app)
                .expect(""),
            &PredefinedMenuItem::quit(app, Some("Quit")).expect(""),
        ])
        .build()
        .expect("Failed to build tray menu");

    let mut tray = TrayIconBuilder::with_id("tray")
        .menu(&tray_menu)
        .icon(Image::from_bytes(include_bytes!("../../icons/128x128.png")).expect(""))
        .on_menu_event(move |app, event| match event.id().as_ref() {
            "record" => {
                toggle_cropper(app);
            }
            "feedback" => {
                open("https://www.helmer.app/support").expect("Failed to open feedback link");
            }
            "about" => {
                open("https://www.helmer.app/micro").expect("failed to open about link");
            }
            "updates" => {
                updater::check_for_update(app.clone()).expect("Failed to check for updates");
            }
            "record_cursor" => {
                // TODO: implement
                println!("Record cursor")
            }
            "start_at_login" => {
                // TODO: implement
                println!("Start at login")
            }
            "share_usage_data" => {
                // TODO: implement
                println!("Share usage data")
            }
            _ => (),
        })
        .on_tray_icon_event(|tray, event| {
            if event.click_type == ClickType::Left {
                // TODO: if not already recording
                let app = tray.app_handle();
                toggle_cropper(app);
            }
        });

    #[cfg(target_os = "macos")]
    {
        tray = tray
            .menu_on_left_click(false)
            .icon(
                Image::from_bytes(include_bytes!("../../icons/mac/TrayIdleTemplate@3x.png"))
                    .expect("Couldn't find icon"),
            )
            .icon_as_template(true)
    }

    tray.build(app).expect("Failed to build tray");
}
