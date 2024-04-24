use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

pub fn init_cropper(app: &AppHandle) {
    //  get size of primary monitor
    let primary_monitor = app.primary_monitor().unwrap().unwrap();
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
                    // set window level
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

pub fn toggle_cropper(app: &AppHandle) {
    // let state_mutex = app.state::<Mutex<AppState>>();
    // let mut state = state_mutex.blocking_lock();
    // match state.status ...

    // TODO: figure out why the above doesn't work
    // Ask in Tauri Discord.

    if !scap::has_permission() {
        crate::open_onboarding(app);
        return;
    }

    if let Some(cropper_win) = app.get_webview_window("cropper") {
        match cropper_win.is_visible() {
            Ok(true) => {
                cropper_win.hide().unwrap();
                cropper_win
                    .emit("reset-cropper", ())
                    .expect("couldn't reset cropper");
                if let Some(toolbar_win) = app.get_webview_window("toolbar") {
                    if toolbar_win.is_visible().unwrap() {
                        toolbar_win.hide().unwrap();
                    }
                }
            }
            Ok(false) => {
                cropper_win.show().unwrap();
                cropper_win.set_focus().unwrap();
            }
            Err(_) => {}
        }
    }
}
