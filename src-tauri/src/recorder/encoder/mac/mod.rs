use swift_rs::swift;
pub use swift_rs::{Int, SRData, SRString};

swift!(pub fn encoder_init(
    width: Int,
    height: Int,
    out_file: SRString
) -> *mut std::ffi::c_void);

swift!(pub fn encoder_ingest_yuv_frame(
    enc: *mut std::ffi::c_void,
    width: Int,
    height: Int,
    display_time: Int,
    luminance_stride: Int,
    luminance_bytes: SRData,
    chrominance_stride: Int,
    chrominance_bytes: SRData
));

swift!(pub fn encoder_ingest_bgra_frame(
    enc: *mut std::ffi::c_void,
    width: Int,
    height: Int,
    display_time: Int,
    bytes_per_row: Int,
    bgra_bytes_raw: SRData
));

swift!(pub fn encoder_finish(enc: *mut std::ffi::c_void));
