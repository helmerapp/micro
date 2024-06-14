use crate::AppState;
use tauri::{
    utils::WindowEffect, AppHandle, LogicalSize, Manager, PhysicalPosition, PhysicalSize, Position,
    WebviewUrl, WebviewWindowBuilder,
};
use tauri_utils::{config::WindowEffectsConfig, WindowEffectState};

// #[cfg(target_os = "windows")]
// fn hide_using_window_affinity(hwnd: windows::Win32::Foundation::HWND) {
//     use windows::Win32::UI::WindowsAndMessaging::{
//         SetWindowDisplayAffinity, WINDOW_DISPLAY_AFFINITY,
//     };

//     let affinity = WINDOW_DISPLAY_AFFINITY(0x00000011);
//     unsafe {
//         let _ = SetWindowDisplayAffinity(hwnd, affinity);
//     }
// }

#[cfg(target_os = "macos")]
fn set_transparency_and_level(win: tauri::WebviewWindow, level: u32) {
    use cocoa::{appkit::NSColor, base::nil, foundation::NSString};
    use objc::{class, msg_send, sel, sel_impl};

    let win_ns_window = win.ns_window().unwrap() as cocoa::base::id;

    unsafe {
        let win_bg_color = NSColor::colorWithSRGBRed_green_blue_alpha_(nil, 0.0, 0.0, 0.0, 0.0);
        // set window level to 25
        let _: cocoa::base::id = msg_send![win_ns_window, setLevel: level];
        // Make window background transparent
        let _: cocoa::base::id = msg_send![win_ns_window, setBackgroundColor: win_bg_color];
    }

    win.with_webview(|webview| unsafe {
        let id = webview.inner();
        let no: cocoa::base::id = msg_send![class!(NSNumber), numberWithBool:0];
        let _: cocoa::base::id =
            msg_send![id, setValue:no forKey: NSString::alloc(nil).init_str("drawsBackground")];
    })
    .ok();
}

fn create_record_button_win(app: &AppHandle) {
    let primary_monitor = app.primary_monitor().unwrap().unwrap();
    let scale_factor = primary_monitor.scale_factor();
    let monitor_size: LogicalSize<f64> = primary_monitor.size().to_logical(scale_factor);

    const RECORD_BUTTON_WIDTH: f64 = 200.0;
    const RECORD_BUTTON_HEIGHT: f64 = 48.0;

    let mut record_win =
        WebviewWindowBuilder::new(app, "record", WebviewUrl::App("/record".into()))
            .title("recorder window")
            .inner_size(RECORD_BUTTON_WIDTH, RECORD_BUTTON_HEIGHT)
            .position(
                (monitor_size.width / 2.0) - (RECORD_BUTTON_WIDTH / 2.0),
                monitor_size.height - 200.0,
            )
            .accept_first_mouse(true)
            .skip_taskbar(true)
            .shadow(true)
            .always_on_top(true)
            .decorations(false)
            .resizable(false)
            .visible(false)
            .effects(WindowEffectsConfig {
                #[cfg(target_os = "macos")]
                effects: vec![WindowEffect::Popover],
                #[cfg(target_os = "macos")]
                state: Some(WindowEffectState::Active),
                #[cfg(target_os = "macos")]
                radius: Some(10.0),
                #[cfg(target_os = "windows")]
                effects: vec![WindowEffect::Mica],
                ..WindowEffectsConfig::default()
            });

    #[cfg(not(target_os = "macos"))]
    {
        record_win = record_win.transparent(true);
    }

    let record_win = record_win
        .build()
        .expect("Failed to build record button window");
    record_win
        .to_owned()
        .set_visible_on_all_workspaces(true)
        .expect("Couldn't set visible on all workspaces");

    // #[cfg(target_os = "windows")]
    // hide_using_window_affinity(record_win.hwnd().unwrap());

    #[cfg(target_os = "macos")]
    set_transparency_and_level(record_win, 26);
}

fn is_pointer_on_monitor(app: &AppHandle, monitor: &tauri::window::Monitor) -> bool {
    let cursor_position = app.cursor_position().unwrap();
    let posx = cursor_position.x;
    let posy = cursor_position.y;
    let monitor_start = monitor.position();
    let monitor_boundaries = monitor.size();

    let ms_x: f64 = monitor_start.x.into();
    let ms_y: f64 = monitor_start.y.into();
    let mb_w: f64 = monitor_boundaries.width.into();
    let mb_h: f64 = monitor_boundaries.height.into();

    if ((posx >= ms_x) && (posy >= ms_y) && (posx <= (ms_x + mb_w)) && (posy <= (ms_y + mb_h))) {
        return true;
    } else {
        return false;
    }
}

fn monitor_from_point(app: &AppHandle) -> Option<tauri::window::Monitor> {
    let monitors = app.available_monitors().unwrap();
    let cursor_position = app.cursor_position().unwrap();
    let posx = cursor_position.x;
    let posy = cursor_position.y;

    for monitor in monitors {
        if is_pointer_on_monitor(app, &monitor) {
            return Some(monitor);
        }
    }
    return None;
}

fn create_cropper_win(app: &AppHandle) {
    //  get size of primary monitor
    // let monitors = app.available_monitors();
    let primary_monitor = app.primary_monitor().unwrap().unwrap();
    // let monitors = app.available_monitors();
    // println!("{:?}", monitors);
    let scale_factor = primary_monitor.scale_factor();
    let monitor_size = primary_monitor.size().to_logical(scale_factor);

    // create cropper window
    let mut cropper_win =
        WebviewWindowBuilder::new(app, "cropper", WebviewUrl::App("/cropper".into()))
            .title("cropper window")
            .inner_size(monitor_size.width, monitor_size.height)
            .accept_first_mouse(true)
            .skip_taskbar(true)
            .position(0.0, 0.0)
            .always_on_top(true)
            .decorations(false)
            .resizable(false)
            .visible(false)
            .shadow(false)
            .focused(false);

    // set transparent only on windows and linux
    #[cfg(not(target_os = "macos"))]
    {
        cropper_win = cropper_win.transparent(true);
    }

    let cropper_win = cropper_win.build().expect("Failed to open cropper");
    cropper_win.set_visible_on_all_workspaces(true).unwrap();

    // #[cfg(target_os = "windows")]
    // hide_using_window_affinity(cropper_win.hwnd().unwrap());

    #[cfg(target_os = "macos")]
    set_transparency_and_level(cropper_win, 25);
}

pub fn init_cropper(app: &AppHandle) {
    create_record_button_win(app);
    create_cropper_win(app);
}

pub fn toggle_cropper(app: &AppHandle) {
    if let (Some(cropper_win), Some(record_win)) = (
        app.get_webview_window("cropper"),
        app.get_webview_window("record"),
    ) {
        let cursor_position = app.cursor_position();
        let some_monitor = app.monitor_from_point(1.0, 1.0);

        let current_monitor = monitor_from_point(app).unwrap();
        let position =
            PhysicalPosition::new(current_monitor.position().x, current_monitor.position().y);
        let current_monitor_size = current_monitor.size();

        let cmsw: f64 = current_monitor_size.width.into(); // all because a simple cast from i32 to u32 is black magic in rust
        let cmsh: f64 = current_monitor_size.height.into();

        let px: f64 = position.x.into();
        let py: f64 = position.y.into();

        let recorder_position = PhysicalPosition::new(((cmsw) / 2.0) + px, ((cmsh) - 200.0) + py);

        let current_monitor_logical_size: LogicalSize<f64> = current_monitor
            .size()
            .to_logical(current_monitor.scale_factor());
        let size = PhysicalSize::new(current_monitor_size.width, current_monitor_size.height);

        match cropper_win.is_visible().unwrap() || record_win.is_visible().unwrap() {
            true => {
                record_win.hide().unwrap();
                cropper_win.hide().unwrap();
                cropper_win.set_position(position);
                cropper_win.set_size(size);
                println!(
                    "Current monitor size on hide: {:?}",
                    cropper_win.inner_size()
                );
                app.emit("reset-area", ()).expect("couldn't reset area");
            }
            false => match scap::has_permission() {
                true => {
                    app.emit("reset-area", ()).expect("couldn't reset area");
                    record_win.set_position(recorder_position);
                    record_win.show().unwrap();
                    cropper_win.show().unwrap();
                    cropper_win.set_position(position); // must set postition before setting the size.
                    cropper_win.set_size(size);
                    println!(
                        "Current monitor size on show: {:?}",
                        cropper_win.inner_size()
                    );
                    cropper_win.set_focus().unwrap();
                }
                false => {
                    crate::open_welcome_window(app);
                }
            },
        }
    }
}

#[tauri::command]
pub async fn hide_cropper(app: AppHandle) {
    if let (Some(cropper_win), Some(record_win)) = (
        app.get_webview_window("cropper"),
        app.get_webview_window("record"),
    ) {
        record_win.hide().unwrap();
        cropper_win.hide().unwrap();
        app.emit("reset-area", ()).expect("couldn't reset area");
    }
}

#[tauri::command]
pub async fn update_crop_area(app: AppHandle, area: Vec<u32>) {
    println!("area: {:?}", area);

    if let Some(record_window) = app.get_webview_window("record") {
        record_window
            .emit("updated-crop-area", area.clone())
            .expect("couldn't pass crop area to record_window");

        record_window.set_focus().unwrap();
    }

    let state = app.state::<AppState>();
    let mut cropped_area = state.cropped_area.lock().await;
    *cropped_area = area.clone();
    drop(cropped_area);
}
