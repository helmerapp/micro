use crate::AppState;
use tauri::{AppHandle, Manager, PhysicalPosition, PhysicalSize, Position, WebviewUrl, WebviewWindowBuilder};

fn create_record_button_win(app: &AppHandle) {
    let mut record_win =
        WebviewWindowBuilder::new(app, "record", WebviewUrl::App("/record".into()))
            .inner_size(64.0, 64.0)
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

    let record_win = record_win
        .build()
        .expect("Failed to build record button window");

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

    if ((posx >= ms_x)
        && (posy >= ms_y)
        && (posx <= ((ms_x + mb_w) ))
        && (posy <= ((ms_y + mb_h))))
    {
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


fn spawn_window(app: &AppHandle) {
    // grab a list of monitors.
    let monitors = app.available_monitors().unwrap();

    // grab the monitor with my cursor on it.
    // let cursor_position = app.cursor_position().unwrap();
    // let posx = cursor_position.x;
    // let posy = cursor_position.y;
    // println!("Cursor on Pos: {:?}",posx);
    // println!("Cursor on Pos: {:?}",posy);
    // println!("primary monitor: {:?}",monitors);
    // println!("Cursor on monitor: {:?}", app.monitor_from_point(posx, posy)); // returns None regardless of cursor position
    println!("Current monitor ?: {:?}", monitor_from_point(app));

    // create the cropper win there.
    create_cropper_win(app);
    // create the record button win there.

    create_record_button_win(app);
}

fn create_cropper_win(app: &AppHandle) {
    //  get size of primary monitor
    // let monitors = app.available_monitors();
    let primary_monitor = app.primary_monitor().unwrap().unwrap();
    let monitors = app.available_monitors();
    println!("{:?}", monitors);
    let scale_factor = primary_monitor.scale_factor();
    let monitor_size = primary_monitor.size().to_logical(scale_factor);

    // create cropper window
    let mut cropper_win =
        WebviewWindowBuilder::new(app, "cropper", WebviewUrl::App("/cropper".into()))
            // .inner_size(monitor_size.width, monitor_size.height)
            .inner_size(monitor_size.width, monitor_size.height)
            .accept_first_mouse(true)
            .skip_taskbar(true)
            .position(0.0, 0.0)
            .always_on_top(true)
            .decorations(false)
            .resizable(false)
            .visible(false)
            .position(10.00, 10.00)
            .focused(false);

    // set transparent only on windows and linux
    #[cfg(not(target_os = "macos"))]
    {
        cropper_win = cropper_win.transparent(true);
    }

    let cropper_win = cropper_win.build().expect("Failed to open cropper");
    cropper_win.set_visible_on_all_workspaces(true).unwrap();

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
    spawn_window(app);
    // create_record_button_win(app);
    // create_cropper_win(app);
}

pub fn toggle_cropper(app: &AppHandle) {
    if !scap::has_permission() {
        crate::open_welcome_window(app);
        return;
    }
    let cursor_position = app.cursor_position();
    let some_monitor = app.monitor_from_point(1.0, 1.0);

    let current_monitor = monitor_from_point(app).unwrap();
    let position = PhysicalPosition::new(current_monitor.position().x, current_monitor.position().y);
    let current_monitor_size = current_monitor.size();
    let size = PhysicalSize::new(current_monitor_size.width, current_monitor_size.height);
    
    // println!("Current monitor ?: {:?}", monitor_from_point(app));
    println!("CURSOR PHYSICAL POSITION: {:?}", current_monitor.position());
    if let Some(cropper_win) = app.get_webview_window("cropper") {
        match cropper_win.is_visible() {
            Ok(true) => {
                cropper_win.hide().unwrap();
                cropper_win
                    .emit("reset-cropper", ())
                    .expect("couldn't reset cropper");
                    cropper_win.set_position(position);
                    cropper_win.set_size(size);
                    println!("Current monitor size on hide: {:?}", cropper_win.inner_size());
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
                cropper_win.set_position(position); // must set postition before setting the size.
                cropper_win.set_size(size);
                println!("Current monitor size on show: {:?}", cropper_win.inner_size());
                cropper_win.show().unwrap();
                cropper_win.set_focus().unwrap();
            }
            Err(_) => {}
        }
    }
}

#[tauri::command]
pub async fn update_crop_area(app: AppHandle, button_coords: Vec<u32>, area: Vec<u32>) {
    println!("button_coords: {:?}", button_coords);
    println!("area: {:?}", area);

    if let Some(record_button_window) = app.get_webview_window("record") {
        let pos = Position::Logical((button_coords[0], button_coords[1]).into());
        record_button_window.set_position(pos).unwrap();

        // wait to ensure window is positioned correctly
        std::thread::sleep(std::time::Duration::from_millis(100));

        record_button_window.show().unwrap();
    }

    let state = app.state::<AppState>();
    let mut cropped_area = state.cropped_area.lock().await;
    *cropped_area = area.clone();
    drop(cropped_area);
}
