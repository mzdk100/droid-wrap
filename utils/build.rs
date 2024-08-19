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

use cargo_emit::rerun_if_changed;
use noak::{writer::ClassWriter, AccessFlags};
use std::{
    collections::HashMap,
    env::{set_var, var},
    fs::OpenOptions,
    io::Write,
    path::PathBuf,
};

pub fn gen_class(
    class: &str,
    interfaces: &[&str],
    method_map: &HashMap<&str, (bool, String)>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut bytes = Vec::new();
    ClassWriter::new()
        .version(noak::Version::V8)?
        .access_flags(AccessFlags::PUBLIC | AccessFlags::SUPER)?
        .this_class(class)?
        .super_class("java/lang/Object")?
        .interfaces(|faces| {
            for interface in interfaces.iter() {
                faces.begin(|i| i.interface(*interface))?;
            }
            Ok(())
        })?
        .fields(|_| Ok(()))?
        .methods(|methods| {
            for (name, (is_static, sig)) in method_map.iter() {
                methods.begin(|m| {
                    let mut flags = AccessFlags::PUBLIC | AccessFlags::NATIVE;
                    if *is_static {
                        flags |= AccessFlags::STATIC;
                    }
                    m.access_flags(flags)?
                        .name(*name)?
                        .descriptor(sig.as_str())?
                        .attributes(|_| Ok(()))
                })?;
            }
            Ok(())
        })?
        .attributes(|_| Ok(()))?
        .write_bytes_to(&mut bytes)?;

    Ok(bytes)
}

//noinspection SpellCheckingInspection
fn main() {
    let target_os = var("CARGO_CFG_TARGET_OS").unwrap();

    if target_os != "android" {
        return;
    }

    rerun_if_changed!("build.rs");
    let out_dir = PathBuf::from(var("OUT_DIR").unwrap());
    const CLASS: &str = "rust/CallMethodHook";
    let java_class_path = out_dir.join("CallMethodHook.class");
    let mut java_class_file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(&java_class_path)
        .unwrap();

    let mut methods = HashMap::new();
    methods.insert(
        "invoke",
        (
            false,
            "(Ljava/lang/Object;Ljava/lang/reflect/Method;[Ljava/lang/Object;)Ljava/lang/Object;"
                .to_string(),
        ),
    );
    let class = gen_class(CLASS, &["java/lang/reflect/InvocationHandler"], &methods).unwrap();
    java_class_file.write_all(&class).unwrap();

    set_var("ANDROID_API_LEVEL", "35");
    set_var("ANDROID_BUILD_TOOLS_VERSION", "35.0.0");
    let android_jar_path = android_build::android_jar(None).expect("Failed to find android.jar");
    let d8_jar_path = android_build::android_d8_jar(None).expect("Failed to find d8.jar");

    let _ = android_build::JavaRun::new()
        .class_path(d8_jar_path)
        .main_class("com.android.tools.r8.D8")
        .arg("--classpath")
        .arg(android_jar_path)
        .arg("--output")
        .arg(&out_dir)
        .arg(&java_class_path)
        .run()
        .expect("failed to acquire exit status for java d8.jar invocation")
        .success();
}
