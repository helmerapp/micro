mod updater;

use crate::{cropper::toggle_cropper, AppState};
use opener::open;
use tauri::{
    image::Image,
    menu::{
        AboutMetadataBuilder, CheckMenuItemBuilder, IsMenuItem, MenuBuilder, MenuItemBuilder,
        PredefinedMenuItem, Submenu,
    },
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, EventLoopMessage, Manager,
};

use tauri_plugin_store::StoreBuilder;

pub use updater::check_for_update;

pub fn build(app: &AppHandle) {
    let about_metadata = AboutMetadataBuilder::new()
        .short_version("Beta".into())
        .icon(Some(
            Image::from_bytes(include_bytes!("../../icons/128x128.png")).expect(""),
        ))
        .copyright("©2024 Helmer Media Private Limited".into())
        .website("https://www.helmer.app/micro".into())
        .website_label("Visit Website".into())
        .license("AGPL-3.0".into())
        .version(app.package_info().version.to_string().into())
        .build();

    let tray_menu = MenuBuilder::new(app)
        .items(&[
            &MenuItemBuilder::with_id("record", "Start Recording")
                .accelerator("CommandOrControl+Shift+2")
                .build(app)
                .expect(""),
            &PredefinedMenuItem::separator(app).expect(""),
            &CheckMenuItemBuilder::with_id("record_cursor", "Record Mouse Cursor")
                .checked(get_tray_setting(app, "record_cursor".to_string()))
                .build(app)
                .expect(""),
            &get_targets_submenu(app),
            &CheckMenuItemBuilder::with_id("share_usage_data", "Share Usage Data")
                .checked(get_tray_setting(app, "share_usage_data".to_string()))
                .build(app)
                .expect(""),
            &PredefinedMenuItem::separator(app).expect(""),
            &MenuItemBuilder::with_id("website", "Visit Website")
                .build(app)
                .expect(""),
            &MenuItemBuilder::with_id("feedback", "Give Feedback")
                .build(app)
                .expect(""),
            &PredefinedMenuItem::separator(app).expect(""),
            &MenuItemBuilder::with_id("updates", "Check for Updates")
                .build(app)
                .expect(""),
            &PredefinedMenuItem::about(app, "About Helmer Micro".into(), Some(about_metadata))
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
                open("https://www.helmer.app/feedback").expect("Failed to open feedback link");
            }
            "website" => {
                open("https://www.helmer.app/micro").expect("failed to open about link");
            }
            "updates" => {
                check_for_update(app.clone(), false).expect("Failed to check for updates");
            }
            "record_cursor" => update_tray_setting(app, "record_cursor".to_string()),
            "share_usage_data" => update_tray_setting(app, "share_usage_data".to_string()),
            _label => {
                let state = app.state::<AppState>();
                let mut i: i8 = 0;
                for target in state.targets.iter() {
                    let id: String = {
                        if let scap::Target::Display(target) = target {
                            format!("target_{}", target.id).clone()
                        } else {
                            String::new()
                        }
                    };
                    if _label == id.as_str() {
                        update_target(app, i);
                    } else {
                        if let Some(an_item) = tray_menu.get("targets_submenu") {
                            if let Some(submenu) = an_item.as_submenu() {
                                if let Some(menu_item) = submenu.get(id.as_str()) {
                                    if let Some(check_menu_item) = menu_item.as_check_menuitem() {
                                        check_menu_item.set_checked(false).unwrap();
                                    }
                                }
                            }
                        }
                    }
                    i += 1;
                }
            }
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Down,
                ..
            } = event
            {
                let app: &AppHandle = tray.app_handle();
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

pub fn get_tray_setting(app: &AppHandle, key: String) -> bool {
    let mut store = StoreBuilder::new("app_data.bin").build(app.clone());
    store.load().unwrap_or_default();

    let setting_value = store
        .get(key.clone())
        .unwrap_or(&serde_json::Value::Bool(true))
        .as_bool()
        .unwrap();

    setting_value
}

fn update_tray_setting(app: &AppHandle, key: String) {
    let mut store = StoreBuilder::new("app_data.bin").build(app.clone());
    store.load().unwrap_or_default();

    // Get current value or true if not found
    let setting_value = get_tray_setting(app, key.clone());

    match setting_value {
        true => {
            store.insert(key.clone(), false.into()).unwrap();
        }
        false => {
            store.insert(key.clone(), true.into()).unwrap();
        }
    }
    store.save().expect("Failed to save store");

    // log updated value
    let updated_value = get_tray_setting(app, key.clone());

    println!("{}: {}", key, updated_value);
}

fn get_targets_submenu(
    app: &AppHandle,
) -> tauri::menu::Submenu<tauri_runtime_wry::Wry<EventLoopMessage>> {
    let mut enabled = true;
    #[cfg(not(target_os = "macos"))]
    {
        enabled = false;
    }

    let mut check_menu_items = vec![];

    let state = app.state::<AppState>();
    for (i, target) in state.targets.iter().enumerate() {
        // Default display for recording
        // - kept as first one
        // - usually the primary display
        if i == 0 {
            update_target(app, i as i8);
        }
        match target {
            scap::Target::Display(target) => {
                let menu_item = CheckMenuItemBuilder::with_id(
                    format!("target_{}", target.id),
                    target.title.as_str(),
                )
                .checked(i == 0)
                .build(app)
                .expect("");
                check_menu_items.push(menu_item);
            }
            scap::Target::Window(_) => {
                // yet to be supported.
            }
        }
    }

    let boxed_items: Vec<Box<dyn IsMenuItem<tauri_runtime_wry::Wry<EventLoopMessage>>>> =
        check_menu_items
            .into_iter()
            .map(|item| {
                Box::new(item) as Box<dyn IsMenuItem<tauri_runtime_wry::Wry<EventLoopMessage>>>
            })
            .collect();

    let unboxed_targets: &[&dyn IsMenuItem<tauri_runtime_wry::Wry<EventLoopMessage>>] =
        &(*boxed_items)
            .iter()
            .map(|item| &(**item) as &dyn IsMenuItem<tauri_runtime_wry::Wry<EventLoopMessage>>)
            .collect::<Vec<_>>()[..];

    let submenu: Submenu<tauri_runtime_wry::Wry<EventLoopMessage>> = Submenu::with_id_and_items(
        app,
        "targets_submenu",
        "Target Display",
        enabled,
        unboxed_targets,
    )
    .expect("");
    submenu
}

fn update_target(app: &AppHandle, target_index: i8) -> bool {
    let state = app.state::<AppState>();

    let mut data = match state.current_target_ind.try_lock() {
        Ok(data) => data,
        Err(_) => {
            eprintln!("[update_target]: Could not get lock on current target index!");
            return false;
        }
    };

    *data = target_index;
    drop(data);
    true
}

#[tauri::command]
pub async fn is_ok_sharing_usage_data(app: AppHandle) -> bool {
    get_tray_setting(&app, "share_usage_data".to_string())
}
