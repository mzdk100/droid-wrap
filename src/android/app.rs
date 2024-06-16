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

use droid_wrap_derive::{java_method, java_class};
use droid_wrap_utils::{android_context, vm_attach};

use crate::{java::lang::CharSequence, JObjRef, JType};

#[java_class(name = "android/app/Activity")]
pub struct Activity;

impl Activity {
    #[java_method]
    pub fn finish(&self) {}

    #[java_method]
    pub fn set_title(&self, title: CharSequence) {}

    pub fn from_ctx() -> Self {
        let ctx = android_context();
        let obj = vm_attach(|env| env.new_global_ref(&ctx).unwrap());
        Self {
            _obj: obj,
        }
    }
}
