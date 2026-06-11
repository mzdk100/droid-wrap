# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`droid-wrap` is a Rust crate providing high-level bindings for Android/Java APIs via JNI. Version 1.0 uses **auto-generated bindings** from Android SDK Java source files using the `java-lang` AST parser and jni 0.22's `bind_java_type!` macro.

## Build & Test Commands

```bash
# Build (host, all features)
cargo build --verbose --all-features

# Check compilation
cargo check --all-features

# Run on-device tests (requires Android emulator/device + adb)
cargo apk2 run -p droid-wrap-test --all-features

# Run examples on device
cargo apk2 run -p droid-wrap-example --example activity-example
cargo apk2 run -p droid-wrap-example --example java-example

# Generate docs
cargo doc --all-features
```

**Prerequisites:** `ANDROID_HOME` pointing to Android SDK with sources installed; `cargo install cargo-apk2`.

## Workspace Structure

| Crate | Path | Role |
|---|---|---|
| `droid-wrap` | root | Main library — includes auto-generated bindings |
| `droid-wrap-codegen` | `codegen/` | Build-time code generator (parses SDK `.java` files → `bind_java_type!` code) |
| `droid-wrap-macros` | `macros/` | Proc-macro crate (only `#[android_main]`) |
| `droid-wrap-utils` | `utils/` | JVM helpers (`with_env`, `android_vm`, proxy, error types) |
| `droid-wrap-test` | `tests/` | On-device test suite |
| `droid-wrap-example` | `example/` | Example Android apps |

## Architecture (1.0)

**Codegen pipeline:** SDK `.java` files → `java-lang` parser → AST → `mapper.rs` → `bind_java_type!` Rust code → included via `include!()` at build time.

**Utils** provides: `with_env(|env| { ... })` (closure-based JVM access), `android_vm()`, `android_context()`, `load_class()`, `new_proxy()`, `bind_proxy_handler()`.

**Main library** (`src/lib.rs`) includes generated modules and re-exports utilities.

**Feature flags** control which packages are generated: `android`, `java`, `dalvik`, plus granular flags like `android_view`, `java_lang`, etc.

**Java dynamic proxy** (`utils/src/proxy.rs`) enables Rust to implement Java interfaces for callbacks.

## Key Conventions

- Rust Edition 2024
- jni 0.22 API: `Env` (not `JNIEnv`), `Global<T>` (not `GlobalRef`), `JValue` (not `JValueGen`)
- JNI strings use `jni_str!()` and `jni_sig!()` macros for compile-time string creation
- Methods take `&mut Env` as first parameter (closure-based API via `with_env`)
- Codegen falls back to empty placeholders when `ANDROID_HOME` is not set
