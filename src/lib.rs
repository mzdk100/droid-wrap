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

//! 封装Android系统的API。
//! [官方参考文档](https://developer.android.google.cn/reference/packages)

#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

/**
包含平台中包含的应用程序使用的资源类，并定义应用程序 系统功能的权限。
*/
#[cfg(feature = "android")]
pub mod android;

/**
Dalvik 虚拟机。
*/
#[cfg(feature = "dalvik")]
pub mod dalvik {
    /**
    Dalvik 虚拟机系统。
    */
    #[cfg(feature = "dalvik_system")]
    pub mod system;
}

/**
Java API。
*/
#[cfg(feature = "java")]
pub mod java {
    /**
    通过数据流、序列化和文件系统提供系统输入和输出。
    */
    #[cfg(feature = "java_io")]
    pub mod io;

    /**
    提供对 Java 设计至关重要的类 程序设计语言。
    */
    #[cfg(feature = "java_lang")]
    pub mod lang;

    /**
    定义缓冲区，它是用于数据的容器，并概述了其他NIO软件包。
    */
    #[cfg(feature = "java_nio")]
    pub mod nio;
}

pub use droid_wrap_macros::*;
droid_wrap_utils::import!();
