mod enum_page;
mod xcodebuild;
use std::process;

use crate::enum_page::IosPlatform;


fn main() {
    // ask for scheme name
    let scheme_name = xcodebuild::get_user_input("What's the scheme name?");
    println!("Scheme name: {}", scheme_name);

    // ask project path
    let new_project_path = check_project_path();

    // ask archive file path
    let archive_path = xcodebuild::get_user_input("Archive project folder path: ").replace("\\", "").replace("'", "");
    println!("Archive path: {}!", archive_path);

    // ask for platform xcframework should contain
    let platform_number = xcodebuild::get_user_input("Platform: 0->simulator, 1->iOS, 2->MacOS, 3->0+1, 4->1+2, 5->0+2, 6->ALL.");
    if platform_number == "0".to_string() {
        let archive_file_path = create_archive_path(scheme_name.clone(), archive_path.clone(), &enum_page::IosPlatform::Simulator);
        xcodebuild::xcodebuild_xcarchive(scheme_name.clone(), new_project_path, archive_file_path.clone(), IosPlatform::Simulator);
        let paths: &[String] = &[find_framework_path_in_xcarchive(scheme_name.clone(), archive_file_path)];
        let xcframework_path = create_xcframework_path(scheme_name, archive_path);
        let xcframework_result = xcodebuild::xcodebuild_create_xcframework(paths, &xcframework_path);
        println!("{:?}", xcframework_result);
    } else if platform_number == "1".to_string() {
        let archive_file_path = create_archive_path(scheme_name.clone(), archive_path.clone(), &enum_page::IosPlatform::Ios);
        xcodebuild::xcodebuild_xcarchive(scheme_name.clone(), new_project_path, archive_file_path.clone(), IosPlatform::Ios);
        let paths: &[String] = &[find_framework_path_in_xcarchive(scheme_name.clone(), archive_file_path)];
        let xcframework_path = create_xcframework_path(scheme_name, archive_path);
        let xcframework_result = xcodebuild::xcodebuild_create_xcframework(paths, &xcframework_path);
        println!("{:?}", xcframework_result);
    } else if platform_number == "2".to_string() {
        let archive_file_path = create_archive_path(scheme_name.clone(), archive_path.clone(), &enum_page::IosPlatform::MacOS);
        xcodebuild::xcodebuild_xcarchive(scheme_name.clone(), new_project_path, archive_file_path.clone(), IosPlatform::MacOS);
        let paths: &[String] = &[find_framework_path_in_xcarchive(scheme_name.clone(), archive_file_path)];
        let xcframework_path = create_xcframework_path(scheme_name, archive_path);
        let xcframework_result = xcodebuild::xcodebuild_create_xcframework(paths, &xcframework_path);
        println!("{:?}", xcframework_result);
    } else if platform_number == "3".to_string() {
        let archive_file_path_sim = create_archive_path(scheme_name.clone(), archive_path.clone(), &enum_page::IosPlatform::Simulator);
        xcodebuild::xcodebuild_xcarchive(scheme_name.clone(), new_project_path.clone(), archive_file_path_sim.clone(), IosPlatform::Simulator);
        let archive_file_path_ios = create_archive_path(scheme_name.clone(),archive_path.clone(), &enum_page::IosPlatform::Ios);
        xcodebuild::xcodebuild_xcarchive(scheme_name.clone(), new_project_path, archive_file_path_ios.clone(), IosPlatform::Ios);
        let paths: &[String] = &[find_framework_path_in_xcarchive(scheme_name.clone(), archive_file_path_sim), (find_framework_path_in_xcarchive(scheme_name.clone(), archive_file_path_ios))];
        let xcframework_path = create_xcframework_path(scheme_name, archive_path);
        let xcframework_result = xcodebuild::xcodebuild_create_xcframework(paths, &xcframework_path);
        println!("{:?}", xcframework_result);
    } else if platform_number == "4".to_string() {
        let archive_file_path_ios = create_archive_path(scheme_name.clone(),archive_path.clone(), &enum_page::IosPlatform::Ios);
        xcodebuild::xcodebuild_xcarchive(scheme_name.clone(), new_project_path.clone(), archive_file_path_ios.clone(), IosPlatform::Ios);
        let archive_file_path_mac = create_archive_path(scheme_name.clone(), archive_path.clone(), &enum_page::IosPlatform::MacOS);
        xcodebuild::xcodebuild_xcarchive(scheme_name.clone(), new_project_path, archive_file_path_mac.clone(), IosPlatform::MacOS);
        let paths: &[String] = &[find_framework_path_in_xcarchive(scheme_name.clone(), archive_file_path_ios), (find_framework_path_in_xcarchive(scheme_name.clone(), archive_file_path_mac))];
        let xcframework_path = create_xcframework_path(scheme_name, archive_path);
        let xcframework_result = xcodebuild::xcodebuild_create_xcframework(paths, &xcframework_path);
        println!("{:?}", xcframework_result);
    } else if platform_number == "5".to_string() {
        let archive_file_path_sim = create_archive_path(scheme_name.clone(), archive_path.clone(), &enum_page::IosPlatform::Simulator);
        xcodebuild::xcodebuild_xcarchive(scheme_name.clone(), new_project_path.clone(), archive_file_path_sim.clone(), IosPlatform::Simulator);
        let archive_file_path_mac = create_archive_path(scheme_name.clone(), archive_path.clone(), &enum_page::IosPlatform::MacOS);
        xcodebuild::xcodebuild_xcarchive(scheme_name.clone(), new_project_path, archive_file_path_mac.clone(), IosPlatform::MacOS);
        let paths: &[String] = &[find_framework_path_in_xcarchive(scheme_name.clone(), archive_file_path_sim), (find_framework_path_in_xcarchive(scheme_name.clone(), archive_file_path_mac))];
        let xcframework_path = create_xcframework_path(scheme_name, archive_path);
        let xcframework_result = xcodebuild::xcodebuild_create_xcframework(paths, &xcframework_path);
        println!("{:?}", xcframework_result);
    } else if platform_number == "6".to_string() {
        let archive_file_path_sim = create_archive_path(scheme_name.clone(), archive_path.clone(), &enum_page::IosPlatform::Simulator);
        xcodebuild::xcodebuild_xcarchive(scheme_name.clone(), new_project_path.clone(), archive_file_path_sim.clone(), IosPlatform::Simulator);
        let archive_file_path_ios = create_archive_path(scheme_name.clone(),archive_path.clone(), &enum_page::IosPlatform::Ios);
        xcodebuild::xcodebuild_xcarchive(scheme_name.clone(), new_project_path.clone(), archive_file_path_ios.clone(), IosPlatform::Ios);
        let archive_file_path_mac = create_archive_path(scheme_name.clone(), archive_path.clone(), &enum_page::IosPlatform::MacOS);
        xcodebuild::xcodebuild_xcarchive(scheme_name.clone(), new_project_path, archive_file_path_mac.clone(), IosPlatform::MacOS);
        let paths: &[String] = &[find_framework_path_in_xcarchive(scheme_name.clone(), archive_file_path_sim), (find_framework_path_in_xcarchive(scheme_name.clone(), archive_file_path_ios)), (find_framework_path_in_xcarchive(scheme_name.clone(), archive_file_path_mac))];
        let xcframework_path = create_xcframework_path(scheme_name, archive_path);
        let xcframework_result = xcodebuild::xcodebuild_create_xcframework(paths, &xcframework_path);
        println!("{:?}", xcframework_result);
    } else {
        println!("Platform Error. Termination");
        process::exit(1);
    }
}

/// create the path which the user want to put archive files in.
/// 
/// # Arguments
/// 
/// * 'scheme_name' - The project scheme name.
/// * 'folder_path' - The folder want to put in.
/// * 'platform' - Generate platform for the xcarchive file
/// 
/// # Returns
/// 
/// Return the xcrchive file path
fn create_archive_path(scheme_name: String, folder_path: String, platform: &IosPlatform) -> String {
    let platfrom_string = enum_page::generate_archive_platform_str(&platform);
    let scheme_str: &str = &scheme_name;
    let slash: &str = "/";
    let dash: &str = "-";
    let dot: &str = ".";
    let framework_str: &str = "framework";
    let xcarchive_str: &str = "xcarchive";
    let result = folder_path + slash + scheme_str + dot + framework_str + dash + platfrom_string + dot + xcarchive_str;
    println!("Archive path: {}!", result);
    return result;
}

/// Check the user input project path's suffix is "xcodeproj"
/// 
/// # Returns
/// 
/// Return the path of project and remove the "\" and "'".
fn check_project_path() -> String {
    let project_path = xcodebuild::get_user_input("iOS project path(xcodeproj): ").replace("'", "");
    // check the suffix is .xcodeproj
    if project_path.to_lowercase().ends_with("xcodeproj") {
        let new_project_path = project_path.replace("\\", "");
        println!("Project path: {}", new_project_path);
        return new_project_path;
    } else {
        let project_path_sec = xcodebuild::get_user_input("Path error, please put the xcodeproj file in.");
        if project_path_sec.to_lowercase().ends_with("xcodeproj") {
            let new_project_path = project_path_sec.replace("\\", "");
            println!("Project path: {}", new_project_path);
            return new_project_path;
        } else {
            println!("Path error. Termination");
            process::exit(1);
        }
    }
}

/// Before create xcframework, find the framework file in the archive file.
/// 
/// # Arguments
/// 
/// * 'scheme_name' - The project scheme name.
/// * 'archive_file_path' - The archive file path.
/// 
/// # Returns
/// 
/// return the xcframework path that the file will be
fn find_framework_path_in_xcarchive(scheme_name: String, archive_file_path: String) -> String {
    let scheme_str: &str = &scheme_name;
    let dot: &str = ".";
    let products: &str = "/Products/Library/Frameworks/";
    let framework_str: &str = "framework";
    let result = archive_file_path + products + scheme_str + dot + framework_str;
    println!("framework path: {}!", result);
    return result;
}

/// Before create xcframework, create the xcframework path that the command line tool "xcodebuild" needs
/// 
/// # Arguments
/// 
/// * 'scheme_name' - The project scheme name.
/// * 'folder_path' - The folder path where archive files are.
/// 
/// # Returns
/// 
/// return the xcframework path that the file will be
fn create_xcframework_path(scheme_name: String, folder_path: String) -> String {
    let scheme_str: &str = &scheme_name;
    let slash: &str = "/";
    let dot: &str = ".";
    let xcframework_str: &str = "xcframework";
    let result = folder_path + slash + scheme_str + dot + xcframework_str;
    println!("xcframework path: {}!", result);
    return result;
}