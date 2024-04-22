#[cfg(target_os = "macos")]
mod mac;

#[cfg(target_os = "windows")]
use windows_capture::encoder::{
    VideoEncoder as WVideoEncoder, VideoEncoderQuality, VideoEncoderType,
};

use anyhow::Error;
use scap::frame::Frame;

mod utils;
use utils::flip_image_vertical_bgra;

pub struct VideoEncoder {
    first_timestamp: u64,

    #[cfg(target_os = "macos")]
    encoder: *mut c_void,

    #[cfg(target_os = "windows")]
    encoder: Option<WVideoEncoder>,
}

#[derive(Debug)]
pub struct VideoEncoderOptions {
    pub width: usize,
    pub height: usize,
    pub path: String,
}

impl VideoEncoder {
    pub fn new(options: VideoEncoderOptions) -> Self {
        #[cfg(target_os = "windows")]
        let encoder = WVideoEncoder::new(
            VideoEncoderType::Mp4,
            VideoEncoderQuality::Uhd2160p,
            options.width as u32,
            options.height as u32,
            options.path,
        )
        .expect("Failed to create video encoder");

        Self {
            encoder: Some(encoder),
            first_timestamp: 0,
        }
    }

    pub fn ingest_next_frame(&mut self, frame: &Frame) -> Result<(), Error> {
        match frame {
            Frame::BGRA(frame) => {
                #[cfg(target_os = "windows")]
                {
                    let timespan_nanos =
                        std::time::Duration::from_nanos(frame.display_time - self.first_timestamp);

                    // TODO: why does the magic number 10 work here?
                    let timespan_micros = timespan_nanos.as_micros() as i64;
                    let timespan_micros_10 = timespan_micros * 10;

                    let buffer = flip_image_vertical_bgra(
                        &frame.data,
                        frame.width as usize,
                        frame.height as usize,
                    );

                    if self.encoder.is_some() {
                        self.encoder
                            .as_mut()
                            .unwrap()
                            .send_frame_buffer(&buffer, timespan_micros_10)
                            .expect("failed to send frame");
                    }
                }
            }
            Frame::YUVFrame(frame) => {
                #[cfg(target_os = "macos")]
                {
                    let timestamp = data.display_time - self.first_timestamp;

                    #[cfg(target_os = "macos")]
                    unsafe {
                        encoder_ingest_yuv_frame(
                            self.encoder,
                            data.width as Int,
                            data.height as Int,
                            timestamp as Int,
                            data.luminance_stride as Int,
                            data.luminance_bytes.as_slice().into(),
                            data.chrominance_stride as Int,
                            data.chrominance_bytes.as_slice().into(),
                        );
                    }
                }
            }
            _ => {
                println!("encoder doesn't currently support this pixel format")
            }
        }

        Ok(())
    }

    pub fn finish(&mut self) -> Result<(), Error> {
        #[cfg(target_os = "windows")]
        {
            self.encoder
                .take()
                .unwrap()
                .finish()
                .expect("Failed to finish encoding");
        }

        #[cfg(target_os = "macos")]
        unsafe {
            encoder_finish(self.encoder);
        }

        Ok(())
    }
}
