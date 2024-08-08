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

use crate::{java::lang::Object, JObjNew, JObjRef, JType};
use droid_wrap_derive::{java_class, java_interface, java_method};

/**
InvocationHandler 是代理实例的调用处理程序实现的接口。每个代理实例都有一个关联的调用处理程序。当在代理实例上调用方法时，方法调用将被编码并分派到其调用处理程序的invoke方法。
*/
#[java_interface(name = "java/lang/InvocationHandler")]
pub trait InvocationHandler {
    /**
    处理代理实例上的方法调用并返回结果。当在与其关联的代理实例上调用方法时，将在调用处理程序上调用此方法。
    返回：从代理实例上的方法调用返回的值。如果接口方法声明的返回类型是原始类型，则此方法返回的值必须是相应原始包装器类的实例；否则，它必须是可分配给声明的返回类型的类型。如果此方法返回的值为 null，并且接口方法的返回类型为原始类型，则代理实例上的方法调用将抛出 NullPointerException。如果此方法返回的值与接口方法声明的返回类型不兼容（如上所述），则代理实例上的方法调用将抛出 ClassCastException。
    抛出:Throwable – 代理实例上的方法调用抛出的异常。异常的类型必须可分配给接口方法的 throws 子句中声明的任何异常类型，或可分配给未经检查的异常类型 java.lang.RuntimeException 或 java.lang.Error。如果此方法抛出的已检查异常无法分配给接口方法的 throws 子句中声明的任何异常类型，则代理实例上的方法调用将抛出包含此方法抛出的异常的 UndeclaredThrowableException。
    `proxy` 调用该方法的代理实例
    `method` 与在代理实例上调用的接口方法相对应的 Method 实例。Method 对象的声明类将是声明该方法的接口，该接口可能是代理类通过其继承方法的代理接口的超接口。
    `args` 包含代理实例上的方法调用中传递的参数值的对象数组，如果接口方法不带参数，则为 null。原始类型的参数包装在适当的原始包装器类的实例中，例如 java.lang.Integer 或 java.lang.Boolean。
    */
    fn invoke(proxy: &Object, method: &Method, args: &[Object]);
}

/// 方法和构造函数的共同功能的共享超类。
#[java_class(name = "java/lang/reflect/Executable")]
pub struct Executable;

impl Executable {
    /**
    返回此对象所代表的可执行文件的名称。
    */
    #[java_method]
    pub fn get_name(&self) -> String {}

    /**
    返回此对象所代表的可执行文件的形式参数数量（无论是显式声明、隐式声明还是均未声明）。
    返回：此对象所代表的可执行文件的形式参数数量
    */
    #[java_method]
    pub fn get_parameter_count(&self) -> i32 {}
}

/**
方法提供有关类或接口上的单个方法的信息和访问权。反射方法可以是类方法或实例方法（包括抽象方法）。当将要调用的实际参数与底层方法的形式参数匹配时，方法允许发生扩展转换，但如果发生缩小转换，则会引发 IllegalArgumentException。
*/
#[java_class(name = "java/lang/reflect/Method", extends=Executable)]
pub struct Method;

impl Method {
    /**
    以字符串形式返回此 Method 对象所表示的方法的名称。
    */
    pub fn get_name(&self) -> String {
        self._based.get_name()
    }

    /**
    返回此对象所代表的可执行文件的形式参数数量（无论是显式声明、隐式声明还是均未声明）。
    返回：此对象所代表的可执行文件的形式参数数量
    */
    pub fn get_parameter_count(&self) -> i32 {
        self._based.get_parameter_count()
    }
}

/// 测试java.lang.reflect
#[cfg(feature = "test_java_lang_reflect")]
pub fn test() {}
