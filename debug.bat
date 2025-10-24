adb shell input keyevent KEYCODE_BACK
adb logcat -c
cargo apk2 run -p droid-wrap-test --all-features
pause