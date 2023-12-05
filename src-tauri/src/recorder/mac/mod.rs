use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io;
use std::path::PathBuf;
use tauri::api::process::Command;
use tempfile::NamedTempFile;
use tokio::time::{sleep, Duration};

const APERTURE_SIDECAR: &str = "aperture";

#[derive(Serialize)]
pub struct CropArea {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

pub struct Options {
    pub fps: u32,
    pub screen_id: u32,
    pub show_cursor: bool,
    pub highlight_clicks: bool,
    pub video_codec: Option<String>,
    pub audio_device_id: Option<String>,
    pub crop_area: Option<CropArea>,
}

fn get_random_id() -> String {
    let random_number: u64 = rand::thread_rng().gen();
    let id = format!("{:x}", random_number);
    id.chars().take(13).collect()
}

fn supports_hevc_hardware_encoding() -> bool {
    let output = Command::new("sysctl")
        .args(&["-n", "machdep.cpu.brand_string"])
        .output()
        .expect("Failed to get CPU info");

    let cpu_model = &output.stdout;

    // All Apple silicon Macs support HEVC hardware encoding.
    if cpu_model.starts_with("Apple ") {
        return true;
    }

    let re = regex::Regex::new(r#"Intel.*Core.*i\d+-(\d)"#).unwrap();
    if let Some(captures) = re.captures(&cpu_model) {
        if let Ok(generation) = captures[1].parse::<u32>() {
            // Intel Core generation 6 or higher supports HEVC hardware encoding
            return generation >= 6;
        }
    }

    false
}

pub struct Aperture {
    process_id: String,
    recorder: Option<tauri::api::process::CommandChild>,
    temp_path: Option<PathBuf>,
    is_file_ready: bool,
}

impl Aperture {
    pub fn new() -> Self {
        Aperture {
            process_id: "".into(),
            recorder: None,
            temp_path: None,
            is_file_ready: false,
        }
    }

    pub async fn start_recording(
        &mut self,
        options: Options,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let process_id = get_random_id();
        self.process_id = process_id.clone();

        if self.recorder.is_some() {
            return Err("Call `stop_recording()` first".into());
        }

        let file_name = format!("Helmer-{}.mp4", &process_id);

        let path = NamedTempFile::new()?
            .into_temp_path()
            .with_file_name(&file_name);

        self.temp_path = Some(path);

        let file_url = url::Url::from_file_path(&self.temp_path.as_ref().unwrap())
            .unwrap()
            .to_string();

        let recorder_options = json!({
            "destination": file_url,
            "screenId": options.screen_id,
            "framesPerSecond": options.fps,
            "showCursor": options.show_cursor,
            "highlightClicks": options.highlight_clicks,
            "videoCodec": options.video_codec.unwrap_or("hvc1".into()),
            // "cropRect": [[crop_area.x, crop_area.y], [crop_area.width, crop_area.height]],
        });

        let timeout = sleep(Duration::from_secs(5));
        let start_event = self.wait_for_event("onStart");

        let child = Command::new_sidecar(APERTURE_SIDECAR)?
            .args(&[
                "record",
                "--process-id",
                &self.process_id,
                &recorder_options.to_string(),
            ])
            .spawn()?;

        tokio::select! {
            _ = timeout => {
                // child.kill()?;
                return Err("Could not start recording within 5 seconds".into());
            }
            _ = start_event => {
                // Wait for additional 1s after the promise resolves for the recording to actually start
                sleep(Duration::from_secs(1)).await;
                self.recorder = Some(child.1);
                self.wait_for_event("onFileReady").await.unwrap();
                self.is_file_ready = true;
                Ok(())
            }
        }
    }

    fn throw_if_not_started(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if self.recorder.is_none() {
            Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                "Call `.start_recording()` first",
            )))
        } else {
            Ok(())
        }
    }

    async fn wait_for_event(
        &self,
        name: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let command = Command::new_sidecar(APERTURE_SIDECAR)?
            .args(&[
                "events",
                "listen",
                "--exit",
                "--process-id",
                &self.process_id,
                &name,
            ])
            .output()
            .expect(format!("Failed to wait for event: {}", name).as_str());
        Ok(command.stdout)
    }

    pub fn stop_recording(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        self.throw_if_not_started().unwrap();
        if let Some(recorder) = self.recorder.take() {
            // This command simulates a SIGTERM which the std library doesn't support
            // Exiting this way ensures your video file doesn't get corrupted
            Command::new("kill")
                .args(&["-SIGTERM", &recorder.pid().to_string()])
                .output()?;
        }

        let temp_path = self
            .temp_path
            .take()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Temporary path not found"))?;

        Ok(temp_path.to_string_lossy().to_string())
    }
}
