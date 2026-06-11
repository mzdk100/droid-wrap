use cargo_emit::rerun_if_changed;
use noak::{AccessFlags, writer::ClassWriter};
use std::{collections::HashMap, env::var, fs::OpenOptions, io::Write, path::PathBuf};

fn main() {
    let out_dir = var("OUT_DIR").unwrap();
    let generated = std::path::Path::new(&out_dir).join("generated");

    // Generate DEX bytecode for proxy support
    generate_dex(&out_dir);

    // Generate bindings from Android SDK sources
    let sdk_root = var("CARGO_APK2_SDK_HOME")
        .ok()
        .filter(|s| !s.is_empty())
        .or_else(|| android_build::android_sdk().map(|p| p.to_string_lossy().into_owned()))
        .unwrap_or_default();

    if sdk_root.is_empty() {
        println!("cargo:warning=Android SDK not found (set CARGO_APK2_SDK_HOME or ANDROID_HOME), generating empty bindings");
        droid_wrap_codegen::generate_empty_modules(&generated);
        return;
    }

    let api_level = var("ANDROID_API_LEVEL").unwrap_or_else(|_| "37".into());

    let enabled_features: std::collections::HashSet<String> = std::env::vars()
        .filter_map(|(key, _)| key.strip_prefix("CARGO_FEATURE_").map(|f| f.to_lowercase()))
        .collect();

    let cargo_toml_path = format!("{}/Cargo.toml", var("CARGO_MANIFEST_DIR").unwrap());
    let defined_features = parse_features_from_cargo_toml(&cargo_toml_path);

    droid_wrap_codegen::generate(
        &sdk_root,
        &api_level,
        &generated,
        &defined_features,
        &enabled_features,
    );

    println!("cargo:rerun-if-env-changed=CARGO_APK2_SDK_HOME");
    println!("cargo:rerun-if-env-changed=ANDROID_HOME");
    println!("cargo:rerun-if-env-changed=ANDROID_SDK_ROOT");
    println!("cargo:rerun-if-env-changed=ANDROID_API_LEVEL");
}

fn parse_features_from_cargo_toml(path: &str) -> std::collections::HashSet<String> {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return std::collections::HashSet::new(),
    };
    let mut features = std::collections::HashSet::new();
    let mut in_features = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed == "[features]" {
            in_features = true;
            continue;
        }
        if in_features {
            if trimmed.starts_with('[') {
                break;
            }
            if let Some((name, _)) = trimmed.split_once('=') {
                features.insert(name.trim().to_string());
            }
        }
    }

    features
}

fn gen_class(
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
                        flags = flags | AccessFlags::STATIC;
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

fn generate_dex(out_dir: &str) {
    let target_os = var("CARGO_CFG_TARGET_OS").unwrap();
    if target_os != "android" {
        return;
    }

    rerun_if_changed!("build.rs");
    let out_dir = PathBuf::from(out_dir);
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
