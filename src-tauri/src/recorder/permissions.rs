// This file is Mac-specific, it ensures we have the right permissions.

use crate::AppState;
use std::process::Command;
use tauri::{AppHandle, Manager};
use tauri_plugin_dialog::DialogExt;

fn open_permission_settings(app: &AppHandle) {
    app.dialog()
        .message("Helmer Micro needs permission to record your screen.")
        .title("Screen Recording")
        .ok_button_label("Open Security Settings")
        .show(|result| match result {
            true => {
                Command::new("open")
                    .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_ScreenCapture")
                    .output()
                    .expect("failed to open security settings");
            }
            false => {
                println!("User denied permission")
            }
        });
}

pub async fn ensure_recording_permissions(app: &AppHandle) {
    // scap::request_permission returns immediately
    // It uses CGRequestScreenCaptureAccess from Apple's CoreGraphics framework
    // Unfortunately there are no callbacks or events we can listen to.

    // FLOW 1: ✅ if the user has already granted permission, returns true.
    // FLOW 2: 🔴 if the user has explicitly denied permission, returns false with no prompt
    // FLOW 3: ⚠️ if user has not yet granted or denied permission, returns false but also prompts

    let state = app.state::<AppState>();
    let mut shown_permission_prompt = state.shown_permission_prompt.lock().await;

    if !scap::request_permission() {
        if shown_permission_prompt.to_owned() {
            // FLOW 2

            // if shown_permission_prompt is false
            // assume we have already prompted the user
            // they've either accidentally or explicitly denied it
            // so we need to manually prompt them
            open_permission_settings(&app);
        }

        // FLOW 3
        // if shown_permission_prompt is false
        // assume the system has prompted the user for permission
        // they are (hopefully) about to grant it and restart app
        // so we update the state to true and do nothing else
        *shown_permission_prompt = true;
        return;
    }
}
