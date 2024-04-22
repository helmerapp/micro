use std::{collections::HashMap, sync::{Arc, atomic::AtomicBool}};

#[derive(Debug, Clone)]
pub struct EncoderConfig {
    pub encoder: String,
    pub pixel_format: String,
    pub encoding: String,
    pub options: HashMap<String, String>,
    pub force_idr: Arc<AtomicBool>,
}

#[derive(Debug, Clone)]
pub struct InputConfig {
    pub height: usize,
    pub width: usize,
    pub base_timestamp: Option<i64>,
}

pub fn libx264() -> EncoderConfig {
    EncoderConfig {
        encoder: "libx264".to_string(),
        pixel_format: "nv12".to_string(),
        encoding: "video/H264".to_string(),
        options: HashMap::from([
            ("profile".into(), "baseline".into()),
            ("preset".into(), "ultrafast".into()),
            ("tune".into(), "zerolatency".into()),
        ]),
        force_idr: Arc::new(AtomicBool::new(false)),
    }
}

pub fn libx264rgb() -> EncoderConfig {
    EncoderConfig {
        encoder: "libx264".to_string(),
        pixel_format: "nv12".to_string(),
        encoding: "video/H264".to_string(),
        options: HashMap::from([
            ("preset".into(), "ultrafast".into()),
            ("tune".into(), "zerolatency".into()),
        ]),
        force_idr: Arc::new(AtomicBool::new(false)),
    }
}

pub fn libx264bgr() -> EncoderConfig {
    EncoderConfig {
        encoder: "libx264".to_string(),
        pixel_format: "nv12".to_string(),
        encoding: "video/H264".to_string(),
        options: HashMap::from([
            ("preset".into(), "ultrafast".into()),
            ("tune".into(), "zerolatency".into()),
        ]),
        force_idr: Arc::new(AtomicBool::new(false)),
    }
}