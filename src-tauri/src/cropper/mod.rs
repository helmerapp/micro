use tauri::{AppHandle, Manager, PhysicalSize, Position, Size, WebviewUrl, WebviewWindowBuilder};

pub fn init_cropper(app: &AppHandle) {
    // create cropper window
    let mut cropper_win =
        WebviewWindowBuilder::new(app, "cropper", WebviewUrl::App("/cropper".into()))
            .accept_first_mouse(true)
            .skip_taskbar(true)
            .always_on_top(true)
            .decorations(false)
            .resizable(false)
            .visible(false)
            .focused(false)
            .center();

    // set transparent only on windows and linux
    #[cfg(not(target_os = "macos"))]
    {
        cropper_win = cropper_win.transparent(true);
    }

    let cropper_win = cropper_win.build().expect("Failed to open cropper");

    let monitor = cropper_win.primary_monitor().unwrap().unwrap();

    let size = Size::Physical(PhysicalSize {
        width: monitor.size().width,
        height: monitor.size().height,
    });

    let pos = Position::Physical((0, 0).into());

    cropper_win.set_size(size).unwrap();
    cropper_win.set_position(pos).unwrap();

    #[cfg(target_os = "macos")]
    {
        use cocoa::{appkit::NSColor, base::nil, foundation::NSString};
        use objc::{class, msg_send, sel, sel_impl};

        cropper_win
        .to_owned()
        .run_on_main_thread(move || unsafe {
            let id = cropper_win.ns_window().unwrap() as cocoa::base::id;
            let color =
                NSColor::colorWithSRGBRed_green_blue_alpha_(nil, 0.0, 0.0, 0.0, 0.0);
            let _: cocoa::base::id = msg_send![id, setBackgroundColor: color];
            cropper_win.with_webview(|webview| {
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

pub fn toggle_cropper(app: &AppHandle) {
    // TODO: figure out why state doesn't work here.
    // Ask in Tauri Discord.

    // let state_mutex = app.state::<Mutex<AppState>>();
    // let mut state = state_mutex.blocking_lock();

    // match state.status {
    //     Status::Idle => {
    let cropper_win = app.get_webview_window("cropper").unwrap();
    if cropper_win.is_visible().unwrap() {
        cropper_win.hide().unwrap();
        // state.status = Status::Idle;
    } else {
        cropper_win.show().unwrap();
        // state.status = Status::Cropper;
    }
    // }
    // _ => {}
    // }
}
