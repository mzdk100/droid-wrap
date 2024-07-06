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

//noinspection SpellCheckingInspection
/**
 * 视图和输入法（如软键盘）之间交互的框架类。
 * */
#[cfg(feature = "android_view_inputmethod")]
#[cfg_attr(docsrs, doc(cfg(feature = "android_view_inputmethod")))]
#[cfg(feature = "android_view_inputmethod")]
pub mod inputmethod;

use crate::{
    android::content::Context,
    java::lang::{CharSequence, Integer},
    JObjNew, JObjRef, JProxy, JType,
};
use droid_wrap_derive::{
    java_class, java_constructor, java_field, java_implement, java_interface, java_method,
};
use std::sync::Arc;

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
    /// 用于标记没有ID的View。
    pub const NO_ID: i32 = -1;

    /// 赋予不属于活动的视图的最后一个 ID。
    pub const LAST_APP_AUTOFILL_ID: i32 = i32::MAX / 2;

    /// 此视图可见。与 setVisibility 和 ` ` 一起使用。
    pub const VISIBLE: i32 = 0x00000000;

    /// 此视图不可见，但出于布局目的它仍占用空间。与 setVisibility 和 ` ` 一起使用。
    pub const INVISIBLE: i32 = 0x00000004;

    /// 此视图是看不见的，并且没有任何空间用于布局目的。与setVisibility一起使用。
    pub const GONE: i32 = 0x00000008;

    /**
     * 从代码创建视图时使用的简单构造函数。
     * `context` 视图在其中运行的上下文，通过它可以访问当前主题、资源等。
     * */
    #[java_constructor]
    pub fn new(context: &Context) -> Self {}

    /**
     * 发送 AccessibilityEvent 的便捷方法。TYPE_ANNOUNCEMENT AccessibilityEvent 建议无障碍服务向其用户宣布指定的文本。
     * 注意：使用此 API 生成的事件不具有语义含义，仅适用于特殊情况。应用通常可以通过准确提供其 UI 的语义来实现正确的无障碍行为。它们不需要指定向用户宣布的具体内容。一般来说，只宣布转换，不要为按钮按下等简单操作生成确认消息。相反，请简明扼要地标记控件，对于重大的 UI 更改（如窗口更改），请使用 android.app.Activity.setTitle(CharSequence) 和 setAccessibilityPaneTitle(CharSequence)。使用 setAccessibilityLiveRegion(int) 通知用户用户界面内关键视图的更改。这些仍应谨慎使用，因为它们可能会在每次更新视图时生成通知。
     * `text` 通知文本。
     * */
    #[java_method]
    pub fn announce_for_accessibility<CS: CharSequence>(&self, text: &CS) {}

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
     * 当此视图想要放弃焦点时调用。如果焦点被清除，则调用 onFocusChanged(boolean, int, Rect)。
     * 注意：当不处于触摸模式时，框架将在焦点清除后尝试将焦点放在从顶部开始的第一个可聚焦视图上。因此，如果此视图是从顶部开始的第一个可获得焦点的视图，则将调用与清除焦点相关的所有回调，之后框架将焦点放在此视图上。
     * */
    #[java_method]
    pub fn clear_focus(&self) {}

    /**
     * 在以此视图为根的层次结构中查找当前具有焦点的视图。
     * 返回：当前具有焦点的视图，如果找不到焦点视图，则返回 null。
     * */
    #[java_method]
    pub fn find_focus(&self) -> Option<Self> {}

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

    /**
     * 查找具有给定 ID 的第一个后代视图，如果 ID 与 getId() 匹配，则查找视图本身，如果 ID 无效 (< 0) 或层次结构中没有匹配的视图，则返回 null。
     * 注意：在大多数情况下 - 取决于编译器支持 - 生成的视图会自动转换为目标类类型。如果目标类类型不受约束，则可能需要显式转换。
     *  返回：如果找到，则返回具有给定 ID 的视图，否则返回 null
     * `id` 要搜索的 ID
     * */
    #[java_method]
    pub fn find_view_by_id(&self, id: i32) -> Option<Self> {}

    /**
     * 返回视图的内容描述。
     * 注意：不要覆盖此方法，因为它不会影响呈现给无障碍服务的内容描述。您必须调用 setContentDescription(CharSequence) 来修改内容描述。
     * 返回：内容描述
     * */
    #[java_method]
    pub fn get_content_description<CS: CharSequence>(&self) -> Option<CS> {}

    /**
     * 设置视图的内容描述。内容描述简要描述视图，主要用于辅助功能支持，以确定应如何向用户呈现视图。对于没有文本表示的视图（如 android.widget.ImageButton），有用的内容描述会解释视图的作用。例如，用于拨打电话的带有电话图标的图像按钮可以使用“呼叫”作为其内容描述。用于保存文件的软盘图像可以使用“保存”。这应该省略角色或状态。角色是指视图的用户界面元素类型，例如按钮或复选框。状态是指视图经常变化的属性，例如按钮的开/关状态或音量滑块的音频级别。内容描述更新并不频繁，并且在元素的语义内容（而不是状态）发生变化时使用。例如，在音乐播放期间，播放按钮可能会更改为暂停按钮。
     * `content_description` 内容描述。
     * */
    #[java_method]
    pub fn set_content_description<CS: CharSequence>(&self, content_description: Option<CS>) {}

    /**
     * 设置此视图的标识符。标识符在此视图的层次结构中不必唯一。标识符应为正数。
     * `id` 用于标识视图的数字
     * */
    #[java_method]
    pub fn set_id(&self, id: i32) {}

    /**
     * 返回此视图的标识符。
     * 返回：用于标识视图的正整数，如果视图没有 ID，则返回 NO_ID
     * */
    #[java_method]
    pub fn get_id(&self) -> i32 {}

    /**
     * 单击此视图时注册要调用的回调。如果此视图不可点击，则将设为可点击。
     * `l` 将运行的回调
     * */
    #[java_method]
    pub fn set_on_click_listener<L: View_OnClickListener + JProxy>(&self, l: &L) {}

    /**
     * 获取与此视图关联的 LayoutParams。所有视图都应具有布局参数。这些参数为此视图的父级提供参数，指定应如何排列。
     * ViewGroup.LayoutParams 有许多子类，这些子类对应于负责排列其子级的 ViewGroup 的不同子类。如果此视图未附加到父 ViewGroup 或 setLayoutParams(ViewGroup.LayoutParams) 未成功调用，则此方法可能返回 null。当视图附加到父 ViewGroup 时，此方法不得返回 null。
     * 返回：与此视图关联的 LayoutParams，如果尚未设置参数，则返回 null
     * */
    #[java_method]
    pub fn get_layout_params(&self) -> Option<ViewGroup_LayoutParams> {}

    /**
     * 设置与此视图相关的布局参数。这些参数为该视图的父级提供参数，指定应如何排列。ViewGroup 有许多子类。LayoutParams，这些对应于负责排列其子级的 ViewGroup 的不同子类。
     * `params` 此视图的布局参数，不能为空
     * */
    #[java_method]
    pub fn set_layout_params(&self, params: &ViewGroup_LayoutParams) {}

    /**
     * 设置此视图的可见性状态。
     * `visibility` VISIBLE、INVISIBLE 或 GONE 之一。
     * */
    #[java_method]
    pub fn set_visibility(&self, visibility: i32) {}

    /**
     * 返回此视图的可见性状态。
     * 返回：VISIBLE、INVISIBLE 或 GONE 之一。
     * */
    #[java_method]
    pub fn get_visibility(&self) -> i32 {}
}

/// 当视图被点击时调用的回调的接口定义。
#[allow(non_camel_case_types)]
#[java_interface(name = "android/view/View$OnClickListener")]
pub trait View_OnClickListener {
    /**
     * 当单击某个视图时调用。
     * `v` 被单击的视图。
     * */
    fn on_click(&self, v: View);
}

#[allow(non_camel_case_types)]
#[java_class(name = "android/view/View$OnClickListenerImpl")]
pub struct View_OnClickListenerImpl(Box<dyn Fn(View) + Sync + Send>);

impl View_OnClickListenerImpl {
    pub fn from_fn(func: impl Fn(View) + Sync + Send + 'static) -> Arc<Self> {
        Self::new(View_OnClickListenerImplDefault(Box::new(func)))
    }
}

impl Default for View_OnClickListenerImplDefault {
    fn default() -> Self {
        Self(Box::new(|_| ()))
    }
}

#[java_implement]
impl View_OnClickListener for View_OnClickListenerImpl {
    fn on_click(&self, v: View) {
        self.0(v)
    }
}

/**
 * 接口允许您向 Activity 添加和删除子视图。要获取此类的实例，请调用 Context.getSystemService()。
 * */
#[java_interface(name = "android/view/ViewManager")]
pub trait ViewManager {
    /**
     * 将传递的 LayoutParams 分配给传递的 View，并将该视图添加到窗口。对于某些编程错误，例如在未移除第一个视图的情况下向窗口添加第二个视图，将抛出 WindowManager.BadTokenException。
     * 如果窗口位于辅助显示器上并且找不到指定的显示器，则抛出 WindowManager.InvalidDisplayException（请参阅 android.app.Presentation）。
     * `view` 要添加到此窗口的视图。
     * `params` 要分配给视图的 LayoutParams。
     * */
    fn add_view(&self, view: &View, params: &ViewGroup_LayoutParams);
    fn update_view_layout(&self, view: &View, params: &ViewGroup_LayoutParams);
    fn remove_view(&self, view: &View);
}

#[java_interface(name = "android/view/WindowManager")]
pub trait WindowManager: ViewManager {
    const PARCEL_KEY_SHORTCUTS_ARRAY: &'static str = "shortcuts_array";

    /**
     * removeView 的特殊变体，在返回之前立即调用给定视图层次结构的 View.onDetachedFromWindow() 方法。这不适用于普通应用程序；正确使用它需要非常小心。
     * `view` 要删除的视图。
     * */
    fn remove_view_immediate(&self, view: &View);

    /**
     * 返回此WindowManager实例将创建新窗口的显示器。尽管有此方法的名称，但返回的显示器不一定是系统的主要显示器（请参见Display.DEFAULT_DISPLAY）。
     * 返回的显示可能是此窗口管理器实例正在管理的辅助显示。将其视为此WindowManager实例默认使用的显示。要在其他显示器上创建窗口，您需要为该显示器获得一个WindowManager。 （有关更多信息，请参见WindowManager类文档。）
     * 返回：此窗口管理器正在管理的显示器。
     * */
    #[deprecated(note = "改用 Context.getDisplay()。")]
    #[java_method]
    fn get_default_display(&self) -> Display {}
}

#[java_class(name = "android/view/WindowManagerImpl")]
pub struct WindowManagerImpl;

impl ViewManager for WindowManagerImpl {
    #[java_method]
    fn add_view(&self, view: &View, params: &ViewGroup_LayoutParams) {}

    #[java_method]
    fn update_view_layout(&self, view: &View, params: &ViewGroup_LayoutParams) {}

    #[java_method]
    fn remove_view(&self, view: &View) {}
}

impl WindowManager for WindowManagerImpl {
    #[java_method]
    fn remove_view_immediate(&self, view: &View) {}
}

/**
 * ViewGroup 是一种特殊视图，可以包含其他视图（称为子视图）。视图组是布局和视图容器的基类。
 * 此类还定义了 ViewGroup.LayoutParams 类，该类用作布局参数的基类。另请参阅 ViewGroup.LayoutParams 以了解布局属性。
 *
 * 开发者指南
 * 有关创建用户界面布局的更多信息，请阅读 XML 布局开发者指南。以下是自定义 ViewGroup 的完整实现，它实现了简单的 android.widget.FrameLayout，并能够在左右边缘堆叠子视图。
 * @sample development/samples/ApiDemos/src/com/example/android/apis/view/CustomLayout.java 完整版
 * 如果您正在实现示例中所示的 XML 布局属性，则这是它们在 res/values/attrs.xml 中的对应定义：
 * @sample development/samples/ApiDemos/res/values/attrs.xml CustomLayout
 * 最后，布局管理器可以在 XML 布局中使用，如下所示：
 * @sample development/samples/ApiDemos/res/layout/custom_layout.xml 完整版
 * */
#[java_class(name = "android/view/ViewGroup", extends=View)]
pub struct ViewGroup;

impl ViewGroup {
    /**
     * 添加子视图。如果子视图上尚未设置布局参数，则此 ViewGroup 的默认参数将设置在该子视图上。
     * 注意：不要从 draw(Canvas)、onDraw(Canvas)、dispatchDraw(Canvas) 或任何相关方法调用此方法。
     * `child` 要添加的子视图
     * */
    #[java_method]
    pub fn add_view(&self, child: &View) {}

    /**
     * 调用此方法可从 ViewGroup 中删除所有子视图。
     * 注意：不要从 draw(Canvas)、onDraw(Canvas)、dispatchDraw(Canvas) 或任何相关方法调用此方法。
     * */
    #[java_method]
    pub fn remove_all_views(&self) {}

    /**
     * 当 ViewGroup 子类必须先知道其在屏幕上的大小，然后才能计算要渲染的子视图数量时，此方法会由 ViewGroup 子类调用，以从自身移除子视图。例如 Gallery 或 ListView，它们可能“有”50 个子视图，但实际上只渲染当前可容纳在屏幕上的对象内的子视图数量。除非您正在扩展 ViewGroup 并了解视图测量和布局管道，否则请不要调用此方法。
     * 注意：不要从 draw(Canvas)、onDraw(Canvas)、dispatchDraw(Canvas) 或任何相关方法调用此方法。
     * */
    pub fn remove_all_views_in_layout(&self) {}
}

impl ViewManager for ViewGroup {
    #[java_method]
    fn add_view(&self, view: &View, params: &ViewGroup_LayoutParams) {}

    #[java_method]
    fn update_view_layout(&self, view: &View, params: &ViewGroup_LayoutParams) {}

    #[java_method]
    fn remove_view(&self, view: &View) {}
}

/**
 * 视图使用 LayoutParams 来告诉其父级它们希望如何布局。请参阅 ViewGroup 布局属性，了解此类支持的所有子视图属性的列表。
 * 基本 LayoutParams 类仅描述视图希望的宽度和高度。对于每个维度，它可以指定以下之一：
 * FILL_PARENT（在 API 级别 8 及更高版本中重命名为 MATCH_PARENT），这意味着视图希望与其父级一样大（减去填充）
 * WRAP_CONTENT，这意味着视图希望足够大以包含其内容（加上填充）
 *
 * 一个确切的数字
 * 对于不同的 ViewGroup 子类，有 LayoutParams 的子类。例如，AbsoluteLayout 有自己的 LayoutParams 子类，它添加了 X 和 Y 值。
 *
 * 开发人员指南
 * 有关创建用户界面布局的更多信息，请阅读 XML 布局开发人员指南。
 * */
#[allow(non_camel_case_types)]
#[java_class(name = "android/view/ViewGroup$LayoutParams")]
pub struct ViewGroup_LayoutParams;

impl ViewGroup_LayoutParams {
    /// 视图请求的高度或宽度的特殊值。FILL_PARENT 表示视图希望与其父级一样大，减去父级的填充（如果有）。
    #[deprecated(note = "从 API 级别 8 开始，此值已弃用，并由 MATCH_PARENT 取代。")]
    pub const FILL_PARENT: i32 = -1;

    /// 视图请求的高度或宽度的特殊值。MATCH_PARENT 表示视图希望与其父级一样大，减去父级的填充（如果有）。在 API 级别 8 中引入。
    pub const MATCH_PARENT: i32 = -1;

    /// 视图请求的高度或宽度的特殊值。WRAP_CONTENT 表示视图希望足够大以容纳其自己的内部内容，同时考虑其自己的填充。
    pub const WRAP_CONTENT: i32 = -2;

    /// 有关视图所需宽度的信息。可以是常量 FILL_PARENT（在 API 级别 8 中由 MATCH_PARENT 取代）或 WRAP_CONTENT 之一，也可以是确切的大小。
    #[java_field]
    pub fn get_width(&self) -> i32 {}

    /// 有关视图所需宽度的信息。可以是常量 FILL_PARENT（在 API 级别 8 中由 MATCH_PARENT 取代）或 WRAP_CONTENT 之一，也可以是确切的大小。
    #[java_field]
    pub fn set_width(&self, value: i32) {}

    /// 有关视图所需高度的信息。可以是常量 FILL_PARENT（在 API 级别 8 中由 MATCH_PARENT 取代）或 WRAP_CONTENT 之一，也可以是确切的大小。
    #[java_field]
    pub fn get_height(&self) -> i32 {}

    /// 有关视图所需高度的信息。可以是常量 FILL_PARENT（在 API 级别 8 中由 MATCH_PARENT 取代）或 WRAP_CONTENT 之一，也可以是确切的大小。
    #[java_field]
    pub fn set_height(&self, value: i32) {}

    /**
     * 创建一组具有指定宽度和高度的新布局参数。
     * `width` 宽度，可以是 WRAP_CONTENT、FILL_PARENT（在 API 级别 8 中由 MATCH_PARENT 替换），也可以是固定大小（以像素为单位）
     * `height` 高度，可以是 WRAP_CONTENT、FILL_PARENT（在 API 级别 8 中由 MATCH_PARENT 替换），也可以是固定大小（以像素为单位）
     * */
    #[java_constructor]
    pub fn new(width: i32, height: i32) -> Self {}
}

/**
 * 支持边距的布局的每个子视图布局信息。请参阅 ViewGroup 边距布局属性，查看此类支持的所有子视图属性的列表。
 * */
#[allow(non_camel_case_types)]
#[java_class(name = "android/view/ViewGroup$MarginLayoutParams", extends=ViewGroup_LayoutParams)]
pub struct ViewGroup_MarginLayoutParams;

impl ViewGroup_MarginLayoutParams {
    /// 默认的开始和结束边距。
    pub const DEFAULT_MARGIN_RELATIVE: u32 = Integer::MIN_VALUE;

    /// 子项的左边距（以像素为单位）。边距值应为正数。为此字段重新分配新值后，调用 setLayoutParams(ViewGroup. LayoutParams)。
    #[java_field]
    pub fn get_left_margin(&self) -> i32 {}

    /// 子项的左边距（以像素为单位）。边距值应为正数。为此字段重新分配新值后，调用 setLayoutParams(ViewGroup. LayoutParams)。
    #[java_field]
    pub fn set_left_margin(&self, value: i32) {}

    /// 子项的上边距（以像素为单位）。边距值应为正数。为此字段重新分配新值后，调用 setLayoutParams(ViewGroup. LayoutParams)。
    #[java_field]
    pub fn get_top_margin(&self) -> i32 {}

    /// 子项的上边距（以像素为单位）。边距值应为正数。为此字段重新分配新值后，调用 setLayoutParams(ViewGroup. LayoutParams)。
    #[java_field]
    pub fn set_top_margin(&self, value: i32) {}

    /// 子项的右边距（以像素为单位）。边距值应为正数。为此字段重新分配新值后，调用 setLayoutParams(ViewGroup. LayoutParams)。
    #[java_field]
    pub fn get_right_margin(&self) -> i32 {}

    /// 子项的右边距（以像素为单位）。边距值应为正数。为此字段重新分配新值后，调用 setLayoutParams(ViewGroup. LayoutParams)。
    #[java_field]
    pub fn set_right_margin(&self, value: i32) {}

    /// 子项的底部边距（以像素为单位）。边距值应为正数。为此字段重新分配新值后，调用 setLayoutParams(ViewGroup. LayoutParams)。
    #[java_field]
    pub fn get_bottom_margin(&self) -> i32 {}

    /// 子项的底部边距（以像素为单位）。边距值应为正数。为此字段重新分配新值后，调用 setLayoutParams(ViewGroup. LayoutParams)。
    #[java_field]
    pub fn set_bottom_margin(&self, value: i32) {}

    #[java_constructor]
    pub fn new(width: i32, height: i32) -> Self {}

    #[java_constructor]
    pub fn from_layout_params(source: &ViewGroup_LayoutParams) -> Self {}
}

#[allow(non_camel_case_types)]
#[java_class(name = "android/view/WindowManager$LayoutParams", extends=ViewGroup_LayoutParams)]
pub struct WindowManager_LayoutParams;

impl WindowManager_LayoutParams {
    /// 代表正常应用程序窗口的窗口类型的启动。
    pub const FIRST_APPLICATION_WINDOW: i32 = 1;

    /// 窗口类型：作为整个应用程序的“基础”窗口的应用程序窗口；所有其他应用程序窗口将显示在其上方。在多用户系统中，仅显示在拥有该应用程序的用户窗口上。
    pub const TYPE_BASE_APPLICATION: i32 = 1;

    /// 窗口类型：普通应用程序窗口。令牌必须是活动令牌，用于标识窗口所属用户。在多用户系统中，仅显示在所属用户的窗口上。
    pub const TYPE_APPLICATION: i32 = 2;

    /// 窗口类型：应用程序启动时显示的特殊应用程序窗口。应用程序本身不使用它；系统使用它来显示某些内容，直到应用程序可以显示自己的窗口。在多用户系统中，显示在所有用户的窗口上。
    pub const TYPE_APPLICATION_STARTING: i32 = 3;

    /// 窗口类型：TYPE_APPLICATION 的变体，确保窗口管理器将等待此窗口绘制完毕后再显示应用。在多用户系统中，仅显示在拥有该应用的用户窗口上。
    pub const TYPE_DRAWN_APPLICATION: i32 = 4;

    /// 申请窗口类型的结束。
    pub const LAST_APPLICATION_WINDOW: i32 = 99;

    /// 子窗口类型的开始。这些窗口的标记必须设置为它们所附加到的窗口。这些类型的窗口在 Z 顺序上紧挨着它们所附加到的窗口，并且它们的坐标空间相对于它们所附加到的窗口。
    pub const FIRST_SUB_WINDOW: i32 = 1000;

    /// 窗口类型：位于应用程序窗口顶部的面板。这些窗口出现在其附属窗口的顶部。
    pub const TYPE_APPLICATION_PANEL: i32 = Self::FIRST_SUB_WINDOW;

    /// 窗口类型：用于显示媒体（如视频）的窗口。这些窗口显示在其附属窗口的后面。
    pub const TYPE_APPLICATION_MEDIA: i32 = Self::FIRST_SUB_WINDOW + 1;

    /// 窗口类型：应用程序窗口顶部的子面板。这些窗口显示在其附属窗口和任何 TYPE_APPLICATION_PANEL 面板的顶部。
    pub const TYPE_APPLICATION_SUB_PANEL: i32 = Self::FIRST_SUB_WINDOW + 2;

    /// 窗口类型：类似于 TYPE_APPLICATION_PANEL，但窗口的布局与顶层窗口一样，而不是作为其容器的子窗口。
    pub const TYPE_APPLICATION_ATTACHED_DIALOG: i32 = Self::FIRST_SUB_WINDOW + 3;

    /// 窗口类型：用于在媒体窗口顶部显示覆盖层的窗口。这些窗口显示在 TYPE_APPLICATION_MEDIA 和应用程序窗口之间。它们应该是半透明的，以便于使用。这是一个非常丑陋的 hack，因此：
    pub const TYPE_APPLICATION_MEDIA_OVERLAY: i32 = Self::FIRST_SUB_WINDOW + 4;

    /// 窗口类型：位于应用程序窗口及其子面板窗口上方的子面板。这些窗口显示在其附属窗口和任何 TYPE_APPLICATION_SUB_PANEL 面板的上方。
    pub const TYPE_APPLICATION_ABOVE_SUB_PANEL: i32 = Self::FIRST_SUB_WINDOW + 5;

    /// 子窗口类型的结束。
    pub const LAST_SUB_WINDOW: i32 = 1999;

    /// 启动系统特定的窗口类型。这些窗口类型通常不是由应用程序创建的。
    pub const FIRST_SYSTEM_WINDOW: i32 = 2000;

    /// 窗口类型：状态栏。只能有一个状态栏窗口；它位于屏幕顶部，所有其他窗口都向下移动，因此它们位于其下方。在多用户系统中显示在所有用户的窗口上。
    pub const TYPE_STATUS_BAR: i32 = Self::FIRST_SYSTEM_WINDOW;

    /// 窗口类型：搜索栏。只能有一个搜索栏窗口；它位于屏幕顶部。在多用户系统中显示在所有用户的窗口上。
    pub const TYPE_SEARCH_BAR: i32 = Self::FIRST_SYSTEM_WINDOW + 1;

    /// 窗口类型：电话。这些是非应用程序窗口，用于为用户提供与电话的交互（尤其是来电）。这些窗口通常位于所有应用程序之上，但在状态栏后面。在多用户系统中显示在所有用户的窗口上。
    #[deprecated(note = "适用于非系统应用。请改用 TYPE_APPLICATION_OVERLAY。")]
    pub const TYPE_PHONE: i32 = Self::FIRST_SYSTEM_WINDOW + 2;

    /// 窗口类型：系统窗口，例如低电量警报。这些窗口始终位于应用程序窗口之上。在多用户系统中，仅显示在拥有用户的窗口上。
    #[deprecated(note = "适用于非系统应用。请改用 TYPE_APPLICATION_OVERLAY。")]
    pub const TYPE_SYSTEM_ALERT: i32 = Self::FIRST_SYSTEM_WINDOW + 3;

    /// 窗口类型：键盘保护窗口。在多用户系统中显示在所有用户的窗口上。
    pub const TYPE_KEYGUARD: i32 = Self::FIRST_SYSTEM_WINDOW + 4;

    /// 窗口类型：瞬态通知。在多用户系统中，仅显示在拥有用户的窗口上。
    #[deprecated(note = "适用于非系统应用。请改用 TYPE_APPLICATION_OVERLAY。")]
    pub const TYPE_TOAST: i32 = Self::FIRST_SYSTEM_WINDOW + 5;

    /// 窗口类型：系统覆盖窗口，需要显示在所有其他窗口之上。这些窗口不得获取输入焦点，否则会干扰键盘保护。在多用户系统中，仅显示在拥有用户的窗口上。
    #[deprecated(note = "适用于非系统应用。请改用 TYPE_APPLICATION_OVERLAY。")]
    pub const TYPE_SYSTEM_OVERLAY: i32 = Self::FIRST_SYSTEM_WINDOW + 6;

    /// 窗口类型：优先手机 UI，即使键盘锁处于活动状态也需要显示。这些窗口不得获取输入焦点，否则会干扰键盘锁。在多用户系统中显示在所有用户的窗口上。
    #[deprecated(note = "适用于非系统应用。请改用 TYPE_APPLICATION_OVERLAY。")]
    pub const TYPE_PRIORITY_PHONE: i32 = Self::FIRST_SYSTEM_WINDOW + 7;

    /// 窗口类型：在多用户系统中从状态栏滑出的面板显示在所有用户的窗口上。
    pub const TYPE_SYSTEM_DIALOG: i32 = Self::FIRST_SYSTEM_WINDOW + 8;

    /// 窗口类型：键盘保护显示的对话框在多用户系统中显示在所有用户的窗口上。
    pub const TYPE_KEYGUARD_DIALOG: i32 = Self::FIRST_SYSTEM_WINDOW + 9;

    /// 窗口类型：内部系统错误窗口，显示在所有可能出现的窗口之上。在多用户系统中，仅显示在所属用户的窗口上。
    #[deprecated(note = "适用于非系统应用。请改用 TYPE_APPLICATION_OVERLAY。")]
    pub const TYPE_SYSTEM_ERROR: i32 = Self::FIRST_SYSTEM_WINDOW + 10;

    /// 窗口类型：内部输入法窗口，显示在正常 UI 上方。可以调整应用程序窗口的大小或平移，以便在显示此窗口时保持输入焦点可见。在多用户系统中，仅显示在拥有用户的窗口上。
    pub const TYPE_INPUT_METHOD: i32 = Self::FIRST_SYSTEM_WINDOW + 11;

    /// 窗口类型：内部输入法对话窗口，显示在当前输入法窗口上方。在多用户系统中，仅显示在所属用户的窗口上。
    pub const TYPE_INPUT_METHOD_DIALOG: i32 = Self::FIRST_SYSTEM_WINDOW + 12;

    /// 窗口类型：壁纸窗口，放置在任何想要位于壁纸之上的窗口后面。在多用户系统中，仅显示在拥有该壁纸的用户窗口上。
    pub const TYPE_WALLPAPER: i32 = Self::FIRST_SYSTEM_WINDOW + 13;

    /// 窗口类型：在多用户系统中从状态栏滑出的面板显示在所有用户的窗口上。
    pub const TYPE_STATUS_BAR_PANEL: i32 = Self::FIRST_SYSTEM_WINDOW + 14;

    /**
     * 窗口类型：安全系统覆盖窗口，需要显示在所有其他窗口之上。这些窗口不得获取输入焦点，否则会干扰键盘保护。
     * 这与 TYPE_SYSTEM_OVERLAY 完全相同，不同之处在于只有系统本身才被允许创建这些覆盖层。应用程序无法获得创建安全系统覆盖层的权限。
     * 在多用户系统中仅显示在所属用户的窗口上。
     * */
    pub const TYPE_SECURE_SYSTEM_OVERLAY: i32 = Self::FIRST_SYSTEM_WINDOW + 15;

    /// 窗口类型：拖放式伪窗口。最多只有一个拖放层，并放置在所有其他窗口之上。在多用户系统中，仅显示在拥有该窗口的用户窗口上。
    pub const TYPE_DRAG: i32 = Self::FIRST_SYSTEM_WINDOW + 16;

    /// 窗口类型：在多用户系统中，从状态栏滑出的面板显示在所有用户的窗口上。这些窗口显示在状态栏和任何 TYPE_STATUS_BAR_PANEL 窗口的顶部。
    pub const TYPE_STATUS_BAR_SUB_PANEL: i32 = Self::FIRST_SYSTEM_WINDOW + 17;

    /// 窗口类型：（鼠标）指针在多用户系统中显示在所有用户的窗口上。
    pub const TYPE_POINTER: i32 = Self::FIRST_SYSTEM_WINDOW + 18;

    /// 窗口类型：导航栏（与状态栏不同）在多用户系统中显示在所有用户的窗口上。
    pub const TYPE_NAVIGATION_BAR: i32 = Self::FIRST_SYSTEM_WINDOW + 19;

    /// 窗口类型：用户更改系统音量时显示的音量级别覆盖/对话框。在多用户系统中，显示在所有用户的窗口上。
    pub const TYPE_VOLUME_OVERLAY: i32 = Self::FIRST_SYSTEM_WINDOW + 20;

    /// 窗口类型：启动进度对话框，位于所有内容之上。在多用户系统中显示在所有用户的窗口上。
    pub const TYPE_BOOT_PROGRESS: i32 = Self::FIRST_SYSTEM_WINDOW + 21;

    /// 当系统 UI 栏隐藏时，窗口类型会消耗输入事件。在多用户系统中，显示在所有用户的窗口上。
    pub const TYPE_INPUT_CONSUMER: i32 = Self::FIRST_SYSTEM_WINDOW + 22;

    /// 窗口类型：导航栏面板（当导航栏与状态栏不同时）在多用户系统中显示在所有用户的窗口上。
    pub const TYPE_NAVIGATION_BAR_PANEL: i32 = Self::FIRST_SYSTEM_WINDOW + 24;

    /// 窗口类型：显示覆盖窗口。用于模拟辅助显示设备。在多用户系统中显示在所有用户的窗口上。
    pub const TYPE_DISPLAY_OVERLAY: i32 = Self::FIRST_SYSTEM_WINDOW + 26;

    /// 窗口类型：放大覆盖窗口。用于在启用辅助放大功能时突出显示放大的部分。在多用户系统中显示在所有用户的窗口上。
    pub const TYPE_MAGNIFICATION_OVERLAY: i32 = Self::FIRST_SYSTEM_WINDOW + 27;

    /// 窗口类型：私人虚拟显示器上方的演示窗口。
    pub const TYPE_PRIVATE_PRESENTATION: i32 = Self::FIRST_SYSTEM_WINDOW + 30;

    /// 窗口类型：语音交互层的窗口。
    pub const TYPE_VOICE_INTERACTION: i32 = Self::FIRST_SYSTEM_WINDOW + 31;

    //noinspection SpellCheckingInspection
    /// 窗口类型：仅由连接的 android.accessibilityservice.AccessibilityService 覆盖的窗口，用于拦截用户交互，而不会更改无障碍服务可以自检的窗口。具体而言，无障碍服务只能自检视力正常的用户可以与之交互的窗口，也就是说，他们可以触摸这些窗口或在这些窗口中键入内容。例如，如果有一个可触摸的全屏无障碍覆盖，那么它下面的窗口即使被可触摸窗口覆盖，也将是无障碍服务可自检的。
    pub const TYPE_ACCESSIBILITY_OVERLAY: i32 = Self::FIRST_SYSTEM_WINDOW + 32;

    /// 窗口类型：语音交互层的起始窗口。
    pub const TYPE_VOICE_INTERACTION_STARTING: i32 = Self::FIRST_SYSTEM_WINDOW + 33;

    /// 用于显示用于调整停靠堆栈大小的句柄的窗口。此窗口归系统进程所有。
    pub const TYPE_DOCK_DIVIDER: i32 = Self::FIRST_SYSTEM_WINDOW + 34;

    /// 窗口类型：类似于 TYPE_APPLICATION_ATTACHED_DIALOG，但由快速设置图块使用。
    pub const TYPE_QS_DIALOG: i32 = Self::FIRST_SYSTEM_WINDOW + 35;

    /// 窗口类型：直接显示在键盘保护上方。此层保留用于屏幕截图动画、区域选择和 UI。在多用户系统中，仅显示在拥有用户的窗口上。
    pub const TYPE_SCREENSHOT: i32 = Self::FIRST_SYSTEM_WINDOW + 36;

    /// 窗口类型：在外部显示器上演示的窗口。
    pub const TYPE_PRESENTATION: i32 = Self::FIRST_SYSTEM_WINDOW + 37;

    /// 窗口类型：应用叠加窗口显示在所有活动窗口（类型介于 FIRST_APPLICATION_WINDOW 和 LAST_APPLICATION_WINDOW 之间）上方，但位于状态栏或 IME 等关键系统窗口下方。系统可能会随时更改这些窗口的位置、大小或可见性，以减少用户的视觉混乱并管理资源。需要 android.Manifest.permission#SYSTEM_ALERT_WINDOW 权限。系统将调整具有此窗口类型的进程的重要性，以降低低内存终止程序终止它们的可能性。在多用户系统中，仅显示在拥有用户的屏幕上。
    pub const TYPE_APPLICATION_OVERLAY: i32 = Self::FIRST_SYSTEM_WINDOW + 38;

    /// 窗口类型：用于在其他窗口上方添加辅助功能窗口放大功能的窗口。这会将窗口置于覆盖窗口中。
    pub const TYPE_ACCESSIBILITY_MAGNIFICATION_OVERLAY: i32 = Self::FIRST_SYSTEM_WINDOW + 39;

    /// 窗口类型：通知栏和键盘保护。只能有一个状态栏窗口；它位于屏幕顶部，所有其他窗口都向下移动，位于其下方。在多用户系统中，显示在所有用户的窗口上。
    pub const TYPE_NOTIFICATION_SHADE: i32 = Self::FIRST_SYSTEM_WINDOW + 40;

    /// 窗口类型：用于在屏幕的非常规部分（即屏幕的左侧或底部）显示状态栏。在多用户系统中显示在所有用户的窗口上。
    pub const TYPE_STATUS_BAR_ADDITIONAL: i32 = Self::FIRST_SYSTEM_WINDOW + 41;

    /// 系统窗口类型的结束。
    pub const LAST_SYSTEM_WINDOW: i32 = 2999;

    /// 当没有合适的类型时在内部使用。
    pub const INVALID_WINDOW_TYPE: i32 = -1;

    #[deprecated(note = "这将被忽略，该值在需要时自动设置。")]
    pub const MEMORY_TYPE_NORMAL: i32 = 0;

    #[deprecated(note = "这将被忽略，该值在需要时自动设置。")]
    pub const MEMORY_TYPE_HARDWARE: i32 = 1;

    #[deprecated(note = "这将被忽略，该值在需要时自动设置。")]
    pub const MEMORY_TYPE_GPU: i32 = 2;

    #[deprecated(note = "这将被忽略，该值在需要时自动设置。")]
    pub const MEMORY_TYPE_PUSH_BUFFERS: i32 = 3;

    /// 窗口标志：只要此窗口对用户可见，就允许在屏幕打开时激活锁定屏幕。这可以单独使用，也可以与 FLAG_KEEP_SCREEN_ON 和/或 FLAG_SHOW_WHEN_LOCKED 结合使用
    pub const FLAG_ALLOW_LOCK_WHILE_SCREEN_ON: u32 = 0x00000001;

    /// 窗口标志：此窗口后面的所有内容都将变暗。使用 dimAmount 来控制暗淡程度。
    pub const FLAG_DIM_BEHIND: u32 = 0x00000002;

    /// 窗口标志：为该窗口启用模糊功能。
    pub const FLAG_BLUR_BEHIND: u32 = 0x00000004;

    /**
     * 窗口标志：此窗口永远不会获得按键输入焦点，因此用户无法向其发送按键或其他按钮事件。这些事件将转至其后面的任何可获得焦点的窗口。此标志还将启用 FLAG_NOT_TOUCH_MODAL，无论是否明确设置。
     * 设置此标志还意味着窗口将不需要与软输入法交互，因此它将按 Z 顺序排列并独立于任何活动输入法定位（通常这意味着它在输入法之上按 Z 顺序排列，因此它可以使用全屏显示其内容并在需要时覆盖输入法。您可以使用 FLAG_ALT_FOCUSABLE_IM 来修改此行为。
     * */
    pub const FLAG_NOT_FOCUSABLE: u32 = 0x00000008;

    /**
     * 窗口标志：此窗口永远不能接收触摸事件。
     * 此标志的目的是将触摸留给该窗口下方的某个窗口来处理（按 Z 顺序）。
     * 从Android Build.VERSION_CODES#S开始，出于安全原因，触摸事件通过包含此标志的窗口(即。在该窗口的边界内)将仅在以下一项(或多项)为真的情况下被递送到该触摸消费窗口：
     * 相同的UID：该窗口属于拥有该触摸消费窗口的同一UID。
     * 受信任的窗口：此窗口受信任。可信窗口包括(但不限于)辅助窗口(TYPE_ACCESSIBILITY_OVERLAY)、输入法(TYPE_INPUT_METHOD)和辅助窗口(TYPE_VOICE_INTERACTION)。类型为_APPLICATION_OVERLAY的窗口不受信任，请参见下文。
     * 不可见窗口：该窗口是视图#消失或视图#不可见。
     * 全透明窗口：此窗口的LayoutParams#Alpha等于0。
     * 一个具有足够透明度的SAW窗口：该窗口的类型为_APPLICATION_OVERLAY，其LayoutParams#Alpha小于或等于最大遮挡不透明度(见下文)，并且它是触摸路径中该UID的唯一类型为_APPLICATION_OVERLAY的窗口。
     * 具有足够透明度的多个SAW窗口：从该UID开始的触摸路径中的多个重叠的TYPE_APPLICATION_OVERLAY窗口具有低于或等于最大遮挡不透明度的组合遮挡不透明度。有关如何计算该值的信息，请参见下面的组合遮挡不透明度一节。
     * 如果这些情况都不成立，则不会传递触摸，并且会将一条消息记录到LogCAT。
     * 最大遮挡不透明度此值为 0.8。如果应用希望从系统收集此值（而不是对其进行硬编码），则可能需要使用 android.hardware.input.InputManager#getMaximumObscuringOpacityForTouch()。
     * 组合遮挡不透明度
     * 一组窗口的组合遮挡不透明度是通过使用结合和交换运算将该集合中所有窗口的不透明度值组合而获得的，定义为：opacity({A,B}) = 1 - (1 - opacity(A))*(1 - opacity(B))，其中 ` `) = 1 - (1 - opacity(W1)) * ... * (1 - opacity(Wn))
     * */
    pub const FLAG_NOT_TOUCHABLE: u32 = 0x00000010;

    /// 窗口标志：即使此窗口可聚焦（其 FLAG_NOT_FOCUSABLE 未设置），也允许将窗口外的任何指针事件发送到其后面的窗口。否则它将自己消耗所有指针事件，无论它们是否在窗口内。
    pub const FLAG_NOT_TOUCH_MODAL: u32 = 0x00000020;

    /// 窗口标志：设置后，如果按下触摸屏时设备处于休眠状态，您将收到此首次触摸事件。通常，首次触摸事件会被系统消耗，因为用户看不到他们按下的是什么。
    #[deprecated(note = "该标志无效。")]
    pub const FLAG_TOUCHABLE_WHEN_WAKING: u32 = 0x00000040;

    /// 窗口标志：只要此窗口对用户可见，就保持设备屏幕开启且明亮。
    pub const FLAG_KEEP_SCREEN_ON: u32 = 0x00000080;

    /**
     * 附加窗口的窗口标志：将窗口放置在整个屏幕内，忽略来自父窗口的任何限制。
     * 注意：在具有displayCutout的显示器上，可以将窗口放置，以便在必要时根据LayoutInDisplayCutOutMode避免显示屏区域。
     * */
    pub const FLAG_LAYOUT_IN_SCREEN: u32 = 0x00000100;

    /// 窗口标志：允许窗口延伸到屏幕之外。
    pub const FLAG_LAYOUT_NO_LIMITS: u32 = 0x00000200;

    /**
     * 窗口标志：显示此窗口时隐藏所有屏幕装饰（例如状态栏）。这允许窗口为自己使用整个显示空间 - 当设置了此标志的应用窗口位于顶层时，状态栏将被隐藏。全屏窗口将忽略窗口的 softInputMode 字段的 SOFT_INPUT_ADJUST_RESIZE 值；窗口将保持全屏并且不会调整大小。
     * 您可以通过 android.R.attr#windowFullscreen 属性在您的主题中控制此标志；此属性会在标准全屏主题中自动为您设置，例如 android.R.style#Theme_NoTitleBar_Fullscreen、android.R.style#Theme_Black_NoTitleBar_Fullscreen、android.R.style#Theme_Light_NoTitleBar_Fullscreen、android.R.style#Theme_Holo_NoActionBar_Fullscreen、android.R.style#Theme_Holo_Light_NoActionBar_Fullscreen、android.R.style#Theme_DeviceDefault_NoActionBar_Fullscreen 和 android.R.style#Theme_DeviceDefault_Light_NoActionBar_Fullscreen。
     * */
    #[deprecated(note = "将WindowInsetsController#hide(int)与Type#statusBars()一起使用。")]
    pub const FLAG_FULLSCREEN: u32 = 0x00000400;

    /// 窗口标志：覆盖 FLAG_FULLSCREEN 并强制显示屏幕装饰（例如状态栏）。
    #[deprecated(note = "该值“意外”成为 API，不应被第三方应用程序使用。")]
    pub const FLAG_FORCE_NOT_FULLSCREEN: u32 = 0x00000800;

    /// 窗口标志：将此窗口合成到屏幕时启用抖动。
    #[deprecated(note = "此标志不再使用。")]
    pub const FLAG_DITHER: u32 = 0x00001000;

    /**
     * 窗口标志：将窗口内容视为安全，防止其出现在屏幕截图中或在非安全显示器上查看。
     * 有关安全表面和安全显示的更多详细信息，请参阅 android.view.Display#FLAG_SECURE。
     * */
    pub const FLAG_SECURE: u32 = 0x00002000;

    /// 窗口标志：一种特殊模式，其中布局参数用于在表面合成到屏幕时执行表面的缩放。
    pub const FLAG_SCALED: u32 = 0x00004000;

    /// 窗口标志：用于用户将屏幕贴在脸上时经常使用的窗口，它将积极过滤事件流以防止在这种情况下意外按压（对于特定窗口可能不是所需的），当检测到这样的事件流时，应用程序将收到一个 CANCEL 运动事件来指示这一点，因此应用程序可以通过在手指释放之前不对事件采取任何操作来相应地处理此问题。
    pub const FLAG_IGNORE_CHEEK_PRESSES: u32 = 0x00008000;

    /// 窗口标志：一个特殊选项，仅与 FLAG_LAYOUT_IN_SCREEN 结合使用。当请求在屏幕上布局时，您的窗口可能会出现在屏幕装饰（例如状态栏）的上方或后面。通过同时包含此标志，窗口管理器将报告所需的插入矩形，以确保您的内容不会被屏幕装饰覆盖。此标志通常由 Window 为您设置，如 Window#setFlags 中所述
    #[deprecated(note = "插图将始终传送到您的应用程序。")]
    pub const FLAG_LAYOUT_INSET_DECOR: u32 = 0x00010000;

    /**
     * 窗口标志：设置后，反转窗口的输入法可聚焦性。
     * 设置此标志的效果取决于是否设置了 FLAG_NOT_FOCUSABLE：如果未设置 FLAG_NOT_FOCUSABLE，即当窗口可聚焦时，设置此标志将阻止此窗口成为输入法的目标。因此，它将无法与输入法交互，并将位于输入法之上（除非其上方有另一个输入法目标）。
     * 如果设置了 FLAG_NOT_FOCUSABLE，则设置此标志会要求窗口成为输入法目标，即使窗口无法聚焦。因此，它将位于输入法之下。注意：设置了 FLAG_NOT_FOCUSABLE 的窗口无法与输入法交互，无论此标志如何。
     * */
    pub const FLAG_ALT_FOCUSABLE_IM: u32 = 0x00020000;

    /// 窗口标志：如果您已设置 FLAG_NOT_TOUCH_MODAL，则可以设置此标志以接收单个特殊 MotionEvent，其动作为 MotionEvent#ACTION_OUTSIDE MotionEvent.ACTION_OUTSIDE，用于发生在窗口外的触摸。请注意，您不会收到完整的向下/移动/向上手势，只会收到第一个向下的位置作为 ACTION_OUTSIDE。
    pub const FLAG_WATCH_OUTSIDE_TOUCH: u32 = 0x00040000;

    /// 窗口标志：特殊标志，允许在屏幕锁定时显示窗口。这将使应用程序窗口优先于键盘保护或任何其他锁定屏幕。可以与 FLAG_KEEP_SCREEN_ON 一起使用，在显示键盘保护窗口之前直接打开屏幕并显示窗口。可以与 FLAG_DISMISS_KEYGUARD 一起使用，以自动完全关闭非安全键盘保护。此标志仅适用于最顶部的全屏窗口。
    #[deprecated(
        note = "请使用 android.R.attr#showWhenLocked 或 android.app.Activity#setShowWhenLocked(boolean) 来防止意外的双重生命周期事件。"
    )]
    pub const FLAG_SHOW_WHEN_LOCKED: u32 = 0x00080000;

    /**
     * 窗口标志：要求系统壁纸显示在窗口后面。窗口表面必须是半透明的，才能真正看到其后面的壁纸；此标志仅确保如果此窗口实际上有半透明区域，壁纸表面就会出现。
     * 您可以通过 android.R.attr#windowShowWallpaper 属性在您的主题中控制此标志；此属性会在标准壁纸主题中自动为您设置，例如 android.R.style#Theme_Wallpaper、android.R.style#Theme_Wallpaper_NoTitleBar、android.R.style#Theme_Wallpaper_NoTitleBar_Fullscreen、android.R.style#Theme_Holo_Wallpaper、android.R.style#Theme_Holo_Wallpaper_NoTitleBar、android.R.style#Theme_DeviceDefault_Wallpaper 和 android.R.style#Theme_DeviceDefault_Wallpaper_NoTitleBar。
     * 设置此标志后，发送到此窗口的所有触摸事件也会发送到壁纸，用于与动态壁纸交互。检查 LayoutParams#areWallpaperTouchEventsEnabled()，默认情况下设置为 ` `。在窗口上显示敏感信息时，如果您想禁用将触摸事件发送到壁纸，请使用 LayoutParams#setWallpaperTouchEventsEnabled(boolean)。
     * */
    pub const FLAG_SHOW_WALLPAPER: u32 = 0x00100000;

    /// 窗口标志：当设置为添加窗口或使窗口可见时，一旦窗口显示出来，系统就会触发电源管理器的用户活动（就像用户唤醒了设备一样）来打开屏幕。
    #[deprecated(
        note = "请使用 android.R.attr#turnScreenOn 或 android.app.Activity#setTurnScreenOn(boolean) 来防止意外的双重生命周期事件。"
    )]
    pub const FLAG_TURN_SCREEN_ON: u32 = 0x00200000;

    /// 窗口标志：设置后，仅当窗口不是安全锁定的键盘保护时，才会关闭键盘保护。由于这种键盘保护不是出于安全考虑，因此当用户导航到另一个窗口时，它将永远不会重新出现（与 FLAG_SHOW_WHEN_LOCKED 相反，FLAG_SHOW_WHEN_LOCKED 只会暂时隐藏安全和非安全键盘保护，但确保当用户移动到不隐藏它们的另一个 UI 时它们会重新出现）。如果键盘保护当前处于活动状态且是安全的（需要解锁凭证），则用户仍需要确认它才能看到此窗口，除非也设置了 FLAG_SHOW_WHEN_LOCKED。
    #[deprecated(
        note = "改用 FLAG_SHOW_WHEN_LOCKED 或 KeyguardManager#requestDismissKeyguard。由于只要窗口上带有此标志的活动处于焦点状态，keyguard 就会一直被关闭，因此 keyguard 无法防止意外触摸屏幕，这是不希望的。"
    )]
    pub const FLAG_DISMISS_KEYGUARD: u32 = 0x00400000;

    /// 窗口标志：设置后，窗口将接受超出其范围的触摸事件，并将其发送到也支持拆分触摸的其他窗口。如果未设置此标志，则第一个向下的指针将确定所有后续触摸将转到哪个窗口，直到所有指针都向上移动。如果设置了此标志，则每个向下的指针（不一定是第一个）将确定该指针的所有后续触摸将转到哪个窗口，直到该指针向上移动，从而允许将多个指针的触摸拆分到多个窗口。
    pub const FLAG_SPLIT_TOUCH: u32 = 0x00800000;

    /**
     * 指示该窗口是否应加速硬件。请求硬件加速并不能保证它会发生。
     * 此标志只能通过编程控制以启用硬件加速。要通过编程为给定窗口启用硬件加速，请执行以下操作：
     * Window w = activity.getWindow(); // 在 Activity 的 onCreate() 中
     * 例如
     * w.setFlags(WindowManager.LayoutParams.FLAG_HARDWARE_ACCELERATED, WindowManager.LayoutParams.FLAG_HARDWARE_ACCELERATED);
     * 重要的是要记住，必须在设置活动或对话框的内容视图之前设置此标志。
     * 在使用 android.R.attr#hardwareAccelerated 在清单中启用硬件加速后，此标志不能用于禁用硬件加速。如果您需要有选择地以编程方式禁用硬件加速（例如用于自动测试），请确保在清单中将其关闭，并在需要时使用上述方法在活动或对话框中启用它。
     * 如果活动或应用程序上的 android.R.attr#hardwareAccelerated android:hardwareAccelerated XML 属性设置为 true，则系统会自动设置此标志。
     * */
    pub const FLAG_HARDWARE_ACCELERATED: u32 = 0x01000000;

    //noinspection SpellCheckingInspection
    /**
     * 窗口标志：允许窗口内容延伸到屏幕的过扫描区域（如果有）。窗口仍应正确定位其内容以将过扫描区域考虑在内。
     * 可以通过android.R.attr#windowOverscan属性在您的主题中控制该标志;该属性是在标准过扫描主题中为您自动设置的，例如android.R.style#Theme_Holo_NoActionBar_Overscan、android.R.style#Theme_Holo_Light_NoActionBar_Overscan、android.R.style#Theme_DeviceDefault_NoActionBar_Overscan和android.R.style#Theme_DeviceDefault_Light_NoActionBar_Overscan。
     * 当为窗口启用此标志时，其正常内容可能会被显示屏的过扫描区域在一定程度上遮挡。为了确保用户可以看到该内容的关键部分，您可以使用 View#setFitsSystemWindows(boolean) View.setFitsSystemWindows(boolean) 在视图层次结构中设置应应用适当偏移的点。（这可以通过直接调用此函数、使用视图层次结构中的 android.R.attr#fitsSystemWindows 属性或实现您自己的 View#fitSystemWindows(android.graphics.Rect) View.fitSystemWindows(Rect) 方法来完成）。
     * 这种定位内容元素的机制与布局和 View#setSystemUiVisibility(int) View.setSystemUiVisibility(int) 的等效用法相同；这里有一个示例布局，它将在设置此过扫描标志的情况下正确定位其 UI 元素：
     * development/samples/ApiDemos/res/layout/overscan_activity.xml 完整版
     * */
    #[deprecated(note = "从Android 11开始，任何Android产品都不再设置过范围的区域。")]
    pub const FLAG_LAYOUT_IN_OVERSCAN: u32 = 0x02000000;

    /**
     * 窗口标志：请求一个半透明的状态栏，并带有系统提供的最少的背景保护。
     * 您可以通过 android.R.attr#windowTranslucentStatus 属性在您的主题中控制此标志；此属性会在标准半透明装饰主题中自动为您设置，例如 android.R.style#Theme_Holo_NoActionBar_TranslucentDecor、android.R.style#Theme_Holo_Light_NoActionBar_TranslucentDecor、android.R.style#Theme_DeviceDefault_NoActionBar_TranslucentDecor 和 android.R.style#Theme_DeviceDefault_Light_NoActionBar_TranslucentDecor。
     * 当为窗口启用此标志时，它会自动设置系统 UI 可见性标志 View#SYSTEM_UI_FLAG_LAYOUT_STABLE 和 View#SYSTEM_UI_FLAG_LAYOUT_FULLSCREEN。
     * 注意：对于支持 android.content.pm.PackageManager#FEATURE_AUTOMOTIVE 的设备，可以忽略此标志。
     * */
    #[deprecated(note = "改用半透明颜色的 Window#setStatusBarColor(int)。")]
    pub const FLAG_TRANSLUCENT_STATUS: u32 = 0x04000000;

    /**
     * 窗口标志：请求一个半透明的导航栏，并带有系统提供的最少的背景保护。
     * 您可以通过 android.R.attr#windowTranslucentNavigation 属性在您的主题中控制此标志；此属性会在标准半透明装饰主题中自动为您设置，例如 android.R.style#Theme_Holo_NoActionBar_TranslucentDecor、android.R.style#Theme_Holo_Light_NoActionBar_TranslucentDecor、android.R.style#Theme_DeviceDefault_NoActionBar_TranslucentDecor 和 android.R.style#Theme_DeviceDefault_Light_NoActionBar_TranslucentDecor。
     * 当为窗口启用此标志时，它会自动设置系统 UI 可见性标志 View#SYSTEM_UI_FLAG_LAYOUT_STABLE 和 View#SYSTEM_UI_FLAG_LAYOUT_HIDE_NAVIGATION。
     * 注意：对于支持 android.content.pm.PackageManager#FEATURE_AUTOMOTIVE 的设备，汽车制造商可以禁用此标志。
     * */
    #[deprecated(note = "改用半透明颜色的 Window#setNavigationBarColor(int)。")]
    pub const FLAG_TRANSLUCENT_NAVIGATION: u32 = 0x08000000;

    /// 本地焦点模式窗口的标志。本地焦点模式窗口可以使用 Window#setLocalFocus(boolean, boolean) 独立于窗口管理器控制焦点。通常，此模式下的窗口不会从窗口管理器获取触摸/按键事件，而只能通过使用 Window#injectInputEvent(InputEvent) 的本地注入来获取事件。
    pub const FLAG_LOCAL_FOCUS_MODE: u32 = 0x10000000;

    /**
     * 窗口标志：允许触摸在手势过程中从一个窗口滑入相邻窗口，而不是在手势持续期间被捕获。
     * 此标志仅更改此窗口的触摸焦点行为。触摸可以滑出窗口但不一定滑回（除非具有触摸焦点的其他窗口允许）。
     * */
    pub const FLAG_SLIPPERY: u32 = 0x20000000;

    /// 窗口标志：当请求使用附加窗口进行布局时，附加窗口可能会与父窗口的屏幕装饰（例如导航栏）重叠。通过包含此标志，窗口管理器将在父窗口的装饰框架内布局附加窗口，使其不与屏幕装饰重叠。
    #[deprecated(note = "使用 setFitInsetsTypes(int) 来确定附加窗口是否与系统栏重叠。")]
    pub const FLAG_LAYOUT_ATTACHED_IN_DECOR: u32 = 0x40000000;

    /// 标志，指示此窗口负责绘制系统栏的背景。如果设置，系统栏将以透明背景绘制，并且此窗口中的相应区域将用Window#getStatusBarColor()和Window#getNavationBarColor()中指定的颜色填充。
    pub const FLAG_DRAWS_SYSTEM_BAR_BACKGROUNDS: u32 = 0x80000000;

    /// 在系统进程中，我们全局不使用硬件加速，因为那里有许多线程在处理 UI，它们之间会发生冲突。如果 UI 的某些部分确实需要使用硬件加速，可以设置此标志来强制使用。这基本上是针对锁屏的。如果其他人也使用它，那么你可能错了。
    pub const PRIVATE_FLAG_FORCE_HARDWARE_ACCELERATED: i32 = 1 << 1;

    /// 默认情况下，壁纸在滚动时会发送新的偏移量。如果壁纸没有执行任何有用的操作（它们不会影响壁纸滚动操作），则可以通过调用 android.service.wallpaper.WallpaperService.Engine#setOffsetNotificationsEnabled(boolean) 选择跳过这些通知。
    pub const PRIVATE_FLAG_WANTS_OFFSET_NOTIFICATIONS: i32 = 1 << 2;

    /// 当设置 LayoutParams#TYPE_APPLICATION_OVERLAY 时，窗口将保持可见，即使为另一个可见窗口设置了 LayoutParams#SYSTEM_FLAG_HIDE_NON_SYSTEM_OVERLAY_WINDOWS。
    pub const PRIVATE_FLAG_SYSTEM_APPLICATION_OVERLAY: i32 = 1 << 3;

    /// 在多用户系统中，如果设置了此标志并且所有者是系统进程，则此窗口将显示在所有用户屏幕上。这将覆盖通常仅显示在所有者用户的屏幕上的窗口类型的默认行为。请参阅每种窗口类型以确定其默认行为。
    pub const SYSTEM_FLAG_SHOW_FOR_ALL_USERS: i32 = 1 << 4;

    /// 标记以允许此窗口具有不受限制的手势排除。
    pub const PRIVATE_FLAG_UNRESTRICTED_GESTURE_EXCLUSION: i32 = 1 << 5;

    /// 永远不要为窗口的位置变化制作动画。
    pub const PRIVATE_FLAG_NO_MOVE_ANIMATION: i32 = 1 << 6;

    /// 窗口标志：特殊标志，用于限制窗口大小为原始大小（[320x480] x 密度）。用于为在兼容模式下运行的应用程序创建窗口。
    pub const PRIVATE_FLAG_COMPATIBLE_WINDOW: i32 = 1 << 7;

    /// 窗口标志：用于系统对话框的特殊选项。设置此标志后，窗口在创建时将无条件地要求焦点。
    pub const PRIVATE_FLAG_SYSTEM_ERROR: i32 = 1 << 8;

    /// 标记以指示仅在必要时才测量窗口的视图层次结构。如果窗口大小可以通过 LayoutParams 获知，我们可以使用该大小来重新布局窗口，并且我们不必在布局视图之前测量视图层次结构。这减少了执行测量的机会。
    pub const PRIVATE_FLAG_OPTIMIZE_MEASURE: i32 = 1 << 9;

    /// 阻止当前窗口后面的壁纸接收触摸事件的标志。
    pub const PRIVATE_FLAG_DISABLE_WALLPAPER_TOUCH_EVENTS: i32 = 1 << 10;

    /// 强制状态栏窗口始终可见的标志。如果在设置此标志时状态栏处于隐藏状态，则将再次显示。这只能通过 LayoutParams#TYPE_STATUS_BAR 设置。
    pub const PRIVATE_FLAG_FORCE_SHOW_STATUS_BAR: i32 = 1 << 11;

    /// 标记表示窗口框架应为添加显示切口框架的请求框架。仅当给定的特定尺寸小于父框架且窗口覆盖显示切口时才适用。扩展框架不会大于父框架。
    pub const PRIVATE_FLAG_LAYOUT_SIZE_EXTENDED_BY_CUTOUT: i32 = 1 << 12;

    /// 此标记将使窗口忽略应用程序可见性，而是完全依赖装饰视图可见性来确定窗口可见性。最近使用此功能在启动应用程序后继续绘制。
    pub const PRIVATE_FLAG_FORCE_DECOR_VIEW_VISIBILITY: i32 = 1 << 13;

    /// 标志表明无论当前的窗口模式配置如何，该子窗口都应始终布局在父框架中。
    pub const PRIVATE_FLAG_LAYOUT_CHILD_WINDOW_IN_PARENT_FRAME: i32 = 1 << 14;

    /// 标志表明此窗口始终绘制状态栏背景，无论其他标志是什么。
    pub const PRIVATE_FLAG_FORCE_DRAW_BAR_BACKGROUNDS: i32 = 1 << 15;

    /// 标记用于指示如果设备支持，此窗口需要持续性能模式。
    pub const PRIVATE_FLAG_SUSTAINED_PERFORMANCE_MODE: i32 = 1 << 16;

    /// 标志，用于指示当此窗口可见时，应用程序进程添加的属于TYPE_TOAST类型的任何窗口或需要android.app.AppOpsManager#OP_SYSTEM_ALERT_WINDOW权限的窗口都应被隐藏。
    pub const SYSTEM_FLAG_HIDE_NON_SYSTEM_OVERLAY_WINDOWS: i32 = 1 << 19;

    /// 表示此窗口是某些设备上存在的圆角覆盖，这意味着它将被排除在外：屏幕截图、屏幕放大和镜像。
    pub const PRIVATE_FLAG_IS_ROUNDED_CORNERS_OVERLAY: i32 = 1 << 20;

    //noinspection SpellCheckingInspection
    /**
     * 标记指示在未缩放的屏幕坐标上计算可放大区域时将排除此窗口，这可以避免放大边界上的剪切。它应该用于不可放大的叠加层。
     * 注意与 PRIVATE_FLAG_NOT_MAGNIFIABLE 不同，此标志不影响放大功能。如果您希望窗口不可放大且不导致剪切，则需要将两者结合起来。
     * */
    pub const PRIVATE_FLAG_EXCLUDE_FROM_SCREEN_MAGNIFICATION: i32 = 1 << 21;

    //noinspection SpellCheckingInspection
    /**
     * 标记以防止窗口被辅助功能放大镜放大。
     * TODO(b/190623172): 这只是暂时的解决办法，需要寻找其他方法。
     * */
    pub const PRIVATE_FLAG_NOT_MAGNIFIABLE: i32 = 1 << 22;

    /// 标记表示状态栏窗口处于强制显示导航栏的状态，除非导航栏窗口明确设置为 View#GONE。仅当通过 LayoutParams#TYPE_STATUS_BAR 设置时才会生效。
    pub const PRIVATE_FLAG_STATUS_FORCE_SHOW_NAVIGATION: i32 = 1 << 23;

    /// 标志表明窗口与颜色空间无关，并且颜色可以解释为任何颜色空间。
    pub const PRIVATE_FLAG_COLOR_SPACE_AGNOSTIC: i32 = 1 << 24;

    /// 标记请求创建 BLAST（缓冲区作为 LayerState）层。如果未指定，客户端将收到 BufferQueue 层。
    pub const PRIVATE_FLAG_USE_BLAST: i32 = 1 << 25;

    /// 标志表示窗口正在控制系统栏的外观。因此我们不需要通过读取其系统 UI 标志来调整它以实现兼容性。
    pub const PRIVATE_FLAG_APPEARANCE_CONTROLLED: i32 = 1 << 26;

    /// 标志表示窗口正在控制系统栏的行为。因此我们不需要通过读取其窗口标志或系统 UI 标志来调整它以实现兼容性。
    pub const PRIVATE_FLAG_BEHAVIOR_CONTROLLED: i32 = 1 << 27;

    /// 标记表示窗口正在自行控制如何适应窗口插入。因此我们无需调整其属性来适应窗口插入。
    pub const PRIVATE_FLAG_FIT_INSETS_CONTROLLED: i32 = 1 << 28;

    /// 标记以表明该窗口是受信任的覆盖。
    pub const PRIVATE_FLAG_TRUSTED_OVERLAY: i32 = 1 << 29;

    /// 标志表明窗口的父框架应该由 IME 插入。
    pub const PRIVATE_FLAG_INSET_PARENT_FRAME_BY_IME: i32 = 1 << 30;

    /**
     * 标记表示我们想要拦截和处理所有用户的全局拖放。此标记允许窗口即使不可见也考虑拖放事件，并将接收系统中所有活动用户的拖放。
     * 使用此标志向窗口提供附加数据，包括 ClipData（包含具有 DragEvent#ACTION_DRAG_STARTED 事件的所有项目）和具有 DragEvent#ACTION_DROP 事件的实际拖动表面。如果窗口消耗了拖放，则拖动表面的清理（作为 DragEvent#ACTION_DROP 的一部分提供）将由窗口承担。
     * */
    pub const PRIVATE_FLAG_INTERCEPT_GLOBAL_DRAG_AND_DROP: i32 = 1 << 31;

    /// 确定此窗口的软输入区域所需的可见性状态的位的 softInputMode 掩码。
    pub const SOFT_INPUT_MASK_STATE: i32 = 0x0f;

    /// softInputMode 的可见性状态：未指定状态。当窗口获得焦点时，系统可能会显示或隐藏软件键盘，以获得更好的用户体验。
    pub const SOFT_INPUT_STATE_UNSPECIFIED: i32 = 0;

    /// softInputMode 的可见性状态：请不要改变软输入区域的状态。
    pub const SOFT_INPUT_STATE_UNCHANGED: i32 = 1;

    /// SoftInputMode的可见性状态：通常合适时（用户向前导航到您的窗口时），请隐藏任何软输入区域）。
    pub const SOFT_INPUT_STATE_HIDDEN: i32 = 2;

    /// softInputMode 的可见性状态：当此窗口获得焦点时，请始终隐藏任何软输入区域。
    pub const SOFT_INPUT_STATE_ALWAYS_HIDDEN: i32 = 3;

    /**
     * softInputMode 的可见性状态：请在正常适当的时候显示软输入区域（当用户向前导航到您的窗口时）。
     * 对于以 android.os.Build.VERSION_CODES#P 及更高版本为目标的应用程序，除非存在一个焦点视图，并且在窗口聚焦时从 View#onCheckIsTextEditor() 返回“ ” ，否则此标志将被忽略。
     * */
    pub const SOFT_INPUT_STATE_VISIBLE: i32 = 4;

    /**
     * SoftInputMode的可见性状态：当此窗口接收输入焦点时，请始终使软输入区域可见。
     * 对于以 android.os.Build.VERSION_CODES#P 及更高版本为目标的应用程序，除非存在一个焦点视图，并且在窗口聚焦时从 View#onCheckIsTextEditor() 返回“ ” ，否则此标志将被忽略。
     * */
    pub const SOFT_INPUT_STATE_ALWAYS_VISIBLE: i32 = 5;

    /// 软输入模式 (softInputMode) 的掩码，用于确定应如何调整窗口以适应软输入窗口。
    pub const SOFT_INPUT_MASK_ADJUST: i32 = 0xf0;

    /// softInputMode 的调整选项：未指定。系统将根据窗口的内容尝试选择其中一个。
    pub const SOFT_INPUT_ADJUST_UNSPECIFIED: i32 = 0x00;

    /// softInputMode 的调整选项：设置为允许在显示输入法时调整窗口大小，以便输入法不会覆盖其内容。这不能与 SOFT_INPUT_ADJUST_PAN 结合使用；如果两者都未设置，则系统将尝试根据窗口的内容选择其中一个。如果窗口的布局参数标志包括 FLAG_FULLSCREEN，则将忽略 softInputMode 的此值；窗口不会调整大小，但会保持全屏显示。
    #[deprecated(
        note = "使用 ` ` 调用 Window#setDecorFitsSystemWindows(boolean) 并在根内容视图上安装一个适合 Type#ime() 类型插入的 OnApplyWindowInsetsListener。"
    )]
    pub const SOFT_INPUT_ADJUST_RESIZE: i32 = 0x10;

    /// softInputMode 的调整选项：设置为在显示输入法时让窗口平移，这样它就不需要处理调整大小，而只需由框架平移以确保当前输入焦点可见。这不能与 SOFT_INPUT_ADJUST_RESIZE 结合使用；如果两者都未设置，则系统将尝试根据窗口的内容选择其中一个。
    pub const SOFT_INPUT_ADJUST_PAN: i32 = 0x20;

    /// softInputMode 的调整选项：设置为不根据显示的输入法调整窗口。窗口不会调整大小，也不会平移以使其焦点可见。
    pub const SOFT_INPUT_ADJUST_NOTHING: i32 = 0x30;

    /// for softInputMode：设置用户将向前导航到窗口时。 通常，该系统会自动为您设置，尽管您可能需要在自己显示窗口时在某些情况下设置它。 显示窗口后，该标志将始终自动清除。
    pub const SOFT_INPUT_IS_FORWARD_NAVIGATION: i32 = 0x100;

    /// screenBrightness 和 buttonBrightness 的默认值表示此窗口的亮度值不会被覆盖，而应该使用正常的亮度策略。
    pub const BRIGHTNESS_OVERRIDE_NONE: f32 = -1.0;

    /// screenBrightness 和 buttonBrightness 的值表示当此窗口位于前面时，屏幕或按钮背光亮度应设置为最低值。
    pub const BRIGHTNESS_OVERRIDE_OFF: f32 = 0.0;

    /// screenBrightness 和 buttonBrightness 的值表示当此窗口位于前面时，屏幕或按钮背光亮度应设置为最高值。
    pub const BRIGHTNESS_OVERRIDE_FULL: f32 = 1.0;

    /// 未指定 rotationAnimation 的值，表示缺乏偏好。
    pub const ROTATION_ANIMATION_UNSPECIFIED: i32 = -1;

    /// rotationAnimation 的值指定此窗口在旋转后将在视觉上旋转进或旋转出。
    pub const ROTATION_ANIMATION_ROTATE: i32 = 0;

    //noinspection SpellCheckingInspection
    /// rotationAnimation 的值指定此窗口在旋转后淡入或淡出。
    pub const ROTATION_ANIMATION_CROSSFADE: i32 = 1;

    //noinspection SpellCheckingInspection
    /// rotationAnimation 的值指定此窗口在旋转后将立即消失或出现。
    pub const ROTATION_ANIMATION_JUMPCUT: i32 = 2;

    //noinspection SpellCheckingInspection
    /// rotationAnimation 的值用于指定无缝旋转模式。其工作原理与 JUMPCUT 类似，但如果无法在不暂停屏幕的情况下应用旋转，则会回退到 CROSSFADE。例如，这对于相机应用来说是理想的选择，这些应用不希望取景器内容旋转或淡入淡出（而是无缝），但在无法应用无缝旋转的应用转换场景中也不希望出现 ROTATION_ANIMATION_JUMPCUT。
    pub const ROTATION_ANIMATION_SEAMLESS: i32 = 3;

    /// 指示此窗口是否希望HDR转换被禁用。
    pub const DISPLAY_FLAG_DISABLE_HDR_CONVERSION: i32 = 1 << 0;

    /**
     * 仅当 DisplayCutout 完全包含在系统栏内或 DisplayCutout 深度不超过 16 dp 时，窗口才可以延伸到 DisplayCutout 区域，但这取决于 OEM 的选择。否则，窗口的布局不会与 DisplayCutout 区域重叠。
     * 实际上，这意味着如果窗口未设置 FLAG_FULLSCREEN 或 View#SYSTEM_UI_FLAG_FULLSCREEN，则如果切口位于顶部边缘，则它可以延伸到纵向切口区域。对于 View#SYSTEM_UI_FLAG_HIDE_NAVIGATION 和屏幕底部的切口，情况也是如此。否则（即全屏或横向），它会被布置成不与切口区域重叠。
     * 通常采取的不与状态栏和导航栏重叠的预防措施足以确保没有重要内容与 DisplayCutout 重叠。
     * 注意：当 DisplayCutout 与系统栏位于不同侧时，OEM 可以选择让窗口始终延伸到 DisplayCutout 区域，无论是否设置了凹口标志，前提是 DisplayCutout 与窗口的重叠部分最多为 16dp。在这种情况下，OEM 必须为用户提供选择加入/退出的选项。
     * android:windowLayoutInDisplayCutoutMode
     * */
    pub const LAYOUT_IN_DISPLAY_CUTOUT_MODE_DEFAULT: i32 = 0;

    /**
     * 窗口始终可以延伸到屏幕短边的 DisplayCutout 区域。
     * 窗口永远不会延伸到屏幕长边上的 DisplayCutout 区域，除非 DisplayCutout 的深度不超过 16 dp，但这取决于 OEM 的选择。
     * 注意：OEM 可以选择让窗口延伸到长边的 DisplayCutout 区域，但前提是凹口与窗口的重叠部分最多为 16dp。在这种情况下，OEM 必须为用户提供选择加入/退出的选项。
     * 窗口必须确保没有重要内容与DisplayCutout重叠。
     * 在此模式下，无论窗口是否隐藏了系统栏，窗口都会在纵向和横向显示屏短边的切口下延伸：
     * 角落中的切口被视为位于短边：
     * 另一方面，如果切口位于显示器的长边上，则会应用信箱，使得窗口不会延伸到任一长边的切口中：
     * 注意：Android 可能不允许内容视图在视图级别与系统栏重叠。要覆盖此行为并允许内容能够延伸到切口区域，请使用 ` ` 调用 Window#setDecorFitsSystemWindows(boolean)。
     * android:windowLayoutInDisplayCutoutMode
     * */
    pub const LAYOUT_IN_DISPLAY_CUTOUT_MODE_SHORT_EDGES: i32 = 1;

    /**
     * 窗口永远不会与DisplayCutout区域重叠。
     * 这应该与暂时设置 View#SYSTEM_UI_FLAG_FULLSCREEN 或 View#SYSTEM_UI_FLAG_HIDE_NAVIGATION 的窗口一起使用，以避免在设置或清除相应标志时重新布局窗口。
     * android:windowLayoutInDisplayCutoutMode
     * */
    pub const LAYOUT_IN_DISPLAY_CUTOUT_MODE_NEVER: i32 = 2;

    /**
     * 窗口始终可以延伸到屏幕所有边缘的 DisplayCutout 区域。
     * 窗口必须确保没有重要内容与DisplayCutout重叠。
     * 在这种模式下，无论窗口是否隐藏了系统栏，窗口都会在纵向和横向显示屏的所有边缘的切口下延伸。
     * 注意：Android 可能不允许内容视图在视图级别与系统栏重叠。要覆盖此行为并允许内容能够延伸到切口区域，请使用 ` ` 调用 Window#setDecorFitsSystemWindows(boolean)。
     * android:windowLayoutInDisplayCutoutMode
     * */
    pub const LAYOUT_IN_DISPLAY_CUTOUT_MODE_ALWAYS: i32 = 3;

    /// 不为该窗口构建输入通道。因此该通道将无法接收输入。
    pub const INPUT_FEATURE_NO_INPUT_CHANNEL: i32 = 1 << 0;

    /// 当此窗口获得焦点时，不会针对所有输入事件调用用户活动，因此应用程序必须自行执行此操作。仅应由键盘锁和手机应用程序使用。仅应由键盘锁和手机应用程序使用。
    pub const INPUT_FEATURE_DISABLE_USER_ACTIVITY: i32 = 1 << 1;

    /// 输入侦测窗口。此窗口将接收其可触摸区域内的所有指针事件，但不会阻止事件按 Z 顺序发送到其下方的其他窗口。输入事件将分派到事件坐标处顶部非侦测窗口上方的所有侦测窗口。
    pub const INPUT_FEATURE_SPY: i32 = 1 << 2;

    pub const LAYOUT_CHANGED: i32 = 1 << 0;

    pub const TYPE_CHANGED: i32 = 1 << 1;

    pub const FLAGS_CHANGED: i32 = 1 << 2;

    pub const FORMAT_CHANGED: i32 = 1 << 3;

    pub const ANIMATION_CHANGED: i32 = 1 << 4;

    pub const DIM_AMOUNT_CHANGED: i32 = 1 << 5;

    pub const TITLE_CHANGED: i32 = 1 << 6;

    pub const ALPHA_CHANGED: i32 = 1 << 7;

    pub const MEMORY_TYPE_CHANGED: i32 = 1 << 8;

    pub const SOFT_INPUT_MODE_CHANGED: i32 = 1 << 9;

    pub const SCREEN_ORIENTATION_CHANGED: i32 = 1 << 10;

    pub const SCREEN_BRIGHTNESS_CHANGED: i32 = 1 << 11;

    pub const ROTATION_ANIMATION_CHANGED: i32 = 1 << 12;

    pub const BUTTON_BRIGHTNESS_CHANGED: i32 = 1 << 13;

    pub const SYSTEM_UI_VISIBILITY_CHANGED: i32 = 1 << 14;

    pub const SYSTEM_UI_LISTENER_CHANGED: i32 = 1 << 15;

    pub const INPUT_FEATURES_CHANGED: i32 = 1 << 16;

    pub const PRIVATE_FLAGS_CHANGED: i32 = 1 << 17;

    pub const USER_ACTIVITY_TIMEOUT_CHANGED: i32 = 1 << 18;

    pub const TRANSLUCENT_FLAGS_CHANGED: i32 = 1 << 19;

    pub const SURFACE_INSETS_CHANGED: i32 = 1 << 20;

    pub const PREFERRED_REFRESH_RATE_CHANGED: i32 = 1 << 21;

    pub const DISPLAY_FLAGS_CHANGED: i32 = 1 << 22;

    pub const PREFERRED_DISPLAY_MODE_ID: i32 = 1 << 23;

    pub const ACCESSIBILITY_ANCHOR_CHANGED: i32 = 1 << 24;

    pub const ACCESSIBILITY_TITLE_CHANGED: i32 = 1 << 25;

    pub const COLOR_MODE_CHANGED: i32 = 1 << 26;

    pub const INSET_FLAGS_CHANGED: i32 = 1 << 27;

    pub const MINIMAL_POST_PROCESSING_PREFERENCE_CHANGED: i32 = 1 << 28;

    pub const BLUR_BEHIND_RADIUS_CHANGED: i32 = 1 << 29;

    pub const PREFERRED_MIN_DISPLAY_REFRESH_RATE: i32 = 1 << 30;

    pub const PREFERRED_MAX_DISPLAY_REFRESH_RATE: i32 = 1 << 31;

    /// 此窗口的 X 位置。使用默认重力时，它会被忽略。使用 Gravity.LEFT 或 Gravity.START 或 Gravity.RIGHT 或 Gravity.END 时，它会提供与给定边缘的偏移量。
    #[java_field]
    pub fn get_x(&self) -> i32 {}

    /// 此窗口的 X 位置。使用默认重力时，它会被忽略。使用 Gravity.LEFT 或 Gravity.START 或 Gravity.RIGHT 或 Gravity.END 时，它会提供与给定边缘的偏移量。
    #[java_field]
    pub fn set_x(&self, value: i32) {}

    /// 此窗口的 Y 位置。使用默认重力时，它会被忽略。使用 Gravity.TOP 或 Gravity.BOTTOM 时，它会提供与给定边缘的偏移量。
    #[java_field]
    pub fn get_y(&self) -> i32 {}

    /// 此窗口的 Y 位置。使用默认重力时，它会被忽略。使用 Gravity.TOP 或 Gravity.BOTTOM 时，它会提供与给定边缘的偏移量。
    #[java_field]
    pub fn set_y(&self, value: i32) {}

    /// 指示将为与这些 LayoutParams 关联的视图水平分配多少额外空间。如果视图不应拉伸，请指定 0。否则，额外的像素将在权重大于 0 的所有视图之间按比例分配。
    #[java_field]
    pub fn get_horizontal_weight(&self) -> f32 {}

    /// 指示将为与这些 LayoutParams 关联的视图水平分配多少额外空间。如果视图不应拉伸，请指定 0。否则，额外的像素将在权重大于 0 的所有视图之间按比例分配。
    #[java_field]
    pub fn set_horizontal_weight(&self, value: f32) {}

    /// 指示将为与这些 LayoutParams 关联的视图垂直分配多少额外空间。如果视图不应拉伸，请指定 0。否则，额外的像素将在权重大于 0 的所有视图之间按比例分配。
    #[java_field]
    pub fn get_vertical_weight(&self) -> f32 {}

    /// 指示将为与这些 LayoutParams 关联的视图垂直分配多少额外空间。如果视图不应拉伸，请指定 0。否则，额外的像素将在权重大于 0 的所有视图之间按比例分配。
    #[java_field]
    pub fn set_vertical_weight(&self, value: f32) {}

    /// 窗口的一般类型。窗口类型主要有三类：
    /// 应用程序窗口（范围从 FIRST_APPLICATION_WINDOW 到 LAST_APPLICATION_WINDOW）是普通的顶层应用程序窗口。对于这些类型的窗口，必须将令牌设置为它们所属活动的令牌（如果令牌为空，通常会为您完成此操作）。
    /// 子窗口（范围从 FIRST_SUB_WINDOW 到 LAST_SUB_WINDOW）与另一个顶层窗口相关联。对于这些类型的窗口，令牌必须是它所附加到的窗口的令牌。
    /// 系统窗口（范围从 FIRST_SYSTEM_WINDOW 到 LAST_SYSTEM_WINDOW）是供系统用于特定目的的特殊类型的窗口。它们通常不应由应用程序使用，并且使用它们需要特殊权限。
    #[java_field]
    pub fn get_type(&self) -> i32 {}

    /// 窗口的一般类型。窗口类型主要有三类：
    /// 应用程序窗口（范围从 FIRST_APPLICATION_WINDOW 到 LAST_APPLICATION_WINDOW）是普通的顶层应用程序窗口。对于这些类型的窗口，必须将令牌设置为它们所属活动的令牌（如果令牌为空，通常会为您完成此操作）。
    /// 子窗口（范围从 FIRST_SUB_WINDOW 到 LAST_SUB_WINDOW）与另一个顶层窗口相关联。对于这些类型的窗口，令牌必须是它所附加到的窗口的令牌。
    /// 系统窗口（范围从 FIRST_SYSTEM_WINDOW 到 LAST_SYSTEM_WINDOW）是供系统用于特定目的的特殊类型的窗口。它们通常不应由应用程序使用，并且使用它们需要特殊权限。
    #[java_field]
    pub fn set_type(&self, value: i32) {}

    /**
     * 如果窗口类型是警报窗口，则返回 true。
     * `type` 窗口类型。如果窗口类型是警报窗口，则返回 true。
     * */
    #[java_method]
    pub fn is_system_alert_window_type(r#type: i32) {}

    #[deprecated(note = "这被忽略了")]
    #[java_field]
    pub fn get_memory_type(&self) -> i32 {}

    #[deprecated(note = "这被忽略了")]
    #[java_field]
    pub fn set_memory_type(&self, value: i32) {}

    /// 各种行为选项/标志。默认为无。
    #[java_field]
    pub fn get_flags(&self) -> u32 {}

    /// 各种行为选项/标志。默认为无。
    #[java_field]
    pub fn set_flags(&self, value: u32) {}

    /// 控制平台私有的标志。
    #[java_field]
    pub fn get_private_flags(&self) -> i32 {}

    /// 控制平台私有的标志。
    #[java_field]
    pub fn set_private_flags(&self, value: i32) {}

    /**
     * 给定一组特定的窗口管理器标志，确定此类窗口在获得焦点时是否可以成为输入法的目标。具体来说，这会检查 FLAG_NOT_FOCUSABLE 和 FLAG_ALT_FOCUSABLE_IM 标志，如果两者的组合对应于可以使用输入法的窗口，则返回 true。
     * 返回:如果具有给定标志的窗口能够使用输入法，则返回 true，否则返回 false。
     * `flags` 当前窗口管理器标志。
     * */
    #[java_method]
    pub fn may_use_input_method(flags: i32) -> bool {}

    /// 任何软输入区域所需的操作模式。可以是以下任意组合：
    /// 可见性状态之一 SOFT_INPUT_STATE_UNSPECIFIED、SOFT_INPUT_STATE_UNCHANGED、SOFT_INPUT_STATE_HIDDEN、SOFT_INPUT_STATE_ALWAYS_HIDDEN、SOFT_INPUT_STATE_VISIBLE 或 SOFT_INPUT_STATE_ALWAYS_VISIBLE。
    /// 调整选项之一 SOFT_INPUT_ADJUST_UNSPECIFIED、SOFT_INPUT_ADJUST_RESIZE、SOFT_INPUT_ADJUST_PAN 或 SOFT_INPUT_ADJUST_NOTHING。
    /// 此标志可以在您的主题中通过 android.R.attr.windowSoftInputMode 属性进行控制。
    #[java_field]
    pub fn get_soft_input_mode(&self) -> i32 {}

    /// 任何软输入区域所需的操作模式。可以是以下任意组合：
    /// 可见性状态之一 SOFT_INPUT_STATE_UNSPECIFIED、SOFT_INPUT_STATE_UNCHANGED、SOFT_INPUT_STATE_HIDDEN、SOFT_INPUT_STATE_ALWAYS_HIDDEN、SOFT_INPUT_STATE_VISIBLE 或 SOFT_INPUT_STATE_ALWAYS_VISIBLE。
    /// 调整选项之一 SOFT_INPUT_ADJUST_UNSPECIFIED、SOFT_INPUT_ADJUST_RESIZE、SOFT_INPUT_ADJUST_PAN 或 SOFT_INPUT_ADJUST_NOTHING。
    /// 此标志可以在您的主题中通过 android.R.attr.windowSoftInputMode 属性进行控制。
    #[java_field]
    pub fn set_soft_input_mode(&self, value: i32) {}

    /// 根据 Gravity 在屏幕内放置窗口。Gravity.apply 和 Gravity.applyDisplay 都在窗口布局期间使用，此值指定为所需重力。例如，您可以在此处指定 Gravity.DISPLAY_CLIP_HORIZONTAL 和 Gravity.DISPLAY_CLIP_VERTICAL 来控制 Gravity.applyDisplay 的行为。
    #[java_field]
    pub fn get_gravity(&self) -> i32 {}

    /// 根据 Gravity 在屏幕内放置窗口。Gravity.apply 和 Gravity.applyDisplay 都在窗口布局期间使用，此值指定为所需重力。例如，您可以在此处指定 Gravity.DISPLAY_CLIP_HORIZONTAL 和 Gravity.DISPLAY_CLIP_VERTICAL 来控制 Gravity.applyDisplay 的行为。
    #[java_field]
    pub fn set_gravity(&self, value: i32) {}

    /// 容器和小部件之间的水平边距，以容器宽度的百分比表示。请参阅 Gravity.apply 以了解如何使用。此字段与 x 一起添加以提供 xAdj 参数。
    #[java_field]
    pub fn get_horizontal_margin(&self) -> f32 {}

    /// 容器和小部件之间的水平边距，以容器宽度的百分比表示。请参阅 Gravity.apply 以了解如何使用。此字段与 x 一起添加以提供 xAdj 参数。
    #[java_field]
    pub fn set_horizontal_margin(&self, value: f32) {}

    /// 容器和小部件之间的垂直边距，以容器高度的百分比表示。请参阅 Gravity.apply 以了解如何使用。此字段与 y 一起添加以提供 yAdj 参数。
    #[java_field]
    pub fn get_vertical_margin(&self) -> f32 {}

    /// 容器和小部件之间的垂直边距，以容器高度的百分比表示。请参阅 Gravity.apply 以了解如何使用。此字段与 y 一起添加以提供 yAdj 参数。
    #[java_field]
    pub fn set_vertical_margin(&self, value: f32) {}

    /// 是否已手动设置表面插入图。设置为 false 时，视图根将自动确定适当的表面插入图。
    #[java_field]
    pub fn get_has_manual_surface_insets(&self) -> bool {}

    /// 是否已手动设置表面插入图。设置为 false 时，视图根将自动确定适当的表面插入图。
    #[java_field]
    pub fn set_has_manual_surface_insets(&self, value: bool) {}

    /// 向窗口报告插入内容时是否应使用全局插入内容状态。设置为 true 时，所有插入内容都将报告给窗口，而不管 z 顺序如何。否则，仅报告给定窗口上方的插入内容。
    #[java_field]
    pub fn get_receive_insets_ignoring_z_order(&self) -> bool {}

    /// 向窗口报告插入内容时是否应使用全局插入内容状态。设置为 true 时，所有插入内容都将报告给窗口，而不管 z 顺序如何。否则，仅报告给定窗口上方的插入内容。
    #[java_field]
    pub fn set_receive_insets_ignoring_z_order(&self, value: bool) {}

    /// 是否应使用先前的表面插入图，而不是当前设置的表面插入图。设置为 true 时，视图根将忽略此对象中的表面插入图并使用其当前拥有的表面插入图。
    #[java_field]
    pub fn get_preserve_previous_surface_insets(&self) -> bool {}

    /// 是否应使用先前的表面插入图，而不是当前设置的表面插入图。设置为 true 时，视图根将忽略此对象中的表面插入图并使用其当前拥有的表面插入图。
    #[java_field]
    pub fn set_preserve_previous_surface_insets(&self, value: bool) {}

    /// 所需的位图格式。可能是 PixelFormat 中的常量之一。格式的选择可能会被 setColorMode(int) 覆盖。默认为 OPAQUE。
    #[java_field]
    pub fn get_format(&self) -> i32 {}

    /// 所需的位图格式。可能是 PixelFormat 中的常量之一。格式的选择可能会被 setColorMode(int) 覆盖。默认为 OPAQUE。
    #[java_field]
    pub fn set_format(&self, value: i32) {}

    /// 定义此窗口要使用的动画的样式资源。这必须是系统资源；它不能是应用程序资源，因为窗口管理器无权访问应用程序。
    #[java_field]
    pub fn get_window_animations(&self) -> i32 {}

    /// 定义此窗口要使用的动画的样式资源。这必须是系统资源；它不能是应用程序资源，因为窗口管理器无权访问应用程序。
    #[java_field]
    pub fn set_window_animations(&self, value: i32) {}

    /// 应用于整个窗口的 alpha 值。alpha 值为 1.0 表示完全不透明，alpha 值为 0.0 表示完全透明
    #[java_field]
    pub fn get_alpha(&self) -> f32 {}

    /// 应用于整个窗口的 alpha 值。alpha 值为 1.0 表示完全不透明，alpha 值为 0.0 表示完全透明
    #[java_field]
    pub fn set_alpha(&self, value: f32) {}

    /// 当设置 FLAG_DIM_BEHIND 时，这是要应用的调光量。范围从 1.0（完全不透明）到 0.0（无调光）。
    #[java_field]
    pub fn get_dim_amount(&self) -> f32 {}

    /// 当设置 FLAG_DIM_BEHIND 时，这是要应用的调光量。范围从 1.0（完全不透明）到 0.0（无调光）。
    #[java_field]
    pub fn set_dim_amount(&self, value: f32) {}

    /// 这可用于覆盖用户偏好的屏幕亮度。小于 0 的值（默认值）表示使用偏好的屏幕亮度。0 到 1 可将亮度从暗调整到全亮。
    #[java_field]
    pub fn get_screen_brightness(&self) -> f32 {}

    /// 这可用于覆盖用户偏好的屏幕亮度。小于 0 的值（默认值）表示使用偏好的屏幕亮度。0 到 1 可将亮度从暗调整到全亮。
    #[java_field]
    pub fn set_screen_brightness(&self, value: f32) {}

    /// 这可用于覆盖按钮和键盘背光的标准行为。小于 0 的值（默认值）表示使用标准背光行为。0 到 1 可将亮度从暗调整到全亮。
    #[java_field]
    pub fn get_button_brightness(&self) -> f32 {}

    /// 这可用于覆盖按钮和键盘背光的标准行为。小于 0 的值（默认值）表示使用标准背光行为。0 到 1 可将亮度从暗调整到全亮。
    #[java_field]
    pub fn set_button_brightness(&self, value: f32) {}

    /// 定义设备旋转时此窗口使用的退出和进入动画。这仅在进入和离开的最顶层不透明窗口设置了 #FLAG_FULLSCREEN 位且未被 其他窗口覆盖 时才会产生影响。所有其他情况都默认为 ROTATION_ANIMATION_ROTATE 行为。
    #[java_field]
    pub fn get_rotation_animation(&self) -> i32 {}

    /// 定义设备旋转时此窗口使用的退出和进入动画。这仅在进入和离开的最顶层不透明窗口设置了 #FLAG_FULLSCREEN 位且未被 其他窗口覆盖 时才会产生影响。所有其他情况都默认为 ROTATION_ANIMATION_ROTATE 行为。
    #[java_field]
    pub fn set_rotation_animation(&self, value: i32) {}

    /// 拥有此窗口的包的名称。
    #[java_field]
    pub fn get_package_name(&self) -> Option<String> {}

    /// 拥有此窗口的包的名称。
    #[java_field]
    pub fn set_package_name(&self, value: Option<String>) {}

    /// 窗口的特定方向值。可以是 ActivityInfo.screenOrientation 允许的任何相同值。如果未设置，将使用默认值 ActivityInfo.SCREEN_ORIENTATION_UNSPECIFIED。
    #[java_field]
    pub fn get_screen_orientation(&self) -> i32 {}

    /// 窗口的特定方向值。可以是 ActivityInfo.screenOrientation 允许的任何相同值。如果未设置，将使用默认值 ActivityInfo.SCREEN_ORIENTATION_UNSPECIFIED。
    #[java_field]
    pub fn set_screen_orientation(&self, value: i32) {}

    /// 窗口的首选刷新率。在 API 34 之前，这必须是窗口所在显示器支持的刷新率之一。所选的刷新率将应用于显示器的默认模式。从 API 34 开始，此值不限于从窗口的显示器获得的支持的刷新率：它可以是窗口打算运行的任何刷新率。任何刷新率都可以作为首选窗口刷新率提供。
    /// 操作系统将选择与 preferredRefreshRate 最匹配的刷新率。设置此值相当于使用 (preferred_frame_rate, Surface.FRAME_RATE_COMPATIBILITY_DEFAULT, Surface.CHANGE_FRAME_RATE_ONLY_IF_SEAMLESS) 调用 Surface.setFrameRate。对于想要指定刷新率但不想为任何其他 displayMode 属性（例如分辨率）指定首选项的应用程序，应使用 preferredDisplayModeId。如果设置了 preferredDisplayModeId，则忽略此值。
    #[java_field]
    pub fn get_preferred_refresh_rate(&self) -> f32 {}

    /// 窗口的首选刷新率。在 API 34 之前，这必须是窗口所在显示器支持的刷新率之一。所选的刷新率将应用于显示器的默认模式。从 API 34 开始，此值不限于从窗口的显示器获得的支持的刷新率：它可以是窗口打算运行的任何刷新率。任何刷新率都可以作为首选窗口刷新率提供。
    /// 操作系统将选择与 preferredRefreshRate 最匹配的刷新率。设置此值相当于使用 (preferred_frame_rate, Surface.FRAME_RATE_COMPATIBILITY_DEFAULT, Surface.CHANGE_FRAME_RATE_ONLY_IF_SEAMLESS) 调用 Surface.setFrameRate。对于想要指定刷新率但不想为任何其他 displayMode 属性（例如分辨率）指定首选项的应用程序，应使用 preferredDisplayModeId。如果设置了 preferredDisplayModeId，则忽略此值。
    #[java_field]
    pub fn set_preferred_refresh_rate(&self, value: f32) {}

    /// 窗口首选显示模式的 ID。这必须是窗口所在显示器支持的模式之一。值为 0 表示无偏好。
    #[java_field]
    pub fn get_preferred_display_mode_id(&self) -> i32 {}

    /// 窗口首选显示模式的 ID。这必须是窗口所在显示器支持的模式之一。值为 0 表示无偏好。
    #[java_field]
    pub fn set_preferred_display_mode_id(&self, value: i32) {}

    /// 窗口处于焦点状态时的最小显示刷新率。如果设置了 preferredDisplayModeId，则此值会被忽略。
    #[java_field]
    pub fn get_preferred_min_display_refresh_rate(&self) -> f32 {}

    /// 窗口处于焦点状态时的最小显示刷新率。如果设置了 preferredDisplayModeId，则此值会被忽略。
    #[java_field]
    pub fn set_preferred_min_display_refresh_rate(&self, value: f32) {}

    /// 窗口处于焦点状态时的最大显示刷新率。如果设置了 preferredDisplayModeId，则此值会被忽略。
    #[java_field]
    pub fn get_preferred_max_display_refresh_rate(&self) -> f32 {}

    /// 窗口处于焦点状态时的最大显示刷新率。如果设置了 preferredDisplayModeId，则此值会被忽略。
    #[java_field]
    pub fn set_preferred_max_display_refresh_rate(&self, value: f32) {}

    /// 控制状态栏的可见性。
    #[deprecated(note = "SystemUiVisibility 标志已弃用。请改用 WindowInsetsController。")]
    #[java_field]
    pub fn get_system_ui_visibility(&self) -> i32 {}

    /// 控制状态栏的可见性。
    #[deprecated(note = "SystemUiVisibility 标志已弃用。请改用 WindowInsetsController。")]
    #[java_field]
    pub fn set_system_ui_visibility(&self, value: i32) {}

    /// 此层次结构中的视图所要求的 UI 可见性。组合值应为 systemUiVisibility | subtreeSystemUiVisibility。
    #[java_field]
    pub fn get_subtree_system_ui_visibility(&self) -> i32 {}

    /// 此层次结构中的视图所要求的 UI 可见性。组合值应为 systemUiVisibility | subtreeSystemUiVisibility。
    #[java_field]
    pub fn set_subtree_system_ui_visibility(&self, value: i32) {}

    /// 获取有关系统 UI 可见性变化的回调。TODO：也许应该有一个我们需要的可选回调的位域。
    #[java_field]
    pub fn get_has_system_ui_listeners(&self) -> bool {}

    /// 获取有关系统 UI 可见性变化的回调。TODO：也许应该有一个我们需要的可选回调的位域。
    #[java_field]
    pub fn set_has_system_ui_listeners(&self, value: bool) {}

    /// 如果有 DisplayCutout，则控制窗口的布局方式。默认为 LAYOUT_IN_DISPLAY_CUTOUT_MODE_DEFAULT。
    #[java_field]
    pub fn get_layout_in_display_cutout_mode(&self) -> i32 {}

    /// 如果有 DisplayCutout，则控制窗口的布局方式。默认为 LAYOUT_IN_DISPLAY_CUTOUT_MODE_DEFAULT。
    #[java_field]
    pub fn set_layout_in_display_cutout_mode(&self, value: i32) {}

    /// 控制向应用进程公开的输入子系统的一组功能。警告：请勿使用 android.os.InputConfig 标志！必须将其设置为 WindowManager.LayoutParams.InputFeatureFlags 中包含的标志值。
    #[java_field]
    pub fn get_input_features(&self) -> i32 {}

    /// 控制向应用进程公开的输入子系统的一组功能。警告：请勿使用 android.os.InputConfig 标志！必须将其设置为 WindowManager.LayoutParams.InputFeatureFlags 中包含的标志值。
    #[java_field]
    pub fn set_input_features(&self, value: i32) {}

    /// 设置此窗口获得焦点时用户活动超时发生前的毫秒数。值为 -1 时使用标准超时。值为 0 时使用最小支持显示超时。此属性只能用于减少用户指定的显示超时；它永远不会使超时时间比正常时间长。仅应由键盘锁和手机应用使用。
    #[java_field]
    pub fn get_user_activity_timeout(&self) -> i64 {}

    /// 设置此窗口获得焦点时用户活动超时发生前的毫秒数。值为 -1 时使用标准超时。值为 0 时使用最小支持显示超时。此属性只能用于减少用户指定的显示超时；它永远不会使超时时间比正常时间长。仅应由键盘锁和手机应用使用。
    #[java_field]
    pub fn set_user_activity_timeout(&self, value: i64) {}

    /// 对于具有锚点的窗口（例如 PopupWindow），跟踪锚定窗口的视图。
    #[java_field]
    pub fn get_accessibility_id_of_anchor(&self) -> i64 {}

    /// 对于具有锚点的窗口（例如 PopupWindow），跟踪锚定窗口的视图。
    #[java_field]
    pub fn set_accessibility_id_of_anchor(&self, value: i64) {}

    /// 窗口标题与标题栏中显示的内容不同步，因此我们单独跟踪当前显示的标题以提供可访问性。
    #[java_field]
    pub fn get_accessibility_title<CS: CharSequence>(&self) -> CS {}

    /// 窗口标题与标题栏中显示的内容不同步，因此我们单独跟踪当前显示的标题以提供可访问性。
    #[java_field]
    pub fn set_accessibility_title<CS: CharSequence>(&self, value: &CS) {}

    /// 设置窗口管理器隐藏窗口的超时时间（以毫秒为单位）。对于诸如提示消息之类的瞬时通知很有用，这样我们就不必依赖客户端的配合来确保窗口被隐藏。必须在创建窗口时指定。请注意，应用程序无法处理未经其明确请求而被移除的窗口，并且可能会尝试与已移除的窗口进行交互，从而导致未定义的行为和崩溃。因此，我们确实会隐藏此类窗口，以防止它们覆盖其他应用程序。
    #[java_field]
    pub fn get_hide_timeout_milliseconds(&self) -> i64 {}

    /// 设置窗口管理器隐藏窗口的超时时间（以毫秒为单位）。对于诸如提示消息之类的瞬时通知很有用，这样我们就不必依赖客户端的配合来确保窗口被隐藏。必须在创建窗口时指定。请注意，应用程序无法处理未经其明确请求而被移除的窗口，并且可能会尝试与已移除的窗口进行交互，从而导致未定义的行为和崩溃。因此，我们确实会隐藏此类窗口，以防止它们覆盖其他应用程序。
    #[java_field]
    pub fn set_hide_timeout_milliseconds(&self, value: i64) {}

    /// 指示此窗口是否希望连接的显示器对生成的图像或视频帧进行最少的后期处理。仅当窗口在屏幕上可见时才会请求此设置。当低延迟比图像增强处理具有更高的优先级时，应使用此设置（例如，对于游戏或视频会议）。如果显示器接收器通过 HDMI 连接，则设备将开始发送启用了自动低延迟模式和游戏内容类型的信息帧。这会将连接的显示器切换到最小图像处理模式（如果可用），从而减少延迟，改善游戏或视频会议应用程序的用户体验。有关更多信息，请参阅 HDMI 2.1 规范。
    /// 如果显示器接收器具有内部连接或使用 HDMI 以外的其他协议，效果可能类似，但由实现定义。切换到具有最少后期处理的模式的功能可以通过系统设置菜单中的用户设置禁用。在这种情况下，此字段将被忽略，显示器将保持其当前模式。
    #[java_field]
    pub fn get_prefer_minimal_post_processing(&self) -> bool {}

    /// 指示此窗口是否希望连接的显示器对生成的图像或视频帧进行最少的后期处理。仅当窗口在屏幕上可见时才会请求此设置。当低延迟比图像增强处理具有更高的优先级时，应使用此设置（例如，对于游戏或视频会议）。如果显示器接收器通过 HDMI 连接，则设备将开始发送启用了自动低延迟模式和游戏内容类型的信息帧。这会将连接的显示器切换到最小图像处理模式（如果可用），从而减少延迟，改善游戏或视频会议应用程序的用户体验。有关更多信息，请参阅 HDMI 2.1 规范。
    /// 如果显示器接收器具有内部连接或使用 HDMI 以外的其他协议，效果可能类似，但由实现定义。切换到具有最少后期处理的模式的功能可以通过系统设置菜单中的用户设置禁用。在这种情况下，此字段将被忽略，显示器将保持其当前模式。
    #[java_field]
    pub fn set_prefer_minimal_post_processing(&self, value: bool) {}

    /// 如果指定，则用于计算相对圆角的框架将是此窗口的框架减去此窗口提供的插入部分。任务栏将在其上方绘制假圆角，因此我们需要此插入部分来计算此窗口的正确圆角。
    #[java_field]
    pub fn get_insets_rounded_corner_frame(&self) -> bool {}

    /// 如果指定，则用于计算相对圆角的框架将是此窗口的框架减去此窗口提供的插入部分。任务栏将在其上方绘制假圆角，因此我们需要此插入部分来计算此窗口的正确圆角。
    #[java_field]
    pub fn set_insets_rounded_corner_frame(&self, value: bool) {}

    /**
     * 指定此窗口在布局期间应避免重叠的插入类型。
     * `types` 哪些 WindowInsets。此窗口应避免的插入类型。此对象的初始值包括所有系统栏。
     * */
    #[java_method]
    pub fn set_fit_insets_types(&self, types: i32) {}

    /**
     * 指定此窗口在布局期间应避免重叠的边。
     * `sides` 此窗口应避免与指定类型的边重叠。此对象的初始值包括所有边。
     * */
    #[java_method]
    pub fn set_fit_insets_sides(&self, sides: i32) {}

    /**
     * 指定此窗口是否应适合窗口插入，无论它们是否可见。
     * `ignore` 如果为 true，此窗口将适合给定的类型，即使它们不可见。
     * */
    #[java_method]
    pub fn set_fit_insets_ignoring_visibility(&self, ignore: bool) {}

    /**
     * 指定应将窗口视为受信任的系统覆盖层。考虑窗口在输入调度期间是否被遮挡时，将忽略受信任的系统覆盖层。需要 android.Manifest.permission.INTERNAL_SYSTEM_WINDOW 权限。
     * @see android.view.MotionEvent#FLAG_WINDOW_IS_OBSCURED
     * @see android.view.MotionEvent#FLAG_WINDOW_IS_PARTIALLY_OBSCURED
     * */
    #[java_method]
    pub fn set_trusted_overlay(&self) {}

    /**
     * 当在 TYPE_APPLICATION_OVERLAY 窗口上设置时，它们保持可见，即使为另一个可见窗口设置了 SYSTEM_FLAG_HIDE_NON_SYSTEM_OVERLAY_WINDOWS。
     * `is_system_application_overlay` 系统应用覆盖。
     * */
    #[java_method]
    pub fn set_system_application_overlay(&self, is_system_application_overlay: bool) {}

    /// 如果此窗口被标记为系统应用程序覆盖，则返回。
    #[java_method]
    pub fn is_system_application_overlay(&self) -> bool {}

    /**
     * 设置是否应向系统壁纸发送触摸事件（可以由第三方应用程序提供），以启用背景中显示壁纸的窗口。默认情况下，这将设置为true。检查flag_show_wallpaper以获取有关显示窗口后面系统壁纸的更多信息。
     * `enable` 是否可以向系统壁纸发送触摸事件。
     * */
    #[java_method]
    pub fn set_wallpaper_touch_events_enabled(&self, enable: bool) {}

    /**
     * 返回是否允许在后台显示壁纸的窗口向系统壁纸（可由第三方应用提供）发送触摸事件。检查 FLAG_SHOW_WALLPAPER 以获取有关在窗口后面显示系统壁纸的更多信息。
     * 返回：是否允许向系统壁纸发送触摸事件。
     * */
    #[java_method]
    pub fn are_wallpaper_touch_events_enabled(&self) -> bool {}

    /**
     * 设置是否可以为该窗口的位置变化播放动画。
     * `enable` 如果禁用，窗口将立即移动到其新位置而不播放动画。
     * */
    #[java_method]
    pub fn set_can_play_move_animation(&self, enable: bool) {}

    /// 是否允许在此窗口上的位置改变期间播放动画。这并不能保证在所有此类情况下都会播放动画。例如，拖动调整大小可能会移动窗口但不播放动画。
    #[java_method]
    pub fn can_play_move_animation(&self) -> bool {}

    /// WindowInsets。此窗口避免重叠的类型。
    #[java_method]
    pub fn get_fit_insets_types(&self) -> i32 {}

    /// 此窗口避免重叠的侧面。
    #[java_method]
    pub fn get_fit_insets_sides(&self) -> i32 {}

    /// 如果此窗口适合窗口插图，则无论它们是否可见，都为 true。
    #[java_method]
    pub fn is_fit_insets_ignoring_visibility(&self) -> bool {}

    #[java_method]
    pub fn for_rotation(&self, rotation: i32) -> Self {}

    #[java_constructor]
    pub fn new() -> Self {}

    #[java_method]
    pub fn set_title<CS: CharSequence>(&self, title: Option<CS>) {}

    #[java_method]
    pub fn get_title<CS: CharSequence>(&self) -> Option<CS> {}

    /**
     * 根据输入视图的高度（视觉 z 位置）设置表面插图。
     * */
    #[java_method]
    pub fn set_surface_insets(&self, view: &View, manual: bool, preserve_previous: bool) {}

    /// 返回窗口是否启用了 HDR 转换
    #[java_method]
    pub fn is_hdr_conversion_enabled(&self) -> bool {}

    /// 启用/禁用窗口的 HDR 转换。默认情况下，窗口的 HDR 转换已启用。
    #[java_method]
    pub fn set_hdr_conversion_enabled(&self, enabled: bool) {}

    /**
     * 设置窗口的颜色模式。设置颜色模式可能会覆盖窗口的像素格式。
     * `color_mode` 颜色模式必须是 ActivityInfo.COLOR_MODE_DEFAULT、ActivityInfo.COLOR_MODE_WIDE_COLOR_GAMUT 或 ActivityInfo.COLOR_MODE_HDR 之一。
     * */
    #[java_method]
    pub fn set_color_mode(&self, color_mode: i32) {}

    /// 返回窗口的颜色模式，可以是 ActivityInfo.COLOR_MODE_DEFAULT、ActivityInfo.COLOR_MODE_WIDE_COLOR_GAMUT 或 ActivityInfo.COLOR_MODE_HDR 之一。
    #[java_method]
    pub fn get_color_mode(&self) -> i32 {}

    /**
     * 使窗口后面的屏幕模糊。效果类似于 dimAmount，但不是变暗，而是使窗口后面的内容模糊（或与暗淡量相结合，如果指定了）。模糊的密度由模糊半径设置。半径定义相邻区域的大小，从中对像素进行平均以形成每个像素的最终颜色。该操作近似于高斯模糊。半径为 0 表示无模糊。半径越大，模糊越密集。
     * 请注意与 Window.setBackgroundBlurRadius 的区别，它仅在窗口范围内模糊。后面模糊会使窗口后面的整个屏幕模糊。需要设置 FLAG_BLUR_BEHIND。由于 GPU 限制，某些设备可能不支持跨窗口模糊。它也可以在运行时禁用，例如在省电模式下、使用多媒体隧道时或请求最少的后期处理时。在这种情况下，不会计算或绘制任何模糊，导致窗口与其后面的内容之间没有深度分离。为了避免这种情况，应用程序可能需要在其窗口上使用更多的 dimAmount。
     * 要监听跨窗口模糊启用/禁用事件，请使用 addCrossWindowBlurEnabledListener。
     * `blur_behind_radius` 用于模糊后方的模糊半径（以像素为单位）
     * */
    #[java_method]
    pub fn set_blur_behind_radius(&self, blur_behind_radius: i32) {}

    /// 返回窗口半径后面的模糊程度。
    #[java_method]
    pub fn get_blur_behind_radius(&self) -> i32 {}

    #[java_method]
    pub fn describe_contents(&self) -> i32 {}

    #[java_method]
    pub fn copy_from(&self, o: &Self) -> i32 {}

    /**
     * 缩放布局参数的坐标和大小。
     * `scale` 缩放系数。
     * */
    #[java_method]
    pub fn scale(&self, scale: f32) {}

    /**
     * 如果布局参数将导致窗口覆盖整个屏幕，则为 True；否则为 false。
     * */
    #[java_method]
    pub fn is_fullscreen(&self) -> bool {}

    /// 如果窗口应自行处理所有指针事件（无论它们是否在窗口内），则为 True。如果窗口是模态窗口，其可触摸区域将扩展至其任务的大小。
    #[java_method]
    pub fn is_modal(&self) -> bool {}
}

/**
 * 提供有关逻辑显示器的大小和密度的信息。显示区域有两种不同的描述方式。应用程序显示区域指定可能包含应用程序窗口的显示器部分，不包括系统装饰。应用程序显示区域可能小于实际显示区域，因为系统减去了状态栏等装饰元素所需的空间。
 * 使用 WindowMetrics.getBounds() 查询应用程序窗口边界。实际显示区域指定应用程序在当前系统状态下可访问的显示器部分。在某些情况下，实际显示区域可能小于显示器的物理尺寸。使用 WindowManager.getCurrentWindowMetrics() 确定活动窗口的当前大小。
 * 与 UI 相关的工作（例如选择 UI 布局）应依赖于 WindowMetrics.getBounds()。有关详细信息，请参阅 getRealSize / getRealMetrics。逻辑显示器不一定代表特定的物理显示设备，例如内部显示器或外部显示器。
 * 根据当前连接的设备以​​及是否启用镜像，逻辑显示器的内容可能会显示在一个或多个物理显示器上。
 * */
#[java_class(name = "android/view/Display")]
pub struct Display;

impl Display {
    /// 默认的 Display ID，假设有一个主显示器，则为主显示器的 ID。
    pub const DEFAULT_DISPLAY: i32 = 0;

    /// 无效的显示 ID。
    pub const INVALID_DISPLAY: i32 = -1;

    /// 分辨率宽度无效。
    pub const INVALID_DISPLAY_WIDTH: i32 = -1;

    /// 分辨率高度无效。
    pub const INVALID_DISPLAY_HEIGHT: i32 = -1;

    /// 刷新率无效。
    pub const INVALID_DISPLAY_REFRESH_RATE: f32 = 0.0;

    /// 默认显示组 ID，假设有一个主显示器，则为主显示器的显示组 ID。
    pub const DEFAULT_DISPLAY_GROUP: i32 = 0;

    /// 显示组 ID 无效。
    pub const INVALID_DISPLAY_GROUP: i32 = -1;

    /// 显示标志：表示显示器支持合成存储在受保护图形缓冲区中的内容。如果设置了此标志，则显示设备支持合成受保护缓冲区。如果未设置此标志，则显示设备可能不支持合成受保护缓冲区；用户可能会在屏幕上看到空白区域而不是受保护的内容。安全 (DRM) 视频解码器可以分配受保护的图形缓冲区，以请求在视频解码器和外部显示接收器之间提供受硬件保护的路径。
    /// 如果没有可用的硬件保护路径，则可能无法合成存储在受保护图形缓冲区中的内容。应用程序可以使用此标志的缺失作为提示，表示它不应该为此显示使用受保护的缓冲区，因为内容可能不可见。例如，如果未设置标志，则应用程序可以选择不在此显示器上显示内容、显示信息性错误消息、选择备用内容流或采用不依赖受保护缓冲区的不同内容解码策略。
    pub const FLAG_SUPPORTS_PROTECTED_BUFFERS: i32 = 1 << 0;

    /// 显示标志：表示显示器具有安全的视频输出并支持合成安全表面。如果设置了此标志，则显示设备具有安全的视频输出并能够显示安全表面。它也可能能够显示 FLAG_SUPPORTS_PROTECTED_BUFFERS 受保护的缓冲区。如果未设置此标志，则显示设备可能没有安全的视频输出；用户可能会在屏幕上看到空白区域，而不是安全表面或受保护缓冲区的内容。
    /// 安全表面用于防止应用程序渲染到这些表面的内容出现在屏幕截图中或在非安全显示器上查看。安全视频解码器使用受保护的缓冲区来实现类似目的。应用程序通过指定 WindowManager.LayoutParams#FLAG_SECURE 窗口标志来创建具有安全表面的窗口。同样，应用程序通过在将安全视图附加到其包含窗口之前调用 SurfaceView#setSecure 来创建具有安全表面的 SurfaceView。应用程序可以使用此标志的缺失来提示它不应在此显示器上创建安全表面或受保护的缓冲区，因为内容可能不可见。例如，如果未设置该标志，则应用程序可以选择不在此显示器上显示内容，显示信息性错误消息，选择备用内容流或采用不依赖安全表面或受保护缓冲区的其他策略来解码内容。
    pub const FLAG_SECURE: i32 = 1 << 1;

    /// 显示标志：表示该显示是私有的。只有拥有该显示的应用和已经在该显示上的应用才能在该显示上创建窗口。
    pub const FLAG_PRIVATE: i32 = 1 << 2;

    /// 显示标志：表示该显示器是演示显示器。此标志标识适合用作演示显示器的辅助显示器，例如外部或无线显示器。应用程序可以自动将其内容投影到演示显示器，以提供更丰富的第二屏幕体验。
    pub const FLAG_PRESENTATION: i32 = 1 << 3;

    /// 显示标志：表示显示屏为圆形。此标志标识显示屏为圆形、椭圆形或其他形状，不允许用户看到显示屏的所有逻辑角落。
    pub const FLAG_ROUND: i32 = 1 << 4;

    /// 显示标志：表示在显示非安全键盘保护时，显示器可以显示其内容。此标志标识如果无需输入凭据即可关闭键盘保护，则辅助显示器将继续显示内容。使用的一个示例是虚拟显示器，其内容显示在系统无法直接看到的外部硬件显示器上。
    /// TODO (b/114338689): 删除该标志并使用 IWindowManager#shouldShowWithInsecureKeyguard
    pub const FLAG_CAN_SHOW_WITH_INSECURE_KEYGUARD: i32 = 1 << 5;

    /// 显示标志：表示显示应显示系统装饰。此标志标识应显示系统装饰的辅助显示，例如状态栏、导航栏、主页活动或 IME。请注意，如果没有 FLAG_TRUSTED，此标志不起作用
    /// TODO (b/114338689): 删除该标志并使用 IWindowManager#setShouldShowSystemDecors
    pub const FLAG_SHOULD_SHOW_SYSTEM_DECORATIONS: i32 = 1 << 6;

    /// 标志：信任该显示器可以显示系统装饰并接收输入，无需用户触摸。
    pub const FLAG_TRUSTED: i32 = 1 << 7;

    /// 标志：表示显示器不应该成为默认 DisplayGroup 的一部分，而应该成为新 DisplayGroup 的一部分。
    pub const FLAG_OWN_DISPLAY_GROUP: i32 = 1 << 8;

    /// 标志：表示显示屏应始终解锁。仅在未在默认显示组中的虚拟显示上有效。
    pub const FLAG_ALWAYS_UNLOCKED: i32 = 1 << 9;

    /// 标志：表示当用户触摸屏幕时，显示器不应播放音效或执行触觉反馈。
    pub const FLAG_TOUCH_FEEDBACK_DISABLED: i32 = 1 << 10;

    /**
     * 标志：表示显示器维持自己的焦点和触摸模式。
     * 此标志在行为上与 com.android.internal.R.bool.config_perDisplayFocusEnabled 类似，但仅适用于特定显示器，而不是系统范围的所有显示器。
     * 注意：必须信任显示器才能拥有自己的焦点。
     * */
    pub const FLAG_OWN_FOCUS: i32 = 1 << 11;

    /**
     * 标志：表示显示器不应通过从另一个显示器窃取顶部焦点而成为顶部焦点显示器。
     * 结果是只有目标输入事件（输入事件的 displayId 与显示器的 displayId 匹配）才能到达此显示器。如果系统仅由一个显示器组成，或者所有显示器都设置了此标志，则设置了此标志的显示器仍可成为顶部聚焦显示器。在这两种情况下，默认显示器都会成为顶部聚焦显示器。
     * 注意：如果显示器是顶部聚焦显示器，或者显示器管理自己的焦点（通过 FLAG_OWN_FOCUS）或所有显示器管理自己的焦点（通过 ` ` 标志），则显示器仅具有聚焦窗口。如果显示器没有聚焦窗口，则不会向其发送任何输入事件。因此，此标志仅与 FLAG_OWN_FOCUS 一起使用才有用，如果未设置，将被忽略。
     * 注意：框架仅支持顶部聚焦显示屏上的 IME（b/262520411）。因此，在显示屏上启用此标志会隐式禁用显示任何 IME。这不是预期的行为，但在实施 b/262520411 之前无法修复。如果您需要在显示屏上使用 IME，请不要设置此标志。
     * */
    pub const FLAG_STEAL_TOP_FOCUS_DISABLED: i32 = 1 << 12;

    /// 显示器标志：表示显示器为后置显示器。此标志标识背对用户的互补显示器。
    pub const FLAG_REAR: i32 = 1 << 13;

    /// 显示标志：表示不应缩放显示内容以适应物理屏幕尺寸。仅用于开发以模拟具有较小物理屏幕的设备，同时保持密度。
    pub const FLAG_SCALING_DISABLED: i32 = 1 << 30;

    /// 显示类型：未知的显示类型。
    pub const TYPE_UNKNOWN: i32 = 0;

    /// 显示器类型：通过内部端口连接的物理显示器。
    pub const TYPE_INTERNAL: i32 = 1;

    /// 显示器类型：通过外部端口连接的物理显示器。
    pub const TYPE_EXTERNAL: i32 = 2;

    /// 显示类型：WiFi显示。
    pub const TYPE_WIFI: i32 = 3;

    /// 显示类型：覆盖显示。
    pub const TYPE_OVERLAY: i32 = 4;

    /// 显示类型：虚拟显示。
    pub const TYPE_VIRTUAL: i32 = 5;

    /// 显示状态：显示状态未知。
    pub const STATE_UNKNOWN: i32 = 0;

    /// 显示状态：显示屏关闭。
    pub const STATE_OFF: i32 = 1;

    /// 显示状态：显示屏亮。
    pub const STATE_ON: i32 = 2;

    /// 显示状态：显示器在低功耗状态下处于打瞌睡状态；它仍然处于开启状态，但针对在设备非交互时显示系统提供的内容进行了优化。
    pub const STATE_DOZE: i32 = 3;

    /// 显示状态：显示器处于挂起低功耗状态，处于休眠状态；它仍处于开启状态，但 CPU 不会更新它。这可以用于以下两种情况之一：在设备处于非交互状态时显示系统提供的静态内容，或允许“Sidekick”计算资源更新显示。因此，CPU 不得在此模式下控制显示器。
    pub const STATE_DOZE_SUSPEND: i32 = 4;

    /// 显示状态：显示屏已开启并针对 VR 模式进行了优化。
    pub const STATE_VR: i32 = 5;

    /// 显示状态：显示器处于挂起的全功率状态；它仍然打开，但 CPU 不会更新它。这可以以两种方式之一使用：在设备处于非交互状态时显示系统提供的静态内容，或允许“Sidekick”计算资源更新显示。因此，CPU 不得在此模式下控制显示器。
    pub const STATE_ON_SUSPEND: i32 = 6;

    /* 下面定义的颜色模式常量必须与 system/core/include/system/graphics-base.h 中的常量保持同步 */

    /// 显示颜色模式：当前颜色模式未知或无效。
    pub const COLOR_MODE_INVALID: i32 = -1;

    /// 显示色彩模式：显示器的默认或原生色域。
    pub const COLOR_MODE_DEFAULT: i32 = 0;

    pub const COLOR_MODE_BT601_625: i32 = 1;

    pub const COLOR_MODE_BT601_625_UNADJUSTED: i32 = 2;

    pub const COLOR_MODE_BT601_525: i32 = 3;

    pub const COLOR_MODE_BT601_525_UNADJUSTED: i32 = 4;

    pub const COLOR_MODE_BT709: i32 = 5;

    pub const COLOR_MODE_DCI_P3: i32 = 6;

    pub const COLOR_MODE_SRGB: i32 = 7;

    pub const COLOR_MODE_ADOBE_RGB: i32 = 8;

    pub const COLOR_MODE_DISPLAY_P3: i32 = 9;

    /// 表示当显示屏被移除时，其所有活动将移至主显示屏，并且最顶层的活动将成为焦点。
    /// TODO (b/114338689): 删除该标志并使用 WindowManager#REMOVE_CONTENT_MODE_MOVE_TO_PRIMARY
    pub const REMOVE_MODE_MOVE_CONTENT_TO_PRIMARY: i32 = 0;

    /// 表示当display被移除时，其所有堆栈和任务都将被移除，所有活动将按照通常的生命周期被销毁。
    /// TODO (b/114338689): 删除该标志并使用 WindowManager#REMOVE_CONTENT_MODE_DESTROY
    pub const REMOVE_MODE_DESTROY_CONTENT: i32 = 1;

    pub const DISPLAY_MODE_ID_FOR_FRAME_RATE_OVERRIDE: i32 = 0xFF;

    /**
     * 获取显示 ID。
     * 每个逻辑显示都有一个唯一 ID。默认显示 ID 为 DEFAULT_DISPLAY。
     * */
    #[java_method]
    pub fn get_display_id(&self) -> i32 {}

    /**
     * 获取显示器唯一 ID。
     * 唯一 ID 与显示器 ID 不同，因为物理显示器在重新启动后具有稳定的唯一 ID。
     * */
    #[java_method]
    pub fn get_unique_id(&self) -> Option<String> {}

    /**
     * 如果此显示仍然有效，则返回 true；如果显示已被移除，则返回 false。如果显示无效，则此类的方法将继续报告最近观察到的显示信息。但是，在显示消亡后继续使用 Display 对象是不明智的（而且毫无意义）。如果重新连接具有相同 ID 的显示，则之前无效的显示可能会再次有效。
     * 返回：如果显示仍然有效，则返回 True。
     * */
    #[java_method]
    pub fn is_valid(&self) -> bool {}

    /**
     * 获取显示器的层堆栈。每个显示器都有自己独立的层堆栈，表面放置在其上，由表面投射器进行管理。
     * 返回：显示器的层堆栈编号。
     * */
    #[java_method]
    pub fn get_layer_stack(&self) -> i32 {}

    /**
     * 返回描述显示器功能的标志组合。
     * 返回：显示标志。
     * */
    #[java_method]
    pub fn get_flags(&self) -> i32 {}

    /**
     * 获取显示类型。
     * 返回：显示类型。
     * */
    #[java_method]
    pub fn get_type(&self) -> i32 {}

    /**
     * 获取拥有此显示屏的应用程序的 UID，如果显示屏归系统所有，则获得零。如果显示屏是私有的，则只有所有者可以使用它。
     * */
    #[java_method]
    pub fn get_owner_uid(&self) -> i32 {}

    /**
     * 获取拥有此显示屏的应用程序的软件包名称，如果显示屏归系统所有，则返回 null。如果显示屏是私有的，则只有所有者可以使用它。
     * */
    #[java_method]
    pub fn get_owner_package_name(&self) -> Option<String> {}

    /**
     * 获取显示器的名称。请注意，某些显示器可能会被用户重命名。
     * 返回：显示器的名称。
     * */
    #[java_method]
    pub fn get_name(&self) -> String {}

    /**
     * 获取显示器配置的默认亮度。
     * 返回：默认亮度介于 0.0-1.0 之间
     * */
    #[java_method]
    pub fn get_brightness_default(&self) -> f32 {}

    /**
     * 返回将发生的最大屏幕尺寸。这主要用于壁纸。
     * */
    #[java_method]
    pub fn get_maximum_size_dimension(&self) -> i32 {}

    #[deprecated(note = "改用 WindowMetrics.getBounds.width()。")]
    #[java_method]
    pub fn get_width(&self) -> i32 {}

    #[deprecated(note = "改用#height()。")]
    #[java_method]
    pub fn get_height(&self) -> i32 {}

    /**
     * 返回屏幕从其“自然”方向的旋转。返回值可能是 Surface.ROTATION_0（无旋转）、Surface.ROTATION_90、Surface.ROTATION_180 或 Surface.ROTATION_270。
     * 例如，如果设备具有自然高大的屏幕，并且用户已将其侧放以进入横向方向，则此处返回的值可能是 Surface.ROTATION_90 或 Surface.ROTATION_270，具体取决于旋转的方向。角度是屏幕上绘制图形的旋转，与设备的物理旋转方向相反。例如，如果设备逆时针旋转 90 度，为了补偿渲染将顺时针旋转 90 度，因此此处返回的值将是 Surface.ROTATION_90。此旋转值将与 getMetrics 的结果相匹配：这意味着如果通过活动访问，旋转值将与活动相对应。
     * */
    #[java_method]
    pub fn get_rotation(&self) -> i32 {}

    /// 返回显示器的安装方向。
    #[java_method]
    pub fn get_install_orientation(&self) -> i32 {}

    /// 返回：此显示的方向。
    #[deprecated(note = "使用 getRotation")]
    #[java_method]
    pub fn get_orientation(&self) -> i32 {}

    /**
     * 获取显示器的像素格式。
     * 返回：PixelFormat 中定义的常量之一。
     * */
    #[deprecated(note = "此方法不再受支持。结果始终为 PixelFormat。RGBA_8888。")]
    #[java_method]
    pub fn get_pixel_format(&self) -> i32 {}

    /**
     * 获取此显示器的刷新率（以每秒帧数为单位）。
     * */
    #[java_method]
    pub fn get_refresh_rate(&self) -> f32 {}

    /**
     * 如果可以将连接的显示器切换到使用最小的后处理方式的模式，则返回true。如果显示器接收器通过HDMI连接，则如果显示屏支持自动低潜伏期模式或游戏内容类型，则此方法将返回true。如果显示器接收器具有内部连接或使用HDMI以外的其他协议，则如果可以将接收器切换到实现定义的低延迟图像处理模式，则此方法将返回true。
     * 通过系统设置菜单中的用户设置，可以禁用使用最小后处理模式的模式的能力。在这种情况下，此方法返回false。
     * */
    #[java_method]
    pub fn is_minimal_post_processing_supported(&self) -> bool {}

    /**
     * 请求显示器应用颜色模式。
     * `color_mode` 颜色模式。
     * */
    #[java_method]
    pub fn request_color_mode(&self, color_mode: i32) {}

    /**
     * 返回此显示的活跃颜色模式
     * */
    #[java_method]
    pub fn get_color_mode(&self) -> i32 {}

    /// 获取显示屏的当前移除模式 - 移除显示屏内容时应对其执行哪些操作。在这种情况下，公共显示屏的默认行为是将所有活动移至主显示屏并使其处于焦点状态。对于私人显示屏 - 销毁所有活动。
    /// TODO (b/114338689): 删除方法并使用 IWindowManager#getRemoveContentMode
    #[java_method]
    pub fn get_remove_mode(&self) -> i32 {}

    /**
     * 返回此显示器是否支持任何 HDR 类型。
     * */
    #[java_method]
    pub fn is_hdr(&self) -> bool {}

    /**
     * 显示器是否支持报告 hdr/sdr 比率。如果为 false，则 getHdrSdrRatio() 将始终为 1.0f
     * */
    #[java_method]
    pub fn is_hdr_sdr_ratio_available(&self) -> bool {}

    /**
     * 删除显示器的用户首选显示模式。
     * */
    #[java_method]
    pub fn clear_user_preferred_display_mode(&self) {}

    /**
     * 返回此显示器是否可用于显示广色域内容。这并不一定意味着设备本身可以渲染广色域内容。要确保可以生成广色域内容，请参阅 Configuration.isScreenWideColorGamut()。
     * */
    #[java_method]
    pub fn is_wide_color_gamut(&self) -> bool {}

    /**
     * 获取应用 VSYNC 偏移量（以纳秒为单位）。这是一个正值，表示 Choreographer 提供的 VSYNC 事件相对于显示刷新的相位偏移量。
     * 例如，如果 Choreographer 报告刷新发生在时间 N，则它实际上发生在 (N - appVsyncOffset)。应用通常不需要知道这一点。它仅适用于细粒度的 A/V 同步。
     * */
    #[java_method]
    pub fn get_app_vsync_offset_nanos(&self) -> i64 {}

    //noinspection SpellCheckingInspection
    /**
     * 这是缓冲区必须提前多久排队等待在给定时间进行演示。如果您希望缓冲区在时间 N 出现在屏幕上，则必须在 (N - presentationDeadline) 之前提交缓冲区。
     * 可以使用 android.opengl.EGLExt.eglPresentationTimeANDROID 设置 GLES 渲染所需的演示时间。对于视频解码，请使用 android.media.MediaCodec.releaseOutputBuffer(int, long)。时间以纳秒为单位表示，使用系统单调时钟 (System.nanoTime)。
     * */
    #[java_method]
    pub fn get_presentation_deadline_nanos(&self) -> i64 {}

    /**
     * 确定是否应将 WindowConfiguration.getMaxBounds() 报告为显示尺寸。当应用需要沙盒化时，最大边界字段可能小于逻辑尺寸。取决于 com.android.server.wm.ConfigurationContainer.providesMaxBounds() 中设置的 WindowConfiguration.getMaxBounds()。
     * 在大多数情况下，此值反映当前 DisplayArea 的大小。当应应用最大边界时，返回 true。
     * */
    #[java_method]
    pub fn should_report_max_bounds(&self) -> bool {}

    /**
     * 获取显示器的状态，例如是打开还是关闭。
     * 返回：显示器的状态：STATE_OFF、STATE_ON、STATE_DOZE、STATE_DOZE_SUSPEND、STATE_ON_SUSPEND 或 STATE_UNKNOWN 之一。
     * */
    #[java_method]
    pub fn get_state(&self) -> i32 {}

    /**
     * 如果指定的 UID 有权访问此显示，则返回 true。
     * `uid` UID。
     * */
    #[java_method]
    pub fn has_access(&self, uid: i32) -> bool {}

    /// 如果显示是公共演示文稿显示，则返回true。
    #[java_method]
    pub fn is_public_presentation(&self) -> bool {}

    /// 如果显示器是受信任的显示器，则为 true。
    #[java_method]
    pub fn is_trusted(&self) -> bool {}

    /// 如果显示器可以从另一个显示器窃取顶部焦点，则为 true。
    #[java_method]
    pub fn can_steal_top_focus(&self) -> bool {}

    #[java_method]
    pub fn type_to_string(r#type: i32) -> String {}

    #[java_method]
    pub fn state_to_string(state: i32) -> String {}

    /**
     * 如果在指定的显示器电源状态下可以暂停显示更新，则返回 true。在 SUSPEND 状态下，绝对禁止更新。
     * `state` 状态。
     * */
    #[java_method]
    pub fn is_suspended_state(state: i32) -> bool {}

    /**
     * 如果在指定的显示器电源状态下显示器可能处于降低的操作模式，则返回 true。
     * `state` 状态。
     * */
    #[java_method]
    pub fn is_doze_state(state: i32) {}

    /**
     * 如果显示器处于活动状态（例如 STATE_ON 或 STATE_VR），则返回 true。
     * `state` 状态。
     * */
    #[java_method]
    pub fn is_active_state(state: i32) {}

    /**
     * 如果显示器处于关闭状态（例如 STATE_OFF），则返回 true。
     * `state` 状态。
     * */
    #[java_method]
    pub fn is_off_state(state: i32) -> bool {}

    /**
     * 如果显示器处于开启状态（例如 STATE_ON 或 STATE_VR 或 STATE_ON_SUSPEND），则返回 true。
     * `state` 状态。
     * */
    #[java_method]
    pub fn is_on_state(state: i32) -> bool {}

    /**
     * 如果指定的宽度有效，则返回 true。
     * `width` 宽度。
     * */
    #[java_method]
    pub fn is_width_valid(width: i32) -> bool {}

    /**
     * 如果指定的高度有效，则返回 true。
     * `height` 高度。
     * */
    #[java_method]
    pub fn is_height_valid(height: i32) -> bool {}

    /**
     * 如果指定的刷新率有效，则返回 true。
     * `refresh_rate` 刷新率。
     * */
    #[java_method]
    pub fn is_refresh_rate_valid(refresh_rate: f32) -> bool {}
}

#[cfg(feature = "test_android_view")]
pub fn test() {
    use crate::{
        android::app::Activity,
        java::lang::{CharSequenceExt, CharSequenceImpl},
    };
    let context: Context = (&Activity::fetch()).into();
    let view = View::new(&context);
    assert!(view.to_string().starts_with("android.view.View"));
    view.announce_for_accessibility(&"通知".to_char_sequence::<CharSequenceImpl>());
    view.request_focus();
    view.clear_focus();
    assert_eq!(None, view.find_focus());
    view.set_activated(true);
    assert_eq!(true, view.is_activated());
    view.set_x(20f32);
    assert_eq!(20f32, view.get_x());
    view.set_y(30f32);
    assert_eq!(30f32, view.get_y());
    assert_eq!(None, view.find_view_by_id(0));
    view.set_content_description(Some("测试".to_char_sequence::<CharSequenceImpl>()));
    assert_eq!(
        Some("测试".to_char_sequence()),
        view.get_content_description::<CharSequenceImpl>()
    );
    view.set_id(3);
    assert_eq!(3, view.get_id());
    let l = View_OnClickListenerImpl::from_fn(|_| {
        println!("View is clicked.");
    });
    view.set_on_click_listener(l.as_ref());
    view.set_visibility(View::GONE);
    assert_eq!(View::GONE, view.get_visibility());

    let params = ViewGroup_LayoutParams::new(
        ViewGroup_LayoutParams::MATCH_PARENT,
        ViewGroup_LayoutParams::MATCH_PARENT,
    );
    params.set_width(40);
    assert_eq!(40, params.get_width());
    params.set_height(50);
    assert_eq!(50, params.get_height());
    let params = ViewGroup_MarginLayoutParams::from_layout_params(&params);
    params.set_left_margin(20);
    assert_eq!(20, params.get_left_margin());
    params.set_top_margin(10);
    assert_eq!(10, params.get_top_margin());
    params.set_right_margin(30);
    assert_eq!(30, params.get_right_margin());
    params.set_bottom_margin(40);
    assert_eq!(40, params.get_bottom_margin());
    view.set_layout_params(&params);
    assert!(view.get_layout_params().is_some());
    let params = WindowManager_LayoutParams::new();
    params.set_flags(WindowManager_LayoutParams::FLAG_ALT_FOCUSABLE_IM);
    assert_eq!(
        WindowManager_LayoutParams::FLAG_ALT_FOCUSABLE_IM,
        params.get_flags()
    );
}
