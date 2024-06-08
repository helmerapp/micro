use crate::AppState;
use core_graphics_helmer_fork::display::CGDisplayBounds;
use rand::Rng;
use scap::{
    capturer::{Area, Capturer, Options, Point, Resolution, Size},
    frame::FrameType,
};
use tauri::{AppHandle, Manager};

pub const FRAME_TYPE: FrameType = FrameType::BGRAFrame;

pub async fn start_frame_capture(app_handle: AppHandle, target: &scap::Target) {
    let state: tauri::State<AppState> = app_handle.state::<AppState>();

    // area is of the form [x1, y1, x2, y2]
    // we need it of the form [x1, y1, x2-x1, y2-y1]
    let cropped_area = state.cropped_area.lock().await;
    let abs_area = cropped_area.clone();
    drop(cropped_area);
    let area = {
        if let scap::Target::Display(display) = target {
            let position = unsafe {
                let bounds = CGDisplayBounds(display.id);
                vec![bounds.origin.x, bounds.origin.y]
            };
            vec![
                abs_area[0] as f64 + position[0],
                abs_area[1] as f64 + position[1],
                abs_area[2] as f64 + position[0],
                abs_area[3] as f64 + position[1],
            ]
        } else {
            vec![
                abs_area[0] as f64,
                abs_area[1] as f64,
                abs_area[2] as f64,
                abs_area[3] as f64,
            ]
        }
    };

    let crop_area = vec![area[0], area[1], area[2] - area[0], area[3] - area[1]];
    let record_cursor = crate::tray::get_tray_setting(&app_handle, "record_cursor".into());
    // Initialize scap
    let options = Options {
        fps: 60,
        target: Some(target.clone()),
        show_cursor: record_cursor,
        show_highlight: false,
        excluded_targets: None,
        output_type: FRAME_TYPE,
        output_resolution: Resolution::_1080p, // TODO: doesn't respect aspect ratio yet
        crop_area: Some(Area {
            origin: Point {
                x: crop_area[0],
                y: crop_area[1],
            },
            size: Size {
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
