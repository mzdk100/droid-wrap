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

pub use jni::{
    errors::Error,
    objects::{GlobalRef, JClass, JObject},
    sys::{jboolean, jchar, jdouble, jfloat, jint, jlong, jshort},
    AttachGuard, JavaVM,
};

/**
 * 定义必要的trait，以便于在本地为任何数据类型实现JAVA对象所需的功能。
 * */
#[macro_export]
macro_rules! import {
    () => {
        use droid_wrap_utils::GlobalRef;

        /**
         * JObjectRef trait提供从任何数据类型获取java对象的全局引用。
         * rust使用Arc管理，无须手动释放java的全局引用。
         * */
        pub trait JObjRef<'a> {
            /**
             * 获取java对象引用。
             * */
            fn java_ref(&self) -> GlobalRef;
        }

        /**
         * 用于描述java类的信息。
         * */
        pub trait JType<'j>: JObjRef<'j> {
            /// 错误类型。
            type Error;
            /// java类的名称。
            const CLASS: &'j str;

            /**
             * 获取对象的签名描述。
             * */
            fn get_object_sig() -> String {
                format!("L{};", Self::CLASS)
            }
        }
    };
}

/**
 * 获取android系统的java虚拟机。
 * */
pub fn android_vm() -> JavaVM {
    let ctx = ndk_context::android_context();
    unsafe { JavaVM::from_raw(ctx.vm().cast()) }.unwrap()
}

/// 获取vm，将vm附加到当前线程，随后操作java虚拟机。
///
/// # Arguments
///
/// * `wrapper`: 一个闭包，接收可变引用的env，然后可以使用env操作当前虚拟机环境。
///
/// returns: T
/// 闭包的返回值作为vm_attach的返回值。
///
/// # Examples
///
/// ```
/// use droid_wrap_utils::vm_attach;
/// let class = vm_attach(|env| env.find_class("java/lang/String")).unwrap();
/// dbg!(class);
/// ```
pub fn vm_attach<T>(wrapper: impl Fn(&mut AttachGuard) -> T) -> T {
    let vm = android_vm();
    let mut env = vm.attach_current_thread().unwrap();
    wrapper(&mut env)
}

/**
 * 获取安卓的Context对象，这通常是NativeActivity对象的引用。
 * */
pub fn android_context<'a>() -> JObject<'a> {
    let ctx = ndk_context::android_context();
    unsafe { JObject::from_raw(ctx.context().cast()) }
}
