use scap::{Options, Recorder};
use std::sync::Arc;
use tauri::{AppHandle, Manager};
use tokio::sync::Mutex;

#[tauri::command]
pub async fn start_capturer(area: Vec<u32>, app_handle: AppHandle) {
    println!("Capturing Area: {:?}", area);

    let recorder = new();
}

pub fn new() -> Recorder {
    let targets = scap::get_targets();

    let options = Options {
        fps: 60,
        targets,
        show_cursor: true,
        show_highlight: false,
        excluded_targets: None,
    };

    return Recorder::init(options);
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
