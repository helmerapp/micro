use scap::frame::{Frame, FrameType};
use std::sync::mpsc;

#[cfg(target_os = "macos")]
mod ffmpeg;

pub const FRAME_TYPE: FrameType = FrameType::BGRAFrame;

pub fn preview_encoder_thread_handler(
    rx: mpsc::Receiver<Frame>,
    width: usize,
    height: usize,
    file_path: String,
) {
    #[cfg(target_os = "windows")]
    {
        use windows_capture::encoder::{VideoEncoder, VideoEncoderQuality, VideoEncoderType};

        let mut encoder = VideoEncoder::new(
            VideoEncoderType::Mp4,
            VideoEncoderQuality::HD1080p,
            width as u32,
            height as u32,
            file_path,
        )
        .expect("Failed to create video encoder");

        let mut start_time: u64 = 0;

        // Process data until the channel is closed
        while let Ok(data) = rx.recv() {
            match data {
                Frame::BGRA(frame) => {
                    if start_time == 0 {
                        start_time = frame.display_time;
                    }

                    let timespan_nanos =
                        std::time::Duration::from_nanos(frame.display_time - start_time);

                    // TODO: why does the magic number 10 work here?
                    let timespan_micros = timespan_nanos.as_micros() as i64;
                    let timespan_micros_10 = timespan_micros * 10;

                    let buffer = flip_image_vertical_bgra(&frame.data, width, height);

                    encoder
                        .send_frame_buffer(&buffer, timespan_micros_10)
                        .expect("failed to send frame");
                }
                _ => {}
            }
        }
        println!("Recording stopped");
        let x = encoder.finish();
        match x {
            Ok(_) => {
                println!("Encoding complete");
            }
            Err(e) => println!("Error: {:?}", e),
        }
    }

    #[cfg(target_os = "macos")]
    {
        let file_output = ffmpeg::FileOutput {
            output_filename: file_path,
        };
        let mut encoder = ffmpeg::Encoder::new(ffmpeg::Options {
            output: ffmpeg::Output::FileOutput(file_output),
            input: ffmpeg::InputOptions {
                width: width,
                height: height,
                frame_type: FRAME_TYPE,
                base_timestamp: None,
            },
        });
        // Process data until the channel is closed
        while let Ok(data) = rx.recv() {
            // Process the received data
            let _ = encoder.ingest_next_video_frame(&data);
        }
        println!("Recording stopped");
        let x = encoder.done();
        match x {
            Ok(_) => {
                println!("Encoding complete");
            }
            Err(e) => println!("Error: {:?}", e),
        }
        drop(encoder);
    }
    println!("Processing thread terminated.");
}
