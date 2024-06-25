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

    /**
     * 包含用于访问和发布设备上的数据的类。
     * */
    #[cfg(feature = "android_content")]
    #[cfg_attr(docsrs, doc(cfg(feature = "android_content")))]
    pub mod content;

    /**
     * 提供管理音频和视频中各种媒体接口的类。
     * */
    #[cfg(feature = "android_media")]
    #[cfg_attr(docsrs, doc(cfg(feature = "android_media")))]
    pub mod media;

    /**
     * 提供设备上的基本操作系统服务、消息传递和进程间通信。
     * */
    #[cfg(feature = "android_os")]
    #[cfg_attr(docsrs, doc(cfg(feature = "android_os")))]
    pub mod os;

    /**
     * 语音能力。
     * */
    pub mod speech;

    /**
     * 提供一些类，这些类公开处理屏幕布局和与用户交互的基本用户界面类。
     * */
    #[cfg(feature = "android_view")]
    #[cfg_attr(docsrs, doc(cfg(feature = "android_view")))]
    pub mod view;

    /**
     * 小部件包包含可在应用程序屏幕上使用的（大部分是视觉的）UI 元素。
     * */
    #[cfg(feature = "android_widget")]
    #[cfg_attr(docsrs, doc(cfg(feature = "android_widget")))]
    pub mod widget;
}

/**
 * Dalvik 虚拟机。
 * */
#[cfg(feature = "dalvik")]
#[cfg_attr(docsrs, doc(cfg(feature = "dalvik")))]
pub mod dalvik {
    /**
     * Dalvik 虚拟机系统。
     * */
    #[cfg(feature = "dalvik_system")]
    #[cfg_attr(docsrs, doc(cfg(feature = "dalvik_system")))]
    pub mod system;
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

    /**
     * 定义缓冲区，它是用于数据的容器，并概述了其他NIO软件包。
     * */
    #[cfg(feature = "java_nio")]
    #[cfg_attr(docsrs, doc(cfg(feature = "java_nio")))]
    pub mod nio;
}

droid_wrap_utils::import!();
