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
use droid_wrap_derive::java_class;

/**
 * 字节缓冲区。此类定义了六类针对字节缓冲区的操作：
 * - 绝对和相对获取和放置方法，用于读取和写入单个字节；
 * - 相对批量获取方法，用于将连续的字节序列从此缓冲区传输到数组；
 * - 相对批量放置方法，用于将连续的字节序列从字节数组或其他字节缓冲区传输到此缓冲区；
 * - 绝对和相对获取和放置方法，用于读取和写入其他原始类型的值，并将它们转换为特定字节顺序的字节序列；
 * - 创建视图缓冲区的方法，允许将字节缓冲区视为包含其他原始类型值的缓冲区；以及压缩、复制和切片字节缓冲区的方法。
 * 字节缓冲区可以通过分配（为缓冲区的内容分配空间）或将现有字节数组包装到缓冲区中来创建。
 * 直接与非直接缓冲区
 * 字节缓冲区可以是直接的，也可以是非直接的。给定一个直接字节缓冲区，Java 虚拟机将尽最大努力直接在其上执行本机 I/O 操作。也就是说，它将尝试避免在每次调用底层操作系统的本机 I/O 操作之前（或之后）将缓冲区的内容复制到中间缓冲区（或从中间缓冲区复制）。
 * 可以通过调用此类的 allocateDirect 工厂方法来创建直接字节缓冲区。此方法返回的缓冲区的分配和释放成本通常比非直接缓冲区略高。直接缓冲区的内容可能位于正常垃圾收集堆之外，因此它们对应用程序内存占用的影响可能不明显。因此，建议主要为受底层系统本机 I/O 操作影响的大型、长寿命缓冲区分配直接缓冲区。
 * 通常，最好仅在直接缓冲区对程序性能产生可衡量的增益时才分配直接缓冲区。也可以通过将文件区域直接映射到内存中来创建直接字节缓冲区。 Java 平台的实现可以选择支持通过 JNI 从本机代码创建直接字节缓冲区。如果这些类型的缓冲区之一的实例引用了不可访问的内存区域，则尝试访问该区域将不会更改缓冲区的内容，并且会导致在访问时或稍后抛出未指定的异常。
 * 可以通过调用其 isDirect 方法来确定字节缓冲区是直接的还是非直接的。提供此方法是为了在性能关键型代码中进行显式缓冲区管理。
 * 访问二进制数据
 * 此类定义了用于读取和写入除布尔值之外的所有其他原始类型的值的方法。原始值根据缓冲区的当前字节顺序转换为（或从）字节序列，可以通过 order 方法检索和修改字节序列。特定字节顺序由 ByteOrder 类的实例表示。字节缓冲区的初始顺序始终是 BIG_ENDIAN。
 * 对于访问异构二进制数据（即不同类型的值序列），此类为每种类型定义了一系列绝对和相对地获取和放置方法。例如，对于 32 位浮点值，此类定义：
 * ```java
 * float getFloat()
 * float getFloat(int index)
 * void putFloat(float f)
 * void putFloat(int index, float f)
 * ```
 * char、short、int、long 和 double 类型定义了相应的方法。绝对获取和放置方法的索引参数以字节为单位，而不是以读取或写入的类型为单位。对于访问同质二进制数据（即相同类型的值序列），此类定义了可以创建给定字节缓冲区视图的方法。
 * 视图缓冲区只是另一个缓冲区，其内容由字节缓冲区支持。对字节缓冲区内容的更改将在视图缓冲区中可见，反之亦然；两个缓冲区的位置、限制和标记值是独立的。例如，asFloatBuffer 方法创建 FloatBuffer 类的一个实例，该实例由调用该方法的字节缓冲区支持。为 char、short、int、long 和 double 类型定义了相应的视图创建方法。
 * 与上述类型特定的 get 和 put 方法系列相比，视图缓冲区具有三个重要优势：
 * - 视图缓冲区不是按字节进行索引，而是按其值的类型特定大小进行索引；
 * - 视图缓冲区提供相对批量的 get 和 put 方法，可以在缓冲区和数组或相同类型的其他缓冲区之间传输连续的值序列；
 * - 视图缓冲区可能更高效，因为当且仅当其支持字节缓冲区是直接的时，它才是直接的。视图缓冲区的字节顺序固定为其创建视图时的字节缓冲区的字节顺序。
 * 调用链
 * 此类中没有返回值的方法被指定为返回调用它们的缓冲区。这允许方法调用链。语句序列
 * ```java
 * bb.putInt(0xCAFEBABE);
 * bb.putShort(3);
 * bb.putShort(45);
 * ```
 * 例如，可以用单个语句 `bb.putInt(0xCAFEBABE).putShort(3).putShort(45);` 代替
 * */
#[java_class(name = "java/nio/ByteBuffer")]
pub struct ByteBuffer;

/// 测试java.nio
#[cfg(feature = "test_java_nio")]
pub fn test() {
    let buffer = ByteBuffer::null();
    assert_eq!(buffer, ByteBuffer::null());
}
