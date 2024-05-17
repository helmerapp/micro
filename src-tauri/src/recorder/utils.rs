use crate::AppState;
use rand::Rng;
use scap::{
    capturer::{CGPoint, CGRect, CGSize, Capturer, Options, Resolution},
    frame::FrameType,
};
use tauri::{AppHandle, Manager};

pub const FRAME_TYPE: FrameType = FrameType::BGRAFrame;

pub async fn start_frame_capture(app_handle: AppHandle) {
    let state = app_handle.state::<AppState>();

    // area is of the form [x1, y1, x2, y2]
    // we need it of the form [x1, y1, x2-x1, y2-y1]
    let area = state.cropped_area.lock().await.clone();
    let crop_area = vec![
        area[0] as f64,
        area[1] as f64,
        area[2] as f64 - area[0] as f64,
        area[3] as f64 - area[1] as f64,
    ];

    let record_cursor = crate::tray::get_tray_setting(&app_handle, "record_cursor".into());

    // Initialize scap
    let options = Options {
        fps: 60,
        targets: Vec::new(),
        show_cursor: record_cursor,
        show_highlight: false,
        excluded_targets: None,
        excluded_windows: Some(vec![
            "recorder window".to_string(),
            "cropper window".to_string(),
        ]),
        output_type: FRAME_TYPE,
        output_resolution: Resolution::_1080p, // TODO: doesn't respect aspect ratio yet
        source_rect: Some(CGRect {
            origin: CGPoint {
                x: crop_area[0],
                y: crop_area[1],
            },
            size: CGSize {
                width: crop_area[2],
                height: crop_area[3],
            },
        }),
        ..Default::default()
    };

    let mut frame_capturer = state.recorder.lock().await;
    *frame_capturer = Some(Capturer::new(options));
    (*frame_capturer).as_mut().unwrap().start_capture();
    drop(frame_capturer);
}

pub fn get_random_id() -> String {
    let random_number: u64 = rand::thread_rng().gen();
    let id = format!("{:x}", random_number);
    id.chars().take(13).collect()
}
