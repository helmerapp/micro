use crate::AppState;
use core_graphics_helmer_fork::display::CGDisplayBounds;
use tauri::{
    utils::WindowEffect, AppHandle, LogicalPosition, LogicalSize, Manager, WebviewUrl,
    WebviewWindowBuilder,
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

fn create_record_button_win(app: &AppHandle, target: scap::Target, label: &str) {
    match target {
        scap::Target::Display(target) => {
            let monitor_size: LogicalSize<f64> = {
                let mode = target.raw_handle.display_mode().unwrap();
                LogicalSize::<f64>::new(mode.width() as f64, mode.height() as f64)
            };

            const RECORD_BUTTON_WIDTH: f64 = 200.0;
            const RECORD_BUTTON_HEIGHT: f64 = 48.0;

            let record_win =
                WebviewWindowBuilder::new(app, label, WebviewUrl::App("/record".into()))
                    .title(format!("recorder window {}", target.title.as_str()))
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

            #[cfg(target_os = "macos")]
            {
                let position: LogicalPosition<f64> = unsafe {
                    let bounds = CGDisplayBounds(target.raw_handle.id);
                    LogicalPosition::<f64>::new(
                        bounds.origin.x + (monitor_size.width / 2.0) - (RECORD_BUTTON_WIDTH / 2.0),
                        bounds.origin.y + monitor_size.height - 200.0,
                    )
                };
                record_win.set_position(position).unwrap();
                record_win.center().unwrap();
            }

            record_win
                .to_owned()
                .set_visible_on_all_workspaces(true)
                .expect("Couldn't set visible on all workspaces");
            // #[cfg(target_os = "windows")]
            // hide_using_window_affinity(record_win.hwnd().unwrap());

            #[cfg(target_os = "macos")]
            set_transparency_and_level(record_win, 26);
        }
        scap::Target::Window(_) => {}
    }
}

fn create_cropper_win(app: &AppHandle, target: scap::Target, label: &str) {
    match target {
        scap::Target::Display(target) => {
            // This piece of code can be used from scap if these methods are made public.
            // It is repeated in create_button method (and not purposefully extracted into a method)
            // as this can also be removed after/if we reuse this from scap.
            let scale_factor = {
                #[cfg(target_os = "macos")]
                {
                    let mode = target.raw_handle.display_mode().unwrap();
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
                    let mode = target.raw_handle.display_mode().unwrap();
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

            // create cropper window
            let cropper_win =
                WebviewWindowBuilder::new(app, label, WebviewUrl::App("/cropper".into()))
                    .title(format!("cropper window {}", target.title.as_str()))
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

            // This is required for non-primary monitors - at least on mac
            // Since multi-displays is currently not implemented for other
            // platforms, this won't be required.
            #[cfg(target_os = "macos")]
            {
                let position: LogicalPosition<f64> = unsafe {
                    let bounds = CGDisplayBounds(target.raw_handle.id);
                    LogicalPosition::<f64>::new(
                        bounds.origin.x * scale_factor,
                        bounds.origin.y * scale_factor,
                    )
                };
                cropper_win.set_position(position).unwrap();
                cropper_win.center().unwrap();
            }
            // #[cfg(target_os = "windows")]
            // hide_using_window_affinity(cropper_win.hwnd().unwrap());

            #[cfg(target_os = "macos")]
            set_transparency_and_level(cropper_win, 25);
        }
        scap::Target::Window(_) => {}
    }
    //  get size of primary monitor
}

pub fn init_cropper(app: &AppHandle) {
    // TODO: Remove dependency on static labels and hence fixed
    // number of windows/displays by re-using labels if possible.
    //
    // More Context:
    //
    // The labels need to be unique for all windows and we create
    // different "record" and "capture" windows for all the
    // displays/windows.
    // These labels are statically declared in capabilities.json file
    // and at max 4 displays are supported at the moment(hard-coded)
    // till we figure out destroying windows and also removing labels
    // with that
    //
    // In short: Creating 2 tauri windows with same labels fails and
    // labels need declaration in "windows" property of tauri
    // capabilities.json file.

    let state = app.state::<AppState>();
    state.targets.iter().enumerate().for_each(|(i, target)| {
        create_record_button_win(app, target.to_owned(), format!("record-{}", i).as_str());
        create_cropper_win(app, target.to_owned(), format!("cropper-{}", i).as_str());
    });
}

pub fn toggle_cropper(app: &AppHandle) {
    let state = app.state::<AppState>();

    let curr_ind = match state.current_target_ind.try_lock() {
        Ok(data) => data,
        Err(_) => {
            eprintln!("[toggle_cropper]: Could not get lock on current target index!");
            return;
        }
    };
    let record_win_label = format!("record-{}", curr_ind.clone());
    let cropper_win_label = format!("cropper-{}", curr_ind.clone());
    drop(curr_ind);

    if let (Some(cropper_win), Some(record_win)) = (
        app.get_webview_window(cropper_win_label.as_str()),
        app.get_webview_window(record_win_label.as_str()),
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
    let state = app.state::<AppState>();
    let curr_ind = state.current_target_ind.lock().await;
    let record_win_label = format!("record-{}", curr_ind.clone());
    let cropper_win_label = format!("cropper-{}", curr_ind.clone());
    drop(curr_ind);

    if let (Some(cropper_win), Some(record_win)) = (
        app.get_webview_window(cropper_win_label.as_str()),
        app.get_webview_window(record_win_label.as_str()),
    ) {
        record_win.hide().unwrap();
        cropper_win.hide().unwrap();
        app.emit("reset-area", ()).expect("couldn't reset area");
    }
}

#[tauri::command]
pub async fn update_crop_area(app: AppHandle, area: Vec<u32>) {
    let state = app.state::<AppState>();
    let curr_ind = state.current_target_ind.lock().await;
    let record_win_label = format!("record-{}", curr_ind.to_owned());
    drop(curr_ind);

    if let Some(record_window) = app.get_webview_window(record_win_label.as_str()) {
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
