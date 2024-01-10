use std::sync::Arc;
use tauri::{AppHandle, Manager};
use tokio::sync::Mutex;

#[tauri::command]
pub async fn start_capturer(area: Vec<u32>, app_handle: AppHandle) {
    println!("Capturing Area: {:?}", area);
}

pub struct Recorder {}

pub fn new() -> Recorder {
    return Recorder {};
}

pub async fn start(capturer: &Arc<Mutex<Recorder>>) {
    let mut capturer = capturer.lock().await;
    println!("Starting recorder");
}

pub async fn stop(capturer: &Arc<Mutex<Recorder>>) -> String {
    let mut capturer = capturer.lock().await;

    println!("Stopping recorder");

    "".into()
}
