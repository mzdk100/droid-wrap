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

use crate::{android::content::Context, JObjNew, JObjRef, JType};
use droid_wrap_derive::{java_class, java_constructor, java_method};

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
    pub fn new(context: &Context) -> Self {}

    /**
     * 返回视图的宽度。
     * 返回：视图的宽度（以像素为单位）。
     * */
    #[java_method]
    pub fn get_width(&self) -> i32 {}

    /**
     * 返回视图的高度。
     * 返回：视图的高度（以像素为单位）。
     * */
    #[java_method]
    pub fn get_height(&self) -> i32 {}

    //noinspection SpellCheckingInspection
    /**
     * 调用此操作可以尝试将焦点集中到特定视图或其子视图之一。如果视图不可聚焦（isFocusable返回false），或者由于其他条件而无法聚焦（当设备处于触摸模式、不可见、未启用或没有大小时，在触摸模式下无法聚焦（isForusableInTouchMode）），则视图实际上不会聚焦。另请参阅focusSearch(int)，这是用来表示有焦点，并希望视图的父母寻找下一个焦点的方法。这相当于用参数FOCUS_DOWN和null调用requestFocus(int, Rect)。
     * 返回：这个视图或它的一个后代是否真的成为焦点。
     * */
    #[java_method]
    pub fn request_focus(&self) -> bool {}

    /**
     * 指示此视图的激活状态。
     * 返回：如果视图已激活，则返回 true，否则返回 false
     * */
    #[java_method]
    pub fn is_activated(&self) -> bool {}

    /**
     * 更改此视图的激活状态。视图可以激活也可以不激活。请注意，激活与选择不同。选择是一种瞬时属性，表示用户当前正在与之交互的视图（层次结构）。激活是一种长期状态，用户可以将视图移入或移出。例如，在启用单选或多选的列表视图中，当前选择集中的视图处于激活状态。（嗯，是的，我们对这里的术语深感抱歉。）激活状态会向下传播到设置该状态的视图的子级。
     * `activated` 如果必须激活视图，则为 true，否则为 false
     * */
    #[java_method]
    pub fn set_activated(&self, activated: bool) {}

    /**
     * 此视图的可视 x 位置（以像素为单位）。这相当于 TranslationX 属性加上当前 Left 属性。
     * 返回：此视图的可视 x 位置（以像素为单位）。
     * */
    #[java_method]
    pub fn get_x(&self) -> f32 {}

    /**
     * 设置此视图的可视 x 位置（以像素为单位）。这相当于将 TranslationX 属性设置为传入的 x 值与当前 left 属性之间的差值。
     * `x` 此视图的可视 x 位置（以像素为单位）。
     * */
    #[java_method]
    pub fn set_x(&self, x: f32) {}

    /**
     * 此视图的视觉 y 位置（以像素为单位）。这相当于 TranslationY 属性加上当前的 Top 属性。
     * 返回：此视图的视觉 y 位置（以像素为单位）。
     * */
    #[java_method]
    pub fn get_y(&self) -> f32 {}

    /**
     * 设置此视图的视觉 y 位置（以像素为单位）。这相当于将 TranslationY 属性设置为传入的 y 值与当前 top 属性之间的差值。
     * `y` 此视图的视觉 y 位置（以像素为单位）。
     * */
    #[java_method]
    pub fn set_y(&self, y: f32) {}
}

#[cfg(feature = "test_android_view")]
pub fn test() {
    use crate::android::app::Activity;
    let ctx: Context = (&Activity::fetch()).into();
    let view = View::new(&ctx);
    assert!(view.to_string().starts_with("android.view.View"));
    view.set_activated(true);
    assert_eq!(true, view.is_activated());
    view.set_x(20f32);
    assert_eq!(20f32, view.get_x());
    view.set_y(30f32);
    assert_eq!(30f32, view.get_y());
}
