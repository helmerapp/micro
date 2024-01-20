use scap::capturer::{Capturer, Options};
use std::sync::Arc;
use tauri::{AppHandle, Manager};
use tokio::sync::Mutex;

#[tauri::command]
pub async fn start_capturer(area: Vec<u32>, app_handle: AppHandle) {
    println!("Capturing Area: {:?}", area);
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
