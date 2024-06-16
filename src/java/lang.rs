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
use droid_wrap_utils::{jint, vm_attach, GlobalRef};

use crate::{JObjRef, JType};

impl<'a> JObjRef<'a> for String {
    fn java_ref(&self) -> GlobalRef {
        CharSequence::from(self.as_str()).java_ref()
    }
}

impl<'a> JType<'a> for String {
    const CLASS: &'a str = "java/lang/String";
}

impl<'a> JObjRef<'a> for &'static str {
    fn java_ref(&self) -> GlobalRef {
        CharSequence::from(*self).java_ref()
    }
}

impl<'a> JType<'a> for &'static str {
    const CLASS: &'a str = "java/lang/String";
}

#[java_class(name = "java/lang/Integer")]
pub struct Integer;

impl Integer {
    pub fn new(value: i32) -> Self {
        let obj = vm_attach(|env| {
            let i = env
                .new_object(Self::CLASS, "(I)V", &[(value as jint).into()])
                .unwrap();
            env.new_global_ref(&i).unwrap()
        });
        Self { _obj: obj }
    }
}

#[java_class(name = "java/lang/CharSequence")]
pub struct CharSequence;

impl From<&str> for CharSequence {
    fn from(value: &str) -> Self {
        let obj = vm_attach(|env| {
            let cs = env.new_string(value).unwrap();
            env.new_global_ref(&cs).unwrap()
        });
        Self { _obj: obj }
    }
}

#[java_class(name = "java/lang/System")]
pub struct System;

impl System {
    #[java_method]
    pub fn current_time_millis() -> i64 {}
}
