//! High-level Rust bindings for Android APIs.
//!
//! Bindings are auto-generated from Android SDK Java source files at build time
//! using the `droid-wrap-codegen` crate and jni 0.22's `bind_java_type!` macro.
//!
//! [Official Android API reference](https://developer.android.google.cn/reference/packages)

#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

/// Core utilities: JVM access, error types, proxy support, and Java type wrappers.
pub mod core;

/// Auto-generated Android SDK bindings.
#[cfg(feature = "android")]
#[allow(unused_imports, non_camel_case_types)]
pub mod android {
    pub use jni::bind_java_type;
    include!(concat!(env!("OUT_DIR"), "/generated/android/mod.rs"));
}

/// Auto-generated Java standard library bindings.
#[cfg(feature = "java")]
#[allow(unused_imports, non_camel_case_types)]
pub mod java {
    pub use jni::bind_java_type;
    include!(concat!(env!("OUT_DIR"), "/generated/java/mod.rs"));
}

#[cfg(feature = "javax")]
#[allow(unused_imports, non_camel_case_types)]
pub mod javax {
    pub use jni::bind_java_type;
    include!(concat!(env!("OUT_DIR"), "/generated/javax/mod.rs"));
}

#[cfg(feature = "dalvik")]
#[allow(unused_imports, non_camel_case_types)]
pub mod dalvik {
    pub use jni::bind_java_type;
    include!(concat!(env!("OUT_DIR"), "/generated/dalvik/mod.rs"));
}

#[cfg(feature = "org")]
#[allow(unused_imports, non_camel_case_types)]
pub mod org {
    pub use jni::bind_java_type;
    include!(concat!(env!("OUT_DIR"), "/generated/org/mod.rs"));
}

pub use droid_wrap_macros::android_main;
pub use jni::{jni_str, jni_sig};
pub use core::{
    android_context, android_vm, java_object_equals, java_object_to_string,
    load_class, with_env, DroidWrapError, Result,
};
