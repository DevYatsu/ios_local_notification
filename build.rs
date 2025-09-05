const PACKAGE_NAME: &str = "LocalNotification";
const PACKAGE_PATH: &str = "./ios/";

fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();

    if target_os != "ios" {
        return;
    }

    use swift_rs::SwiftLinker;

    // swift-rs has a minimum of macOS 10.13
    // Ensure the same minimum supported macOS version is specified as in your `Package.swift` file.
    SwiftLinker::new("10.15")
        // Only if you are also targetting iOS
        // Ensure the same minimum supported iOS version is specified as in your `Package.swift` file
        .with_ios("13")
        .with_package(PACKAGE_NAME, PACKAGE_PATH)
        .link();

    // Other build steps
}
