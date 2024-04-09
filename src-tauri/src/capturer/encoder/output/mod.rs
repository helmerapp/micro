use std::{fs::File, fmt, sync::{Arc, Mutex}};

use ac_ffmpeg::{
    codec::CodecParameters,
    format::{
        io::IO,
        muxer::{Muxer, OutputFormat},
    },
    Error, packet,
};

pub struct VideoFileHandle {
    muxer: Arc<Mutex<Muxer<File>>>
}

impl VideoFileHandle {
    pub fn new(path: &str, codec_parameters: &[CodecParameters]) -> Self {
        let output_format = OutputFormat::guess_from_file_name(path)
        .ok_or_else(|| Error::new(format!("unable to guess output format for file: {}", path))).unwrap();

        let output = File::create(path)
            .map_err(|err| Error::new(format!("unable to create output file {}: {}", path, err))).unwrap();

        let io = IO::from_seekable_write_stream(output);

        let mut muxer_builder = Muxer::builder();

        for codec_parameter in codec_parameters {
            muxer_builder.add_stream(codec_parameter);
        }

        let muxer = muxer_builder.build(io, output_format).unwrap();

        return VideoFileHandle {
            muxer: Arc::new(Mutex::new(muxer))
        };
    }

    pub fn push(&self, packet: packet::Packet) {
        let _ = self.muxer.lock().unwrap().push(packet);
    }
    
    pub fn flush(&self) {
        let _ = self.muxer.lock().unwrap().flush();
    }
}

impl fmt::Debug for VideoFileHandle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<handle>")
    }
}