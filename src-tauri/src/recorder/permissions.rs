// This file is Mac-specific, it ensures we have the right permissions.

use tauri::AppHandle;

fn open_permission_settings(app: &AppHandle) {
    use std::process::Command;
    use tauri_plugin_dialog::DialogExt;

    // open a dialog to request permission
    app.dialog()
        .message("Helmer Micro needs permission to record your screen.")
        .title("Screen Recording")
        .ok_button_label("Open Settings")
        .show(move |result| {
            match result {
                true => {
                    Command::new("open")
                    .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_ScreenCapture")
                    .output()
                    .expect("failed to open security settings");
                }
                false => {
                    println!("User denied permission")
                }
            }
        });
}

pub fn ensure_recording_permissions(app: &AppHandle) {
    // scap::request_permission returns immediately
    // It uses CGRequestScreenCaptureAccess from Apple's CoreGraphics framework
    // Unfortunately there are no callbacks or events we can listen to.

    // FLOW 1: ✅ if the user has already granted permission, returns true.
    // FLOW 2: 🔴 if the user has explicitly denied permission, returns false with no prompt
    // FLOW 3: ⚠️ if user has not yet granted or denied permission, returns false but also prompts

    if !scap::request_permission() {
        println!("Don't have permission atm.");
        // 2 situations:

        // IF

        // the system has prompted the user for permission
        // they are about to grant it and restart app

        // Then do nothing, the user will restart the app eventually

        // ELSE

        // user has accidentally denied permission now or before
        // and we need to manually prompt them

        // Demand permissions and open settings
        // open_permission_settings(&app);
    }
}
