use crate::AppState;
use core_graphics_helmer_fork::display::{self, CGDisplayBounds};
use mouse_position::mouse_position::Mouse;
use tauri::{
    utils::WindowEffect, App, AppHandle, LogicalPosition, LogicalSize, Manager, Monitor,
    PhysicalSize, Size, WebviewUrl, WebviewWindow, WebviewWindowBuilder,
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

fn tune_record_size_position(app: &AppHandle, target: &scap::Target) {
    let win_maybe = app.get_webview_window("record");
    if let Some(win) = win_maybe {
        const RECORD_BUTTON_WIDTH: f64 = 200.0;
        const RECORD_BUTTON_HEIGHT: f64 = 48.0;
        match target {
            scap::Target::Display(display) => {
                let monitor_size: LogicalSize<f64> = {
                    let mode = display.raw_handle.display_mode().unwrap();
                    LogicalSize::<f64>::new(mode.width() as f64, mode.height() as f64)
                };

                #[cfg(target_os = "macos")]
                {
                    let position: LogicalPosition<f64> = unsafe {
                        let bounds = CGDisplayBounds(display.raw_handle.id);
                        LogicalPosition::<f64>::new(
                            bounds.origin.x + (monitor_size.width / 2.0)
                                - (RECORD_BUTTON_WIDTH / 2.0),
                            bounds.origin.y + monitor_size.height - 200.0,
                        )
                    };
                    win.set_position(position).unwrap();
                    win.center().unwrap();
                }
                win.set_size(LogicalSize::new(RECORD_BUTTON_WIDTH, RECORD_BUTTON_HEIGHT))
                    .unwrap();
            }
            scap::Target::Window(_) => {}
        }
    }
}

fn tune_cropper_size_position(app_handle: &AppHandle, target: &scap::Target) {
    let win_maybe = app_handle.get_webview_window("cropper");
    if let Some(win) = win_maybe {
        match target {
            scap::Target::Display(display) => {
                let scale_factor = {
                    #[cfg(target_os = "macos")]
                    {
                        let mode = display.raw_handle.display_mode().unwrap();
                        (mode.pixel_width() / mode.width()) as f64
                    }
                    #[cfg(not(target_os = "macos"))]
                    {
                        let primary_monitor = app.primary_monitor().unwrap().unwrap();
                        primary_monitor.scale_factor();
                        primary_monitor.size().to_logical(scale_factor);
                    }
                };
                let monitor_size: LogicalSize<f64> = {
                    #[cfg(target_os = "macos")]
                    {
                        let mode = display.raw_handle.display_mode().unwrap();
                        LogicalSize::<f64>::new(
                            mode.width() as f64 * scale_factor,
                            mode.height() as f64 * scale_factor,
                        )
                    }
                    #[cfg(not(target_os = "macos"))]
                    {
                        let primary_monitor = app.primary_monitor().unwrap().unwrap();

                        primary_monitor.size().to_logical(scale_factor);
                    }
                };
                // This is required for non-primary monitors - at least on mac
                // Since multi-displays is currently not implemented for other
                // platforms, this won't be required.
                #[cfg(target_os = "macos")]
                {
                    let position: LogicalPosition<f64> = unsafe {
                        let bounds = CGDisplayBounds(display.raw_handle.id);
                        LogicalPosition::<f64>::new(
                            bounds.origin.x * scale_factor,
                            bounds.origin.y * scale_factor,
                        )
                    };
                    win.set_position(position).unwrap();
                    win.center().unwrap();
                }

                win.set_size(LogicalSize::new(monitor_size.width, monitor_size.height))
                    .unwrap();
            }

            scap::Target::Window(_) => {}
        }
    }
}

fn create_record_button_win(app: &AppHandle, label: &str) {
    let record_win = WebviewWindowBuilder::new(app, label, WebviewUrl::App("/record".into()))
        .title(format!("recorder window {}", "record"))
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

fn create_cropper_win(app: &AppHandle, label: &str) {
    // This piece of code can be used from scap if these methods are made public.
    // It is repeated in create_button method (and not purposefully extracted into a method)
    // as this can also be removed after/if we reuse this from scap.

    // create cropper window
    let cropper_win = WebviewWindowBuilder::new(app, label, WebviewUrl::App("/cropper".into()))
        .title(format!("cropper window {}", "cropper"))
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
    //  get size of primary monitor
}

pub fn init_cropper(app: &AppHandle) {
    create_record_button_win(app, "record");
    create_cropper_win(app, "cropper");
}

pub fn toggle_cropper(app: &AppHandle) {
    let position = Mouse::get_mouse_position();
    match position {
        Mouse::Position { x, y } => {
            if let Some(monitor) = app.monitor_from_point(x as f64, y as f64).unwrap() {
                if let Some(target) = get_target_from_monitor(app, monitor) {
                    tune_record_size_position(app, &target);
                    tune_cropper_size_position(app, &target);
                } else {
                    eprintln!("Could not deduce display target from tuari monitor");
                    return;
                }
            } else {
                println!("Could not get tauri monitor from mouse point");
            }
        }
        Mouse::Error => {
            eprintln!("Error getting mouse position");
            return;
        }
    }
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
            false => match scap::has_permission() {
                true => {
                    app.emit("reset-area", ()).expect("couldn't reset area");
                    record_win.show().unwrap();
                    cropper_win.show().unwrap();
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

fn get_target_from_monitor(app: &AppHandle, monitor: Monitor) -> Option<scap::Target> {
    let pos = monitor.position();
    let state = app.state::<AppState>();
    let mut ret_target = None;
    for target in state.targets.iter() {
        match target {
            scap::Target::Display(display) => {
                // No other fancy way to infer target from monitor other than physical position
                // which remains same across all monitors and display targets.
                unsafe {
                    let bounds = CGDisplayBounds(display.raw_handle.id);
                    if bounds.origin.x == (pos.x as f64) && bounds.origin.y == (pos.y as f64) {
                        update_target(app, target.clone());
                        ret_target = Some(target.clone());
                    }
                };
            }
            scap::Target::Window(_) => {}
        }
    }
    ret_target
}

fn update_target(app: &AppHandle, target: scap::Target) {
    let state = app.state::<AppState>();

    let mut data = match state.current_target.try_lock() {
        Ok(data) => data,
        Err(_) => {
            eprintln!("[update_target]: Could not get lock on current target index!");
            return;
        }
    };

    *data = Some(target);
    drop(data);
}
