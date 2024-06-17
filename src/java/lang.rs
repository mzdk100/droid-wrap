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

use droid_wrap_derive::{java_class, java_constructor, java_method};
use droid_wrap_utils::{vm_attach, Error, GlobalRef};

use crate::{JObjRef, JType};

impl<'a> JObjRef<'a> for String {
    fn java_ref(&self) -> GlobalRef {
        CharSequence::from(self.as_str()).java_ref()
    }
}

impl<'a> JType<'a> for String {
    type Error = Error;
    const CLASS: &'a str = "java/lang/String";
}

impl<'a> JObjRef<'a> for &'static str {
    fn java_ref(&self) -> GlobalRef {
        CharSequence::from(*self).java_ref()
    }
}

impl<'a> JType<'a> for &'static str {
    type Error = Error;
    const CLASS: &'a str = "java/lang/String";
}

/**
 * Integer 类将原始类型 int 的值包装在对象中。Integer 类型的对象包含一个类型为 int 的字段。此外，此类还提供了几种将 int 转换为 String 和将 String 转换为 int 的方法，以及处理 int 时有用的其他常量和方法。
 * 实现说明：“位操作”方法（如 highestOneBit 和 numberOfTrailingZeros）的实现基于 Henry S. Warren, Jr. 的《Hacker's Delight》（Addison Wesley，2002 年）中的材料。
 * */
#[java_class(name = "java/lang/Integer")]
pub struct Integer;

impl Integer {
    /**
     * 构造一个新分配的 Integer 对象，该对象表示指定的 int 值。
     * `value` Integer 对象要表示的值。
     * */
    #[deprecated(
        note = "很少适合使用此构造函数。静态工厂 valueOf(int) 通常是更好的选择，因为它可能会产生更好的空间和时间性能。"
    )]
    #[java_constructor]
    pub fn new(value: i32) -> Self {}

    //noinspection RsRedundantColonColon
    /**
     * 返回表示指定 int 值的 Integer 实例。如果不需要新的 Integer 实例，则通常应优先使用此方法而不是构造函数 Integer(int)，因为此方法通过缓存频繁请求的值可能会产生更好的空间和时间性能。此方法将始终缓存 -128 到 127 范围内的值（含），并且可能会缓存此范围之外的其他值。
     * 返回：表示 i 的 Integer 实例。
     * `i` 一个 int 值。
     * */
    #[java_method]
    pub fn value_of<'j>(i: i32) -> Result<Self, <Self as JType<'j>>::Error> {}
}

/**
 * Float 类将原始类型浮点的值包装在对象中。Float 类型的对象包含一个类型为浮点的字段。此外，此类还提供了几种将浮点转换为字符串和将字符串转换为浮点的方法，以及处理浮点时有用的其他常量和方法。浮点相等、等价和比较 java.lang.Double 类讨论了浮点值的相等、等价和比较，其中相等适用于浮点值。
 * */
#[java_class(name = "java/lang/Float")]
pub struct Float;

impl Float {
    /**
     * 构造一个新分配的 Float 对象，该对象表示原始 float 参数。
     * `value` Float 要表示的值。
     * */
    #[deprecated(
        note = "很少适合使用此构造函数。静态工厂 valueOf(float) 通常是更好的选择，因为它可能会产生更好的空间和时间性能。"
    )]
    #[java_constructor]
    pub fn new(value: f32) -> Self {}

    /**
     * 返回表示指定浮点值的 Float 实例。如果不需要新的 Float 实例，则通常应优先使用此方法而不是构造函数 Float(float)，因为此方法可能会通过缓存频繁请求的值来显著提高空间和时间性能。
     * 返回：表示 f 的 Float 实例。
     * `f` 浮点值。
     * */
    #[java_method]
    pub fn value_of<'j>(f: f32) -> Result<Self, <Self as JType<'j>>::Error> {}
}

/**
 * CharSequence 是可读的 char 值序列。此接口提供对许多不同种类的 char 序列的统一、只读访问。char 值表示基本多语言平面 (BMP) 中的字符或代理。有关详细信息，请参阅 Unicode 字符表示。此接口不会细化 equals 和 hashCode 方法的一般约定。因此，测试两个实现 CharSequence 的对象是否相等的结果通常是不确定的。每个对象可能由不同的类实现，并且不能保证每个类都能够测试其实例与另一个类的实例是否相等。因此，将任意 CharSequence 实例用作集合中的元素或映射中的键是不合适的。
 * */
#[java_class(name = "java/lang/CharSequence")]
pub struct CharSequence;

impl From<&str> for CharSequence {
    /// 从字符串引用获取CharSequence对象。
    ///
    /// # Arguments
    ///
    /// * `value`: 字符串引用。
    ///
    /// returns: CharSequence
    ///
    /// # Examples
    ///
    /// ```
    /// use droid_wrap::java::lang::CharSequence;
    /// let cs = CharSequence::from("hello");
    /// dbg!(cs);
    /// ```
    fn from(value: &str) -> Self {
        let obj = vm_attach(|env| {
            let cs = env.new_string(value).unwrap();
            env.new_global_ref(&cs).unwrap()
        });
        Self::_new(&obj)
    }
}

/**
 * System 类包含几个有用的类字段和方法。它无法实例化。 System 类提供的功能包括标准输入、标准输出和错误输出流；访问外部定义的属性和环境变量；加载文件和库的方法；以及用于快速复制数组一部分的实用方法。
 * */
#[java_class(name = "java/lang/System")]
pub struct System;

impl System {
    /**
     * 返回当前时间（以毫秒为单位）。请注意，虽然返回值的时间单位是毫秒，但值的粒度取决于底层操作系统，可能更大。例如，许多操作系统以数十毫秒为单位测量时间。有关“计算机时间”和协调世界时 (UTC) 之间可能出现的细微差异的讨论，请参阅 Date 类的描述。
     * 返回：当前时间与 1970 年 1 月 1 日午夜 UTC 之间的差值（以毫秒为单位）。
     * */
    #[java_method]
    pub fn current_time_millis() -> i64 {}

    /**
     * 运行垃圾收集器。调用 gc 方法表明 Java 虚拟机会努力回收未使用的对象，以便使它们当前占用的内存可供快速重用。当控制权从方法调用返回时，Java 虚拟机已尽最大努力从所有丢弃的对象中回收空间。调用 System.gc() 实际上等同于调用： Runtime.getRuntime().gc()
     * */
    #[java_method]
    pub fn gc() {}
}

#[cfg(feature = "test_java_lang")]
pub fn test() {
    let integer = Integer::value_of(100).unwrap();
    assert_eq!("100", integer.to_string());
    let float = Float::value_of(423.3).unwrap();
    assert_eq!("423.3", float.to_string());
    let cs = CharSequence::from("hello");
    assert_eq!("hello", cs.to_string());
    assert!(System::current_time_millis() > 0);
    System::gc();
}
