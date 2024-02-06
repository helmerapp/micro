// use scap::capturer::{Capturer, Options};
use crate::{AppState, Status};
use tauri::{api::path::desktop_dir, AppHandle, Manager};
use tokio::sync::Mutex;

#[tauri::command]
pub async fn start_capture(area: Vec<u32>, app_handle: AppHandle) {
    app_handle.emit_all("capture-started", false).unwrap();

    // TODO: initialize scap and start capturing
    println!("Cropped Area: {:?}", area);

    // Update app state
    let state_mutex = app_handle.state::<Mutex<AppState>>();
    let mut state = state_mutex.lock().await;
    state.status = Status::Recording;

    // TEMP: stop capturing after 5 seconds, to get to editor
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    stop_capture(app_handle.clone()).await;
    println!("complete"); // this never runs
}

#[tauri::command]
pub async fn stop_capture(app_handle: AppHandle) {
    println!("Capture stopped");
    app_handle.emit_all("capture-stopped", false).unwrap();
    crate::cropper::toggle_cropper(&app_handle);

    // TODO: stop capturing with scap

    // Hide cropper, create editor
    crate::cropper::toggle_cropper(&app_handle);
    crate::toolbar::toggle_toolbar(&app_handle);
    // INFO: assume we have a video file path that can be used as preview
    // Currently I have hardcoded this to "Preview.mov" on desktop for testing.
    let preview_file = String::from(desktop_dir().unwrap().join("Preview.mov").to_str().unwrap());

    // Create editor and pass on the preview file to it
    crate::editor::init_editor(&app_handle, preview_file);

    // Update app state
    let state_mutex = app_handle.state::<Mutex<AppState>>();
    let mut state = state_mutex.lock().await; // TODO: this line is not running
    println!("Updating app state");

    state.status = Status::Editing;

    println!("Status: {:?}", state.status);
}
