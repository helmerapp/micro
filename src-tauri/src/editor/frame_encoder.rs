use imgref::Img;
use rgb::RGBA;
use scap::frame::{self, BGRAFrame, BGRFrame, Frame, RGBFrame};
use std::sync::Arc;

const TIMEBASE: f64 = 1000000000.0; // TODO: Verify for windows. This value may be different

pub struct FrameEncoder {
    gif_encoder: Arc<gifski::Collector>,
    index: usize,
    base_ts: u64,
}

impl FrameEncoder {
    pub fn new(gif_encoder: Arc<gifski::Collector>, index: usize, base_ts: u64) -> Self {
        Self {
            gif_encoder,
            index,
            base_ts,
        }
    }

    fn transform_frame_bgr(&self, frame: &BGRFrame) -> Img<Vec<RGBA<u8>>> {
        let frame_data = &frame.data;
        let mut rgba_data: Vec<RGBA<u8>> = Vec::with_capacity(frame_data.len() / 4);

        for src in frame_data.chunks_exact(3) {
            rgba_data.push(RGBA::new(src[2], src[1], src[0], 255))
        }

        Img::new(rgba_data, frame.width as usize, frame.height as usize)
    }

    pub fn encode_bgr(&self, bgr_frame: &BGRFrame) {
        let img = self.transform_frame_bgr(bgr_frame);
        self.gif_encoder
            .add_frame_rgba(
                self.index,
                img,
                (bgr_frame.display_time - self.base_ts) as f64 / TIMEBASE,
            )
            .unwrap_or_else(|err| {
                eprintln!("Error adding frame to encoder: {:?}", err);
            });
    }

    fn transform_frame_bgra(&self, frame: &BGRAFrame) -> Img<Vec<RGBA<u8>>> {
        let frame_data = &frame.data;
        let mut rgba_data: Vec<RGBA<u8>> = Vec::with_capacity(frame_data.len() / 4);

        for src in frame_data.chunks_exact(4) {
            rgba_data.push(RGBA::new(src[2], src[1], src[0], 255))
        }

        Img::new(rgba_data, frame.width as usize, frame.height as usize)
    }

    pub fn encode_bgra(&self, bgra_frame: &BGRAFrame) {
        let img = self.transform_frame_bgra(bgra_frame);
        self.gif_encoder
            .add_frame_rgba(
                self.index,
                img,
                (bgra_frame.display_time - self.base_ts) as f64 / TIMEBASE,
            )
            .unwrap_or_else(|err| {
                eprintln!("Error adding frame to encoder: {:?}", err);
            });
    }

    fn transform_frame_rgb(&self, frame: &RGBFrame) -> Img<Vec<RGBA<u8>>> {
        let frame_data = &frame.data;
        let mut rgba_data: Vec<RGBA<u8>> = Vec::with_capacity(frame_data.len() / 4);

        for src in frame_data.chunks_exact(3) {
            rgba_data.push(RGBA::new(src[0], src[1], src[2], 255))
        }

        Img::new(rgba_data, frame.width as usize, frame.height as usize)
    }

    pub fn encode_rgb(&self, rgb_frame: &RGBFrame, speed: f32, start_ts: f64, end_ts: f64) {
        let img = self.transform_frame_rgb(rgb_frame);
        let frame_pts = (rgb_frame.display_time - self.base_ts) as f64 / TIMEBASE;

        if frame_pts < start_ts || frame_pts > end_ts {
            println!("Ignoring frame {} with t {}", self.index, frame_pts);
            return;
        }

        self.gif_encoder
            .add_frame_rgba(self.index, img, frame_pts / (speed as f64))
            .unwrap_or_else(|err| {
                eprintln!("Error adding frame to encoder: {:?}", err);
            });
    }
}
