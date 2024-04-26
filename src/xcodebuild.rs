use std::process::Command;
use std::io::{self, Write};

fn xcodebuild_xcarchive(project_name: String, project_path: String, archive_path: String) {

    let output = Command::new("xcodebuild")
        .arg("archive")
        .arg("-project")
        .arg("/Users/stephen003/Documents/work project/ios_bt_sdk/IOS_SWIFT_BLE_SDK/IOS_SWIFT_BLE_SDK.xcodeproj")
        .arg("-scheme")
        .arg("IOS_SWIFT_BLE_SDK")
        .arg("-configuration")
        .arg("Release")
        .arg("-destination")
        .arg("generic/platform=iOS")
        .arg("-archivePath")
        .arg("/Users/stephen003/Documents/work project/buildSDK/test_auto/IOS_SWIFT_BLE_SDK.xcarchive")
        .arg("SKIP_INSTALL=NO")
        .arg("BUILD_LIBRARIES_FOR_DISTRIBUTION=NO")
        .output()
        .expect("Failed to execute command");

    print!("{:?}", output);
}


fn get_user_input(path: &str) -> String {
    print!("{}", path);

    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();

    io::stdin()
     .read_line(&mut input)
     .expect("Failed to read line");

    input.trim().to_string()
}

fn main() {
    let project_name = get_user_input("What's the project name?");
    println!("name: {}", project_name);
    let project_path = get_user_input("Input iOS project path: ");
    let new_project_path = project_path.replace("\\", "");
    println!("path1: {}", new_project_path);
    let archive_path = get_user_input("Input archive project path: ");
    let new_archive_path = archive_path.replace("\\", "");
    println!("path2: {}!", new_archive_path);

    xcodebuild_xcarchive(project_name, new_project_path, new_archive_path)
}