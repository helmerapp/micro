use scap::capturer::{Capturer, Options};
use std::sync::Arc;
use tauri::{AppHandle, Manager};
use tokio::sync::Mutex;

#[tauri::command]
pub async fn start_capturing(area: Vec<u32>, app_handle: AppHandle) {
    println!("Cropped Area: {:?}", area);

    //     let capturer = new();
    //     capturer.start_capture();

    // TEMP: stop capturing after 5 seconds, so I get get to editor
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    stop_capturing(app_handle).await;
}

#[tauri::command]
pub async fn stop_capturing(app_handle: AppHandle) {
    // capturer.stop_capture();
    println!("Capturer stopped");

    // TODO: hide cropper and toolbar
    // TODO: show editor window
    crate::editor::init_editor(&app_handle);

    // TODO; create editor window here
}

pub fn new() -> Capturer {
    let options = Options::default();

    return Capturer::new(options);
}

pub async fn start(capturer: &Arc<Mutex<Capturer>>) {
    let mut capturer = capturer.lock().await;
    println!("Starting recorder");
}

pub async fn stop(capturer: &Arc<Mutex<Capturer>>) -> String {
    let mut capturer = capturer.lock().await;

    println!("Stopping recorder");

    "".into()
}
