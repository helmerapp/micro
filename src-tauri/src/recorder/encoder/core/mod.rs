use ac_ffmpeg::{codec::{video, CodecParameters, Encoder}, time::{self, Timestamp}, packet};
use anyhow::Error;
use rgb2yuv420::convert_rgb_to_yuv420sp_nv12;
use scap::frame::Frame;
mod utils;

use super::config;

pub fn convert_bgra_to_rgb(frame_data: &Vec<u8>) -> Vec<u8> {
    let width = frame_data.len();
    let width_without_alpha = (width / 4) * 3;

    let mut data: Vec<u8> = vec![0; width_without_alpha];

    for (src, dst) in frame_data.chunks_exact(4).zip(data.chunks_exact_mut(3)) {
        dst[0] = src[2];
        dst[1] = src[1];
        dst[2] = src[0];
    }

    return data;
}
pub struct Core {
    encoder: video::VideoEncoder,
    encoder_config: config::EncoderConfig,
    input_config: config::InputConfig,
}

impl Core {
    pub fn new(input_config: config::InputConfig, encoder_config: config::EncoderConfig) -> Self {
        let width = if input_config.width % 2 == 0 {
            input_config.width
        } else {
            input_config.width + 1
        };
        let height = if input_config.height % 2 == 0 {
            input_config.height
        } else {
            input_config.height + 1
        };

        let time_base = time::TimeBase::new(1, 1000000000); // Maybe move to config?

        let pixel_format = video::frame::get_pixel_format(&encoder_config.pixel_format);

        let mut encoder = video::VideoEncoder::builder(&encoder_config.encoder)
            .unwrap()
            .pixel_format(pixel_format)
            .width(width)
            .height(height)
            .time_base(time_base);

        for option in &encoder_config.options {
            encoder = encoder.set_option(option.0, option.1);
        }

        let encoder = encoder.build().unwrap();

        Self {
            encoder,
            encoder_config,
            input_config,
        }
    }

    pub fn encode_next_video_frame(&mut self, frame: &Frame) -> Result<(), Error>{
        let mut new_frame;
        match frame {
            Frame::YUVFrame(frame) => {

                new_frame = video::VideoFrameMut::black(
                    video::frame::get_pixel_format(self.encoder_config.pixel_format.as_str()),
                    frame.width as usize,
                    frame.height as usize,
                );

                let mut base_frame_timestamp;

                match self.input_config.base_timestamp {
                    Some(val) => {
                        base_frame_timestamp = val;
                    }
                    None => {
                        base_frame_timestamp = frame.display_time as i64;
                        self.input_config.base_timestamp = Some(base_frame_timestamp);
                    }
                }

                let timestamp = time::Timestamp::from_nanos(frame.display_time as i64 - base_frame_timestamp);

                // let frame_timestamp = time::Timestamp::new(frame.display_time.try_into().unwrap(), time_base);
                new_frame = new_frame.with_pts(timestamp).with_picture_type(
                    if self
                        .encoder_config
                        .force_idr
                        .swap(false, std::sync::atomic::Ordering::Relaxed)
                    {
                        video::frame::PictureType::I
                    } else {
                        video::frame::PictureType::None
                    },
                );

                let encoder_buffer_len = new_frame.planes_mut()[0].data_mut().len();
                let encoder_line_size = encoder_buffer_len / (frame.height as usize);
                let encoder_num_lines = (frame.width as usize);

                utils::copy_nv12(
                    &frame.luminance_bytes,
                    frame.luminance_stride as usize,
                    encoder_line_size,
                    encoder_num_lines,
                    new_frame.planes_mut()[0].data_mut(),
                );
                utils::copy_nv12(
                    &frame.chrominance_bytes,
                    frame.chrominance_stride as usize,
                    encoder_line_size,
                    encoder_num_lines,
                    new_frame.planes_mut()[1].data_mut(),
                );
            },
            Frame::BGRA(frame) => {
                new_frame = video::VideoFrameMut::black(
                    video::frame::get_pixel_format(self.encoder_config.pixel_format.as_str()),
                    frame.width as usize,
                    frame.height as usize,
                );

                let mut base_frame_timestamp;

                match self.input_config.base_timestamp {
                    Some(val) => {
                        base_frame_timestamp = val;
                    }
                    None => {
                        base_frame_timestamp = frame.display_time as i64;
                        self.input_config.base_timestamp = Some(base_frame_timestamp);
                    }
                }

                let timestamp = time::Timestamp::from_nanos(frame.display_time as i64 - base_frame_timestamp);
                new_frame = new_frame.with_pts(timestamp).with_picture_type(
                    if self
                        .encoder_config
                        .force_idr
                        .swap(false, std::sync::atomic::Ordering::Relaxed)
                    {
                        video::frame::PictureType::I
                    } else {
                        video::frame::PictureType::None
                    },
                );

                // TODO: Do these 2 operations together
                let rgb_data = convert_bgra_to_rgb(&frame.data);
                let yuv_data = convert_rgb_to_yuv420sp_nv12(&rgb_data, frame.width.try_into().unwrap(), frame.height.try_into().unwrap(), 3);

                let len = yuv_data.len();
                let encoder_buffer_len = new_frame.planes_mut()[0].data_mut().len();
                let encoder_line_size = encoder_buffer_len / (frame.height as usize);
                let encoder_num_lines = (frame.width as usize);
                utils::copy_nv12(
                    &yuv_data[0..(2*len/3)],
                    frame.width as usize,
                    encoder_line_size,
                    encoder_num_lines,
                    new_frame.planes_mut()[0].data_mut(),
                );

                utils::copy_nv12(
                    &yuv_data[(2*len/3)..len],
                    frame.width as usize,
                    encoder_line_size,
                    encoder_num_lines,
                    new_frame.planes_mut()[1].data_mut(),
                );
            }
            Frame::BGR0(frame) => {
                new_frame = video::VideoFrameMut::black(
                    video::frame::get_pixel_format(self.encoder_config.pixel_format.as_str()),
                    frame.width as usize,
                    frame.height as usize,
                );

                let mut base_frame_timestamp;

                match self.input_config.base_timestamp {
                    Some(val) => {
                        base_frame_timestamp = val;
                    }
                    None => {
                        base_frame_timestamp = frame.display_time as i64;
                        self.input_config.base_timestamp = Some(base_frame_timestamp);
                    }
                }

                // let frame_timestamp = time::Timestamp::new(frame.display_time.try_into().unwrap(), time_base);
                let timestamp = time::Timestamp::from_nanos(frame.display_time as i64 - base_frame_timestamp);
                new_frame = new_frame.with_pts(timestamp).with_picture_type(
                    if self
                        .encoder_config
                        .force_idr
                        .swap(false, std::sync::atomic::Ordering::Relaxed)
                    {
                        video::frame::PictureType::I
                    } else {
                        video::frame::PictureType::None
                    },
                );

                let yuv_data = convert_rgb_to_yuv420sp_nv12(&frame.data, frame.width.try_into().unwrap(), frame.height.try_into().unwrap(), 3);

                let len = yuv_data.len();
                let encoder_buffer_len = new_frame.planes_mut()[0].data_mut().len();
                let encoder_line_size = encoder_buffer_len / (frame.height as usize);
                let encoder_num_lines = (frame.width as usize);
                utils::copy_nv12(
                    &yuv_data[0..(2*len/3)],
                    frame.width as usize,
                    encoder_line_size,
                    encoder_num_lines,
                    new_frame.planes_mut()[0].data_mut(),
                );

                utils::copy_nv12(
                    &yuv_data[(2*len/3)..len],
                    frame.width as usize,
                    encoder_line_size,
                    encoder_num_lines,
                    new_frame.planes_mut()[1].data_mut(),
                );

                /*
                let mut data_ptr: usize = 0;
                let line_size_to_be_written = (frame.width * 3) as usize;
                for mut line in new_frame.planes_mut()[0].lines_mut() {
                    let _ = line.write(&frame.data[data_ptr..(data_ptr + line_size_to_be_written)]);
                    data_ptr += line_size_to_be_written;
                }
                */
            }
            Frame::RGB(frame) => {
                new_frame = video::VideoFrameMut::black(
                    video::frame::get_pixel_format(self.encoder_config.pixel_format.as_str()),
                    frame.width as usize,
                    frame.height as usize,
                );

                let mut base_frame_timestamp;

                match self.input_config.base_timestamp {
                    Some(val) => {
                        base_frame_timestamp = val;
                    }
                    None => {
                        base_frame_timestamp = frame.display_time as i64;
                        self.input_config.base_timestamp = Some(base_frame_timestamp);
                    }
                }

                let timestamp = time::Timestamp::from_nanos(frame.display_time as i64 - base_frame_timestamp);
                new_frame = new_frame.with_pts(timestamp).with_picture_type(
                    if self
                        .encoder_config
                        .force_idr
                        .swap(false, std::sync::atomic::Ordering::Relaxed)
                    {
                        video::frame::PictureType::I
                    } else {
                        video::frame::PictureType::None
                    },
                );

                let yuv_data = convert_rgb_to_yuv420sp_nv12(&frame.data, frame.width.try_into().unwrap(), frame.height.try_into().unwrap(), 3);

                let len = yuv_data.len();
                let encoder_buffer_len = new_frame.planes_mut()[0].data_mut().len();
                let encoder_line_size = encoder_buffer_len / (frame.height as usize);
                let encoder_num_lines = (frame.width as usize);
                utils::copy_nv12(
                    &yuv_data[0..(2*len/3)],
                    frame.width as usize,
                    encoder_line_size,
                    encoder_num_lines,
                    new_frame.planes_mut()[0].data_mut(),
                );

                utils::copy_nv12(
                    &yuv_data[(2*len/3)..len],
                    frame.width as usize,
                    encoder_line_size,
                    encoder_num_lines,
                    new_frame.planes_mut()[1].data_mut(),
                );

            }
            _ => panic!("Unknown frame format!")
        }

        let new_frame = new_frame.freeze();
        self.encoder.push(new_frame)?;
        Ok(())
    }

    pub fn get_next_encoded_frame(&mut self) -> Result<Option<packet::Packet>, Error> {
        let packet = self.encoder.take();
        return Ok(packet?);
    }

    pub fn codec_parameters(&self) -> CodecParameters {
        return self.encoder.codec_parameters().into();
    }

    pub fn flush(&mut self) -> Result<(), Error> {
        return Ok(self.encoder.flush()?);
    }

    pub fn take(&mut self) -> Result<Option<packet::Packet>, Error> {
        return Ok(self.encoder.take()?);
    }
}