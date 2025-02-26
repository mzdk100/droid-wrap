/*
 * Copyright (c) 2024. The RigelA open source project team and
 * its contributors reserve all rights.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 * http://www.apache.org/licenses/LICENSE-2.0
 * Unless required by applicable law or agreed to in writing, software distributed under the
 * License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and limitations under the License.
 */

use android_build::{ANDROID_BUILD_TOOLS_VERSION, PathExt, android_sdk};
use std::{
    env::{args, var},
    path::PathBuf,
    process::{Command, exit},
};

/// Returns the path to the `aapt2` executable for the given build tools version.
///
/// If the `ANDROID_AAPT2` environment variable is set and points to a file that exists,
/// that path is returned.
/// If `build_tools_version` is `None`, the value of the `ANDROID_BUILD_TOOLS_VERSION` environment variable is used
/// to find the `aapt2` executable from the Android SDK root directory.
pub fn android_aapt2_path(build_tools_version: Option<&str>) -> Option<PathBuf> {
    const ANDROID_AAPT2: &str = "ANDROID_AAPT2";
    const AAPT2: &str = if cfg!(windows) { "aapt2.exe" } else { "aapt2" };
    // Check if ANDROID_AAPT2 environment variable is set and points to a valid file
    var(ANDROID_AAPT2)
        .ok()
        .and_then(PathExt::path_if_exists)
        .map(PathBuf::from)
        .or_else(|| {
            android_sdk().and_then(|sdk| {
                sdk.join("build-tools")
                    .join(
                        build_tools_version
                            .map(ToString::to_string)
                            .unwrap_or_else(|| {
                                var(ANDROID_BUILD_TOOLS_VERSION)
                        .expect("either ANDROID_AAPT2 or ANDROID_BUILD_TOOLS_VERSION must be set")
                            }),
                    )
                    .join(AAPT2)
                    .path_if_exists()
            })
        })
}

fn main() {
    let aapt2 = android_aapt2_path(None).unwrap();
    exit(
        Command::new(&aapt2)
            .args(args().skip(2))
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .code()
            .unwrap_or(1),
    );
}
