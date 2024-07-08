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

/**
 * 提供用于获取有关类和对象的反射信息的类和接口。
 * */
#[cfg(feature = "java_lang_reflect")]
#[cfg_attr(docsrs, doc(cfg(feature = "java_lang_reflect")))]
pub mod reflect;

use droid_wrap_derive::{
    java_class, java_constructor, java_implement, java_interface, java_method,
};
use droid_wrap_utils::{vm_attach, Error, GlobalRef};
use std::sync::Arc;

use crate::{JObjNew, JObjRef, JProxy, JType};

/**
 * Object 类是类层次结构的根。每个类都有 Object 作为超类。所有对象（包括数组）都实现了此类的方法。
 * */
#[java_class(name = "java/lang/Object")]
pub struct Object;

impl Object {
    /**
     * 返回对象的哈希码值。此方法支持哈希表，例如 java.util.HashMap 提供的哈希表。
     * hashCode 的一般约定是：在 Java 应用程序执行期间，只要对同一对象多次调用该方法，则 hashCode 方法必须始终返回相同的整数，前提是未修改对象上 equals 比较中使用的信息。此整数不必在应用程序的一次执行和同一应用程序的另一次执行之间保持一致。如果根据 equals(Object) 方法，两个对象相等，则对这两个对象中的每一个调用 hashCode 方法必须产生相同的整数结果。如果根据 equals(Object) 方法，两个对象不相等，则对这两个对象中的每一个调用 hashCode 方法不必产生不同的整数结果。
     * 但是，程序员应该知道，为不相等的对象产生不同的整数结果可能会提高哈希表的性能。在合理可行的范围内，Object 类定义的 hashCode 方法确实会为不同的对象返回不同的整数。 （hashCode 可能被实现为某个时间点的对象内存地址的某个函数，也可能不被实现。）
     * 返回：此对象的哈希码值。
     * */
    #[java_method]
    pub fn hash_code(&self) -> i32 {}
}

pub trait ObjectExt {
    fn cast<T: JType>(&self) -> T
    where
        <T as JObjNew>::Fields: Default;
}

impl ObjectExt for Object {
    fn cast<T: JType>(&self) -> T
    where
        <T as JObjNew>::Fields: Default,
    {
        T::_new(self.as_ref(), Default::default())
    }
}

impl JObjRef for String {
    fn java_ref(&self) -> GlobalRef {
        vm_attach!(mut env);
        let s = env.new_string(self.as_str()).unwrap();
        env.new_global_ref(&s).unwrap()
    }
}

impl JObjNew for String {
    type Fields = ();

    fn _new(this: &GlobalRef, _: Self::Fields) -> Self {
        if this.is_null() {
            return Self::default();
        }
        vm_attach!(mut env);
        if let Ok(s) = env.get_string(this.as_obj().into()) {
            return Self::from(s.to_str().unwrap());
        }
        Self::null()
    }
}

impl JType for String {
    type Error = Error;
    const CLASS: &'static str = "java/lang/String";
}

/**
 * Boolean 类将原始类型布尔值包装在对象中。布尔类型的对象包含一个类型为布尔的字段。此外，此类还提供了许多将布尔值转换为字符串和将字符串转换为布尔值的方法，以及处理布尔值时有用的其他常量和方法。
 * */
#[java_class(name = "java/lang/Boolean")]
pub struct Boolean;

impl Boolean {
    /**
     * 分配一个表示值参数的布尔对象。
     * `value` 布尔值。
     * */
    #[deprecated(
        note = "使用此构造函数很少适合。静态工厂valueOf(boolean)通常是一个更好的选择，因为它可能会产生更好的空间和时间性能。还要考虑如果可能的话，请考虑使用最终字段。"
    )]
    #[java_constructor]
    pub fn new(value: bool) -> Self {}

    /**
     * 返回表示指定布尔值的布尔实例。如果指定的布尔值为真，则此方法返回 Boolean.TRUE；如果为假，则此方法返回 Boolean.FALSE。如果不需要新的布尔实例，则通常应优先使用此方法而不是构造函数 Boolean(boolean)，因为此方法可能会产生更好的空间和时间性能。
     * 返回：表示 b 的布尔实例。
     * `b` 布尔值。
     * */
    #[java_method]
    pub fn value_of(b: bool) -> Result<Self, <Self as JType>::Error> {}
}

impl From<&bool> for Boolean {
    fn from(value: &bool) -> Self {
        Self::value_of(*value).unwrap()
    }
}

pub trait CharSequenceExt {
    fn to_char_sequence<CS: CharSequence>(&self) -> CS
    where
        <CS as JObjNew>::Fields: Default;
}

impl<'a> CharSequenceExt for &'a str {
    fn to_char_sequence<CS: CharSequence>(&self) -> CS
    where
        <CS as JObjNew>::Fields: Default,
    {
        vm_attach!(mut env);
        let s = env.new_string(*self).unwrap();
        let s = env.new_global_ref(&s).unwrap();
        CS::_new(&s, Default::default())
    }
}

/**
 * Integer 类将原始类型 int 的值包装在对象中。Integer 类型的对象包含一个类型为 int 的字段。此外，此类还提供了几种将 int 转换为 String 和将 String 转换为 int 的方法，以及处理 int 时有用的其他常量和方法。
 * 实现说明：“位操作”方法（如 highestOneBit 和 numberOfTrailingZeros）的实现基于 Henry S. Warren, Jr. 的《Hacker's Delight》（Addison Wesley，2002 年）中的材料。
 * */
#[java_class(name = "java/lang/Integer")]
pub struct Integer;

impl Integer {
    /// 具有-231的常数保持最小值。
    pub const MIN_VALUE: u32 = 0x80000000;

    /// 一个常数，保存 int 可以拥有的最大值 231-1。
    pub const MAX_VALUE: u32 = 0x7fffffff;

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
    pub fn value_of(i: i32) -> Result<Self, <Self as JType>::Error> {}
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
    pub fn value_of(f: f32) -> Result<Self, <Self as JType>::Error> {}
}

/**
 * CharSequence 是可读的 char 值序列。此接口提供对许多不同种类的 char 序列的统一、只读访问。char 值表示基本多语言平面 (BMP) 中的字符或代理。有关详细信息，请参阅 Unicode 字符表示。此接口不会细化 equals 和 hashCode 方法的一般约定。因此，测试两个实现 CharSequence 的对象是否相等的结果通常是不确定的。每个对象可能由不同的类实现，并且不能保证每个类都能够测试其实例与另一个类的实例是否相等。因此，将任意 CharSequence 实例用作集合中的元素或映射中的键是不合适的。
 * */
#[java_interface(name = "java/lang/CharSequence")]
pub trait CharSequence {
    /**
     * 返回此字符序列的长度。长度是序列中的16位字符的数量。
     * 返回：此序列中的字符数量
     * */
    fn length(&self) -> i32;

    /**
     * 返回指定索引处的 char 值。索引范围从零到 length() - 1。序列的第一个 char 值位于索引零处，下一个位于索引一处，依此类推，就像数组索引一样。如果索引指定的 char 值是代理，则返回代理值。
     * 抛出 IndexOutOfBoundsException 如果 index 参数为负数或不小于 length()
     * `index` 要返回的 char 值的索引
     * 返回：指定的 char 值
     * */
    fn char_at(&self, index: i32) -> Result<char, Error>;
}

#[java_class(name = "java/lang/CharSequenceImpl")]
pub struct CharSequenceImpl;

impl CharSequence for CharSequenceImpl {
    #[java_method]
    fn length(&self) -> i32 {}

    #[java_method]
    fn char_at(&self, index: i32) -> Result<char, Error> {}
}

/**
 * 任何实例旨在由线程执行的类都应实现 Runnable 接口。该类必须定义一个名为 run 的无参数方法。此接口旨在为希望在活动期间执行代码的对象提供通用协议。例如，Runnable 由 Thread 类实现。
 * 活动状态仅表示线程已启动且尚未停止。此外，Runnable 还提供了在不子类化 Thread 的情况下使类处于活动状态的方法。实现 Runnable 的类可以通过实例化 Thread 实例并将其自身作为目标传递，而无需子类化 Thread 即可运行。
 * 在大多数情况下，如果您只打算覆盖 run() 方法而不覆盖其他 Thread 方法，则应使用 Runnable 接口。这一点很重要，因为除非程序员打算修改或增强类的基本行为，否则不应子类化类。
 * */
#[java_interface(name = "java/lang/Runnable")]
pub trait Runnable {
    /**
     * 当使用实现 Runnable 接口的对象创建线程时，启动该线程会导致在该单独执行的线程中调用该对象的 run 方法。
     * 方法 run 的一般约定是它可以采取任何操作。
     * */
    fn run(&self);
}

#[java_class(name = "java/lang/RunnableImpl")]
pub struct RunnableImpl(Box<dyn Fn() + Sync + Send>);

impl RunnableImpl {
    pub fn from_fn(func: impl Fn() + Sync + Send + 'static) -> Arc<Self> {
        Self::new(RunnableImplDefault(Box::new(func)))
    }
}

impl Default for RunnableImplDefault {
    fn default() -> Self {
        Self(Box::new(|| ()))
    }
}

#[java_implement]
impl Runnable for RunnableImpl {
    fn run(&self) {
        self.0();
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

    /**
     * 终止当前正在运行的 Java 虚拟机。该参数用作状态代码；按照惯例，非零状态代码表示异常终止。此方法调用 Runtime 类中的 exit 方法。此方法永远不会正常返回。调用 System.exit(n) 实际上等同于调用： Runtime.getRuntime().exit(n)
     * `status` 退出状态。
     * 抛出:SecurityException – 如果存在安全管理器并且其 checkExit 方法不允许以指定状态退出。
     * */
    #[java_method]
    pub fn exit(status: i32) -> Result<(), <Self as JType>::Error> {}
}

/**
 * 类加载器是负责加载类的对象。ClassLoader 类是一个抽象类。给定类的二进制名称，类加载器应尝试定位或生成构成该类定义的数据。典型的策略是将名称转换为文件名，然后从文件系统中读取该名称的“类文件”。
 * 每个 Class 对象都包含对定义它的 ClassLoader 的引用。数组类的类对象不是由类加载器创建的，而是由 Java 运行时根据需要自动创建的。Class.getClassLoader() 返回的数组类的类加载器与其元素类型的类加载器相同；如果元素类型是原始类型，则数组类没有类加载器。
 * 应用程序实现 ClassLoader 的子类，以扩展 Java 虚拟机动态加载类的方式。安全管理器通常可以使用类加载器来指示安全域。 ClassLoader 类使用委托模型来搜索类和资源。
 * ClassLoader 的每个实例都有一个关联的父类加载器。当请求查找类或资源时，ClassLoader 实例会将对类或资源的搜索委托给其父类加载器，然后再尝试自己查找类或资源。虚拟机的内置类加载器称为“引导类加载器”，它本身没有父类，但可以充当 ClassLoader 实例的父类。
 * 支持并发加载类的类加载器称为具有并行能力的类加载器，需要在类初始化时通过调用 ClassLoader.registerAsParallelCapable 方法来注册自己。请注意，默认情况下，ClassLoader 类被注册为具有并行能力。但是，如果其子类具有并行能力，则仍需要注册自己。
 * 在委托模型不是严格分层的环境中，类加载器需要具有并行能力，否则类加载可能会导致死锁，因为加载器锁会在整个类加载过程中一直保持（请参阅 loadClass 方法）。
 * 通常，Java 虚拟机以与平台相关的方式从本地文件系统加载类。例如，在 UNIX 系统上，虚拟机从 CLASSPATH 环境变量定义的目录中加载类。但是，某些类可能不是源自文件；它们可能源自其他来源，例如网络，或者可以由应用程序构造。方法 defineClass 将字节数组转换为 Class 类的实例。
 * 可以使用 Class.newInstance 创建此新定义类的实例。类加载器创建的对象的方法和构造函数可能会引用其他类。为了确定引用的类，Java 虚拟机会调用最初创建该类的类加载器的 loadClass 方法。例如，应用程序可以创建网络类加载器以从服务器下载类文件。示例代码可能如下所示：
 * ```java
 * ClassLoader loader = new NetworkClassLoader(host, port);
 * Object main = loader.loadClass("Main", true).newInstance();
 * ...
 * ```
 * 网络类加载器子类必须定义 findClass 和 loadClassData 方法，以便从网络加载类。下载组成类的字节后，应使用 defineClass 方法创建类实例。示例实现如下：
 * ```java
 * class NetworkClassLoader extends ClassLoader {
 * String host;
 * int port;
 * public Class findClass(String name) {
 * byte[] b = loadClassData(name);
 * return defineClass(name, b, 0, b.length);
 * }
 * private byte[] loadClassData(String name) {
 * // 从连接加载类数据
 * ...
 * }
 * }
 * ```
 * 二进制名称作为 String 参数提供给 ClassLoader 中方法的任何类名都必须是 Java™ 语言规范定义的二进制名称。有效类名的示例包括：
 * "java.lang.String" "javax.swing.JSpinner$DefaultEditor" "java.security.KeyStore$Builder$FileBuilder$1" "java.net.URLClassLoader$3$1"
 * */
#[java_class(name = "java/lang/ClassLoader")]
pub struct ClassLoader;

#[cfg(feature = "test_java_lang")]
pub fn test() {
    let integer = Integer::value_of(100).unwrap();
    assert_eq!("100", integer.to_string());
    let float = Float::value_of(423.3).unwrap();
    assert_eq!("423.3", float.to_string());
    let cs = "hello".to_char_sequence::<CharSequenceImpl>();
    assert_eq!("hello", cs.to_string());
    assert_eq!(5, cs.length());
    assert_eq!('h', cs.char_at(0).unwrap());
    assert!(System::current_time_millis() > 0);
    System::gc();
    let cl = ClassLoader::null();
    assert_eq!(cl, ClassLoader::null());
    let func = RunnableImpl::from_fn(move || println!("Runnable is running."));
    dbg!(func);
    // System::exit(0).unwrap();
}
