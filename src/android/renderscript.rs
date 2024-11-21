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

#![allow(deprecated)]


use crate::{JType,JObjRef,JObjNew};
use droid_wrap_derive::java_class;


//noinspection SpellCheckingInspection
/**
此类提供将数据传递到 RenderScript 内核和从 RenderScript 内核传递数据的主要方法。Allocation 为给定类型提供后备存储。
Allocation 还包含一组使用标志，用于指示如何使用 Allocation。例如，Allocation 可能具有使用标志，指定它既可以从脚本中使用，也可以作为 Sampler 的输入。
开发人员必须使用 syncAll 同步这些不同的用法，以确保 Allocation 的不同用户对内存具有一致的视图。例如，如果将 Allocation 用作一个内核的输出，并将其用作后续内核的 Sampler 输入，则开发人员必须在启动第二个内核之前调用 syncAll(Allocation.USAGE_SCRIPT) 以确保正确性。
Allocation 可以使用 copyFrom 例程填充。对于更复杂的 Element 类型，可以使用 copyFromUnchecked 方法从字节数组或类似构造中复制。

开发人员指南
有关创建使用 RenderScript 的应用程序的更多信息，请阅读 RenderScript 开发人员指南。
*/
#[deprecated(note = "Renderscript 已在 API 级别 31 中弃用。请参阅迁移指南了解建议的替代方案。")]
#[java_class(name = "android/renderscript/Allocation")]
pub struct Allocation;


//noinspection SpellCheckingInspection
/**
此类提供对 RenderScript 上下文的访问，该上下文控制 RenderScript 初始化、资源管理和拆卸。必须先创建 RenderScript 类的实例，然后才能创建任何其他 RS 对象。

开发者指南
有关创建使用 RenderScript 的应用程序的更多信息，请阅读 RenderScript 开发者指南。
*/
#[deprecated(note = "Renderscript 已在 API 级别 31 中弃用。请参阅迁移指南以了解建议的替代方案。")]
#[java_class(name = "android/renderscript/RenderScript")]
pub struct RenderScript;
