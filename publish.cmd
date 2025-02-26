@echo off
cargo doc --all-features &&^
cargo publish -p droid-wrap-macros --all-features &&^
cargo publish -p droid-wrap-utils --all-features &&^
cargo publish --all-features
pause