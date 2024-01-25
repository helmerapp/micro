// use scap::capturer::{Capturer, Options};
use std::sync::Arc;
use tauri::{AppHandle, Manager};
use tokio::sync::Mutex;

#[tauri::command]
pub async fn start_capture(area: Vec<u32>, app_handle: AppHandle) {
    println!("Cropped Area: {:?}", area);

    // TODO: fire event
    // TODO: initialize scap and start capturing
    // TODO: update app state

    // TEMP: stop capturing after 5 seconds, to get to editor
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    stop_capture(app_handle).await;
}

#[tauri::command]
pub async fn stop_capture(app_handle: AppHandle) {
    println!("Capture stopped");

    // TODO: fire event
    // TODO: stop capturing with scap and cleanup

    // Hide cropper, create editor
    crate::cropper::toggle_cropper(&app_handle);
    crate::editor::init_editor(&app_handle);
}
