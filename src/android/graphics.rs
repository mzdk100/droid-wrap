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
Canvas 类包含“绘制”调用。要绘制某些内容，您需要 4 个基本组件：用于保存像素的 Bitmap、用于托管绘制调用（写入位图）的 Canvas、绘图基元（例如 Rect、Path、文本、Bitmap）和 Paint（用于描述绘图的颜色和样式）。

开发者指南
有关如何使用 Canvas 的更多信息，请阅读 Canvas 和 Drawables 开发者指南。
*/
#[java_class(name = "android/graphics/Canvas")]
pub struct Canvas;

/**
Rect 保存矩形的四个整数坐标。矩形由其 4 条边（左、上、右、下）的坐标表示。这些字段可以直接访问。使用 width() 和 height() 检索矩形的宽度和高度。
注意：大多数方法不会检查坐标是否正确排序（即左 <= 右且上 <= 下）。
请注意，右坐标和下坐标是互斥的。这意味着在 Canvas 上绘制的未转换的 Rect 将绘制到其左坐标和上坐标所描述的列和行中，但不会绘制到其下坐标和右坐标中。
*/
#[java_class(name = "android/graphics/Rect")]
pub struct Rect;

//noinspection SpellCheckingInspection
/**
将图像流中的帧捕获为 OpenGL ES 纹理。图像流可能来自相机预览或视频解码。从 SurfaceTexture 创建的 Surface 可用作 android.hardware.camera2、android.media.MediaCodec、android.media.MediaPlayer 和 android.renderscript.Allocation API 的输出目标。
调用 updateTexImage 时，创建 SurfaceTexture 时指定的纹理对象的内容将更新为包含图像流中的最新图像。这可能会导致跳过流的某些帧。
在指定较旧的 android.hardware.Camera API 的输出目标时，还可以使用 SurfaceTexture 代替 SurfaceHolder。这样做会导致图像流中的所有帧都被发送到 SurfaceTexture 对象，而不是设备的显示器。
一种典型的模式是使用 SurfaceTexture 将帧渲染到 TextureView；但是，使用纹理对象不需要 TextureView。纹理对象可以用作 OpenGL ES 着色器的一部分。
从纹理采样时，首先应使用通过 getTransformMatrix(float[]) 查询的矩阵转换纹理坐标。每次调用 updateTexImage 时，转换矩阵可能会发生变化，因此每次更新纹理图像时都应重新查询。
此矩阵将传统的 2D OpenGL ES 纹理坐标列向量（形式为 (s, t, 0, 1)，其中 s 和 t 位于包含区间 [0, 1] 上）转换为流式纹理中的正确采样位置。此转换可补偿图像流源的任何属性，这些属性会导致它看起来与传统的 OpenGL ES 纹理不同。
例如，可以通过使用查询的矩阵转换列向量 (0, 0, 0, 1) 来完成从图像左下角的采样，而通过转换 (1, 1, 0, 1) 来完成从图像右上角的采样。纹理对象使用 GL_TEXTURE_EXTERNAL_OES 纹理目标，该目标由 GL_OES_EGL_image_external OpenGL ES 扩展定义。
这限制了纹理的使用方式。每次绑定纹理时，它都必须绑定到 GL_TEXTURE_EXTERNAL_OES 目标，而不是 GL_TEXTURE_2D 目标。此外，任何从纹理采样的 OpenGL ES 2.0 着色器都必须声明其对此扩展的使用，例如使用“#extension GL_OES_EGL_image_external : require”指令。
此类着色器还必须使用 samplerExternalOES GLSL 采样器类型访问纹理。 SurfaceTexture 对象可以在任何线程上创建。updateTexImage 只能在具有包含纹理对象的 OpenGL ES 上下文的线程上调用。帧可用回调在任意线程上调用，因此除非特别小心，否则不应直接从回调中调用 updateTexImage。
*/
#[java_class(name = "android/graphics/SurfaceTexture")]
pub struct SurfaceTexture;

/**
点具有两个整数坐标
*/
#[java_class(name = "android/graphics/Point")]
pub struct Point;
