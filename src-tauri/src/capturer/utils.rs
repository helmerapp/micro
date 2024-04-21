use crate::{AppState, Status};
use scap::capturer::{CGPoint, CGRect, CGSize, Capturer, Options, Resolution};
use tauri::{AppHandle, Manager};

use scap::frame::FrameType;
pub const FRAME_TYPE: FrameType = FrameType::BGRAFrame;

pub async fn start_recorder(app_handle: AppHandle) {
    let cropper_win = app_handle.get_webview_window("cropper").unwrap();
    cropper_win.set_ignore_cursor_events(true).unwrap();
    cropper_win.emit("capture-started", ()).unwrap();

    let state = app_handle.state::<AppState>();

    let mut status = state.status.lock().await;
    *status = Status::Recording;
    drop(status);

    // area is of the form [x1, y1, x2, y2]
    // we need it of the form [x1, y1, x2-x1, y2-y1]
    let area = state.cropped_area.lock().await.clone();
    let crop_area = vec![
        area[0] as f64,
        area[1] as f64,
        area[2] as f64 - area[0] as f64,
        area[3] as f64 - area[1] as f64,
    ];

    // Initialize scap
    let options = Options {
        fps: 60,
        targets: Vec::new(),
        show_cursor: true,
        show_highlight: true,
        excluded_targets: None,
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

    let mut recorder = state.recorder.lock().await;
    *recorder = Some(Capturer::new(options));
    (*recorder).as_mut().unwrap().start_capture();
    drop(recorder);
}