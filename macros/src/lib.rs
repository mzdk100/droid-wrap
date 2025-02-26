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

mod entry;
mod java;
mod utils;

use proc_macro::TokenStream;

/// 安卓平台的入口
///
/// 将此宏标记在`fn main()`函数上可以自动实现安卓应用的入口函数。
///
/// # 示例
///
/// ```
/// use droid_wrap_macros::android_main;
///
/// #[android_main]
/// fn main() {}
/// ```
#[proc_macro_attribute]
pub fn android_main(_args: TokenStream, input: TokenStream) -> TokenStream {
    entry::android_main(input.into()).into()
}

/// 定义java class，将此属性标记在struct上，可以自动实现操作java对象的必要功能。
///
/// # Arguments
///
/// * `attrs`: 属性输入。
/// * `input`: struct输入。
///
/// returns: TokenStream
///
/// # Examples
///
/// ```
/// use droid_wrap_macros::java_class;
/// #[java_class(name = "java/lang/System")]
/// struct System;
/// ```
#[proc_macro_attribute]
pub fn java_class(attrs: TokenStream, input: TokenStream) -> TokenStream {
    java::java_class(attrs.into(), input.into()).into()
}

/// 实现java类的方法，将此属性标记在fn函数上，可以自动实现调用java方法，可以自动识别静态方法（如果参数中没有“self”）。
///
/// # Arguments
///
/// * `attrs`: 属性输入。
/// * `input`: 函数输入。
///
/// returns: TokenStream
///
/// # Examples
///
/// ```
/// use droid_wrap_macros::java_method;
/// struct System;
/// impl System {
/// #[java_method]
/// fn current_time_millis() -> i64 {}
/// }
/// ```
#[proc_macro_attribute]
pub fn java_method(attrs: TokenStream, input: TokenStream) -> TokenStream {
    java::java_method(attrs.into(), input.into()).into()
}

/// 实现java类的构造器，将此属性标记在fn函数上，可以自动实现调用java类的构造器。
///
/// # Arguments
///
/// * `_`: 未使用。
/// * `input`: 函数输入。
///
/// returns: TokenStream
///
/// # Examples
///
/// ```
/// use droid_wrap_macros::java_constructor;
/// struct Integer;
/// impl Integer {
/// #[java_constructor]
/// fn new(value: i32) -> Self {}
/// }
/// ```
#[proc_macro_attribute]
pub fn java_constructor(_: TokenStream, input: TokenStream) -> TokenStream {
    java::java_constructor(input.into()).into()
}

/// 定义java interface，将此属性标记在trait上，可以自动实现提供java对象与rust对象的互操作的功能。
///
/// # Arguments
///
/// * `attrs`: 属性。
/// * `input`: 特征输入。
///
/// returns: TokenStream
///
/// # Examples
///
/// ```
/// use droid_wrap_macros::java_interface;
/// trait Runnable {
/// fn run(&self);
/// }
/// ```
#[proc_macro_attribute]
pub fn java_interface(attrs: TokenStream, input: TokenStream) -> TokenStream {
    java::java_interface(attrs.into(), input.into()).into()
}

/// 实现java interface，将此属性标记在impl上，可以自动实现java接口的动态代理，从而实现java层回调rust层。
/// 其中在接口中定义的每一个方法将自动实现并暴露给java层，但以下划线“_”开头的函数除外。
///
/// # Arguments
///
/// * `attrs`: 属性。
/// * `input`: impl输入。
///
/// returns: TokenStream
///
/// # Examples
///
/// ```
/// use std::fmt::{Debug, Formatter};
/// use droid_wrap_macros::{java_interface,java_implement};
/// #[java_interface(name = "java/lang/Runnable")]
/// trait Runnable {
/// fn run(&self);
/// }
/// struct RunnableImpl;
/// impl PartialEq for RunnableImpl {
///     fn eq(&self, other: &Self) -> bool {
///         todo!()
///     }
/// }
/// impl Debug for RunnableImpl {
///     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
///         todo!()
///     }
/// }
/// #[java_implement]
/// impl Runnable for RunnableImpl {
/// fn run(&self) {
/// println!("Called from java.");
/// }
/// }
/// ```
#[proc_macro_attribute]
pub fn java_implement(attrs: TokenStream, input: TokenStream) -> TokenStream {
    java::java_implement(attrs.into(), input.into()).into()
}

/// 实现java类的字段，将此属性标记在带有get或set的fn函数上，可以自动实现访问java字段的能力，可以自动识别静态字段（如果参数中没有“self”）。
///
/// # Arguments
///
/// * `_`: 未使用。
/// * `input`: 函数输入。
///
/// returns: TokenStream
///
/// # Examples
///
/// ```
/// use droid_wrap_macros::java_field;
///
/// pub struct LayoutParams;
///
/// impl LayoutParams {
///     #[java_field]
///     pub fn get_width(&self) -> i32 {}
///
///     #[java_field]
///     pub fn set_width(&self, value: i32) {}
/// }
/// ```
#[proc_macro_attribute]
pub fn java_field(attrs: TokenStream, input: TokenStream) -> TokenStream {
    java::java_field(attrs.into(), input.into()).into()
}
