use crate::AppState;
use tauri::{AppHandle, LogicalSize, Manager, Position, Size, WindowBuilder, WindowUrl};
use tokio::sync::Mutex;

pub fn init_toolbar(app: &AppHandle) {
    let mut toolbar_win = WindowBuilder::new(app, "toolbar", WindowUrl::App("/toolbar".into()))
        .accept_first_mouse(true)
        .always_on_top(true)
        .decorations(false)
        .resizable(false)
        .visible(false)
        .focused(true)
        .center();

    #[cfg(not(target_os = "macos"))]
    {
        toolbar_win = toolbar_win.transparent(true);
    }

    let toolbar_win = toolbar_win.build().expect("Failed to open toolbar");

    let size = Size::Logical(LogicalSize {
        width: 150.0,
        height: 70.0,
    });

    toolbar_win.set_size(size).unwrap();

    #[cfg(target_os = "macos")]
    {
        use cocoa::{appkit::NSColor, base::nil, foundation::NSString};
        use objc::{class, msg_send, sel, sel_impl};

        toolbar_win
        .to_owned()
        .run_on_main_thread(move || unsafe {
            let id = toolbar_win.ns_window().unwrap() as cocoa::base::id;
            let color =
                NSColor::colorWithSRGBRed_green_blue_alpha_(nil, 0.0, 0.0, 0.0, 0.0);
            let _: cocoa::base::id = msg_send![id, setBackgroundColor: color];
            toolbar_win.with_webview(|webview| {
                // !!! has delay
                let id = webview.inner();
                let no: cocoa::base::id = msg_send![class!(NSNumber), numberWithBool:0];
                let _: cocoa::base::id =
                        msg_send![id, setValue:no forKey: NSString::alloc(nil).init_str("drawsBackground")];
        })
        .ok();
        })
        .unwrap();
    }
}

// create a toggle_toolbar function
pub fn toggle_toolbar(app: &AppHandle) {
    let toolbar_win = app.get_window("toolbar").unwrap();
    if toolbar_win.is_visible().unwrap() {
        toolbar_win.hide().unwrap();
    } else {
        toolbar_win.show().unwrap();
    }
}

#[tauri::command]
pub async fn show_toolbar(button_coords: Vec<u32>, area: Vec<u32>, app: AppHandle) {
    if app.get_window("toolbar").is_none() {
        println!("creating new toolbar window");
        crate::toolbar::init_toolbar(&app);
    }
    println!("Toolbar at {:?}", button_coords);
    let toolbar_win = app.get_window("toolbar").unwrap();
    let pos = Position::Logical((button_coords[0], button_coords[1]).into());
    toolbar_win.set_position(pos).unwrap();
    toolbar_win.show().unwrap();
    toolbar_win.set_focus().unwrap();
    let state = app.state::<AppState>();
    let mut cropped_area = state.cropped_area.lock().await;
    *cropped_area = area.clone();
    drop(cropped_area);
}

#[tauri::command]
pub async fn hide_toolbar(app: AppHandle) {
    let toolbar_win = app.get_window("toolbar").unwrap();
    toolbar_win.hide().unwrap();
}
