use scap::capturer::{Capturer, Options};
use std::sync::Arc;
use tauri::{AppHandle, Manager};
use tokio::sync::Mutex;

#[tauri::command]
pub async fn start_capturer(area: Vec<u32>, app_handle: AppHandle) {
    println!("Cropped Area: {:?}", area);

    let capturer = new();
    capturer.start_capture();
    println!("Capturer started");

    // wait for 10 seconds
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

    capturer.stop_capture();
    println!("Capturer stopped");
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
