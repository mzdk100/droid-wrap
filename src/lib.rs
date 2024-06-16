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

#![doc = include_str!("../README.md")]

/**
 * 包含平台中包含的应用程序使用的资源类，并定义应用程序 系统功能的权限。
 * */
#[cfg(feature = "android")]
#[cfg_attr(docsrs, doc(cfg(feature = "android")))]
pub mod android {
    /**
     * 包含封装整个 Android 应用程序模型的高级类。
     * */
    #[cfg(feature = "android_app")]
    #[cfg_attr(docsrs, doc(cfg(feature = "android_app")))]
    pub mod app;
}

/**
 * Java API。
 * */
#[cfg(feature = "java")]
#[cfg_attr(docsrs, doc(cfg(feature = "java")))]
pub mod java {
    /**
     * 提供对 Java 设计至关重要的类 程序设计语言。
     * */
    #[cfg(feature = "java_lang")]
    #[cfg_attr(docsrs, doc(cfg(feature = "java_lang")))]
    pub mod lang;
}

droid_wrap_utils::import!();
