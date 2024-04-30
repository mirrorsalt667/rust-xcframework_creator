pub enum IosPlatform {
    Simulator,
    Ios,
    MacOS
}

pub fn generate_archive_platform_str(platform: &IosPlatform) -> &str {
    match platform {
        IosPlatform::Simulator => {
            return "iphonesimulator";
        }
        IosPlatform::Ios => {
            return "iphoneos";
        }
        IosPlatform::MacOS => {
            return "catalyst";
        }
    }
}