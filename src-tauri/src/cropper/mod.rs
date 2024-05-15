use crate::AppState;
use tauri::{
    utils::WindowEffect, AppHandle, LogicalSize, Manager, WebviewUrl, WebviewWindowBuilder,
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

fn create_cropper_win(app: &AppHandle) {
    //  get size of primary monitor
    let primary_monitor = app.primary_monitor().unwrap().unwrap();
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
    // Note: we need to create the record button window first
    // Because the JS in cropper window needs a handle to it
    create_record_button_win(app);
    create_cropper_win(app);
}

pub fn toggle_cropper(app: &AppHandle) {
    if let (Some(cropper_win), Some(record_win)) = (
        app.get_webview_window("cropper"),
        app.get_webview_window("record"),
    ) {
        match cropper_win.is_visible().unwrap() || record_win.is_visible().unwrap() {
            true => {
                record_win.hide().unwrap();
                cropper_win.hide().unwrap();
                app.emit("reset-area", ()).expect("couldn't reset area");
            }
            false => {
                app.emit("reset-area", ()).expect("couldn't reset area");
                record_win.show().unwrap();
                cropper_win.show().unwrap();
                cropper_win.set_focus().unwrap();
            }
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
