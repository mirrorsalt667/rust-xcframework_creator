## Description
create .xcarchive files and .xcframework file

## Environment
MacOS
Xcode
xcodebuild
Rust

#### To execute the application
$ crago run

#### The steps
1. scheme name: Key in your project scheme name. It will be your xcframework name, too.
![image](https://raw.githubusercontent.com/mirrorsalt667/rust-xcframework_creator/master/images/scheme.png)

2. project path: Drag the .xcodeproj file into your terminal. It'll show the path automatically.

3. archive path: Drag the specific folder you want into terminal.

4. platform: select the platforms you want to build.
    - 0 for iOS simulator
    - 1 for iOS
    - 2 for MacOS catalyst
    - 3 for both iOS simulator and iOS
    - 4 for both iOS and MacOS catalyst
    - 5 for both iOS simulator and MacOS
    - 6 for all platform

#### Command Demo
##### create archive file
$ xcodebuild archive \
-scheme <font color="#F7A004"><scheme_name></font> \
-configuration Release \
-destination 'generic/platform=<font color="#F7A004"><platform></font>' \
-archivePath '<font color="#F7A004"><archive_path></font>' \
SKIP_INSTALL=NO \
BUILD_LIBRARIES_FOR_DISTRIBUTION=NO

##### create xcframework file
$ xcodebuild -create-xcframework \
-framework '<font color="#F7A004"><archive_path></font>/Products/Library/Frameworks/<font color="#F7A004"><scheme_name></font>.framework' \
-output '<font color="#F7A004"><folder_path></font>/<font color="#F7A004"><scheme_name></font>.xcframework'
