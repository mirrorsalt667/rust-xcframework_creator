use std::process::Command;
use std::io::{self, Write};

use crate::enum_page::IosPlatform;


pub fn xcodebuild_xcarchive(scheme_name: String, project_file_path: String, archive_file_path: String, platform: IosPlatform) {

    let platform_str = framework_platform_str(&platform);
    let platform_command = format!("generic/platform={}",platform_str);
    
    let _archive_output = Command::new("xcodebuild")
        .arg("archive")
        .arg("-project")
        .arg(project_file_path)
        .arg("-scheme")
        .arg(scheme_name)
        .arg("-configuration")
        .arg("Release")
        .arg("-destination")
        .arg(platform_command)
        .arg("-archivePath")
        .arg(archive_file_path)
        .arg("SKIP_INSTALL=NO")
        .arg("BUILD_LIBRARIES_FOR_DISTRIBUTION=NO")
        .output()
        .expect("Failed to execute command");

    // print!("{:?}", archive_output);
}

pub fn xcodebuild_create_xcframework(files_path: &[String], xcframework_path: &str) -> io::Result<()> {
    let mut command = Command::new("xcodebuild");
    command.arg("-create-xcframework");

    for file in files_path {
        command.arg("-framework").arg(file);
    }
    command.arg("-output").arg(xcframework_path);

    let output = command.output()?;

    if output.status.success() {
        println!("XCFramework created successfully");
    } else {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }

    if !output.status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "Failed to create XCFramework"));
    }

    // print!("{:?}", command);

    Ok(())
}

// ask user to enter info
pub fn get_user_input(sentence: &str) -> String {
    // show the sentence to the user
    print!("{}", sentence);

    io::stdout()
      .flush()
      .expect("Failed to flush stdout");

    let mut input = String::new();

    io::stdin()
     .read_line(&mut input)
     .expect("Failed to read line");

    input.trim().to_string()
}

pub fn framework_platform_str(platform: &IosPlatform) -> &str {
    match platform {
        IosPlatform::Simulator => {
            return "iOS Simulator";
        }
        IosPlatform::Ios => {
            return "iOS";
        }
        IosPlatform::MacOS => {
            return "macOS,arch=x86_64,variant=Mac Catalyst";
        }
    }
}