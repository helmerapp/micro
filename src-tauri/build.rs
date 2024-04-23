fn main() {
    // Ensure the same macos_min_version is specified in `Package.swift`
    #[cfg(target_os = "macos")]
    swift_rs::SwiftLinker::new("11")
        .with_package("enc-swift", "./src/recorder/encoder/mac/enc-swift")
        .link();

    tauri_build::build()
}
