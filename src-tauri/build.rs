fn main() {
    let ffmpeg_link_mode = "static";

    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-search=native=/opt/homebrew/Cellar/zlib/1.3.1/lib/");
        println!("cargo:rustc-link-search=native=/opt/homebrew/opt/x264/lib/");
        println!("cargo:rustc-link-search=native=/opt/homebrew/opt/bzip2/lib/");
        println!("cargo:rustc-link-lib=framework=CoreServices");
        println!("cargo:rustc-link-lib=framework=CoreGraphics");
        println!("cargo:rustc-link-lib=framework=CoreText");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=AudioUnit");
        println!("cargo:rustc-link-lib=framework=AudioToolbox");
        println!("cargo:rustc-link-lib=framework=CoreAudio");
        println!("cargo:rustc-link-lib=framework=Security");
        println!("cargo:rustc-link-lib=framework=VideoToolbox");

        link("z", ffmpeg_link_mode);
        link("x264", ffmpeg_link_mode);
        link("bz2", ffmpeg_link_mode);
    }

    tauri_build::build()
}

// Link a given library.
fn link(lib: &str, mode: &str) {
    println!("cargo:rustc-link-lib={}={}", mode, lib);
}
