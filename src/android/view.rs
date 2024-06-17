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

use droid_wrap_derive::{java_class, java_constructor};
use crate::{
    android::content::Context,
    JType,JObjRef
};

/**
 * 此类代表用户界面组件的基本构建块。View 占据屏幕上的矩形区域，负责绘制和事件处理。View 是小部件的基类，用于创建交互式 UI 组件（按钮、文本字段等）。ViewGroup 子类是布局的基类，布局是不可见的容器，用于容纳其他 View（或其他 ViewGroup）并定义其布局属性。
 * 开发人员指南
 * 有关使用此类开发应用程序用户界面的信息，请阅读用户界面开发人员指南。
 * 使用视图窗口中的所有视图都排列在一棵树中。您可以从代码中添加视图，也可以通过在一个或多个 XML 布局文件中指定视图树来添加视图。有许多专门的视图子类，它们充当控件或能够显示文本、图像或其他内容。创建视图树后，通常您可能希望执行以下几种常见操作：
 * - 设置属性：例如，设置 android.widget.TextView 的文本。不同视图子类的可用属性和设置方法会有所不同。请注意，在构建时已知的属性可以在 XML 布局文件中设置。
 * - 设置焦点：框架将处理移动焦点以响应用户输入。要强制将焦点移到特定视图，请调用 requestFocus。
 * - 设置侦听器：视图允许客户端设置侦听器，当视图发生有趣的事情时，这些侦听器将收到通知。例如，所有视图都允许您设置一个侦听器，以便在视图获得或失去焦点时收到通知。您可以使用 setOnFocusChangeListener(View.OnFocusChangeListener) 注册此类侦听器。其他视图子类提供更专业的侦听器。例如，按钮会公开一个侦听器，以便在单击按钮时通知客户端。
 * - 设置可见性：您可以使用 setVisibility(int) 隐藏或显示视图。注意：Android 框架负责测量、布局和绘制视图。除非您实际实施了 ViewGroup，否则您不应自己调用对视图执行这些操作的方法。
 * 有关更多信息，请查阅官方文档。
 * */
#[java_class(name = "android/view/View")]
pub struct View;

impl View {
    /**
     * 从代码创建视图时使用的简单构造函数。
     * `context` 视图在其中运行的上下文，通过它可以访问当前主题、资源等。
     * */
    #[java_constructor]
    pub fn new(context: Context) -> Self {}
}

#[cfg(feature = "test_android_view")]
pub fn test() {
    use crate::android::app::Activity;
    let ctx: Context = (&Activity::fetch()).into();
    let view = View::new(ctx);
    assert!(view.to_string().starts_with("android.view.View"));
}