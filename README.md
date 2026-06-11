# droid-wrap — High-level Rust bindings for Android APIs

[GitCode](https://gitcode.com/mzdk100/droid-wrap.git)
[GitHub](https://github.com/mzdk100/droid-wrap)
[crates.io](https://crates.io/crates/droid-wrap)
[docs.rs](https://docs.rs/droid-wrap/latest/droid_wrap/)

## Overview

droid-wrap provides high-level Rust bindings for Android and Java APIs via JNI. Version 1.0 introduces **auto-generated bindings** from Android SDK Java source files using the [java-lang](https://github.com/mzdk100/javalang) parser and jni 0.22's `bind_java_type!` macro.

Key features:
- **Auto-generated bindings** — no hand-written wrappers, generated from SDK `.java` sources at build time
- **jni 0.22 `bind_java_type!`** — generates type-safe Rust wrappers with cached method/field IDs
- **Closure-based API** — `with_env(|env| { ... })` for safe JVM interaction
- **Java dynamic proxy** — implement Java interfaces from Rust (callbacks)
- **Javadoc extraction** — doc comments from Java sources are preserved as Rust docs

## Prerequisites

- Android SDK with sources installed (`$ANDROID_HOME/sources/android-<level>/`)
- Rust toolchain with Android targets
- `cargo-apk2` for building APKs:
  ```shell
  cargo install cargo-apk2
  ```

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `ANDROID_HOME` | Android SDK root path | (required) |
| `ANDROID_API_LEVEL` | API level for source generation | `37` |

## Usage

```rust
use droid_wrap::{android_main, with_env, jni_str, jni_sig, Result};

#[android_main]
fn main() -> Result<()> {
    with_env(|env| {
        // Call System.currentTimeMillis()
        let time = env.call_static_method(
            jni_str!("java/lang/System"),
            jni_str!("currentTimeMillis"),
            jni_sig!("()J"),
            &[],
        )?.j()?;
        println!("Current time: {}", time);
        Ok(())
    })
}
```

When auto-generated bindings are available (with `ANDROID_HOME` set):

```rust
use droid_wrap::{android_main, with_env, java::lang::System, Result};

#[android_main]
fn main() -> Result<()> {
    with_env(|env| {
        let time = System::current_time_millis(env)?;
        println!("Current time: {}", time);
        Ok(())
    })
}
```

## Running Examples

```shell
git clone https://gitcode.net/mzdk100/droid-wrap.git
cd droid-wrap
cargo apk2 run -p droid-wrap-example --example java-example
cargo apk2 run -p droid-wrap-example --example activity-example
```

## Running Tests

```shell
cargo apk2 run -p droid-wrap-test --all-features
```

## Architecture

```
droid-wrap/
  build.rs          — invokes codegen at build time
  codegen/          — parses SDK .java files, generates bind_java_type! code
  utils/            — JVM helpers (with_env, android_vm, proxy)
  macros/           — #[android_main] entry point
  src/lib.rs        — includes generated bindings
```

## About cargo-apk2

[cargo-apk2](https://github.com/mzdk100/cargo-apk2) is a lightweight Android APK packaging tool forked from the deprecated [cargo-apk](https://github.com/rust-mobile/cargo-apk).
