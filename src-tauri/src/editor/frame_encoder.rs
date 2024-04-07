use imgref::Img;
use rgb::RGBA;
use scap::frame::{self, BGRAFrame, BGRFrame, Frame, RGBFrame};
use std::sync::Arc;

const TIMEBASE: f64 = 1000000000.0; // TODO: Verify for windows. This value may be different

fn transform_frame_bgr0(frame: &BGRFrame) -> Img<Vec<RGBA<u8>>> {
    let frame_data = &frame.data;
    let mut rgba_data: Vec<RGBA<u8>> = Vec::with_capacity(frame_data.len() / 4);

    for src in frame_data.chunks_exact(3) {
        rgba_data.push(RGBA::new(src[2], src[1], src[0], 255))
    }

    Img::new(rgba_data, frame.width as usize, frame.height as usize)
}

fn transform_frame_bgra(frame: &BGRAFrame) -> Img<Vec<RGBA<u8>>> {
    let frame_data = &frame.data;
    let mut rgba_data: Vec<RGBA<u8>> = Vec::with_capacity(frame_data.len() / 4);

    for src in frame_data.chunks_exact(4) {
        rgba_data.push(RGBA::new(src[2], src[1], src[0], 255))
    }

    Img::new(rgba_data, frame.width as usize, frame.height as usize)
}

fn transform_frame_rgb(frame: &RGBFrame) -> Img<Vec<RGBA<u8>>> {
    let frame_data = &frame.data;
    let mut rgba_data: Vec<RGBA<u8>> = Vec::with_capacity(frame_data.len() / 4);

    for src in frame_data.chunks_exact(3) {
        rgba_data.push(RGBA::new(src[0], src[1], src[2], 255))
    }

    Img::new(rgba_data, frame.width as usize, frame.height as usize)
}

pub fn bgr_frame_encoder(
    gif_encoder: Arc<gifski::Collector>,
    i: usize,
    bgr_frame: &BGRFrame,
    base_ts: u64,
) {
    let img = transform_frame_bgr0(bgr_frame);
    gif_encoder
        .add_frame_rgba(i, img, (bgr_frame.display_time - base_ts) as f64 / TIMEBASE)
        .unwrap_or_else(|err| {
            eprintln!("Error adding frame to encoder: {:?}", err);
        });
}

pub fn bgra_frame_encoder(
    gif_encoder: Arc<gifski::Collector>,
    i: usize,
    bgra_frame: &BGRAFrame,
    base_ts: u64,
) {
    let img = transform_frame_bgra(bgra_frame);
    gif_encoder
        .add_frame_rgba(
            i,
            img,
            (bgra_frame.display_time - base_ts) as f64 / TIMEBASE,
        )
        .unwrap_or_else(|err| {
            eprintln!("Error adding frame to encoder: {:?}", err);
        });
}

pub fn rgb_frame_encoder(
    gif_encoder: Arc<gifski::Collector>,
    i: usize,
    rgb_frame: &RGBFrame,
    base_ts: u64,
    speed: f32,
    frame_start_time: f64,
    frame_end_time: f64,
) {
    let img = transform_frame_rgb(rgb_frame);
    let frame_pts = (rgb_frame.display_time - base_ts) as f64 / TIMEBASE;

    if frame_pts < frame_start_time || frame_pts > frame_end_time {
        println!("Ignoring frame {} with t {}", i, frame_pts);
        return;
    }

    gif_encoder
        .add_frame_rgba(i, img, frame_pts / (speed as f64))
        .unwrap_or_else(|err| {
            eprintln!("Error adding frame to encoder: {:?}", err);
        });
}
