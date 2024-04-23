#[cfg(target_os = "macos")]
mod mac;

#[cfg(target_os = "macos")]
use mac::{
    encoder_finish, encoder_ingest_bgra_frame, encoder_ingest_yuv_frame, encoder_init, Int, SRData,
    SRString,
};

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
    encoder: *mut std::ffi::c_void,

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
        let encoder = Some(
            WVideoEncoder::new(
                VideoEncoderType::Mp4,
                VideoEncoderQuality::Uhd2160p,
                options.width as u32,
                options.height as u32,
                options.path,
            )
            .expect("Failed to create video encoder"),
        );

        #[cfg(target_os = "macos")]
        let encoder = unsafe {
            encoder_init(
                options.width as Int,
                options.height as Int,
                options.path.as_str().into(),
            )
        };

        Self {
            encoder,
            first_timestamp: 0,
        }
    }

    pub fn ingest_next_frame(&mut self, frame: &Frame) -> Result<(), Error> {
        match frame {
            Frame::BGRA(frame) => {
                if self.first_timestamp == 0 {
                    self.first_timestamp = frame.display_time;
                }

                let timestamp = frame.display_time - self.first_timestamp;

                #[cfg(target_os = "windows")]
                {
                    let timestamp_nanos = std::time::Duration::from_nanos(timestamp);

                    // TODO: why does the magic number 10 work here?
                    let timestamp_micros = timestamp_nanos.as_micros() as i64;
                    let timestamp_micros_10 = timestamp_micros * 10;

                    let buffer = flip_image_vertical_bgra(
                        &frame.data,
                        frame.width as usize,
                        frame.height as usize,
                    );

                    if self.encoder.is_some() {
                        self.encoder
                            .as_mut()
                            .unwrap()
                            .send_frame_buffer(&buffer, timestamp_micros_10)
                            .expect("failed to send frame");
                    }
                }

                #[cfg(target_os = "macos")]
                unsafe {
                    encoder_ingest_bgra_frame(
                        self.encoder,
                        frame.width as Int,
                        frame.height as Int,
                        timestamp as Int,
                        frame.width as Int,
                        frame.data.as_slice().into(),
                    );
                }
            }
            Frame::YUVFrame(frame) => {
                #[cfg(target_os = "macos")]
                {
                    if self.first_timestamp == 0 {
                        self.first_timestamp = frame.display_time;
                    }

                    let timestamp = frame.display_time - self.first_timestamp;

                    #[cfg(target_os = "macos")]
                    unsafe {
                        encoder_ingest_yuv_frame(
                            self.encoder,
                            frame.width as Int,
                            frame.height as Int,
                            timestamp as Int,
                            frame.luminance_stride as Int,
                            frame.luminance_bytes.as_slice().into(),
                            frame.chrominance_stride as Int,
                            frame.chrominance_bytes.as_slice().into(),
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
