// use scap::capturer::{Capturer, Options};
use crate::{AppState, Status};
use std::sync::Arc;
use tauri::{AppHandle, Manager};
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
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    stop_capture(app_handle.clone()).await;
    println!("complete"); // this never runs
}

#[tauri::command]
pub async fn stop_capture(app_handle: AppHandle) {
    println!("Capture stopped");
    app_handle.emit_all("capture-stopped", false).unwrap();

    // TODO: stop capturing with scap

    // Hide cropper, create editor
    crate::cropper::toggle_cropper(&app_handle);
    crate::toolbar::toggle_toolbar(&app_handle);
    crate::editor::init_editor(&app_handle);

    // Update app state
    let state_mutex = app_handle.state::<Mutex<AppState>>();
    let mut state = state_mutex.lock().await; // TODO: this line is not running
    println!("Updating app state");

    state.status = Status::Editing;

    println!("Status: {:?}", state.status);
}
