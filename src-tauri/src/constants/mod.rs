

#[cfg(target_os = "windows")]
pub const CAPTURER_OUTPUT_TYPE: scap::frame::FrameType = scap::frame::FrameType::RGBx;

#[cfg(target_os = "linux")]
pub const CAPTURER_OUTPUT_TYPE: scap::frame::FrameType = scap::frame::FrameType::RGB;

#[cfg(target_os = "macos")]
pub const CAPTURER_OUTPUT_TYPE: scap::frame::FrameType = scap::frame::FrameType::RGB;
