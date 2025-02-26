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

use crate::{
    JObjNew, JObjRef, JType, Result,
    android::{
        graphics::{Point, Rect, SurfaceTexture},
        renderscript::{Allocation, RenderScript},
        view::{Surface, SurfaceHolder},
    },
    java_class, java_constructor, java_field, java_interface, java_method,
};

/// 振动器
#[cfg(feature = "android_hardware_vibrator")]
pub mod vibrator;

/**
Camera类用于设置图像捕获设置，开始/停止预览，拍摄照片，并检索用于编码的视频帧。
此类是Camera服务的客户端，该服务管理实际的相机硬件。

要访问设备相机，您必须在Android清单中声明android.Manifest.permission#CAMERA权限。还要确保包含 `<uses-feature>` 清单元素，以声明应用程序使用的相机功能。
例如，如果您使用相机和自动对焦功能，您的清单应包含以下内容：
```xml
<uses-permission android:name="android.permission.CAMERA" />
<uses-feature android:name="android.hardware.camera" />
<uses-feature android:name="android.hardware.camera.autofocus" />
```

使用此类拍照的步骤如下：
- 从open(int)获取Camera实例。
- 使用getParameters()获取现有（默认）设置。
- 如果需要，修改返回的Camera.Parameters对象并调用setParameters(Camera.Parameters)。
- 调用setDisplayOrientation(int)以确保预览的正确方向。
- 重要：将完全初始化的SurfaceHolder传递给setPreviewDisplay(SurfaceHolder)。没有表面，相机将无法启动预览。
- 重要：调用startPreview()以开始更新预览表面。在拍照之前必须先启动预览。
- 当您想拍照时，调用takePicture(Camera.ShutterCallback,Camera.PictureCallback, Camera.PictureCallback, Camera.PictureCallback)来捕获照片。等待回调提供实际图像数据。
- 拍照后，预览显示将停止。要拍摄更多照片，请先调用startPreview()。
- 调用stopPreview()停止更新预览表面。
- 重要：调用release()释放相机供其他应用程序使用。应用程序应在android.app.Activity#onPause()中立即释放相机（并在android.app.Activity#onResume()中重新open()）。

要快速切换到视频录制模式，请使用以下步骤：
- 获取并初始化Camera并启动预览，如上所述。
- 调用unlock()允许媒体进程访问相机。
- 将相机传递给android.media.MediaRecorder#setCamera(Camera)。有关视频录制的更多信息，请参阅android.media.MediaRecorder。
- 完成录制后，调用reconnect()重新获取并锁定相机。
- 如果需要，重新启动预览并拍摄更多照片或视频。
- 调用stopPreview()和release()，如上所述。

此类不是线程安全的，旨在从单个事件线程使用。大多数长时间运行的操作（预览、对焦、拍照等）都是异步执行的，并在必要时调用回调。
回调将在open(int)被调用的事件线程上调用。此类的方法绝不能同时从多个线程调用。

<p class="caution">
<strong>注意：</strong>
不同的Android设备可能具有不同的硬件规格，例如百万像素评级和对焦能力。为了使您的应用程序与更多设备兼容，您不应对设备相机规格做出假设。
</p>

开发者指南<
有关使用相机的更多信息，请阅读开发者指南。
*/
#[java_class(name = "android/hardware/Camera")]
#[deprecated(note = "我们建议使用新的android.hardware.camera2 API来编写新应用程序。")]
pub struct Camera;

impl Camera {
    /// 未指定的相机错误。
    pub const CAMERA_ERROR_UNKNOWN: i32 = 1;

    /// 由于优先级较高的用户正在使用，相机已断开连接。
    pub const CAMERA_ERROR_EVICTED: i32 = 2;

    /// 由于设备策略更改或客户端应用程序进入后台，相机已断开连接。
    pub const CAMERA_ERROR_DISABLED: i32 = 3;

    /// 媒体服务器死机。在这种情况下，应用程序必须释放相机对象并实例化一个新的。
    pub const CAMERA_ERROR_SERVER_DIED: i32 = 100;

    /**
    广播操作：相机拍摄了一张新照片，照片的条目已添加到媒体存储中。
    android.content.Intent#getData是照片的URI。
    在android.os.Build.VERSION_CODES#N Android N中，此广播已被移除，应用程序建议使用android.app.job.JobInfo.Builder JobInfo.Builder.android.app.job.JobInfo.Builder#addTriggerContentUri代替。
    在android.os.Build.VERSION_CODES#O Android O中，此广播已恢复，但仅限于已注册的接收器。
    如果应用程序正在积极运行，并且希望立即收到有关拍摄照片的清晰信号，则可以再次监听广播，但任何因此而进行繁重工作（或需要启动）的内容应仍应使用JobScheduler。
    */
    pub const ACTION_NEW_PICTURE: &'static str = "android.hardware.action.NEW_PICTURE";

    /**
    广播操作：相机录制了一段新视频，视频的条目已添加到媒体存储中。
    android.content.Intent#getData是视频的URI。
    在android.os.Build.VERSION_CODES#N Android N中，此广播已被移除，应用程序建议使用android.app.job.JobInfo.Builder JobInfo.Builder.android.app.job.JobInfo.Builder#addTriggerContentUri代替。
    在android.os.Build.VERSION_CODES#O Android O中，此广播已恢复，但仅限于已注册的接收器。
    如果应用程序正在积极运行，并且希望立即收到有关录制视频的清晰信号，则可以再次监听广播，但任何因此而进行繁重工作（或需要启动）的内容应仍应使用JobScheduler。
    */
    pub const ACTION_NEW_VIDEO: &'static str = "android.hardware.action.NEW_VIDEO";

    /// Camera HAL设备API版本1.0
    pub const CAMERA_HAL_API_VERSION_1_0: i32 = 0x100;

    /// Camera HAL设备API版本3.0
    pub const CAMERA_HAL_API_VERSION_3_0: i32 = 0x300;

    /**
    返回此设备上可用的物理相机的数量。
    如果设备支持外部相机并且连接或断开了外部相机，则此方法的返回值可能会动态变化。
    如果系统中存在android.hardware.camera2.CameraCharacteristics#REQUEST_AVAILABLE_CAPABILITIES_LOGICAL_MULTI_CAMERA逻辑多相机，为了保持应用程序向后兼容性，此方法将只暴露每个面向的逻辑相机和物理相机组中的一个相机。
    使用camera2 API以查看所有相机。
    返回：可访问的相机设备总数，如果没有相机或枚举它们时发生错误，则为0。
    */
    #[java_method]
    pub fn get_number_of_cameras() -> i32 {}

    /**
    返回有关特定相机的信息。如果 getNumberOfCameras() 返回 N，则有效 ID 为 0 到 N-1。
    抛出:RuntimeException – 如果提供了无效 ID，或者检索信息时出错（通常是由于硬件或其他低级故障）。
    */
    #[java_method]
    pub fn get_camera_info(camera_id: i32, camera_info: &Camera_CameraInfo) -> Result<()> {}

    /**
    创建一个新的 Camera 对象以访问特定的硬件摄像头。如果其他应用程序打开了同一个摄像头，这将引发 RuntimeException。
    使用完摄像头后，您必须调用 release()，否则它将保持锁定状态，其他应用程序无法使用。
    对于特定的硬件摄像头，您的应用程序一次只能激活一个 Camera 对象。来自其他方法的回调将传递到调用 open() 的线程的事件循环。
    如果此线程没有事件循环，则回调将传递到主应用程序事件循环。如果没有主应用程序事件循环，则不会传递回调。
    注意：在某些设备上，此方法可能需要很长时间才能完成。最好从工作线程（可能使用 android.os.AsyncTask）调用此方法，以避免阻塞主应用程序 UI 线程。
    返回：一个新的 Camera 对象，已连接、已锁定并可供使用。
    抛出：RuntimeException – 如果打开相机失败（例如，如果相机正在被另一个进程使用或设备策略管理器已禁用相机）。
    `camera_id` 要访问的硬件摄像头，介于 0 和 getNumberOfCameras()-1 之间。
    */
    #[java_method]
    pub fn open(camera_id: i32) -> Result<Self> {}

    /**
    创建一个新的 Camera 对象来访问设备上的第一个后置摄像头。如果设备没有后置摄像头，则返回 null。否则，其作用类似于 open(int) 调用。
    返回：第一个后置摄像头的新 Camera 对象，如果没有后置摄像头，则返回 null
    */
    #[java_method(overload = open)]
    pub fn open_convenience() -> Option<Self> {}

    /**
    使用给定的 hal API 版本创建一个新的 Camera 对象来访问特定的硬件摄像头。如果其他应用程序打开了同一个摄像头，或者此设备不支持该 hal API 版本，则会抛出 RuntimeException。
    从 Android 12 开始，不再支持 HAL 版本 1。使用完摄像头后，您必须调用 release()，否则它将保持锁定状态，其他应用程序无法使用。
    对于特定的硬件摄像头，您的应用程序一次只能激活一个 Camera 对象。来自其他方法的回调将传递到调用 open() 的线程的事件循环。
    如果此线程没有事件循环，则回调将传递到主应用程序事件循环。如果没有主应用程序事件循环，则不会传递回调。
    注意：在某些设备上，此方法可能需要很长时间才能完成。最好从工作线程（可能使用 android.os.AsyncTask）调用此方法，以避免阻塞主应用程序 UI 线程。
    返回：一个新的 Camera 对象，已连接、已锁定且可供使用。
    抛出：
    - IllegalArgumentException – 如果 halVersion 无效
    - RuntimeException – 如果打开摄像头失败（例如，如果摄像头正在被另一个进程使用或设备策略管理器已禁用摄像头）。
    `camera_id` 要访问的硬件摄像头，介于 0 和 getNumberOfCameras()-1 之间。
    `hal_version` 要打开的此摄像头设备的 HAL API 版本。
    */
    #[java_method]
    pub fn open_legacy(camera_id: i32, hal_version: i32) -> Result<Self> {}

    #[doc(hidden)]
    #[java_method]
    pub fn check_init_errors(err: i32) -> bool {}

    #[doc(hidden)]
    #[java_method]
    pub fn open_uninitialized() -> Self {}

    /**
    断开并释放 Camera 对象资源。您必须在完成 Camera 对象使用后立即调用此方法。
    */
    #[java_method]
    pub fn release(&self) {}

    /**
    解锁摄像头以允许其他进程访问它。通常，摄像头会锁定到具有活动 Camera 对象的进程，直到调用 release()。
    为了允许进程之间快速切换，您可以调用此方法暂时释放摄像头以供其他进程使用；其他进程完成后，您可以调用 reconnect() 来回收摄像头。
    必须在调用 android.media.MediaRecorder.setCamera(Camera) 之前完成此操作。录制开始后无法调用此方法。如果您没有录制视频，则可能不需要此方法。
    抛出：RuntimeException – 如果无法解锁摄像头。
    */
    #[java_method]
    pub fn unlock(&self) -> Result<()> {}

    /**
    重新锁定摄像头以防止其他进程访问它。除非调用 unlock()，否则摄像头对象默认处于锁定状态。通常使用 reconnect()。
    从 API 级别 14 开始，android.media.MediaRecorder.start() 中的应用程序会自动锁定摄像头。应用程序可以在录制开始后使用摄像头（例如：缩放）。录制开始或停止后无需调用此方法。如果您没有录制视频，则可能不需要此方法。
    抛出：RuntimeException – 如果无法重新锁定摄像头（例如，如果另一个进程仍在使用摄像头）。
    */
    #[java_method]
    pub fn lock(&self) -> Result<()> {}

    /**
    在另一个进程使用摄像头服务后重新连接到该服务。调用 unlock() 后，另一个进程可以使用摄像头；当该进程完成后，您必须重新连接到摄像头，这将重新获取锁定并允许您继续使用摄像头。
    从 API 级别 14 开始，android.media.MediaRecorder.start() 中的应用程序会自动锁定摄像头。应用程序可以在录制开始后使用摄像头（例如：缩放）。录制开始或停止后无需调用此方法。如果您没有录制视频，则可能不需要此方法。
    抛出：
    - IOException - 如果无法重新建立连接（例如，如果另一个进程仍在使用摄像头）。
    - RuntimeException - 如果已在此 Camera 实例上调用 release()。
    */
    #[java_method]
    pub fn reconnect(&self) -> Result<()> {}

    /**
    设置用于实时预览的 Surface。预览需要表面或表面纹理，而预览是拍照所必需的。
    可以重新设置相同的表面而不会造成损害。设置预览表面将取消设置通过 setPreviewTexture 设置的任何预览表面纹理。
    调用此方法时，SurfaceHolder 必须已包含表面。如果您使用的是 android.view.SurfaceView，则需要使用 SurfaceHolder.addCallback(SurfaceHolder.Callback) 注册 SurfaceHolder.Callback，并在调用 setPreviewDisplay() 或开始预览之前等待 SurfaceHolder.Callback.surfaceCreated(SurfaceHolder)。
    此方法必须在 startPreview() 之前调用。一个例外是，如果在调用 startPreview() 之前未设置预览表面（或设置为 null），则可以使用非 null 参数调用此方法一次以设置预览表面。 （这允许相机设置和表面创建并行进行，从而节省时间。）预览运行时，预览表面可能不会发生变化。
    抛出:
    - IOException – 如果该方法失败（例如，如果表面不可用或不合适）。
    - RuntimeException – 如果已在此 Camera 实例上调用 release()。
    `holder` – 包含放置预览的表面，或 null 以删除预览表面
    */
    #[java_method]
    pub fn set_preview_display<SH: SurfaceHolder>(&self, holder: Option<SH>) -> Result<()> {}

    /**
    抛出:IOException
    */
    #[java_method]
    pub fn set_preview_surface(&self, surface: &Surface) -> Result<()> {}

    /**
    设置用于实时预览的 SurfaceTexture。预览需要表面或表面纹理，而预览需要拍照。可以重新设置相同的表面纹理而不会造成任何损害。
    设置预览表面纹理将取消设置通过 setPreviewDisplay 设置的任何预览表面。此方法必须在 startPreview() 之前调用。唯一的例外是，如果在调用 startPreview() 之前未设置预览表面纹理（或将其设置为 null），则可以使用非 null 参数调用此方法一次以设置预览表面。（这允许并行进行相机设置和表面创建，从而节省时间。）
    预览表面纹理在预览运行时可能不会发生变化。 SurfaceTexture.getTimestamp() 为设置为预览纹理的 SurfaceTexture 提供的时间戳具有未指定的零点，并且无法直接在不同的相机或同一相机的不同实例之间进行比较，也无法在同一程序的多次运行之间进行比较。
    如果您使用预览数据来创建视频或静态图像，强烈建议使用 android.media.MediaActionSound 来向用户正确指示图像捕获或录制开始/停止。
    抛出:
    - IOException – 如果该方法失败（例如，如果表面纹理不可用或不合适）。
    - RuntimeException – 如果已在此 Camera 实例上调用 release()。
    `surface_texture` 预览图像要发送到的 SurfaceTexture 或 null 以删除当前预览表面纹理
    */
    #[java_method]
    pub fn set_preview_texture(&self, surface_texture: Option<&SurfaceTexture>) -> Result<()> {}

    //noinspection SpellCheckingInspection
    /**
    开始捕获预览帧并将其绘制到屏幕上。在使用 setPreviewDisplay(SurfaceHolder) 或 setPreviewTexture(SurfaceTexture) 提供表面之前，预览不会真正开始。
    如果调用了 setPreviewCallback(Camera.PreviewCallback)、setOneShotPreviewCallback(Camera.PreviewCallback) 或 setPreviewCallbackWithBuffer(Camera.PreviewCallback)，则在预览数据可用时将调用 Camera.PreviewCallback.onPreviewFrame(byte[], Camera)。
    抛出:RuntimeException – 如果启动预览失败；这通常是由于硬件或其他低级错误，或者因为已在此 Camera 实例上调用 release()。setPreviewSize 和 setPictureSize 中提到的 QCIF (176x144) 异常也可能导致抛出此异常。
    */
    #[java_method]
    pub fn start_preview(&self) -> Result<()> {}

    /**
    停止捕获和将预览帧绘制到表面，并重置相机以便将来调用 startPreview()。
    抛出：RuntimeException – 如果停止预览失败；这通常是由于硬件或其他低级错误，或者因为已在此 Camera 实例上调用 release()。
    */
    #[java_method]
    pub fn stop_preview(&self) -> Result<()> {}

    /**
    返回当前预览状态。
    FIXME：发布前取消隐藏
    */
    #[java_method]
    pub fn preview_enabled(&self) -> bool {}

    /**
    安装一个回调，用于在屏幕上显示每个预览帧并调用它们。只要预览处于活动状态，就会重复调用回调。
    此方法可随时调用，即使预览处于活动状态。任何其他预览回调都将被覆盖。
    如果您使用预览数据创建视频或静态图像，强烈建议使用 android.media.MediaActionSound 向用户正确指示图像捕获或录制开始/停止。
    抛出:RuntimeException – 如果已在此 Camera 实例上调用 release()。
    `cb` 一个回调对象，用于接收每个预览帧的副本，或 null 以停止接收回调。
    */
    #[java_method]
    pub fn set_preview_callback<C: Camera_PreviewCallback>(&self, cb: Option<C>) -> Result<()> {}

    /**
    除了在屏幕上显示下一个预览帧外，还安装一个回调函数，用于调用该回调函数来获取下一个预览帧。调用一次后，回调函数将被清除。
    此方法可随时调用，即使预览处于实时状态。任何其他预览回调函数都将被覆盖。
    如果您使用预览数据来创建视频或静态图像，强烈建议使用 android.media.MediaActionSound 来向用户正确指示图像捕获或录制开始/停止。
    抛出:RuntimeException – 如果已在此 Camera 实例上调用 release()。
    `cb` 一个回调对象，用于接收下一个预览帧的副本，或 null 以停止接收回调函数。
    */
    #[java_method]
    pub fn set_one_shot_preview_callback<C: Camera_PreviewCallback>(
        &self,
        cb: Option<C>,
    ) -> Result<()> {
    }

    /**
    安装一个回调函数，使用 addCallbackBuffer(byte[]) 提供的缓冲区为每个预览帧调用该回调函数，并将它们显示在屏幕上。
    只要预览处于活动状态且缓冲区可用，就会重复调用回调函数。任何其他预览回调函数都将被覆盖。
    此方法的目的是通过允许预览帧内存重用来提高预览效率和帧速率。您必须在某个时间点（在调用此方法之前或之后）调用 addCallbackBuffer(byte[])，否则将不会收到任何回调函数。
    如果使用空回调函数调用此方法、调用 setPreviewCallback(Camera.PreviewCallback) 或调用 setOneShotPreviewCallback(Camera.PreviewCallback)，则缓冲区队列将被清除。
    如果您使用预览数据创建视频或静态图像，强烈建议使用 android.media.MediaActionSound 向用户正确指示图像捕获或录制开始/停止。
    抛出：RuntimeException – 如果已在此 Camera 实例上调用 release()。
    `cb` 一个接收预览帧副本的回调对象，或 null 以停止接收回调并清除缓冲区队列。
    */
    #[java_method]
    pub fn set_preview_callback_with_buffer<C: Camera_PreviewCallback>(
        &self,
        cb: Option<C>,
    ) -> Result<()> {
    }

    /**
    将预分配的缓冲区添加到预览回调缓冲区队列。应用程序可以向队列添加一个或多个缓冲区。当预览帧到达且至少有一个可用缓冲区时，将使用该缓冲区并将其从队列中删除。
    然后使用该缓冲区调用预览回调。如果帧到达且没有剩余缓冲区，则丢弃该帧。
    应用程序应在完成处理缓冲区中的数据后将其添加回去。对于 YV12 以外的格式，缓冲区的大小由预览图像宽度、高度和每像素字节数相乘来确定。
    宽度和高度可以从 Camera.Parameters.getPreviewSize() 中读取。可以使用 Camera.Parameters.getPreviewFormat() 中的图像格式，从 ImageFormat.getBitsPerPixel(int)/8 计算每像素字节数。
    如果使用 ImageFormat.YV12 格式，则可以使用 Camera.Parameters.setPreviewFormat 中列出的公式计算大小。仅当使用 setPreviewCallbackWithBuffer(Camera.PreviewCallback) 时才需要此方法。
    使用 setPreviewCallback(Camera.PreviewCallback) 或 setOneShotPreviewCallback(Camera.PreviewCallback) 时，会自动分配缓冲区。当提供的缓冲区太小而无法容纳预览帧数据时，预览回调将返回 null，并且缓冲区将从缓冲区队列中删除。
    `callback_buffer` 是要添加到队列的缓冲区。缓冲区的大小必须与上面描述的值匹配。
    */
    #[java_method]
    pub fn add_callback_buffer(&self, callback_buffer: &[u8]) {}

    /**
    将预分配的缓冲区添加到原始图像回调缓冲区队列。应用程序可以向队列添加一个或多个缓冲区。
    当原始图像帧到达且至少有一个可用缓冲区时，该缓冲区将用于保存原始图像数据并从队列中删除。然后使用缓冲区调用原始图像回调。
    如果原始图像帧到达但没有剩余缓冲区，则丢弃该帧。应用程序应在完成处理缓冲区中的数据后通过再次调用此方法重新添加缓冲区，以避免耗尽原始图像回调缓冲区。
    缓冲区的大小由原始图像宽度、高度和每像素字节数相乘确定。宽度和高度可以从 Camera.Parameters.getPictureSize() 中读取。
    每像素字节数可以从 ImageFormat.getBitsPerPixel(int)/8 计算，使用 Camera.Parameters.getPreviewFormat() 中的图像格式。仅当在调用 takePicture(Camera.ShutterCallback, Camera.PictureCallback, Camera.PictureCallback, Camera.PictureCallback) 时使用原始图像的 PictureCallback 时，才需要此方法。
    请注意，通过调用此方法，将触发应用程序管理的回调缓冲区模式。如果从未调用过此方法，则原始图像回调将返回 null，因为没有可用的图像回调缓冲区。
    此外，当提供的缓冲区太小而无法容纳原始图像数据时，原始图像回调将返回 null，并且缓冲区将从缓冲区队列中删除。
    `callback_buffer` 是要添加到原始图像回调缓冲区队列的缓冲区。大小应为宽度 * 高度 *（每像素位数）/ 8。null 的回调缓冲区将被忽略，不会添加到队列中。
    */
    #[java_method]
    pub fn add_raw_image_callback_buffer(&self, callback_buffer: &[u8]) {}

    /**
    创建 RenderScript 分配以用作预览回调帧的目标。使用 setPreviewCallbackAllocation 将创建的分配用作相机预览帧的目标。
    分配将使用 YUV 类型创建，并且必须使用 rsGetElementAtYuv_* 访问器方法在 RenderScript 中访问其内容。其大小将基于为此相机配置的当前预览大小。
    返回：尺寸等于当前预览大小的新 YUV 类型分配。
    抛出：RSIllegalArgumentException - 如果使用标志与 YUV 分配不兼容。
    `rs` 此分配的 RenderScript 上下文。
    `usage` 为分配设置的其他使用标志。使用标志 Allocation.USAGE_IO_INPUT 将始终设置在创建的分配上，但可以在此处提供其他标志。
    */
    #[java_method]
    pub fn create_preview_allocation(&self, rs: &RenderScript, usage: i32) -> Result<Allocation> {}

    /**
    将 Allocation 设置为预览回调数据的目标。使用此方法可以高效地处理带有 RenderScript 的相机预览数据。
    必须使用 createPreviewAllocation 方法创建 Allocation。设置预览分配将禁用由 setPreviewCallback 或 setPreviewCallbackWithBuffer 设置的任何活动预览回调，反之亦然。
    使用预览分配仍需要设置活动的标准预览目标，使用 setPreviewTexture 或 setPreviewDisplay。要在 Allocation 有新帧可用时收到通知，请使用 Allocation.setIoInputNotificationHandler。
    要将当前可从 Allocation 访问的帧更新为最新的预览帧，请调用 Allocation.ioReceive。要禁用 Allocation 中的预览，请使用 null 参数调用此方法。
    一旦设置了预览分配，setPreviewSize 设置的预览大小就无法更改。如果您希望更改预览大小，请先通过调用 setPreviewCallbackAllocation(null) 删除预览分配，然后更改预览大小，使用 createPreviewAllocation 创建新的预览分配，并将其设置为新的预览回调分配目标。
    如果您使用预览数据创建视频或静态图像，强烈建议使用 android.media.MediaActionSound 向用户正确指示图像捕获或录制开始/停止。
    抛出:IOException – 如果配置相机以使用分配进行预览失败。 IllegalArgumentException – 如果分配的尺寸或其他参数不符合要求。
    `preview_allocation` 用作预览目标的分配
    */
    #[java_method]
    pub fn set_preview_callback_allocation(
        &self,
        preview_allocation: Option<&Allocation>,
    ) -> Result<()> {
    }

    /**
    启动相机自动对焦并注册一个回调函数，以便在相机对焦时运行。此方法仅在预览处于活动状态时有效（在 startPreview() 之间和 stopPreview() 之前）。
    调用者应检查 android.hardware.Camera.Parameters.getFocusMode() 以确定是否应调用此方法。如果相机不支持自动对焦，则为无操作，并且将立即调用 Camera.AutoFocusCallback.onAutoFocus(boolean, Camera) 回调。
    如果您的应用程序不应安装在没有自动对焦功能的设备上，则必须使用 清单元素声明您的应用程序使用自动对焦。如果当前闪光灯模式不是 android.hardware.Camera.Parameters.FLASH_MODE_OFF，则闪光灯可能会在自动对焦期间触发，具体取决于驱动程序和相机硬件。
    自动曝光锁 android.hardware.Camera.Parameters.getAutoExposureLock() 和自动白平衡锁 android.hardware.Camera.Parameters.getAutoWhiteBalanceLock() 在自动对焦期间和之后不会改变。但自动对焦例程可能会在对焦期间暂时停止自动曝光和自动白平衡。
    使用 stopPreview() 停止预览，或使用 takePicture(Camera.ShutterCallback, Camera.PictureCallback, Camera.PictureCallback) 触发静态图像捕获，不会改变焦点位置。应用程序必须调用 cancelAutoFocus 来重置焦点。
    如果自动对焦成功，请考虑使用 android.media.MediaActionSound 向用户正确播放自动对焦成功声音。
    抛出:RuntimeException – 如果启动自动对焦失败；通常是由于硬件或其他低级错误，或者因为已在此 Camera 实例上调用 release()。
    `cb` 要运行的回调
    */
    #[java_method]
    pub fn auto_focus<C: Camera_AutoFocusCallback>(&self, cb: &C) -> Result<()> {}

    /**
    取消正在进行的任何自动对焦功能。无论自动对焦当前是否正在进行，此函数都会将焦点位置返回到默认值。
    如果相机不支持自动对焦，则此操作无效。
    抛出：RuntimeException – 如果取消自动对焦失败；这通常是由于硬件或其他低级错误，或者因为已在此 Camera 实例上调用 release()。
    */
    #[java_method]
    pub fn cancel_auto_focus(&self) -> Result<()> {}

    /**
    设置相机自动对焦移动回调。
    抛出：RuntimeException – 如果启用对焦移动回调失败；这通常是由于硬件或其他低级错误，或者因为已在此 Camera 实例上调用 release()。
    `cb` 要运行的回调
    */
    #[java_method]
    pub fn set_auto_focus_move_callback<C: Camera_AutoFocusMoveCallback>(&self, cb: &C) {}

    /**
    相当于 takePicture(Shutter, raw, null, jpeg)。
    */
    #[java_method(overload = takePicture)]
    pub fn take_picture_convenience<C: Camera_ShutterCallback, D: Camera_PictureCallback>(
        &self,
        shutter: Option<&C>,
        raw: Option<&D>,
        jpeg: Option<&D>,
    ) -> Result<()> {
    }

    /**
    触发异步图像捕获。相机服务将在图像捕获过程中向应用程序发起一系列回调。
    快门回调发生在图像捕获后。这可用于触发声音，让用户知道图像已捕获。
    原始回调发生在原始图像数据可用时（注意：如果没有可用的原始图像回调缓冲区或原始图像回调缓冲区不足以容纳原始图像，则数据将为空）。
    后视图回调发生在缩放的、完全处理的后视图图像可用时（注意：并非所有硬件都支持此功能）。
    jpeg 回调发生在压缩图像可用时。如果应用程序不需要特定的回调，则可以传递 null 而不是回调方法。
    此方法仅在预览处于活动状态时有效（在 startPreview() 之后）。拍摄图像后将停止预览；如果调用者想要重新启动预览或拍摄更多照片，则必须再次调用 startPreview()。
    不应在 android.media.MediaRecorder.start() 和 android.media.MediaRecorder.stop() 之间调用此方法。调用此方法后，在 JPEG 回调返回之前，您不得调用 startPreview() 或拍摄另一张照片。
    抛出:RuntimeException – 如果开始图片捕获失败；这通常是由于硬件或其他低级错误，或者因为已在此 Camera 实例上调用 release()。
    `shutter` 图像捕获时刻的回调，或为 null
    `raw` 原始（未压缩）图像数据的回调，或为 null
    `post_view` 带有 post_view 图像数据的回调，可能为 null
    `jpeg` JPEG 图像数据的回调，或为 null
    */
    #[java_method]
    pub fn take_picture<C: Camera_ShutterCallback, D: Camera_PictureCallback>(
        &self,
        shutter: Option<&C>,
        raw: Option<&D>,
        post_view: Option<&D>,
        jpeg: Option<&D>,
    ) -> Result<()> {
    }

    /**
    平稳缩放到请求的值。驱动程序将通知 Camera.OnZoomChangeListener 缩放值以及缩放是否在此时停止。
    例如，假设当前缩放为 0，并使用值 3 调用 startSmoothZoom。Camera.OnZoomChangeListener.onZoomChange(int, boolean, Camera) 方法将被调用三次，缩放值分别为 1、2 和 3。
    应用程序可以调用 stopSmoothZoom 以提前停止缩放。应用程序不应在缩放停止前再次调用 startSmoothZoom 或更改缩放值。
    如果提供的缩放值等于当前缩放值，则不会生成缩放回调。如果 android.hardware.Camera.Parameters.isSmoothZoomSupported 返回 true，则支持此方法。
    抛出：
    - IllegalArgumentException – 如果缩放值无效。
    - RuntimeException – 如果方法失败。
    `value` 缩放值。有效范围为 0 到 android.hardware.Camera.Parameters.getMaxZoom。
    */
    #[java_method]
    pub fn start_smooth_zoom(&self, value: i32) -> Result<()> {}

    /**
    停止平滑缩放。应用程序应等待 Camera.OnZoomChangeListener 来获知缩放何时真正停止。
    如果 android.hardware.Camera.Parameters.isSmoothZoomSupported 为 true，则支持此方法。
    抛出:RuntimeException – 如果该方法失败。
    */
    #[java_method]
    pub fn stop_smooth_zoom(&self) -> Result<()> {}

    /**
    设置预览显示的顺时针旋转度数。这会影响预览帧和快照后显示的图片。此方法对于人像模式应用很有用。
    请注意，前置摄像头的预览显示在旋转之前会水平翻转，即图像沿摄像头传感器的中心垂直轴反射。因此用户可以看到自己就像在看镜子一样。
    这不会影响在 Camera.PreviewCallback.onPreviewFrame、JPEG 图片或录制的视频中传递的字节数组的顺序。不允许在预览期间调用此方法。
    如果要使相机图像以与显示屏相同的方向显示，可以使用以下代码。
    ```java
    public static void setCameraDisplayOrientation(Activity activity, int cameraId, android.hardware.Camera camera) {
        android.hardware.Camera.CameraInfo info = new android.hardware.Camera.CameraInfo();
        android.hardware.Camera.getCameraInfo(cameraId, info);
        int rotation = activity.getWindowManager().getDefaultDisplay().getRotation();
        int degrees = 0;
        switch (rotation) {
          case Surface.ROTATION_0:
            degrees = 0;
            break;
          case Surface.ROTATION_90:
            degrees = 90;
            break;
          case Surface.ROTATION_180:
            degrees = 180;
            break;
          case Surface.ROTATION_270:
            degrees = 270;
            break;
        }
        int result;
        if (info.facing == Camera.CameraInfo.CAMERA_FACING_FRONT) {
            result = (info.orientation + degrees) % 360;
            result = (360 - result) % 360;  // 补偿镜子
        } else {
            // 背面
            result = (info.orientation - degrees + 360) % 360;
        }
        camera.setDisplayOrientation(result);
    }
    ```
    从 API 级别 14 开始，可以在预览处于活动状态时调用此方法。注意：在 API 级别 24 之前，方向的默认值为 0。
    从 API 级别 24 开始，默认方向将是这样的，即强制横向模式下的应用程序将具有正确的预览方向，该方向可以是默认值 0 或 180。在纵向模式下运行或允许更改方向的应用程序仍必须在每次方向更改后调用此方法，以确保在所有情况下都能正确显示预览。
    抛出：RuntimeException - 如果设置方向失败；通常是由于硬件或其他低级错误，或者因为已在此 Camera 实例上调用 release()。
    `degrees` 图片顺时针旋转的角度。有效值为 0、90、180 和 270。
    */
    #[java_method]
    pub fn set_display_orientation(&self, degrees: i32) -> Result<()> {}

    /**
    启用或禁用拍照时的默认快门声音。默认情况下，调用 takePicture 时，相机会播放系统定义的相机快门声音。
    使用此方法可以禁用快门声音。强烈建议在禁用系统快门声音时在 Camera.ShutterCallback 中播放替代快门声音。
    请注意，设备可能并不总是允许禁用相机快门声音。如果无法将快门声音状态设置为所需值，则此方法将返回 false。
    Camera.CameraInfo.canDisableShutterSound 可用于确定设备是否允许禁用快门声音。
    返回：如果成功更改了快门声音状态，则返回 true。如果无法更改快门声音状态，则返回 false。如果快门声音播放已设置为请求的状态，也会返回 true。
    抛出：RuntimeException – 如果调用失败；通常是由于硬件或其他低级错误，或者因为已在此 Camera 实例上调用 release()。
    `enabled` 调用 takePicture 时相机是否应播放系统快门声音。
    */
    #[java_method]
    pub fn enable_shutter_sound(&self, enabled: bool) -> Result<bool> {}

    /**
    无条件禁用快门声音。这只保证适用于旧式相机（即使用 cameraInitUnspecified 初始化的相机）。
    尝试在普通相机上调用此功能将强制在相机服务中进行条件检查。
    返回：如果成功更改了快门声音状态，则返回 true。如果无法更改快门声音状态，则返回 false。如果快门声音播放已设置为请求的状态，也会返回 true。
    */
    #[java_method]
    pub fn disable_shutter_sound(&self) -> bool {}

    /**
    注册一个监听器，以便在平滑变焦期间相机驱动程序更新变焦值时收到通知。
    `listener` 要通知的监听器
    */
    #[java_method]
    pub fn set_zoom_change_listener<L: Camera_OnZoomChangeListener>(&self, listener: &L) {}

    /**
    注册一个侦听器，以便接收有关预览帧中检测到的面部的通知。
    `listener` 要通知的侦听器
    */
    #[java_method]
    pub fn set_face_detection_listener<L: Camera_FaceDetectionListener>(&self, listener: &L) {}

    /**
    开始人脸检测。应在预览启动后调用此方法。相机将通知 Camera.FaceDetectionListener 预览帧中检测到的人脸。
    检测到的人脸可能与之前的相同。应用程序应调用 stopFaceDetection 来停止人脸检测。如果 Camera.Parameters.getMaxNumDetectedFaces() 返回大于 0 的数字，则支持此方法。
    如果人脸检测已启动，应用程序不应再次调用此方法。 当人脸检测正在运行时，Camera.Parameters.setWhiteBalance(String)、Camera.Parameters.setFocusAreas(List) 和 Camera.Parameters.setMeteringAreas(List) 不起作用。
    相机使用检测到的人脸进行自动白平衡、自动曝光和自动对焦。 如果应用程序调用 autoFocus(Camera.AutoFocusCallback)，相机将停止发送人脸回调。
    最后一个人脸回调指示用于进行自动对焦的区域。对焦完成后，人脸检测将恢复发送人脸回调。如果应用调用 cancelAutoFocus()，人脸回调也将恢复。
    调用 takePicture(Camera.ShutterCallback, Camera.PictureCallback, Camera.PictureCallback) 或 stopPreview() 后，再使用 startPreview() 恢复预览，应用应再次调用此方法以恢复人脸检测。
    抛出:
    - IllegalArgumentException – 如果不支持人脸检测。
    - RuntimeException – 如果方法失败或人脸检测已在运行。
    */
    #[java_method]
    pub fn start_face_detection(&self) -> Result<()> {}

    /**
    停止人脸检测。
    */
    #[java_method]
    pub fn stop_face_detection(&self) {}

    /**
    注册一个在发生错误时调用的回调。
    `cb` 要运行的回调
    */
    #[java_method]
    pub fn set_error_callback<C: Camera_ErrorCallback>(&self, cb: &C) {}

    /**
    注册一个在发生错误时调用的回调。详细的错误回调可能包含错误代码，提供有关错误的更多详细信息。设置详细回调后，通过 #setErrorCallback(ErrorCallback) 设置的回调将停止接收 onError 调用。
    `cb` 要运行的回调
    */
    #[java_method]
    pub fn set_detailed_error_callback<C: Camera_ErrorCallback>(&self, cb: &C) {}

    /**
    更改此相机服务的设置。
    抛出:RuntimeException – 如果任何参数无效或不受支持。
    `params` 此相机服务要使用的参数
    */
    #[java_method]
    pub fn set_parameters(&self, params: Camera_Parameters) -> Result<()> {}

    /**
    返回此相机服务的当前设置。如果对返回的参数进行了修改，则必须将其传递给 setParameters(Camera.Parameters) 才能生效。
    抛出:RuntimeException – 如果读取参数失败；这通常是由于硬件或其他低级错误，或者因为已在此相机实例上调用 release()。
    */
    #[java_method]
    pub fn get_parameters(&self) -> Result<Camera_Parameters> {}

    /**
    返回一个空的 Camera.Parameters 以用于测试目的。
    返回：一个参数对象。
    */
    #[java_method]
    pub fn get_empty_parameters() -> Camera_Parameters {}

    /**
    返回一个复制的 Camera.Parameters；仅供 shim 使用。
    返回：一个 Parameter 对象，所有参数均从参数中复制而来。
    抛出：NullPointerException – 如果参数为空
    `parameters` 一个非空参数
    */
    #[java_method]
    pub fn get_parameters_copy(parameters: &Camera_Parameters) -> Result<Camera_Parameters> {}

    /**
    设置相机音频限制模式。
    */
    #[java_method]
    pub fn set_audio_restriction(&self, mode: i32) {}

    /**
    获取当前应用的相机音频限制模式。
    */
    #[java_method]
    pub fn get_audio_restriction(&self) -> i32 {}
}

/**
用于在显示预览帧时提供其副本的回调接口。
*/
#[allow(non_camel_case_types)]
#[deprecated(note = "我们建议对新应用程序使用新的 android.hardware.camera2 API。")]
#[java_interface(name = "android/hardware/Camera$PreviewCallback")]
pub trait Camera_PreviewCallback {
    /**
    在显示预览帧时调用。此回调在调用 open(int) 的事件线程上调用。
    如果使用 ImageFormat.YV12 格式，请参阅 Camera.Parameters.setPreviewFormat 中的公式，了解预览回调缓冲区中像素数据的排列。
    `data` 是预览帧的内容，格式由 ImageFormat 定义，可以使用 android.hardware.Camera.Parameters.getPreviewFormat() 进行查询。如果从未调用 android.hardware.Camera.Parameters.setPreviewFormat(int)，则默认为 YCbCr_420_SP (NV21) 格式。
    `camera` 是相机服务对象。
    */
    fn on_preview_frame(&self, data: &[u8], camera: &Camera);
}

/**
用于通知相机自动对焦完成的回调接口。不支持自动对焦的设备将收到对此接口的“假”回调。
如果您的应用需要自动对焦，并且不应安装在没有自动对焦的设备上，则您必须在 清单元素中声明您的应用使用 android.hardware.camera.autofocus 功能。
*/
#[allow(non_camel_case_types)]
#[deprecated(note = "我们建议对新应用使用新的 android.hardware.camera2 API。")]
#[java_interface(name = "android/hardware/Camera$AutoFocusCallback")]
pub trait Camera_AutoFocusCallback {
    /**
    当相机自动对焦完成时调用。如果相机不支持自动对焦并且调用了 autoFocus，则将立即调用 onAutoFocus，并将 false 值 success 设置为 true。
    自动对焦例程完成后不会锁定自动曝光和自动白平衡。
    `success` 如果对焦成功，则为 true，否则为 false
    `camera` 相机服务对象
    */
    fn on_auto_focus(&self, success: bool, camera: &Camera);
}

/**
用于通知自动对焦开始和停止的回调接口。这仅在连续自动对焦模式下受支持 - Camera.Parameters.FOCUS_MODE_CONTINUOUS_VIDEO 和 Camera.Parameters.FOCUS_MODE_CONTINUOUS_PICTURE。
应用程序可以在此基础上显示自动对焦动画。
*/
#[allow(non_camel_case_types)]
#[deprecated(note = "我们建议新应用程序使用新的 android.hardware.camera2 API。")]
#[java_interface(name = "android/hardware/Camera$AutoFocusMoveCallback")]
pub trait Camera_AutoFocusMoveCallback {
    /**
    当相机自动对焦开始或停止时调用。
    `start` 如果焦点开始移动则为 true，如果焦点停止移动则为 false
    `camera` 相机服务对象
    */
    fn on_auto_focus_moving(&self, start: bool, camera: &Camera);
}

/**
用于发出实际图像捕获时刻信号的回调接口。
*/
#[allow(non_camel_case_types)]
#[deprecated(note = "我们建议在新应用中使用新的 android.hardware.camera2 API。")]
#[java_interface(name = "android/hardware/Camera$ShutterCallback")]
pub trait Camera_ShutterCallback {
    /**
    尽可能在照片从传感器捕获时调用。这是播放快门声音或提供相机操作的其他反馈的好机会。
    这可能发生在照片触发后的某个时间，但在实际数据可用之前的某个时间。
    */
    fn on_shutter(&self);
}

/**
用于提供照片捕获图像数据的回调接口。
*/
#[deprecated(note = "我们建议在新应用程序中使用新的 android.hardware.camera2 API。")]
#[allow(non_camel_case_types)]
#[java_interface(name = "android/hardware/Camera$PictureCallback")]
pub trait Camera_PictureCallback {
    /**
    拍摄照片后，当图像数据可用时调用。数据的格式取决于回调上下文和 Camera.Parameters 设置。
    `data` 图像数据的字节数组 camera – Camera 服务对象
    */
    fn on_picture_taken(&self, data: &[u8], camera: &Camera);
}

/**
平滑缩放操作期间缩放变化的回调接口。
*/
#[allow(non_camel_case_types)]
#[deprecated(note = "我们建议新应用使用新的 android.hardware.camera2 API。")]
#[java_interface(name = "android/hardware/Camera$OnZoomChangeListener")]
pub trait Camera_OnZoomChangeListener {
    /**
    在平滑缩放期间缩放值发生变化时调用。
    `zoom_value` 当前缩放值。在平滑缩放模式下，相机会为每个新的 缩放值调用此函数。
    `stopped` 平滑缩放是否停止。如果值为 true，则这是应用程序的-最后一次缩放更新。
    `camera` 相机服务对象
    */
    fn on_zoom_change(&self, zoom_value: i32, stopped: bool, camera: &Camera);
}

/**
预览框中检测到人脸的回调接口。
*/
#[allow(non_camel_case_types)]
#[deprecated(note = "我们建议新应用使用新的 android.hardware.camera2 API。")]
#[java_interface(name = "android/hardware/Camera$FaceDetectionListener")]
pub trait Camera_FaceDetectionListener {
    /**
    通知侦听器在预览帧中检测到的脸部。
    `faces` 列表中检测到的脸部
    `camera` 相机服务对象
    */
    fn on_face_detection(&self, faces: &[Camera_Face], camera: &Camera);
}

/**
通过相机人脸检测识别的人脸信息。当使用相机进行人脸检测时，Camera.FaceDetectionListener 将返回用于对焦和测光的人脸对象列表。
*/
#[allow(non_camel_case_types)]
#[deprecated(note = "我们建议在新应用中使用新的 android.hardware.camera2 API。")]
#[java_class(name = "android/hardware/Camera$Face")]
pub struct Camera_Face;

impl Camera_Face {
    /**
    創建一個空的臉。
    */
    #[java_constructor]
    pub fn new() -> Self {}

    /**
    获取面部的边界。(-1000, -1000) 表示相机视野的左上角，(1000, 1000) 表示视野的右下角。
    例如，假设取景器 UI 的大小为 800x480。从驱动程序传递的矩形为 (-1000, -1000, 0, 0)。相应的取景器矩形应为 (0, 0, 400, 240)。保证左 < 右且上 < 下。
    坐标可以小于 -1000 或大于 1000。但至少一个顶点位于 (-1000, -1000) 和 (1000, 1000) 之间。
    方向相对于传感器方向，即传感器所见的方向。方向不受 setDisplayOrientation(int) 的旋转或镜像的影响。
    面部边界矩形不提供任何有关面部方向的信息。这是将驱动程序坐标转换为像素中的视图坐标的矩阵。
    ```java
    Matrix matrix = new Matrix();
    CameraInfo info = CameraHolder.instance().getCameraInfo()[cameraId];
    // 前置摄像头需要镜子。
    boolean mirror = (info.facing == CameraInfo.CAMERA_FACING_FRONT);
    matrix.setScale(mirror ? -1 : 1, 1);
    // 这是 android.hardware.Camera.setDisplayOrientation 的值。
    matrix.postRotate(displayOrientation);
    // 摄像头驱动程序坐标范围从 (-1000, -1000) 到 (1000, 1000)。
    // UI 坐标范围从 (0, 0) 到 (width, height)。
    matrix.postScale(view.getWidth()/2000f，view.getHeight()/2000f);
    matrix.postTranslate(view.getWidth()/2f，view.getHeight()/2f);
    ```
    */
    #[java_field]
    pub fn get_rect(&self) -> Rect {}

    /**
    设置面部的边界。(-1000, -1000) 表示相机视野的左上角，(1000, 1000) 表示视野的右下角。
    例如，假设取景器 UI 的大小为 800x480。从驱动程序传递的矩形为 (-1000, -1000, 0, 0)。相应的取景器矩形应为 (0, 0, 400, 240)。保证左 < 右且上 < 下。
    坐标可以小于 -1000 或大于 1000。但至少一个顶点位于 (-1000, -1000) 和 (1000, 1000) 之间。
    方向相对于传感器方向，即传感器所见的方向。方向不受 setDisplayOrientation(int) 的旋转或镜像的影响。
    面部边界矩形不提供任何有关面部方向的信息。这是将驱动程序坐标转换为像素中的视图坐标的矩阵。
    ```java
    Matrix matrix = new Matrix();
    CameraInfo info = CameraHolder.instance().getCameraInfo()[cameraId];
    // 前置摄像头需要镜子。
    boolean mirror = (info.facing == CameraInfo.CAMERA_FACING_FRONT);
    matrix.setScale(mirror ? -1 : 1, 1);
    // 这是 android.hardware.Camera.setDisplayOrientation 的值。
    matrix.postRotate(displayOrientation);
    // 摄像头驱动程序坐标范围从 (-1000, -1000) 到 (1000, 1000)。
    // UI 坐标范围从 (0, 0) 到 (width, height)。
    matrix.postScale(view.getWidth()/2000f，view.getHeight()/2000f);
    matrix.postTranslate(view.getWidth()/2f，view.getHeight()/2f);
    ```
    */
    #[java_field]
    pub fn set_rect(&self, value: &Rect) {}

    /**
    获取人脸检测的置信度。范围是 1 到 100。100 表示置信度最高。
    根据设备的不同，甚至可能列出置信度非常低的人脸，因此应用程序应根据用例过滤掉置信度低的人脸。
    对于希望在检测到的人脸周围显示矩形的典型傻瓜相机应用程序，建议过滤掉置信度低于 50 的人脸。
    */
    #[java_field]
    pub fn get_score(&self) -> i32 {}

    /**
    设置人脸检测的置信度。范围是 1 到 100。100 表示置信度最高。
    根据设备的不同，甚至可能列出置信度非常低的人脸，因此应用程序应根据用例过滤掉置信度低的人脸。
    对于希望在检测到的人脸周围显示矩形的典型傻瓜相机应用程序，建议过滤掉置信度低于 50 的人脸。
    */
    #[java_field]
    pub fn set_score(&self, value: i32) {}

    /**
    当人脸对跟踪器可见时，每个人脸都有一个唯一的 ID。如果人脸离开视野后又回来，它将获得一个新的 ID。
    这是一个可选字段，可能并非所有设备都支持。如果不支持，ID 将始终设置为 -1。可选字段作为一组支持。要么它们全部有效，要么它们都无效。
    */
    #[java_field(default_value = -1)]
    pub fn get_id(&self) -> i32 {}

    /**
    当人脸对跟踪器可见时，每个人脸都有一个唯一的 ID。如果人脸离开视野后又回来，它将获得一个新的 ID。
    这是一个可选字段，可能并非所有设备都支持。如果不支持，ID 将始终设置为 -1。可选字段作为一组支持。要么它们全部有效，要么它们都无效。
    */
    #[java_field]
    pub fn set_id(&self, value: i32) {}

    /**
    获取左眼中心的坐标。坐标与矩形的坐标位于同一空间。这是一个可选字段，可能并非所有设备都支持。如果不支持，则该值将始终设置为 null。可选字段以集合形式受支持。要么全部有效，要么全部无效。
    */
    #[java_field(default_value = None)]
    pub fn get_left_eye(&self) -> Option<Point> {}

    /**
    设置左眼中心的坐标。坐标与矩形的坐标位于同一空间。这是一个可选字段，可能并非所有设备都支持。如果不支持，则该值将始终设置为 null。可选字段以集合形式受支持。要么全部有效，要么全部无效。
    */
    #[java_field]
    pub fn set_left_eye(&self, value: Option<&Point>) {}

    /**
    获取右眼中心的坐标。该坐标与矩形的坐标位于同一空间。这是一个可选字段，可能并非所有设备都支持。如果不支持，则该值将始终设置为 null。可选字段以集合形式受支持。要么全部有效，要么全部无效。
    */
    #[java_field(default_value = None)]
    pub fn get_right_eye(&self) -> Option<Point> {}

    /**
    设置右眼中心的坐标。该坐标与矩形的坐标位于同一空间。这是一个可选字段，可能并非所有设备都支持。如果不支持，则该值将始终设置为 null。可选字段以集合形式受支持。要么全部有效，要么全部无效。
    */
    #[java_field]
    pub fn set_right_eye(&self, value: Option<&Point>) {}

    /**
    获取嘴部中心的坐标。坐标与矩形的坐标位于同一空间。这是一个可选字段，可能并非所有设备都支持。如果不支持，则该值将始终设置为 null。可选字段以集合形式受支持。要么全部有效，要么全部无效。
    */
    #[java_field(default_value = None)]
    pub fn get_mouth(&self) -> Option<Point> {}

    /**
    设置嘴部中心的坐标。坐标与矩形的坐标位于同一空间。这是一个可选字段，可能并非所有设备都支持。如果不支持，则该值将始终设置为 null。可选字段以集合形式受支持。要么全部有效，要么全部无效。
    */
    #[java_field]
    pub fn set_mouth(&self, value: Option<&Point>) {}
}

/**
相机错误通知的回调接口。
*/
#[deprecated(note = "我们建议新应用使用新的android.hardware.camera2 API。")]
#[allow(non_camel_case_types)]
#[java_interface(name = "android/hardware/Camera$ErrorCallback")]
pub trait Camera_ErrorCallback {
    /**
    相机错误回调。
    `error` 错误代码：CAMERA_ERROR_UNKNOWN、CAMERA_ERROR_SERVER_DIED
    `camera` 相机服务对象
    */
    fn on_error(&self, error: i32, camera: &Camera);
}

/**
相机服务设置。要使相机参数生效，应用程序必须调用 setParameters(Camera.Parameters)。
例如，在调用 setWhiteBalance 后，直到使用更改后的参数对象调用 setParameters(Camera.Parameters)，白平衡才会真正改变。不同的设备可能具有不同的相机功能，例如图片大小或闪光模式。
应用程序应在设置参数之前查询相机功能。例如，应用程序应在调用 setColorEffect(String) 之前调用 getSupportedColorEffects()。
如果相机不支持色彩效果，getSupportedColorEffects() 将返回 null。
*/
#[allow(non_camel_case_types)]
#[deprecated(note = "我们建议新应用程序使用新的 android.hardware.camera2 API。")]
#[java_class(name = "android/hardware/Camera$Parameters")]
pub struct Camera_Parameters;

impl Camera_Parameters {
    // 白平衡设置的值。
    #[doc(hidden)]
    pub const WHITE_BALANCE_AUTO: &'static str = "auto";
    #[doc(hidden)]
    pub const WHITE_BALANCE_INCANDESCENT: &'static str = "incandescent";
    #[doc(hidden)]
    pub const WHITE_BALANCE_FLUORESCENT: &'static str = "fluorescent";
    #[doc(hidden)]
    pub const WHITE_BALANCE_WARM_FLUORESCENT: &'static str = "warm-fluorescent";
    #[doc(hidden)]
    pub const WHITE_BALANCE_DAYLIGHT: &'static str = "daylight";
    #[doc(hidden)]
    pub const WHITE_BALANCE_CLOUDY_DAYLIGHT: &'static str = "cloudy-daylight";
    #[doc(hidden)]
    pub const WHITE_BALANCE_TWILIGHT: &'static str = "twilight";
    #[doc(hidden)]
    pub const WHITE_BALANCE_SHADE: &'static str = "shade";

    // 颜色效果设置的值。
    #[doc(hidden)]
    pub const EFFECT_NONE: &'static str = "none";
    #[doc(hidden)]
    pub const EFFECT_MONO: &'static str = "mono";
    #[doc(hidden)]
    pub const EFFECT_NEGATIVE: &'static str = "negative";
    #[doc(hidden)]
    pub const EFFECT_SOLARIZE: &'static str = "solarize";
    #[doc(hidden)]
    pub const EFFECT_SEPIA: &'static str = "sepia";
    #[doc(hidden)]
    pub const EFFECT_POSTERIZE: &'static str = "posterize";
    #[doc(hidden)]
    pub const EFFECT_WHITEBOARD: &'static str = "whiteboard";
    #[doc(hidden)]
    pub const EFFECT_BLACKBOARD: &'static str = "blackboard";
    #[doc(hidden)]
    pub const EFFECT_AQUA: &'static str = "aqua";

    //noinspection SpellCheckingInspection
    // 抗带设置的值。
    #[doc(hidden)]
    pub const ANTIBANDING_AUTO: &'static str = "auto";
    //noinspection SpellCheckingInspection
    #[doc(hidden)]
    pub const ANTIBANDING_50HZ: &'static str = "50hz";
    //noinspection SpellCheckingInspection
    #[doc(hidden)]
    pub const ANTIBANDING_60HZ: &'static str = "60hz";
    //noinspection SpellCheckingInspection
    #[doc(hidden)]
    pub const ANTIBANDING_OFF: &'static str = "off";

    // 闪光模式设置的值。
    /// 不会发射 Flash。
    pub const FLASH_MODE_OFF: &'static str = "off";

    /// 需要时闪光灯会自动闪光。根据驱动程序的不同，闪光灯可能在预览、自动对焦或快照时闪光。
    pub const FLASH_MODE_AUTO: &'static str = "auto";

    /// 拍摄快照时闪光灯始终会亮起。根据驱动程序的不同，预览或自动对焦时闪光灯也可能亮起。
    pub const FLASH_MODE_ON: &'static str = "on";

    /// 在防红眼模式下，闪光灯将会闪光。
    pub const FLASH_MODE_RED_EYE: &'static str = "red-eye";

    /// 预览、自动对焦和快照时持续发光。这也可以用于视频录制。
    pub const FLASH_MODE_TORCH: &'static str = "torch";

    /// 场景模式已关闭。
    pub const SCENE_MODE_AUTO: &'static str = "auto";

    /// 拍摄快速移动的物体。与 SCENE_MODE_SPORTS 相同。
    pub const SCENE_MODE_ACTION: &'static str = "action";

    /// 拍摄人物照片。
    pub const SCENE_MODE_PORTRAIT: &'static str = "portrait";

    /// 拍摄远处的物体。
    pub const SCENE_MODE_LANDSCAPE: &'static str = "landscape";

    /// 晚上拍照。
    pub const SCENE_MODE_NIGHT: &'static str = "night";

    /// 晚上拍摄人物照片。
    pub const SCENE_MODE_NIGHT_PORTRAIT: &'static str = "night-portrait";

    /// 在剧院拍照。闪光灯已关闭。
    pub const SCENE_MODE_THEATRE: &'static str = "theatre";

    /// 在海滩上拍照。
    pub const SCENE_MODE_BEACH: &'static str = "beach";

    /// 在雪地上拍照。
    pub const SCENE_MODE_SNOW: &'static str = "snow";

    /// 拍摄日落照片。
    pub const SCENE_MODE_SUNSET: &'static str = "sunset";

    //noinspection SpellCheckingInspection
    /// 避免照片模糊（例如由于手抖）。
    pub const SCENE_MODE_STEADYPHOTO: &'static str = "steadyphoto";

    /// 用于拍摄烟花表演。
    pub const SCENE_MODE_FIREWORKS: &'static str = "fireworks";

    /// 拍摄快速移动的物体。与 SCENE_MODE_ACTION 相同。
    pub const SCENE_MODE_SPORTS: &'static str = "sports";

    /// 拍摄室内低光照片。
    pub const SCENE_MODE_PARTY: &'static str = "party";

    /// 捕捉烛光照亮的场景的自然温暖色彩。
    pub const SCENE_MODE_CANDLELIGHT: &'static str = "candlelight";

    /// 应用程序正在寻找条形码。摄像头驱动程序将针对条形码读取进行优化。
    pub const SCENE_MODE_BARCODE: &'static str = "barcode";

    /// 使用高动态范围成像技术捕捉场景。相机将返回与常规捕捉相比具有扩展动态范围的图像。捕捉这样的图像可能比常规捕捉花费更长的时间。
    pub const SCENE_MODE_HDR: &'static str = "hdr";

    /// 自动对焦模式。应用程序应调用 autoFocus(AutoFocusCallback) 以在此模式下启动对焦。
    pub const FOCUS_MODE_AUTO: &'static str = "auto";

    /// 焦点设置在无穷远处。在此模式下，应用程序不应调用 autoFocus(AutoFocusCallback)。
    pub const FOCUS_MODE_INFINITY: &'static str = "infinity";

    /// 微距（特写）对焦模式。应用程序应调用 autoFocus(AutoFocusCallback) 以在此模式下启动对焦。
    pub const FOCUS_MODE_MACRO: &'static str = "macro";

    /// 焦点是固定的。如果焦点不可调节，则相机始终处于此模式。如果相机具有自动对焦，则此模式可以修复焦点，该焦点通常在高焦距距离处。应用程序不应在此模式下调用AutoFocus（AutoFocusCallback）。
    pub const FOCUS_MODE_FIXED: &'static str = "fixed";

    //noinspection SpellCheckingInspection
    /// 扩展景深 (EDOF)。聚焦以数字方式连续完成。应用程序不应在此模式下调用 autoFocus(AutoFocusCallback)。
    pub const FOCUS_MODE_EDOF: &'static str = "edof";

    /**
    连续自动对焦模式适用于视频录制。相机会连续尝试对焦。这是视频录制的最佳选择，因为焦点变化平稳。
    在此模式下，应用程序仍然可以调用 takePicture(Camera.ShutterCallback, Camera.PictureCallback, Camera.PictureCallback)，但拍摄对象可能未对焦。设置参数后，自动对焦开始。
    从 API 级别 14 开始，应用程序可以在此模式下调用 autoFocus(AutoFocusCallback)。对焦回调将立即返回一个布尔值，指示对焦是否清晰。调用 autoFocus 后，对焦位置被锁定。
    如果应用程序想要恢复连续对焦，必须调用 cancelAutoFocus。重新启动预览不会恢复连续自动对焦。要停止连续对焦，应用程序应将对焦模式更改为其他模式。
    */
    pub const FOCUS_MODE_CONTINUOUS_VIDEO: &'static str = "continuous-video";

    /**
    连续自动对焦模式，用于拍照。相机连续尝试对焦。对焦变化速度比 FOCUS_MODE_CONTINUOUS_VIDEO 更快。设置该参数后，自动对焦开始。
    应用程序可以在此模式下调用 autoFocus(AutoFocusCallback)。如果自动对焦正在扫描中，则对焦回调将在扫描完成后返回。如果自动对焦未扫描，则对焦回调将立即返回一个布尔值，该布尔值指示对焦是否清晰。
    然后，应用程序可以决定是立即拍照还是将对焦模式更改为自动，并运行完整的自动对焦周期。调用 autoFocus 后，对焦位置将被锁定。如果应用程序想要恢复连续对焦，则必须调用 cancelAutoFocus。重新启动预览不会恢复连续自动对焦。
    要停止连续对焦，应用程序应将对焦模式更改为其他模式。
    */
    pub const FOCUS_MODE_CONTINUOUS_PICTURE: &'static str = "continuous-picture";

    // 焦距数组的索引。
    /// 与 getFocusDistances(float[]) 一起使用的近焦距数组索引。
    pub const FOCUS_DISTANCE_NEAR_INDEX: i32 = 0;

    /// 与 getFocusDistances(float[]) 一起使用的最佳焦距数组索引。
    pub const FOCUS_DISTANCE_OPTIMAL_INDEX: i32 = 1;

    /// 与 getFocusDistances(float[]) 一起使用的远焦距离数组索引。
    pub const FOCUS_DISTANCE_FAR_INDEX: i32 = 2;

    /// 与 getPreviewFpsRange(int[]) 或 getSupportedPreviewFpsRange() 一起使用的最小预览 fps 的数组索引。
    pub const PREVIEW_FPS_MIN_INDEX: i32 = 0;

    /// 用于“getPreviewFpsRange(int[])”或“getSupportedPreviewFpsRange()”的最大预览 fps 的数组索引。
    pub const PREVIEW_FPS_MAX_INDEX: i32 = 1;

    /**
    使用其他参数的副本覆盖现有参数。仅供旧版 shim 使用。
    */
    #[java_method]
    pub fn copy_from(&self, other: &Self) {}

    /**
    值相等性检查。
    */
    #[java_method]
    pub fn same(&self, other: &Self) -> bool {}

    /**
    将当前参数写入日志。
    */
    #[deprecated]
    #[java_method]
    pub fn dump(&self) {}

    /**
    创建一个包含此参数对象中设置的所有参数的字符串。unflatten(String) 方法执行相反的操作。
    返回：包含此参数对象的所有值的字符串，以分号分隔的键值对形式
    */
    #[java_method]
    pub fn flatten(&self) -> String {}

    /**
    获取扁平化参数字符串并将每个参数添加到此参数对象。flatten() 方法执行相反的操作。
    `flattened` 以分号分隔的参数字符串（键值对）
    */
    #[java_method]
    pub fn unflatten(&self, flattened: String) {}

    #[doc(hidden)]
    #[java_method]
    pub fn remove(&self, key: String) {}

    /**
    设置字符串参数。
    `key` 参数的键名称
    `value` 参数的字符串值
    */
    #[java_method]
    pub fn set(&self, key: String, value: String) {}

    /**
    设置一个整数参数。
    `key` 参数的键名称
    `value` 参数的 int 值
    */
    #[java_method(overload = set)]
    pub fn set_int(&self, key: String, value: i32) {}

    /**
    返回字符串参数的值。
    返回：参数的字符串值
    `key` 参数的键名
    */
    #[java_method]
    pub fn get(&self, key: String) -> Result<String> {}

    /**
    返回整数参数的值。
    返回：参数的 int 值
    `key` 参数的键名称
    */
    #[java_method]
    pub fn get_int(&self, key: String) -> Result<i32> {}

    //noinspection SpellCheckingInspection
    /**
    设置预览图片的尺寸。如果预览已经开始，应用程序应先停止预览，然后再更改预览大小。宽度和高度的边基于相机方向。
    也就是说，预览大小是在按显示方向旋转之前的大小。因此，应用程序在设置预览大小时需要考虑显示方向。
    例如，假设相机支持 480x320 和 320x480 预览大小。应用程序需要 3:2 的预览比例。如果显示方向设置为 0 或 180，则预览大小应设置为 480x320。
    如果显示方向设置为 90 或 270，则预览大小应设置为 320x480。设置图片大小和缩略图大小时也应考虑显示方向。
    176x144 (QCIF) 分辨率的例外情况：相机设备通常具有从较大分辨率缩小到较小分辨率的固定能力，由于高分辨率图像传感器设备上的这一限制，有时不完全支持 QCIF 分辨率。
    因此，尝试配置 QCIF 预览大小，任何图片或视频尺寸大于 1920x1080（宽度或高度）可能不受支持，如果不支持，setParameters(Camera.Parameters) 可能会抛出 RuntimeException。
    `width` 图片的宽度，以像素为单位
    `height` 图片的高度，以像素为单位
    */
    #[java_method]
    pub fn set_preview_size(&self, width: i32, height: i32) -> Result<()> {}

    /**
    返回预览图片的尺寸设置。
    返回：带有预览图片宽度和高度设置的 Size 对象
    */
    #[java_method]
    pub fn get_preview_size(&self) -> Camera_Size {}
}

/**
图像尺寸（宽度和高度尺寸）。
*/
#[allow(non_camel_case_types)]
#[deprecated(note = "我们建议在新应用程序中使用新的 android.hardware.camera2 API。")]
#[java_class(name = "android/hardware/Camera$Size")]
pub struct Camera_Size;

impl Camera_Size {
    /**
    设置图片的尺寸。
    `w` 照片宽度（像素）
    `h` 照片高度（像素）
    */
    #[java_constructor]
    pub fn new(w: i32, h: i32) -> Self {}

    /**
    获取图片宽度
    */
    #[java_field]
    pub fn get_width(&self) -> i32 {}

    /**
    设置图片宽度
    */
    #[java_field]
    pub fn set_width(&self, value: i32) {}
    /**
    获取图片高度
    */
    #[java_field]
    pub fn get_height(&self) -> i32 {}

    /**
    设置图片高度
    */
    #[java_field]
    pub fn set_height(&self, value: i32) {}
}

/**
Area 类用于选择相机在计算自动曝光、自动白平衡和自动对焦时使用的特定测光和对焦区域。
要了解给定相机支持多少个同时区域，请使用 Camera.Parameters.getMaxNumMeteringAreas() 和 Camera.Parameters.getMaxNumFocusAreas()。
如果不支持测光或对焦区域选择，这些方法将返回 0。每个 Area 都由一个指定其边界的矩形和一个确定其重要性的权重组成。
边界与相机的当前视野有关。坐标的映射方式是 (-1000, -1000) 始终是当前视野的左上角，而 (1000, 1000) 始终是当前视野的右下角。
不允许设置边界超出该范围的区域。不允许设置宽度或高度为零或负数的区域。权重必须在 1 到 1000 之间，表示区域中每个像素的权重。
这意味着具有相同权重的较大测光区域与较小区域的测光区域对测光结果的影响更大。测光区域可以重叠，驱动程序将在重叠区域添加权重。
*/
#[allow(non_camel_case_types)]
#[deprecated(note = "我们建议对新应用程序使用新的 android.hardware.camera2 API。")]
#[java_class(name = "android/hardware/Camera$Area")]
pub struct Camera_Area;

impl Camera_Area {
    /**
    创建一个具有指定矩形和权重的区域。
    `rect` 区域的边界。
    `weight` 区域的权重。
    */
    #[java_constructor]
    pub fn new(rect: &Rect, weight: i32) -> Self {}

    /**
    获取区域的边界。(-1000, -1000) 表示摄像头视野的左上角，(1000, 1000) 表示视野的右下角。不允许设置超出该范围的边界。不允许使用宽度或高度为零或负值的边界。
    */
    #[java_field]
    pub fn get_rect(&self) -> Rect {}

    /**
    设置区域的边界。(-1000, -1000) 表示摄像头视野的左上角，(1000, 1000) 表示视野的右下角。不允许设置超出该范围的边界。不允许使用宽度或高度为零或负值的边界。
    */
    #[java_field]
    pub fn set_rect(&self, value: &Rect) {}

    /**
    获取区域的权重。权重必须在 1 到 1000 之间，表示区域中每个像素的权重。这意味着，具有相同权重的较大测光区域与较小区域的测光区域对测光结果的影响更大。测光区域可以重叠，驱动程序将在重叠区域添加权重。
    */
    #[java_field]
    pub fn get_weight(&self) -> i32 {}

    /**
    设置区域的权重。权重必须在 1 到 1000 之间，表示区域中每个像素的权重。这意味着，具有相同权重的较大测光区域与较小区域的测光区域对测光结果的影响更大。测光区域可以重叠，驱动程序将在重叠区域添加权重。
    */
    #[java_field]
    pub fn set_weight(&self, value: i32) {}
}

/// 关于相机的信息
#[allow(non_camel_case_types)]
#[java_class(name = "android/hardware/Camera$CameraInfo")]
#[deprecated(note = "我们建议新应用程序使用新的 android.hardware.camera2 API。")]
pub struct Camera_CameraInfo;

impl Camera_CameraInfo {
    /// 相机朝向与屏幕相反。
    pub const CAMERA_FACING_BACK: i32 = 0;

    /// 相机朝向与屏幕相同。
    pub const CAMERA_FACING_FRONT: i32 = 1;

    #[doc(hidden)]
    #[java_constructor]
    pub fn new() -> Self {}

    /// 获取相机朝向。它应该是 CAMERA_FACING_BACK 或 CAMERA_FACING_FRONT。
    #[java_field]
    pub fn get_facing(&self) -> i32 {}

    /// 设置相机朝向。它应该是 CAMERA_FACING_BACK 或 CAMERA_FACING_FRONT。
    #[java_field]
    pub fn set_facing(&self, value: i32) {}

    /**
    获取相机图像的旋转角度。该值表示相机图像需要顺时针旋转多少度，才能在其自然方向上正确显示在显示设备上。它应该是 0、90、180 或 270。
    例如，假设一个设备的屏幕是自然竖直的。后置相机的传感器安装在横向。你正在看屏幕。如果相机传感器的顶部边缘与屏幕的自然方向上的右侧边缘对齐，则该值应为 90。如果前置相机的顶部边缘与屏幕的右侧对齐，则该值应为 270。
    */
    #[java_field]
    pub fn get_orientation(&self) -> i32 {}

    /**
    设置相机图像的旋转角度。该值表示相机图像需要顺时针旋转多少度，才能在其自然方向上正确显示在显示设备上。它应该是 0、90、180 或 270。
    例如，假设一个设备的屏幕是自然竖直的。后置相机的传感器安装在横向。你正在看屏幕。如果相机传感器的顶部边缘与屏幕的自然方向上的右侧边缘对齐，则该值应为 90。如果前置相机的顶部边缘与屏幕的右侧对齐，则该值应为 270。
    */
    #[java_field]
    pub fn set_orientation(&self, value: i32) {}

    /**
    获取是否可以禁用快门声音。
    在某些设备上，无法通过 enableShutterSound 禁用相机快门声音。此字段可用于确定禁用快门声音的调用是否成功。
    如果此字段设置为 true，则调用 `enableShutterSound(false)` 将成功。如果设置为 false，则该调用将失败，并且在调用 Camera#takePicture takePicture 时将播放快门声音。
    */
    #[java_field]
    pub fn get_can_disable_shutter_sound(&self) -> bool {}

    /**
    设置是否可以禁用快门声音。
    在某些设备上，无法通过 enableShutterSound 禁用相机快门声音。此字段可用于确定禁用快门声音的调用是否成功。
    如果此字段设置为 true，则调用 `enableShutterSound(false)` 将成功。如果设置为 false，则该调用将失败，并且在调用 Camera#takePicture takePicture 时将播放快门声音。
    */
    #[java_field]
    pub fn set_can_disable_shutter_sound(&self, value: bool) {}
}

/// 测试android.hardware
#[cfg(feature = "test_android_hardware")]
pub fn test() {
    use crate::android::app::Activity;
    let act = Activity::fetch().unwrap();
    let camera_permission = crate::android::Manifest_permission::CAMERA.to_string();
    let permissions = vec![camera_permission.clone()];
    act.request_permissions(&permissions, 100).unwrap();
    if act.check_self_permission(camera_permission)
        == crate::android::content::pm::PackageManager::PERMISSION_GRANTED
    {
        let info = Camera_CameraInfo::new();
        Camera::get_camera_info(Camera_CameraInfo::CAMERA_FACING_BACK, &info).unwrap();
        dbg!(&info);
        assert!(Camera::get_number_of_cameras() > 0);
        let camera = Camera::open(0).unwrap();
        camera.release();
        let camera = Camera::open_convenience().unwrap();
        camera.unlock().unwrap();
        camera.lock().unwrap();
        dbg!(&camera);
        camera.release();
    } else {
        println!("No camera permission.");
    }
}
