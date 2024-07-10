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

use droid_wrap_derive::{java_class, java_constructor};

use crate::{
    java::{lang::ClassLoader, nio::ByteBuffer},
    JObjNew, JObjRef, JType,
};

/**
 * 类加载器，用于从包含 classes.dex 条目的 .jar 和 .apk 文件中加载类。这可用于执行未作为应用程序的一部分安装的代码。
 * 在 API 级别 26 之前，此类加载器需要一个应用程序私有的可写目录来缓存优化的类。使用 Context.getCodeCacheDir() 创建这样的目录：
 * ```java
 * File dexOutputDir = context.getCodeCacheDir();
 * ```
 * 不要在外部存储上缓存优化的类。外部存储不提供保护应用程序免受代码注入攻击所需的访问控制。
 * */
#[java_class(name = "dalvik/system/DexClassLoader")]
pub struct DexClassLoader;

impl DexClassLoader {
    /**
     * 创建 DexClassLoader，用于查找解释代码和本机代码。解释类位于 Jar 或 APK 文件中包含的一组 DEX 文件中。路径列表使用 path.separator 系统属性指定的字符分隔，默认为 :。
     * `dex_path` 包含类和资源的 jar/apk 文件列表，由 File.pathSeparator 分隔，在 Android 上默认为“:”
     * `optimized_directory` 此参数已弃用，自 API 级别 26 起无效。
     * `library_search_path` 包含本机库的目录列表，由 File.pathSeparator 分隔；可能为 null
     * `parent` 父类加载器
     * */
    #[java_constructor]
    pub fn new(
        dex_path: String,
        optimized_directory: String,
        library_search_path: String,
        parent: &ClassLoader,
    ) -> Self {
    }
}

/**
 * 从包含 DEX 文件的缓冲区加载类的 ClassLoader 实现。这可用于执行尚未写入本地文件系统的代码。
 * */
#[java_class(name = "dalvik/system/InMemoryDexClassLoader")]
pub struct InMemoryDexClassLoader;

impl InMemoryDexClassLoader {
    /**
     * 创建一个新的内存中 DEX 类加载器。
     * `dex_buffer` 包含 buffer.position() 和 buffer.limit() 之间的 DEX 文件内容的缓冲区。
     * `parent` 用于委托的父类加载器。
     * */
    #[java_constructor]
    pub fn new(dex_buffer: &ByteBuffer, parent: &ClassLoader) -> Self {}
}

/// 测试dalvik.system
#[cfg(feature = "test_dalvik_system")]
pub fn test() {
    use crate::android::{app::Activity, content::Context};
    let context: Context = (&Activity::fetch()).into();
    let loader = DexClassLoader::new(
        "c.dex".to_string(),
        "".to_string(),
        "".to_string(),
        &context.get_class_loader(),
    );
    assert!(loader
        .to_string()
        .starts_with("dalvik.system.DexClassLoader"));
}
