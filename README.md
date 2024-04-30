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


