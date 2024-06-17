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

use droid_wrap_derive::java_class;

use crate::{JObjRef, JType};

/**
 * 与应用程序环境相关的全局信息的接口。这是一个抽象类，其实现由 Android 系统提供。它允许访问特定于应用程序的资源和类，以及对应用程序级操作（如启动活动、广播和接收意图等）的向上调用。
 * */
#[java_class(name = "android/content/Context")]
pub struct Context;

#[cfg(feature = "android_app")]
impl From<&crate::android::app::Activity> for Context {
    fn from(value: &crate::android::app::Activity) -> Self {
        Self {
            _obj: value.java_ref(),
        }
    }
}

#[cfg(feature = "test_android_content")]
pub fn test() {
    let act = crate::android::app::Activity::fetch();
    assert!(act.to_string().starts_with("android.app.NativeActivity"));
    let ctx: Context = (&act).into();
    assert!(ctx.to_string().starts_with("android.app.NativeActivity"));
}
