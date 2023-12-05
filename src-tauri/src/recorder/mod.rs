use std::sync::Arc;
use tokio::sync::Mutex;

#[cfg(target_os = "macos")]
mod mac;

#[cfg(target_os = "macos")]
pub use mac::Aperture as Recorder;

#[cfg(target_os = "windows")]
mod win;

#[cfg(target_os = "windows")]
pub struct Recorder {}

pub fn new() -> Recorder {
    #[cfg(target_os = "macos")]
    return Recorder::new();

    #[cfg(target_os = "windows")]
    {
        return Recorder {};
    }
}

pub async fn start(recorder: &Arc<Mutex<Recorder>>) {
    let mut recorder = recorder.lock().await;

    #[cfg(target_os = "macos")]
    {
        let recorder_options = mac::Options {
            screen_id: 1,
            fps: 30,
            show_cursor: true,
            highlight_clicks: false,
            video_codec: None,
            audio_device_id: None,
            crop_area: None,
        };

        recorder
            .start_recording(recorder_options)
            .await
            .expect("Couldn't start recording");
    }

    #[cfg(target_os = "windows")]
    {
        // TODO: Implement Windows recorder
        win::record();
    }
}

pub async fn stop(recorder: &Arc<Mutex<Recorder>>) -> String {
    let mut recorder = recorder.lock().await;

    // TODO: Implement Windows recorder
    #[cfg(target_os = "windows")]
    return String::from("");

    #[cfg(target_os = "macos")]
    recorder.stop_recording().expect("Couldn't stop recording")
}
