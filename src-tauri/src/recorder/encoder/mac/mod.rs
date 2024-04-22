pub use ac_ffmpeg::time::{TimeBase, Timestamp};
use anyhow::Error;
use scap::frame::{Frame, FrameType};

use self::output::VideoFileHandle;

mod config;
mod core;
mod output;

#[derive(Debug)]
pub struct FileOutput {
    pub output_filename: String,
}

struct FileOutputInternal {
    pub output_filename: String,
    pub handle: VideoFileHandle,
}

#[derive(Debug)]
pub enum Output {
    FileOutput(FileOutput),
}

enum OutputInternal {
    FileOutput(FileOutputInternal),
}

#[derive(Debug)]
pub struct InputOptions {
    pub height: usize,
    pub width: usize,
    pub frame_type: FrameType,
    pub base_timestamp: Option<i64>,
}

struct InputOptionsInternal {
    pub height: usize,
    pub width: usize,
}

#[derive(Debug)]
pub struct Options {
    pub output: Output,
    pub input: InputOptions,
}

struct OptionsInternal {
    pub output: OutputInternal,
    pub input: InputOptionsInternal,
}

pub struct Encoder {
    core: core::Core,
    options: OptionsInternal,
}

impl Encoder {
    pub fn new(options: Options) -> Self {
        let config = match options.input.frame_type {
            FrameType::YUVFrame => config::libx264(),
            FrameType::BGR0 => config::libx264bgr(),
            FrameType::RGB => config::libx264rgb(),
            FrameType::BGRAFrame => config::libx264bgr(),
        };
        let core = core::Core::new(
            config::InputConfig {
                width: options.input.width as usize,
                height: options.input.height as usize,
                base_timestamp: options.input.base_timestamp,
            },
            config,
        );
        let codec_parameters = core.codec_parameters();

        let mut output = options.output;
        let mut output_internal;
        match &output {
            Output::FileOutput(file_output) => {
                let file_handle =
                    VideoFileHandle::new(&file_output.output_filename, &[codec_parameters]);
                output_internal = OutputInternal::FileOutput(FileOutputInternal {
                    output_filename: file_output.output_filename.clone(),
                    handle: file_handle,
                })
            }
        }

        return Encoder {
            core,
            options: OptionsInternal {
                output: output_internal,
                input: InputOptionsInternal {
                    height: options.input.height,
                    width: options.input.width,
                },
            },
        };
    }

    pub fn ingest_next_video_frame(&mut self, frame: &Frame) -> Result<(), Error> {
        self.core.encode_next_video_frame(frame)?;

        while let Some(encoded_packet) = self.core.get_next_encoded_frame()? {
            match &self.options.output {
                OutputInternal::FileOutput(file_output) => {
                    file_output.handle.push(encoded_packet.with_stream_index(0));
                }
            }
        }

        Ok(())
    }

    pub fn done(&mut self) -> Result<(), Error> {
        self.core.flush();
        while let Some(encoded_packet) = self.core.get_next_encoded_frame().unwrap() {
            match &self.options.output {
                OutputInternal::FileOutput(file_output) => {
                    file_output.handle.push(encoded_packet.with_stream_index(0));
                }
            }
        }

        match &self.options.output {
            OutputInternal::FileOutput(file_output) => file_output.handle.flush(),
        }
        return Ok(());
    }
}
