use crate::AppState;
use tauri::{AppHandle, LogicalSize, Manager, WebviewUrl, WebviewWindowBuilder};
use windows::Win32::UI::WindowsAndMessaging::{SetWindowDisplayAffinity, WINDOW_DISPLAY_AFFINITY};


fn create_record_button_win(app: &AppHandle) {

    let primary_monitor = app.primary_monitor().unwrap().unwrap();
    let scale_factor = primary_monitor.scale_factor();
    let monitor_size: LogicalSize<f64> = primary_monitor.size().to_logical(scale_factor);

    let mut record_win =
        WebviewWindowBuilder::new(app, "record", WebviewUrl::App("/record".into()))
            .title("recorder window")
            .inner_size(64.0, 64.0)
            .position((monitor_size.width / 2.0) - 32.0,  monitor_size.height - 200.0)
            .accept_first_mouse(true)
            .skip_taskbar(true)
            .shadow(false)
            .always_on_top(true)
            .decorations(false)
            .resizable(false)
            .visible(false);

    #[cfg(not(target_os = "macos"))]
    {
        record_win = record_win.transparent(true);
    }

    let record_win = record_win.build().expect("Failed to build record button window");

    #[cfg(target_os = "windows")]
    {
        if let Ok(hwnd) = record_win.hwnd() {
            let affinity = WINDOW_DISPLAY_AFFINITY(0x00000011);
            unsafe {
                let _ = SetWindowDisplayAffinity(hwnd, affinity);
            }
        }
    }
    #[cfg(target_os = "macos")]
    {
        use cocoa::{appkit::NSColor, base::nil, foundation::NSString};
        use objc::{class, msg_send, sel, sel_impl};

        record_win
            .to_owned()
            .run_on_main_thread(move || {
                let id = record_win.ns_window().unwrap() as cocoa::base::id;

                unsafe {
                    // set window level to 26
                    let _: cocoa::base::id = msg_send![id, setLevel: 26];

                    let color =
                        NSColor::colorWithSRGBRed_green_blue_alpha_(nil, 0.0, 0.0, 0.0, 0.0);
                    let _: cocoa::base::id = msg_send![id, setBackgroundColor: color];
                    record_win.with_webview(|webview| {
                        // !!! has delay
                        let id = webview.inner();
                        let no: cocoa::base::id = msg_send![class!(NSNumber), numberWithBool:0];
                        let _: cocoa::base::id =
                                msg_send![id, setValue:no forKey: NSString::alloc(nil).init_str("drawsBackground")];
                    }).ok();
                }
            })
        .unwrap();
    }
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
            // .inner_size(monitor_size.width, monitor_size.height)
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

    #[cfg(target_os = "windows")]
    {
        if let Ok(hwnd) = cropper_win.hwnd() {
            let affinity = WINDOW_DISPLAY_AFFINITY(0x00000011);
            unsafe {
                let _ = SetWindowDisplayAffinity(hwnd, affinity);
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        use cocoa::{appkit::NSColor, base::nil, foundation::NSString};
        use objc::{class, msg_send, sel, sel_impl};

        cropper_win
            .to_owned()
            .run_on_main_thread(move || {
                let id = cropper_win.ns_window().unwrap() as cocoa::base::id;

                unsafe {
                    // set window level to 25
                    let _: cocoa::base::id = msg_send![id, setLevel: 25];
                    
                    // Make the webview and window background transparent
                    let color =
                    NSColor::colorWithSRGBRed_green_blue_alpha_(nil, 0.0, 0.0, 0.0, 0.0);
                    let _: cocoa::base::id = msg_send![id, setBackgroundColor: color];
                    cropper_win.with_webview(|webview| {
                        // !!! has delay
                        let id = webview.inner();
                        let no: cocoa::base::id = msg_send![class!(NSNumber), numberWithBool:0];
                        let _: cocoa::base::id = msg_send![id, setValue:no forKey: NSString::alloc(nil).init_str("drawsBackground")];
                    }).ok();
                }
            })
            .unwrap();
    }

}

pub fn init_cropper(app: &AppHandle) {
    // Note: we need to create the record button window first
    // Because the JS in cropper window needs a handle to it
    create_record_button_win(app);
    create_cropper_win(app);
}

pub fn toggle_cropper(app: &AppHandle) {
    if !scap::has_permission() {
        crate::open_welcome_window(app);
        return;
    }

    if let Some(cropper_win) = app.get_webview_window("cropper") {
        match cropper_win.is_visible() {
            Ok(true) => {
                cropper_win.hide().unwrap();
                cropper_win
                    .emit("reset-cropper", ())
                    .expect("couldn't reset cropper");
                if let Some(record_button_win) = app.get_webview_window("record") {
                    if record_button_win.is_visible().unwrap() {
                        record_button_win.hide().unwrap();
                    }
                }
            }
            Ok(false) => {
                cropper_win
                    .emit("reset-cropper", ())
                    .expect("couldn't reset cropper");
                cropper_win.show().unwrap();
                cropper_win.set_focus().unwrap();
            }
            Err(_) => {}
        }
    }
}

#[tauri::command]
pub async fn update_crop_area(app: AppHandle, area: Vec<u32>) {
    println!("area: {:?}", area);

    if let Some(record_button_window) = app.get_webview_window("record") {
        record_button_window.show().unwrap();
    }

    let state = app.state::<AppState>();
    let mut cropped_area = state.cropped_area.lock().await;
    *cropped_area = area.clone();
    drop(cropped_area);
}
