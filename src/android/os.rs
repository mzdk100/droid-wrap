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

use crate::{JObjNew, JObjRef, JType};
use droid_wrap_derive::{java_class, java_constructor};

/**
 * 从字符串键到各种 Parcelable 值的映射。
 * 警告：请注意，Bundle 是一个惰性容器，因此它不实现 equals(Object) 或 hashCode()。
 * */
#[java_class(name = "android/os/Bundle")]
pub struct Bundle;

impl Bundle {
    /**
     * 构造一个新的空 Bundle。
     * */
    #[java_constructor]
    pub fn new() -> Self {}
}

#[cfg(feature = "test_android_content")]
pub fn test() {
    let bundle = Bundle::new();
    assert!(bundle.to_string().starts_with("Bundle"));
}
