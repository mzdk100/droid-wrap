@echo off
cargo doc --target aarch64-linux-android
cargo test --doc --target aarch64-linux-android
cargo publish -p droid-wrap-derive --all-features
cargo publish -p droid-wrap-utils --all-features --target aarch64-linux-android
cargo publish --all-features --target aarch64-linux-android
pause