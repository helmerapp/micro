use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Capturer {}

pub fn new() -> Capturer {
    return Capturer {};
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
