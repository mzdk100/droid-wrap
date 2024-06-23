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

use droid_wrap_derive::{java_class, java_method};
use droid_wrap_utils::{android_context, vm_attach};

use crate::{java::lang::CharSequence, JObjNew, JObjRef, JType};

#[java_class(name = "android/app/Activity")]
pub struct Activity;

impl Activity {
    /**
     * 当您的活动完成后并应被关闭时，请调用此方法。活动结果将通过 onActivityResult() 方法传回给启动者。
     * */
    #[java_method]
    pub fn finish(&self) {}

    /**
     * 更改与此活动关联的标题。如果这是顶级活动，其窗口的标题将会更改。如果这是嵌入活动，则父级可以对其执行任何操作。
     * `title` 标题。
     * */
    #[java_method]
    pub fn set_title<CS: CharSequence>(&self, title: &CS) {}

    /**
     * 获取与此活动关联的标题。
     * */
    #[java_method]
    pub fn get_title<CS: CharSequence>(&self) -> CS {}

    /**
     * 获取实例。
     * */
    pub fn fetch() -> Self {
        let ctx = android_context();
        let obj = vm_attach(|env| env.new_global_ref(&ctx).unwrap());
        Self::_new(&obj)
    }
}

#[cfg(feature = "test_android_app")]
pub fn test() {
    use crate::java::lang::{CharSequenceExt, CharSequenceImpl};
    let act = Activity::fetch();
    assert!(act.to_string().starts_with("android.app.NativeActivity"));
    let cs = "我的应用".to_char_sequence::<CharSequenceImpl>();
    act.set_title(&cs);
    assert_eq!(cs, act.get_title());
}
