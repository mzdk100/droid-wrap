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
    android::{content::Context, os::Bundle},
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
     * 注册一个回调，当此视图被点击并按住时调用。如果此视图不是长按可点击的，则变为长按可点击的。
     * `l` 将运行的回调
     * */
    #[java_method]
    pub fn set_on_long_click_listener<L: View_OnLongClickListener + JProxy>(&self, l: &L) {}

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

    /**
     * 如果已定义，则调用此视图的 OnClickListener。执行与点击相关的所有常规操作：报告可访问性事件、播放声音等。
     * 返回：如果已分配一个被调用的 OnClickListener，则返回 True，否则返回 false。
     * */
    #[java_method]
    pub fn perform_click(&self) -> bool {}

    /**
     * 指示此视图是否对点击事件作出反应。如果视图可点击，则返回 true，否则返回 false
     * */
    #[java_method]
    pub fn is_clickable(&self) -> bool {}

    /**
     * 启用或禁用此视图的点击事件。当视图可点击时，每次点击时它的状态都会更改为“按下”。子类应将视图设置为可点击，以便对用户的点击做出视觉反应。
     * `clickable` true 表示视图可点击，否则为 false
     * */
    #[java_method]
    pub fn set_clickable(&self, clickable: bool) {}

    /**
     * 禁用时启用或禁用此视图的单击事件。
     * `clickable_when_disabled` true使视图可单击，否则为false
     * */
    #[java_method]
    pub fn set_allow_click_when_disabled(&self, clickable_when_disabled: bool) {}

    /**
     * 指示此视图是否对长按事件作出反应。
     * 返回：如果视图可长按，则返回 true，否则返回 false
     * */
    #[java_method]
    pub fn is_long_clickable(&self) -> bool {}

    /**
     * 启用或禁用此视图的长按事件。当视图可长按时，它会对用户按住按钮的时间长于点击做出反应。此事件可以启动侦听器或上下文菜单。
     * `long_clickable` true 表示视图可长按，否则为 false
     * */
    #[java_method]
    pub fn set_long_clickable(&self, long_clickable: bool) {}

    /**
     * 指示此视图是否对上下文点击反应。
     * 返回：如果视图是上下文可单击的，则为false，否则否则
     * */
    #[java_method]
    pub fn is_context_clickable(&self) -> bool {}

    /**
     * 启用或禁用此视图的上下文点击。此事件可以启动侦听器。
     * `context_clickable` true 表示使视图对上下文点击做出反应，否则为 false
     * */
    #[java_method]
    pub fn set_context_clickable(&self, context_clickable: bool) {}

    /**
     * 返回此视图是否具有附加的 OnClickListener。如果有侦听器，则返回 true，如果没有，则返回 false。
     * */
    #[java_method]
    pub fn has_on_click_listeners(&self) -> bool {}

    /**
     * 返回此视图是否具有附加的 OnLongClickListener。如果有侦听器，则返回 true，如果没有，则返回 false。
     * */
    #[java_method]
    pub fn has_on_long_click_listeners(&self) -> bool {}

    /**
     * 返回屏幕是否应保持开启，对应于 KEEP_SCREEN_ON 的当前值。
     * 返回：如果设置了 KEEP_SCREEN_ON，则返回 true。
     * */
    #[java_method]
    pub fn get_keep_screen_on(&self) -> bool {}

    /**
     * 控制屏幕是否应保持开启，修改 KEEP_SCREEN_ON 的值。
     * `keep_screen_on` 提供 true 以设置 KEEP_SCREEN_ON。
     * */
    #[java_method]
    pub fn set_keep_screen_on(keep_screen_on: bool) {}

    /**
     * 获取此视图的父级。请注意，父级是 ViewParent，不一定是 View。
     * 返回：此视图的父级。
     * */
    #[java_method]
    pub fn get_parent<VP: ViewParent>(&self) -> Option<VP> {}
}

/// 定义视图父类的职责。这是视图想要与其父类交互时看到的 API。
#[java_interface(name = "android/view/ViewParent")]
pub trait ViewParent {
    #[doc(hidden)]
    type VP: ViewParent;

    /// 当发生某些更改导致此视图父级的子级布局无效时调用。这将安排视图树的布局过程。
    fn request_layout(&self);

    /// 指示是否在此视图父级上请求布局。
    /// 返回：如果请求布局，则返回 true，否则返回 false
    fn is_layout_requested(&self) -> bool;

    /**
     * 当子视图希望视图层次结构收集透明区域并将其报告给窗口合成器时调用。在视图层次结构中“打”洞的视图（例如 SurfaceView）可以使用此 API 来提高系统的性能。当层次结构中不存在此类视图时，此优化是不必要的，并且可能会略微降低视图层次结构的性能。
     * `child` 请求透明区域计算的视图
     * */
    fn request_transparent_region(&self, child: &View);

    /**
     * 如果父级存在则返回该父级，否则返回 null。
     * 返回：ViewParent，如果此 ViewParent 没有父级则返回 null。
     * */
    fn get_parent(&self) -> Option<Self::VP>;

    /**
     * 当此父级的子级需要焦点时调用
     * `child` 此 ViewParent 需要焦点的子级。此视图将包含焦点视图。它不一定是实际具有焦点的视图。
     * `focused` 实际具有焦点的子级后代视图
     * */
    fn request_child_focus(&self, child: &View, focused: &View);

    /**
     * 告诉视图层次结构需要重新评估全局视图属性。
     * `child` 属性已更改的视图。
     * */
    fn recompute_view_attributes(&self, child: &View);

    /**
     * 当此父级的子级放弃焦点时调用
     * `child` 放弃焦点的视图
     * */
    fn clear_child_focus(&self, child: &View);

    /**
     * 查找指定方向上想要获得焦点的最近视图
     * `v` 当前获得焦点的视图
     * `direction` FOCUS_UP、FOCUS_DOWN、FOCUS_LEFT 和 FOCUS_RIGHT 之一
     * */
    fn focus_search(&self, v: &View, direction: i32) -> View;

    /**
     * 更改子项的 z 顺序，使其位于所有其他子项之上。如果此容器使用顺序相关的布局方案（例如 LinearLayout），则此顺序更改可能会影响布局。在 android.os.Build.VERSION_CODES.KITKAT 之前，此方法应随后调用此父项的 requestLayout() 和 View.invalidate()，以强制父项使用新的子项顺序重新绘制。
     * `child` 要置于 z 顺序顶部的子项
     * */
    fn bring_child_to_front(&self, child: &View);

    /**
     * 显示指定视图或其祖先的上下文菜单。在大多数情况下，子类不需要覆盖此。但是，如果将子类直接添加到窗口管理器（例如，ViewManager.addView(View, ViewGroup.LayoutParams)），则它应该覆盖此并显示上下文菜单。
     * 返回：如果显示上下文菜单，则返回 true，否则返回 false
     * `original_view` 首次调用上下文菜单的源视图
     * */
    fn show_context_menu_for_child(&self, original_view: &View) -> bool;

    /**
     * 当子项的可绘制状态发生改变时，将在父项上调用此方法。
     * `child` 可绘制状态发生改变的子项。
     * */
    fn child_drawable_state_changed(&self, child: &View);

    /**
     * 当子级不希望此父级及其祖先使用 ViewGroup.onInterceptTouchEvent(MotionEvent) 拦截触摸事件时调用。此父级应将此调用传递给其父级。此父级必须在触摸期间遵守此请求（即，仅在此父级收到向上或取消后清除标志。
     * `disallow_intercept` 如果子级不希望父级拦截触摸事件，则为 True。
     * */
    fn request_disallow_intercept_touch_event(&self, disallow_intercept: bool);

    /**
     * 当子视图现在具有或不再跟踪瞬态时调用。 “瞬态”是视图可能持有的任何状态，但不应反映在视图当前呈现的数据模型中。此状态仅影响视图本身向用户呈现的内容，例如正在进行的动画的当前状态或文本选择操作的状态。瞬态可用于向视图系统的其他组件提示特定视图正在跟踪复杂但封装的内容。例如，ListView 可能承认具有瞬态的列表项视图应保留在其位置或稳定项 ID 中，而不是将该视图视为可由后备适配器轻松替换。
     * 这使得适配器实现更简单，而不需要跟踪正在进行的项目视图动画的状态，以便在意外回收和重新绑定附加项目视图时可以恢复它们。当子视图或其子树中的视图开始或结束内部瞬态跟踪时，将在父视图上调用此方法。
     * `child` 状态已改变的子视图
     * `has_transient_state` 如果此子视图具有瞬时状态，则为 true
     * */
    fn child_has_transient_state_changed(&self, child: &View, has_transient_state: bool);

    /**
     * 要求执行新的 View.fitSystemWindows(Rect) 调度。
     * */
    fn request_fit_system_windows(&self);

    /**
     * 获取给定 View 的父级，以实现可访问性。由于某些 View 未暴露给可访问性层，因此可访问性的父级不一定是 View 的直接父级，而是前任。
     * 返回：父级，如果未找到，则返回 null。
     * */
    fn get_parent_for_accessibility(&self) -> Option<Self::VP>;

    /**
     * 通知视图父级，其某个后代的可访问性状态已更改，并且子树的结构不同。
     * `child` 子树已更改的直接子级。
     * `source` 发生更改的后代视图。不能为 null。
     * `change_type` 发生的更改类型的位掩码。以下一个或多个：
     * AccessibilityEvent.CONTENT_CHANGE_TYPE_CONTENT_DESCRIPTION AccessibilityEvent.CONTENT_CHANGE_TYPE_STATE_DESCRIPTION AccessibilityEvent.CONTENT_CHANGE_TYPE_SUBTREE
     * AccessibilityEvent.CONTENT_CHANGE_TYPE_TEXT AccessibilityEvent.CONTENT_CHANGE_TYPE_UNDEFINED AccessibilityEvent.CONTENT_CHANGE_TYPE_DRAG_STARTED
     * AccessibilityEvent.CONTENT_CHANGE_TYPE_DRAG_CANCELLED AccessibilityEvent.CONTENT_CHANGE_TYPE_DRAG_DROPPED
     * */
    fn notify_subtree_accessibility_state_changed(
        &self,
        child: &View,
        source: &View,
        change_type: i32,
    );

    /**
     * 告知此视图父级是否可以解析布局方向。请参阅 View.setLayoutDirection(int)
     * 返回：如果此视图父级可以解析布局方向，则返回 True。
     * */
    fn can_resolve_layout_direction(&self) -> bool;

    /**
     * 告知此视图父级布局方向是否已解析。请参阅 View.setLayoutDirection(int)
     * 返回：如果此视图父级布局方向已解析，则返回 True。
     * */
    fn is_layout_direction_resolved(&self) -> bool;

    /**
     * 返回此视图的父布局方向。请参阅 View.getLayoutDirection()
     * 返回：如果布局方向为 RTL，则返回 View.LAYOUT_DIRECTION_RTL；如果布局方向不是 RTL，则返回 View.LAYOUT_DIRECTION_LTR。
     * */
    fn get_layout_direction(&self) -> i32;

    /**
     * 告知此视图父级是否可以解析文本方向。请参阅 View.setTextDirection(int)
     * 返回：如果此视图父级可以解析文本方向，则返回 True。
     * */
    fn can_resolve_text_direction(&self) -> bool;

    /**
     * 告知此视图父文本方向是否已解析。请参阅 View.setTextDirection(int)
     * 返回：如果此视图父文本方向已解析，则返回 true。
     * */
    fn is_text_direction_resolved(&self) -> bool;

    /**
     * 返回此视图父文本方向。参见 View.getTextDirection()
     * 返回：已解析的文本方向。返回以下之一：
     * View.TEXT_DIRECTION_FIRST_STRONG View.TEXT_DIRECTION_ANY_RTL、View.TEXT_DIRECTION_LTR、View.TEXT_DIRECTION_RTL、View.TEXT_DIRECTION_LOCALE
     * */
    fn get_text_direction(&self) -> i32;

    /**
     * 告知此视图父级是否可以解决文本对齐问题。请参阅 View.setTextAlignment(int)
     * 返回：如果此视图父级可以解决文本对齐问题，则返回 True。
     * */
    fn can_resolve_text_alignment(&self) -> bool;

    /**
     * 告知此视图父级是否可以解决文本对齐问题。请参阅 View.setTextAlignment(int)
     * 返回：如果此视图父级可以解决文本对齐问题，则返回 True。
     * */
    fn is_text_alignment_resolved(&self) -> bool;

    /**
     * 对启动嵌套滚动操作的子视图做出反应，在适当的情况下声明嵌套滚动操作。此方法将在子视图调用 View.startNestedScroll(int) 时调用。视图层次结构中的每个父视图都将有机会通过返回 true 来响应和声明嵌套滚动操作。
     * 此方法可能被 ViewParent 实现覆盖，以指示视图何时愿意支持即将开始的嵌套滚动操作。如果它返回 true，则此 ViewParent 将成为目标视图的嵌套滚动父视图，直至滚动操作完成。当嵌套滚动完成时，此 ViewParent 将收到对 onStopNestedScroll(View) 的调用。
     * 如果此 ViewParent 接受嵌套滚动操作，则返回 true
     * `child` 包含目标的此 ViewParent 的直接子视图
     * `target` 发起嵌套滚动的视图
     * `nested_scroll_axes` 由 View.SCROLL_AXIS_HORIZONTAL、View.SCROLL_AXIS_VERTICAL 或两者组成的标志
     * */
    fn on_start_nested_scroll(&self, child: &View, target: &View, nested_scroll_axes: i32) -> bool;

    /**
     * 对成功声明嵌套滚动操作做出反应。此方法将在 onStartNestedScroll 返回 true 后调用。它为视图及其超类提供了对嵌套滚动执行初始配置的机会。此方法的实现应始终调用其超类对此方法的实现（如果存在）。
     * `child` 此 ViewParent 的直接子级，包含 target
     * `target` 启动嵌套滚动的视图
     * `nested_scroll_axes` 由 View.SCROLL_AXIS_HORIZONTAL、View.SCROLL_AXIS_VERTICAL 或两者组成的标志
     * */
    fn on_nested_scroll_accepted(&self, child: &View, target: &View, nested_scroll_axes: i32);

    /**
     * 对嵌套滚动操作结束做出反应。在嵌套滚动操作后执行清理。当嵌套滚动停止时，将调用此方法，例如当嵌套触摸滚动以 MotionEvent.ACTION_UP 或 MotionEvent.ACTION_CANCEL 事件结束时。此方法的实现应始终调用其超类对此方法的实现（如果存在）。
     * `target` 启动嵌套滚动的视图
     * */
    fn on_stop_nested_scroll(&self, target: &View);

    /**
     * 对正在进行的嵌套滚动做出反应。当 ViewParent 的当前嵌套滚动子视图分派嵌套滚动事件时，将调用此方法。要接收对此方法的调用，ViewParent 必须先前已为 onStartNestedScroll(View, View, int) 调用返回 true。
     * 滚动距离的已消耗部分和未消耗部分均报告给 ViewParent。例如，实现可以选择使用已消耗部分来匹配或追踪多个子元素的滚动位置。未消耗部分可用于允许连续拖动多个滚动或可拖动元素，例如滚动垂直抽屉内的列表，一旦到达内部滚动内容的边缘，抽屉便开始拖动。
     * `target` 控制嵌套滚动的后代视图
     * `dx_consumed` 目标已消耗的水平滚动距离（以像素为单位）
     * `dy_consumed` 目标已消耗的垂直滚动距离（以像素为单位）
     * `dx_unconsumed` 目标未消耗的水平滚动距离（以像素为单位）
     * `dy_unconsumed` 目标未消耗的垂直滚动距离（以像素为单位）
     * */
    fn on_nested_scroll(
        &self,
        target: &View,
        dx_consumed: i32,
        dy_consumed: i32,
        dx_unconsumed: i32,
        dy_unconsumed: i32,
    );

    /**
     * 从嵌套滚动请求一次抛出。此方法表示嵌套滚动子视图已检测到适合抛出的条件。通常，这意味着触摸滚动已在滚动方向上以达到或超过可滚动轴上的最小抛出速度的速度结束。
     * 如果嵌套滚动子视图通常会抛出但位于其自身内容的边缘，则可以使用此方法将抛出委托给其嵌套滚动父视图。父视图可以选择使用抛出或观察子视图的抛出。
     * 返回：如果此父视图消耗了抛出或以其他方式对抛出做出反应，则为 true
     * `target` 发起嵌套滚动的视图。
     * `velocity_x` 水平速度（以像素/秒为单位）。
     * `velocity_y` 垂直速度（以像素/秒为单位）。
     * `consumed` 如果子视图消耗了抛出，则为 true，否则为 false。
     * */
    fn on_nested_fling(
        &self,
        target: &View,
        velocity_x: f32,
        velocity_y: f32,
        consumed: bool,
    ) -> bool;

    /**
     * 在目标视图使用嵌套的快速滑动之前对其做出反应。此方法表示嵌套滚动子视图已检测到沿每个轴具有给定速度的快速滑动。通常，这意味着触摸滚动已在滚动方向上以达到或超过沿可滚动轴的最小快速滑动速度的速度结束。
     * 如果嵌套滚动父视图正在将运动作为预滚动的一部分使用，则它可能也适合使用预快速滑动以完成相同的运动。通过从此方法返回 true，父视图表示子视图也不应该快速滑动其自己的内部内容。
     * 返回：如果此父视图在目标视图之前使用了快速滑动，则返回 true
     * `target` 发起嵌套滚动的视图。
     * `velocity_x` 水平速度（以像素/秒为单位）。
     * `velocity_y` 垂直速度（以像素/秒为单位）。
     * */
    fn on_nested_pre_fling(&self, target: &View, velocity_x: f32, velocity_y: f32) -> bool;

    /**
     * 在目标处理目标后代视图委托的可访问性操作之前，对它做出反应。如果目标希望让其父链中的视图有机会在正常处理发生之前对事件做出反应，则目标后代视图可以调用此方法。
     * 最常见的是滚动事件，例如 android.view.accessibility.AccessibilityNodeInfo.ACTION_SCROLL_FORWARD。支持充当嵌套滚动父级的 ViewParent 应覆盖此方法并采取相应行动，以通过可访问性系统实​​现滚动。
     * 返回：true，如果此 ViewParent 使用了此操作
     * `target` 调度此操作的目标视图。
     * `action` 正在执行的操作；请参阅 android.view.accessibility.AccessibilityNodeInfo。
     * `arguments` 可选的操作参数。
     * */
    fn on_nested_pre_perform_accessibility_action(
        &self,
        target: &View,
        action: i32,
        arguments: Option<Bundle>,
    ) -> bool;
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

#[doc(hidden)]
#[allow(non_camel_case_types)]
#[java_class(name = "android/view/View$OnClickListenerImpl")]
pub struct View_OnClickListenerImpl(Box<dyn Fn(View) + Send + Sync>);

impl View_OnClickListenerImpl {
    pub fn from_fn(func: impl Fn(/* v */ View) + Send + Sync + 'static) -> Arc<Self> {
        Self::new(View_OnClickListenerImplDefault(Box::new(func)))
    }
}

impl Default for View_OnClickListenerImplDefault {
    fn default() -> Self {
        Self(Box::new(|v| unimplemented!("{:?}", v)))
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

    #[doc(hidden)]
    fn update_view_layout(&self, view: &View, params: &ViewGroup_LayoutParams);

    #[doc(hidden)]
    fn remove_view(&self, view: &View);
}

/**
 * 应用用于与窗口管理器通信的接口。每个窗口管理器实例都绑定到一个 Display。要获取与显示器关联的 WindowManager，请调用 Context.createWindowContext(Display, int, Bundle) 以获取显示器的 UI 上下文，然后在 UI 上下文上调用 Context.getSystemService(String) 或 Context.getSystemService(Class)。
 * 在特定显示器上显示窗口的最简单方法是创建一个 Presentation，它会自动获取显示器的 WindowManager 和上下文。
 * */
#[java_interface(name = "android/view/WindowManager")]
pub trait WindowManager: ViewManager {
    #[doc(hidden)]
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

#[doc(hidden)]
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

impl ViewParent for ViewGroup {
    type VP = Self;

    #[java_method]
    fn request_layout(&self) {}

    #[java_method]
    fn is_layout_requested(&self) -> bool {}

    #[java_method]
    fn request_transparent_region(&self, child: &View) {}

    #[java_method(type_bound=(Self::VP, ViewParent))]
    fn get_parent(&self) -> Option<Self::VP> {}

    #[java_method]
    fn request_child_focus(&self, child: &View, focused: &View) {}

    #[java_method]
    fn recompute_view_attributes(&self, child: &View) {}

    #[java_method]
    fn clear_child_focus(&self, child: &View) {}

    #[java_method]
    fn focus_search(&self, v: &View, direction: i32) -> View {}

    #[java_method]
    fn bring_child_to_front(&self, child: &View) {}

    #[java_method]
    fn show_context_menu_for_child(&self, original_view: &View) -> bool {}

    #[java_method]
    fn child_drawable_state_changed(&self, child: &View) {}

    #[java_method]
    fn request_disallow_intercept_touch_event(&self, disallow_intercept: bool) {}

    #[java_method]
    fn child_has_transient_state_changed(&self, child: &View, has_transient_state: bool) {}

    #[java_method]
    fn request_fit_system_windows(&self) {}

    #[java_method(type_bound=(Self::VP, ViewParent))]
    fn get_parent_for_accessibility(&self) -> Option<Self::VP> {}

    #[java_method]
    fn notify_subtree_accessibility_state_changed(
        &self,
        child: &View,
        source: &View,
        change_type: i32,
    ) {
    }

    #[java_method]
    fn can_resolve_layout_direction(&self) -> bool {}

    #[java_method]
    fn is_layout_direction_resolved(&self) -> bool {}

    #[java_method]
    fn get_layout_direction(&self) -> i32 {}

    #[java_method]
    fn can_resolve_text_direction(&self) -> bool {}

    #[java_method]
    fn is_text_direction_resolved(&self) -> bool {}

    #[java_method]
    fn get_text_direction(&self) -> i32 {}

    #[java_method]
    fn can_resolve_text_alignment(&self) -> bool {}

    #[java_method]
    fn is_text_alignment_resolved(&self) -> bool {}

    #[java_method]
    fn on_start_nested_scroll(&self, child: &View, target: &View, nested_scroll_axes: i32) -> bool {
    }

    #[java_method]
    fn on_nested_scroll_accepted(&self, child: &View, target: &View, nested_scroll_axes: i32) {}

    #[java_method]
    fn on_stop_nested_scroll(&self, target: &View) {}

    #[java_method]
    fn on_nested_scroll(
        &self,
        target: &View,
        dx_consumed: i32,
        dy_consumed: i32,
        dx_unconsumed: i32,
        dy_unconsumed: i32,
    ) {
    }

    #[java_method]
    fn on_nested_fling(
        &self,
        target: &View,
        velocity_x: f32,
        velocity_y: f32,
        consumed: bool,
    ) -> bool {
    }

    #[java_method]
    fn on_nested_pre_fling(&self, target: &View, velocity_x: f32, velocity_y: f32) -> bool {}

    #[java_method]
    fn on_nested_pre_perform_accessibility_action(
        &self,
        target: &View,
        action: i32,
        arguments: Option<Bundle>,
    ) -> bool {
    }
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

    #[doc(hidden)]
    #[java_constructor]
    pub fn new(width: i32, height: i32) -> Self {}

    #[doc(hidden)]
    #[java_constructor]
    pub fn from_layout_params(source: &ViewGroup_LayoutParams) -> Self {}
}

#[doc(hidden)]
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

    #[doc(hidden)]
    pub const COLOR_MODE_BT601_625: i32 = 1;

    #[doc(hidden)]
    pub const COLOR_MODE_BT601_625_UNADJUSTED: i32 = 2;

    #[doc(hidden)]
    pub const COLOR_MODE_BT601_525: i32 = 3;

    #[doc(hidden)]
    pub const COLOR_MODE_BT601_525_UNADJUSTED: i32 = 4;

    #[doc(hidden)]
    pub const COLOR_MODE_BT709: i32 = 5;

    #[doc(hidden)]
    pub const COLOR_MODE_DCI_P3: i32 = 6;

    #[doc(hidden)]
    pub const COLOR_MODE_SRGB: i32 = 7;

    #[doc(hidden)]
    pub const COLOR_MODE_ADOBE_RGB: i32 = 8;

    #[doc(hidden)]
    pub const COLOR_MODE_DISPLAY_P3: i32 = 9;

    /// 表示当显示屏被移除时，其所有活动将移至主显示屏，并且最顶层的活动将成为焦点。
    /// TODO (b/114338689): 删除该标志并使用 WindowManager#REMOVE_CONTENT_MODE_MOVE_TO_PRIMARY
    pub const REMOVE_MODE_MOVE_CONTENT_TO_PRIMARY: i32 = 0;

    /// 表示当display被移除时，其所有堆栈和任务都将被移除，所有活动将按照通常的生命周期被销毁。
    /// TODO (b/114338689): 删除该标志并使用 WindowManager#REMOVE_CONTENT_MODE_DESTROY
    pub const REMOVE_MODE_DESTROY_CONTENT: i32 = 1;

    #[doc(hidden)]
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

    #[doc(hidden)]
    #[deprecated(note = "改用 WindowMetrics.getBounds.width()。")]
    #[java_method]
    pub fn get_width(&self) -> i32 {}

    #[doc(hidden)]
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

    #[doc(hidden)]
    #[java_method]
    pub fn type_to_string(r#type: i32) -> String {}

    #[doc(hidden)]
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

/// 输入事件的通用基类。
#[java_class(name = "android/view/InputEvent")]
pub struct InputEvent;

impl InputEvent {
    /**
     * 获取此事件所来自设备的 ID。ID 为零表示事件不是来自物理设备并映射到默认键盘映射。其他数字是任意的，您不应该依赖这些值。
     * 返回：设备 ID。
     * */
    #[java_method]
    pub fn get_device_id(&self) -> i32 {}

    /**
     * 获取事件源。
     * 返回：事件源或输入设备。如果未知，则返回 SOURCE_UNKNOWN。
     * */
    #[java_method]
    pub fn get_source(&self) -> i32 {}

    /**
     * 修改事件的来源。
     * `source` 新来源。
     * */
    #[java_method]
    pub fn set_source(&self, source: i32) {}

    /**
     * 确定事件是否来自给定的源。
     * 返回：事件是否来自给定的源。
     * `source` 要检查的输入源。这可以是特定的设备类型，例如 InputDevice.SOURCE_TOUCH_NAVIGATION，也可以是更通用的设备类，例如 InputDevice.SOURCE_CLASS_POINTER。
     * */
    #[java_method]
    pub fn is_from_source(&self, source: i32) -> bool {}

    /**
     * 获取事件的显示 ID。
     * 返回：与事件关联的显示 ID。
     * */
    #[java_method]
    pub fn get_display_id(&self) -> i32 {}

    /**
     * 修改与事件关联的显示 ID
     * `display_id`
     * */
    #[java_method]
    pub fn set_display_id(&self, display_id: i32) {}

    /**
     * 回收事件。此方法仅应由系统使用，因为应用程序不希望回收 KeyEvent 对象，但可以回收 MotionEvent 对象。
     * 有关详细信息，请参阅 KeyEvent.recycle()。
     * */
    #[java_method]
    pub fn recycle(&self) {}

    /**
     * 在将事件分发给应用程序后，如果合适，则有条件地回收事件。如果事件是 MotionEvent，则回收该事件。如果事件是 KeyEvent，则不回收该事件，因为应用程序希望按键事件是不可变的，因此一旦将事件分发给应用程序，我们就不能再回收它了。
     * */
    #[java_method]
    pub fn recycle_if_needed_after_dispatch(&self) {}

    /**
     * 获取一个私有标志，该标志指示系统何时检测到此输入事件可能与先前传递的输入事件的序列不一致，例如，当发送了按键释放事件但按键未按下时，或者当发送了指针移动事件但指针未按下时。
     * 返回：如果此事件被污染，则返回 True。
     * */
    #[java_method]
    pub fn is_tainted(&self) -> bool {}

    /**
     * 设置一个私有标志，指示系统何时检测到此输入事件可能与先前传递的输入事件的序列不一致，例如，当发送了按键释放事件但按键未按下时，或者当发送了指针移动事件但指针未按下时。
     * `tainted` 如果此事件被污染，则为 True。
     * */
    #[java_method]
    pub fn set_tainted(&self, tainted: bool) {}

    /**
     * 以 android.os.SystemClock.uptimeMillis 时间基准查询此事件发生的时间。
     * 返回：以 android.os.SystemClock.uptimeMillis 时间基准返回此事件发生的时间。
     * */
    #[java_method]
    pub fn get_event_time(&self) -> i64 {}

    /**
     * 查询此事件发生的时间，以 android.os.SystemClock.uptimeMillis 时间为基础，但精度为纳秒（而不是毫秒）。该值以纳秒为精度，但可能不具有纳秒的精度。
     * 返回：返回此事件发生的时间，以 android.os.SystemClock.uptimeMillis 时间为基础，但精度为纳秒（而不是毫秒）。
     * */
    #[java_method]
    pub fn get_event_time_nanos(&self) -> i64 {}

    /**
     * 将输入事件标记为已取消。
     * */
    #[java_method]
    pub fn cancel(&self) {}

    /**
     * 获取此事件的唯一序列号。进程创建或接收的每个输入事件都有唯一的序列号。此外，每次回收事件对象时都会获得一个新的序列号。序列号仅保证在进程内本地唯一。打包事件时不会保留序列号。
     * 返回：此事件的唯一序列号。
     * */
    #[java_method]
    pub fn get_sequence_number(&self) -> i32 {}

    //noinspection SpellCheckingInspection
    /**
     * 获取此事件的 ID。此 ID 在事件创建时生成，并保存到事件的最后阶段。它不会因为事件跨越进程边界而改变，但在进行修改复制时应该会改变。为了避免将应用程序使用情况暴露给其他进程，此 ID 由 CSPRNG 生成。因此，虽然 ID 冲突的可能性相当低，但不能 100% 保证此 ID 的唯一性。经验法则是不要依赖生产逻辑的唯一性，而要依赖跟踪事件（例如日志记录和分析）的良好来源。
     * 返回：此事件的 ID。
     * */
    #[java_method]
    pub fn get_id(&self) -> i32 {}

    #[doc(hidden)]
    #[java_method]
    pub fn describe_contents(&self) -> i32 {}
}

//noinspection SpellCheckingInspection
/**
 * 用于报告按键和按钮事件的对象。每次按键都由一系列按键事件描述。按键以 ACTION_DOWN 按键事件开始。如果按键保持的时间足够长，以至于重复，则初始按下之后是其他按键事件，其中 ACTION_DOWN 和非零 getRepeatCount() 值。最后一个按键事件是按键弹起的 ACTION_UP。
 * 如果取消按键，则按键弹起事件将设置 FLAG_CANCELED 标志。按键事件通常伴随着按键代码 (getKeyCode())、扫描代码 (getScanCode()) 和元状态 (getMetaState())。按键代码常量在此类中定义。扫描代码常量是从操作系统获得的原始设备特定代码，因此除非使用 KeyCharacterMap 进行解释，否则通常对应用程序没有意义。
 * 元状态描述按键修饰符（如 META_SHIFT_ON 或 META_ALT_ON）的按下状态。按键代码通常与输入设备上的各个按键一一对应。许多按键和按键组合在不同的输入设备上具有完全不同的功能，因此在解释它们时必须小心谨慎。将按键映射到字符时，请始终使用与输入设备关联的 KeyCharacterMap。
 * 请注意，可能同时有多个按键输入设备处于活动状态，并且每个设备都有自己的按键字符映射。由于软输入法可以使用多种创新的文本输入方式，因此无法保证软键盘上的任何按键都会生成按键事件：这由 IME 自行决定，实际上不鼓励发送此类事件。您永远不应该依赖于接收软输入法上任何按键的 KeyEvent。
 * 特别是，默认软件键盘永远不会向任何针对 Jelly Bean 或更高版本的应用程序发送任何按键事件，并且只会向针对 Ice Cream Sandwich 或更早版本的应用程序发送某些删除和返回键按下事件。请注意，其他软件输入法可能永远不会发送按键事件，无论版本如何。考虑使用编辑器操作，如 android.view.inputmethod.EditorInfo。
 * 如果您需要与软件键盘进行特定交互，则可以使用 IME_ACTION_DONE，因为它可以让用户更清楚地了解您的应用程序如何对按键做出反应。与 IME 交互时，框架可能会使用特殊操作 ACTION_MULTIPLE 传递按键事件，该操作指定单个重复的按键代码或要插入的字符序列。一般来说，框架无法保证它传递给视图的按键事件始终构成完整的按键序列，因为某些事件可能会在传递之前被包含视图删除或修改。视图实现应该准备好处理 FLAG_CANCELED，并且应该容忍异常情况，例如在没有先收到上一次按键的 ACTION_UP 的情况下收到新的 ACTION_DOWN。
 * 有关不同类型的输入设备和源如何表示按键和按钮的更多信息，请参阅 InputDevice。
 * */
#[java_class(name = "android/view/KeyEvent", extends=InputEvent)]
pub struct KeyEvent;

impl KeyEvent {
    /// 键码常量：未知键码。
    pub const KEYCODE_UNKNOWN: i32 = 0;

    /// 键码常量：软左键。通常位于手机显示屏下方，用作多功能特征键，用于选择显示在显示屏左下角的软件定义功能。
    pub const KEYCODE_SOFT_LEFT: i32 = 1;

    /// 键码常量：软右键。通常位于手机显示屏下方，用作多功能特征键，用于选择显示在显示屏右下角的软件定义功能。
    pub const KEYCODE_SOFT_RIGHT: i32 = 2;

    /// 键码常量：主页键。此键由框架处理，永远不会传递给应用程序。
    pub const KEYCODE_HOME: i32 = 3;

    /// 键码常量：返回键。
    pub const KEYCODE_BACK: i32 = 4;

    /// 键码常量：呼叫键。
    pub const KEYCODE_CALL: i32 = 5;

    //noinspection SpellCheckingInspection
    /// 键码常量：结束呼叫键。
    pub const KEYCODE_ENDCALL: i32 = 6;

    /// 键码常量：'0'键。
    pub const KEYCODE_0: i32 = 7;

    /// 键码常量：'1'键。
    pub const KEYCODE_1: i32 = 8;

    /// 键码常量：'2'键。
    pub const KEYCODE_2: i32 = 9;

    /// 键码常量：'3'键。
    pub const KEYCODE_3: i32 = 10;

    /// 键码常量：'4'键。
    pub const KEYCODE_4: i32 = 11;

    /// 键码常量：'5'键。
    pub const KEYCODE_5: i32 = 12;

    /// 键码常量：'6'键。
    pub const KEYCODE_6: i32 = 13;

    /// 键码常量：'7'键。
    pub const KEYCODE_7: i32 = 14;

    /// 键码常量：'8'键。
    pub const KEYCODE_8: i32 = 15;

    /// 键码常量：'9'键。
    pub const KEYCODE_9: i32 = 16;

    /// 键码常量：'*'键。
    pub const KEYCODE_STAR: i32 = 17;

    /// 键码常量：'#'键。
    pub const KEYCODE_POUND: i32 = 18;

    //noinspection SpellCheckingInspection
    /// 键码常量：方向键向上键。也可以从轨迹球运动合成。
    pub const KEYCODE_DPAD_UP: i32 = 19;

    //noinspection SpellCheckingInspection
    /// 键码常量：方向键向下键。也可以从轨迹球运动合成。
    pub const KEYCODE_DPAD_DOWN: i32 = 20;

    //noinspection SpellCheckingInspection
    /// 键码常量：方向键左键。也可以从轨迹球运动合成。
    pub const KEYCODE_DPAD_LEFT: i32 = 21;

    //noinspection SpellCheckingInspection
    /// 键码常量：方向键右键。也可以从轨迹球运动合成。
    pub const KEYCODE_DPAD_RIGHT: i32 = 22;

    //noinspection SpellCheckingInspection
    /// 键码常量：方向键中心键。也可以从轨迹球运动合成。
    pub const KEYCODE_DPAD_CENTER: i32 = 23;

    /// 键码常量：音量向上键。调整扬声器音量。
    pub const KEYCODE_VOLUME_UP: i32 = 24;

    /// 键码常量：音量减键。调整扬声器音量减小。
    pub const KEYCODE_VOLUME_DOWN: i32 = 25;

    /// 键码常量：电源键。
    pub const KEYCODE_POWER: i32 = 26;

    /// 键码常量：相机键。用于启动相机应用程序或拍照。
    pub const KEYCODE_CAMERA: i32 = 27;

    /// 键码常量：清除键。
    pub const KEYCODE_CLEAR: i32 = 28;

    /// 键码常量：'A'键。
    pub const KEYCODE_A: i32 = 29;

    /// 键码常量：'B'键。
    pub const KEYCODE_B: i32 = 30;

    /// 键码常量：'C'键。
    pub const KEYCODE_C: i32 = 31;

    /// 键码常量：'D'键。
    pub const KEYCODE_D: i32 = 32;

    /// 键码常量：'E'键。
    pub const KEYCODE_E: i32 = 33;

    /// 键码常量：'F'键。
    pub const KEYCODE_F: i32 = 34;

    /// 键码常量：'G'键。
    pub const KEYCODE_G: i32 = 35;

    /// 键码常量：'H'键。
    pub const KEYCODE_H: i32 = 36;

    /// 键码常量：'I'键。
    pub const KEYCODE_I: i32 = 37;

    /// 键码常量：'J'键。
    pub const KEYCODE_J: i32 = 38;

    /// 键码常量：'K'键。
    pub const KEYCODE_K: i32 = 39;

    /// 键码常量：'L'键。
    pub const KEYCODE_L: i32 = 40;

    /// 键码常量：'M'键。
    pub const KEYCODE_M: i32 = 41;

    /// 键码常量：'N'键。
    pub const KEYCODE_N: i32 = 42;

    /// 键码常量：'O'键。
    pub const KEYCODE_O: i32 = 43;

    /// 键码常量：'P'键。
    pub const KEYCODE_P: i32 = 44;

    /// 键码常量：'Q'键。
    pub const KEYCODE_Q: i32 = 45;

    /// 键码常量：'R'键。
    pub const KEYCODE_R: i32 = 46;

    /// 键码常量：'S'键。
    pub const KEYCODE_S: i32 = 47;

    /// 键码常量：'T'键。
    pub const KEYCODE_T: i32 = 48;

    /// 键码常量：'U'键。
    pub const KEYCODE_U: i32 = 49;

    /// 键码常量：'V'键。
    pub const KEYCODE_V: i32 = 50;

    /// 键码常量：'W'键。
    pub const KEYCODE_W: i32 = 51;

    /// 键码常量：'X'键。
    pub const KEYCODE_X: i32 = 52;

    /// 键码常量：'Y'键。
    pub const KEYCODE_Y: i32 = 53;

    /// 键码常量：'Z'键。
    pub const KEYCODE_Z: i32 = 54;

    /// 键码常量：','键。
    pub const KEYCODE_COMMA: i32 = 55;

    /// 键码常量：'.'键。
    pub const KEYCODE_PERIOD: i32 = 56;

    /// 键码常量：左Alt修饰键。
    pub const KEYCODE_ALT_LEFT: i32 = 57;

    /// 键码常量：右Alt修饰键。
    pub const KEYCODE_ALT_RIGHT: i32 = 58;

    /// 键码常量：左Shift修饰键。
    pub const KEYCODE_SHIFT_LEFT: i32 = 59;

    /// 键码常量：右Shift修饰键。
    pub const KEYCODE_SHIFT_RIGHT: i32 = 60;

    /// 键码常量：Tab键。
    pub const KEYCODE_TAB: i32 = 61;

    /// 键码常量：空格键。
    pub const KEYCODE_SPACE: i32 = 62;

    /// 键码常量：符号修饰键。用于输入替代符号。
    pub const KEYCODE_SYM: i32 = 63;

    /// 键码常量：资源管理器特殊功能键。用于启动浏览器应用程序。
    pub const KEYCODE_EXPLORER: i32 = 64;

    /// 键码常量：信封特殊功能键。用于启动邮件应用程序。
    pub const KEYCODE_ENVELOPE: i32 = 65;

    /// 键码常量：Enter键。
    pub const KEYCODE_ENTER: i32 = 66;

    /// 键码常量：退格键。删除插入点前的字符，与KEYCODE_FORWARD_DEL不同。
    pub const KEYCODE_DEL: i32 = 67;

    /// 键码常量：'`'（反引号）键。
    pub const KEYCODE_GRAVE: i32 = 68;

    /// 键码常量：'-'。
    pub const KEYCODE_MINUS: i32 = 69;

    /// 键码常量：'='键。
    pub const KEYCODE_EQUALS: i32 = 70;

    /// 键码常量：'[' 键。
    pub const KEYCODE_LEFT_BRACKET: i32 = 71;

    /// 键码常量：']'键。
    pub const KEYCODE_RIGHT_BRACKET: i32 = 72;

    /// 键码常量：'\'键。
    pub const KEYCODE_BACKSLASH: i32 = 73;

    /// 键码常量：';'键。
    pub const KEYCODE_SEMICOLON: i32 = 74;

    /// 键码常量：'''（单引号）键。
    pub const KEYCODE_APOSTROPHE: i32 = 75;

    /// 键码常量：'/'键。
    pub const KEYCODE_SLASH: i32 = 76;

    /// 键码常量：'@'键。
    pub const KEYCODE_AT: i32 = 77;

    /// 键码常量：数字修饰键。用于输入数字符号。这个键不是Num Lock；它更像是KEYCODE_ALT_LEFT，并被android.text.method.MetaKeyKeyListener解释为ALT键。
    pub const KEYCODE_NUM: i32 = 78;

    //noinspection SpellCheckingInspection
    /// 键码常量：耳机钩键。用于挂断电话并停止媒体。
    pub const KEYCODE_HEADSETHOOK: i32 = 79;

    /// 键码常量：相机对焦键。用于对焦相机。
    pub const KEYCODE_FOCUS: i32 = 80;

    /// 键码常量：'+'键。
    pub const KEYCODE_PLUS: i32 = 81;

    /// 键码常量：菜单键。
    pub const KEYCODE_MENU: i32 = 82;

    /// 键码常量：通知键。
    pub const KEYCODE_NOTIFICATION: i32 = 83;

    /// 键码常量：搜索键。
    pub const KEYCODE_SEARCH: i32 = 84;

    /// 键码常量：媒体播放/暂停键。
    pub const KEYCODE_MEDIA_PLAY_PAUSE: i32 = 85;

    /// 键码常量：媒体停止键。
    pub const KEYCODE_MEDIA_STOP: i32 = 86;

    /// 键码常量：媒体播放下一曲键。
    pub const KEYCODE_MEDIA_NEXT: i32 = 87;

    /// 键码常量：媒体播放上一曲键。
    pub const KEYCODE_MEDIA_PREVIOUS: i32 = 88;

    /// 键码常量：媒体倒带键。
    pub const KEYCODE_MEDIA_REWIND: i32 = 89;

    /// 键码常量：媒体快进键。
    pub const KEYCODE_MEDIA_FAST_FORWARD: i32 = 90;

    /// 键码常量：静音键。用于麦克风的静音键（不同于KEYCODE_VOLUME_MUTE，那是扬声器静音键）。
    pub const KEYCODE_MUTE: i32 = 91;

    /// 键码常量：Page Up键。
    pub const KEYCODE_PAGE_UP: i32 = 92;

    /// 键码常量：Page Down键。
    pub const KEYCODE_PAGE_DOWN: i32 = 93;

    //noinspection SpellCheckingInspection
    /// 键码常量：图片符号修饰键。用于切换符号集（表情符号、颜文字）。
    pub const KEYCODE_PICTSYMBOLS: i32 = 94;

    //noinspection SpellCheckingInspection
    /// 切换符号集（表情符号、颜文字）
    /// 键码常量：切换字符集修饰键。用于切换字符集（汉字、片假名）。
    pub const KEYCODE_SWITCH_CHARSET: i32 = 95; // 切换字符集（汉字、片假名）

    /// 键码常量：A按钮键。在游戏控制器上，A按钮应为标有A的按钮或控制器按钮底部行的第一个按钮。
    pub const KEYCODE_BUTTON_A: i32 = 96;

    /// 键码常量：B按钮键。在游戏控制器上，B按钮应为标有B的按钮或控制器按钮底部行的第二个按钮。
    pub const KEYCODE_BUTTON_B: i32 = 97;

    /// 键码常量：C按钮键。在游戏控制器上，C按钮应为标有C的按钮或控制器按钮底部行的第三个按钮。
    pub const KEYCODE_BUTTON_C: i32 = 98;

    /// 键码常量：X按钮键。在游戏控制器上，X按钮应为标有X的按钮或控制器按钮顶部行的第一个按钮。
    pub const KEYCODE_BUTTON_X: i32 = 99;

    /// 键码常量：Y按钮键。在游戏控制器上，Y按钮应为标有Y的按钮或控制器按钮顶部行的第二个按钮。
    pub const KEYCODE_BUTTON_Y: i32 = 100;

    /// 键码常量：Z按钮键。在游戏控制器上，Z按钮应为标有Z的按钮或控制器按钮顶部行的第三个按钮。
    pub const KEYCODE_BUTTON_Z: i32 = 101;

    /// 键码常量：L1按钮键。在游戏控制器上，L1按钮应为标有L1（或L）的按钮或左上角的触发器按钮。
    pub const KEYCODE_BUTTON_L1: i32 = 102;

    /// 键码常量：R1按钮键。在游戏控制器上，R1按钮应为标有R1（或R）的按钮或右上角的触发器按钮。
    pub const KEYCODE_BUTTON_R1: i32 = 103;

    /// 键码常量：L2按钮键。在游戏控制器上，L2按钮应为标有L2的按钮或左下角的触发器按钮。
    pub const KEYCODE_BUTTON_L2: i32 = 104;

    //noinspection SpellCheckingInspection
    /// 键码常量：左拇指按钮键。在游戏控制器上，左拇指按钮表示按下左（或唯一）操纵杆。
    pub const KEYCODE_BUTTON_THUMBL: i32 = 106;

    //noinspection SpellCheckingInspection
    /// 键码常量：右拇指按钮键。在游戏控制器上，右拇指按钮表示按下了右操纵杆。
    pub const KEYCODE_BUTTON_THUMBR: i32 = 107;

    /// 键码常量：R2 按钮键。在游戏控制器上，R2 按钮应该是标有 R2 的按钮或右下角触发器按钮。
    pub const KEYCODE_BUTTON_R2: i32 = 105;

    /// 键码常量：开始按钮键。在游戏控制器上，标有“开始”的按钮。
    pub const KEYCODE_BUTTON_START: i32 = 108;

    /// 键码常量：选择按钮键。在游戏控制器上，标有“选择”的按钮。
    pub const KEYCODE_BUTTON_SELECT: i32 = 109;

    /// 键码常量：模式按钮键。在游戏控制器上，标有“模式”的按钮。
    pub const KEYCODE_BUTTON_MODE: i32 = 110;

    /// 键码常量：Esc 键。
    pub const KEYCODE_ESCAPE: i32 = 111;

    /// 键码常量：向前删除键。与 KEYCODE_DEL 不同，它删除插入点前面的字符。
    pub const KEYCODE_FORWARD_DEL: i32 = 112;

    /// 键码常量：左 Control 修饰键。
    pub const KEYCODE_CTRL_LEFT: i32 = 113;

    /// 键码常量：右 Control 修饰键。
    pub const KEYCODE_CTRL_RIGHT: i32 = 114;

    /// 键码常量：大写锁定键。
    pub const KEYCODE_CAPS_LOCK: i32 = 115;

    /// 键码常量：滚动锁定键。
    pub const KEYCODE_SCROLL_LOCK: i32 = 116;

    /// 键码常量：左 Meta 修饰键。
    pub const KEYCODE_META_LEFT: i32 = 117;

    /// 键码常量：右 Meta 修饰键。
    pub const KEYCODE_META_RIGHT: i32 = 118;

    /// 键码常量：功能修饰键。
    pub const KEYCODE_FUNCTION: i32 = 119;

    //noinspection SpellCheckingInspection
    /// 键码常量：系统请求/打印屏幕键。
    pub const KEYCODE_SYSRQ: i32 = 120;

    /// 键码常量：Break / Pause 键。
    pub const KEYCODE_BREAK: i32 = 121;

    /// 键码常量：Home 移动键。用于滚动或移动光标到 行 的开始或列表的顶部。
    pub const KEYCODE_MOVE_HOME: i32 = 122;

    /// 键码常量：End 移动键。用于滚动或移动光标到 行 的末尾或列表的底部。
    pub const KEYCODE_MOVE_END: i32 = 123;

    /// 键码常量：Insert 键。切换插入/覆盖编辑模式。
    pub const KEYCODE_INSERT: i32 = 124;

    /// 键码常量：前进键。在历史堆栈中向前导航。与 KEYCODE_BACK 互补。
    pub const KEYCODE_FORWARD: i32 = 125;

    /// 键码常量：播放媒体键。
    pub const KEYCODE_MEDIA_PLAY: i32 = 126;

    /// 键码常量：暂停媒体键。
    pub const KEYCODE_MEDIA_PAUSE: i32 = 127;

    /// 键码常量：关闭媒体键。例如，可用于关闭 CD 托盘。
    pub const KEYCODE_MEDIA_CLOSE: i32 = 128;

    /// 键码常量：弹出媒体键。例如，可用于弹出 CD 托盘。
    pub const KEYCODE_MEDIA_EJECT: i32 = 129;

    /// 键码常量：录制媒体键。
    pub const KEYCODE_MEDIA_RECORD: i32 = 130;

    /// 键码常量：F1 键。
    pub const KEYCODE_F1: i32 = 131;

    /// 键码常量：F2 键。
    pub const KEYCODE_F2: i32 = 132;

    /// 键码常量：F3 键。
    pub const KEYCODE_F3: i32 = 133;

    /// 键码常量：F4 键。
    pub const KEYCODE_F4: i32 = 134;

    /// 键码常量：F5 键。
    pub const KEYCODE_F5: i32 = 135;

    /// 键码常量：F6 键。
    pub const KEYCODE_F6: i32 = 136;

    /// 键码常量：F7 键。
    pub const KEYCODE_F7: i32 = 137;

    /// 键码常量：F8 键。
    pub const KEYCODE_F8: i32 = 138;

    /// 键码常量：F9 键。
    pub const KEYCODE_F9: i32 = 139;

    /// 键码常量：F10 键。
    pub const KEYCODE_F10: i32 = 140;

    /// 键码常量：F11 键。
    pub const KEYCODE_F11: i32 = 141;

    /// 键码常量：数字键盘'('键
    pub const KEYCODE_NUMPAD_LEFT_PAREN: i32 = 162;

    /// 键码常量：数字键盘')'键。
    pub const KEYCODE_NUMPAD_RIGHT_PAREN: i32 = 163;

    /// 键码常量：F12 键。
    pub const KEYCODE_F12: i32 = 142;

    /// 键码常量：Num Lock 键。这是 Num Lock 键，与 KEYCODE_NUM 不同。此键会改变数字键盘上其他键的行为。
    pub const KEYCODE_NUM_LOCK: i32 = 143;

    /// 键码常量：数字键盘 '0' 键。
    pub const KEYCODE_NUMPAD_0: i32 = 144;

    /// 键码常量：数字键盘 '1' 键。
    pub const KEYCODE_NUMPAD_1: i32 = 145;

    /// 键码常量：数字键盘 '2' 键。
    pub const KEYCODE_NUMPAD_2: i32 = 146;

    /// 键码常量：数字键盘 '3' 键。
    pub const KEYCODE_NUMPAD_3: i32 = 147;

    /// 键码常量：数字键盘 '4' 键。
    pub const KEYCODE_NUMPAD_4: i32 = 148;

    /// 键码常量：数字键盘 '5' 键。
    pub const KEYCODE_NUMPAD_5: i32 = 149;

    /// 键码常量：数字键盘 '6' 键。
    pub const KEYCODE_NUMPAD_6: i32 = 150;

    /// 键码常量：数字键盘 '7' 键。
    pub const KEYCODE_NUMPAD_7: i32 = 151;

    /// 键码常量：数字键盘 '8' 键。
    pub const KEYCODE_NUMPAD_8: i32 = 152;

    /// 键码常量：数字键盘 '9' 键。
    pub const KEYCODE_NUMPAD_9: i32 = 153;

    /// 键码常量：数字键盘 '/' 键（用于除法）。
    pub const KEYCODE_NUMPAD_DIVIDE: i32 = 154;

    /// 键码常量：数字键盘 '*' 键（用于乘法）。
    pub const KEYCODE_NUMPAD_MULTIPLY: i32 = 155;

    /// 键码常量：数字键盘 '-' 键（用于减法）。
    pub const KEYCODE_NUMPAD_SUBTRACT: i32 = 156;

    /// 键码常量：数字键盘 '+' 键（用于加法）。
    pub const KEYCODE_NUMPAD_ADD: i32 = 157;

    /// 键码常量：数字键盘 '.' 键（用于小数或数字分组）。
    pub const KEYCODE_NUMPAD_DOT: i32 = 158;

    /// 键码常量：数字键盘 ',' 键（用于小数或数字分组）。
    pub const KEYCODE_NUMPAD_COMMA: i32 = 159;

    /// 键码常量：数字键盘 Enter 键。
    pub const KEYCODE_NUMPAD_ENTER: i32 = 160;

    /// 键码常量：绿色“可编程”键。在电视遥控器上，用作上下文/可编程键。
    pub const KEYCODE_PROG_GREEN: i32 = 184;

    /// 键码常量：数字键盘 '=' 键。
    pub const KEYCODE_NUMPAD_EQUALS: i32 = 161;

    /// 键码常量：音量静音键。用于扬声器的静音键（与 KEYCODE_MUTE 不同，后者是麦克风的静音键）。此键通常应实现为切换键，即第一次按下时静音扬声器，第二次按下时恢复原始音量。
    pub const KEYCODE_VOLUME_MUTE: i32 = 164;

    /// 键码常量：信息键。通常在电视遥控器上，用于显示与当前正在查看的内容相关的附加信息。
    pub const KEYCODE_INFO: i32 = 165;

    /// 键码常量：频道上键。在电视遥控器上，用于增加电视频道。
    pub const KEYCODE_CHANNEL_UP: i32 = 166;

    /// 键码常量：频道下键。在电视遥控器上，用于减少电视频道。
    pub const KEYCODE_CHANNEL_DOWN: i32 = 167;

    /// 键码常量：放大键。
    pub const KEYCODE_ZOOM_IN: i32 = 168;

    /// 键码常量：缩小键。
    pub const KEYCODE_ZOOM_OUT: i32 = 169;

    /// 键码常量：电视键。在电视遥控器上，切换到观看直播电视。
    pub const KEYCODE_TV: i32 = 170;

    /// 键码常量：窗口键。在电视遥控器上，切换画中画模式或其他窗口功能。在 Android Wear 设备上，触发显示偏移。
    pub const KEYCODE_WINDOW: i32 = 171;

    /// 键码常量：指南键。在电视遥控器上，显示节目指南。
    pub const KEYCODE_GUIDE: i32 = 172;

    /// 键码常量：DVR 键。在某些电视遥控器上，切换到录制的节目的 DVR 模式。
    pub const KEYCODE_DVR: i32 = 173;

    /// 键码常量：书签键。在某些电视遥控器上，用于标记内容或网页为书签。
    pub const KEYCODE_BOOKMARK: i32 = 174;

    /// 键码常量：切换字幕键。在电视节目期间，切换闭路字幕文本的模式。
    pub const KEYCODE_CAPTIONS: i32 = 175;

    /// 键码常量：设置键。启动系统设置活动。
    pub const KEYCODE_SETTINGS: i32 = 176;

    /// 键码常量：语言切换键。切换当前输入语言，例如在 QWERTY 键盘上切换英语和日语。在某些设备上，按 Shift+空格键可以执行相同的功能。
    pub const KEYCODE_LANGUAGE_SWITCH: i32 = 204;

    /// 键码常量：电视电源键。在HDMI电视面板设备和不支持HDMI的Android TV设备上，切换设备的电源状态。在HDMI源设备上，通过HDMI-CEC切换HDMI连接电视的电源状态，并使源设备跟随此电源状态。
    pub const KEYCODE_TV_POWER: i32 = 177;

    /// 键码常量：电视输入键。在电视遥控器上，在电视屏幕上切换输入。
    pub const KEYCODE_TV_INPUT: i32 = 178;

    /// 键码常量：机顶盒电源键。在电视遥控器上，切换外部机顶盒的电源。
    pub const KEYCODE_STB_POWER: i32 = 179;

    /// 键码常量：机顶盒输入键。在电视遥控器上，切换外部机顶盒的输入模式。
    pub const KEYCODE_STB_INPUT: i32 = 180;

    /// 键码常量：A/V接收器电源键。在电视遥控器上，切换外部A/V接收器的电源。
    pub const KEYCODE_AVR_POWER: i32 = 181;

    /// 键码常量：A/V接收器输入键。在电视遥控器上，切换外部A/V接收器的输入模式。
    pub const KEYCODE_AVR_INPUT: i32 = 182;

    /// 键码常量：红色“可编程”键。在电视遥控器上，作为上下文/可编程键使用。
    pub const KEYCODE_PROG_RED: i32 = 183;

    /// 键码常量：黄色“可编程”键。在电视遥控器上，作为上下文/可编程键使用。
    pub const KEYCODE_PROG_YELLOW: i32 = 185;

    /// 键码常量：蓝色“可编程”键。在电视遥控器上，作为上下文/可编程键使用。
    pub const KEYCODE_PROG_BLUE: i32 = 186;

    /// 键码常量：应用程序切换键。应该显示应用程序切换器对话框。
    pub const KEYCODE_APP_SWITCH: i32 = 187;

    /// 键码常量：通用游戏板按钮#1。
    pub const KEYCODE_BUTTON_1: i32 = 188;

    /// 键码常量：通用游戏板按钮#2。
    pub const KEYCODE_BUTTON_2: i32 = 189;

    //noinspection SpellCheckingInspection
    /// 键码常量：日语全角/半角键。
    pub const KEYCODE_ZENKAKU_HANKAKU: i32 = 211;

    //noinspection SpellCheckingInspection
    /// 键码常量：日语字母数字键。
    pub const KEYCODE_EISU: i32 = 212;

    //noinspection SpellCheckingInspection
    /// 键码常量：日语非转换键。
    pub const KEYCODE_MUHENKAN: i32 = 213;

    //noinspection SpellCheckingInspection
    /// 键码常量：日语转换键。
    pub const KEYCODE_HENKAN: i32 = 214;

    /// 键码常量：通用游戏板按钮#3。
    pub const KEYCODE_BUTTON_3: i32 = 190;

    /// 键码常量：通用游戏板按钮#4。
    pub const KEYCODE_BUTTON_4: i32 = 191;

    /// 键码常量：通用游戏板按钮#5。
    pub const KEYCODE_BUTTON_5: i32 = 192;

    /// 键码常量：通用游戏板按钮#6。
    pub const KEYCODE_BUTTON_6: i32 = 193;

    /// 键码常量：通用游戏板按钮#7。
    pub const KEYCODE_BUTTON_7: i32 = 194;

    /// 键码常量：通用游戏板按钮#8。
    pub const KEYCODE_BUTTON_8: i32 = 195;

    /// 键码常量：通用游戏板按钮#9。
    pub const KEYCODE_BUTTON_9: i32 = 196;

    /// 键码常量：通用游戏板按钮#10。
    pub const KEYCODE_BUTTON_10: i32 = 197;

    /// 键码常量：通用游戏板按钮#11。
    pub const KEYCODE_BUTTON_11: i32 = 198;

    /// 键码常量：通用游戏板按钮#12。
    pub const KEYCODE_BUTTON_12: i32 = 199;

    /// 键码常量：通用游戏板按钮#13。
    pub const KEYCODE_BUTTON_13: i32 = 200;

    /// 键码常量：通用游戏板按钮#14。
    pub const KEYCODE_BUTTON_14: i32 = 201;

    /// 键码常量：通用游戏板按钮#15。
    pub const KEYCODE_BUTTON_15: i32 = 202;

    /// 键码常量：通用游戏板按钮#16。
    pub const KEYCODE_BUTTON_16: i32 = 203;

    /// 键码常量：礼仪模式键。在某些设置（如在拥挤的火车上）中，打开和关闭静音或振动模式，使设备表现得更加礼貌。在某些设备上，此键可能仅在长按时才有效。
    pub const KEYCODE_MANNER_MODE: i32 = 205;

    /// 键码常量：3D模式键。在2D和3D模式之间切换显示。
    pub const KEYCODE_3D_MODE: i32 = 206;

    /// 键码常量：联系人特殊功能键。用于启动地址簿应用程序。
    pub const KEYCODE_CONTACTS: i32 = 207;

    /// 键码常量：日历特殊功能键。用于启动日历应用程序。
    pub const KEYCODE_CALENDAR: i32 = 208;

    /// 键码常量：音轨键。切换音轨。
    pub const KEYCODE_MEDIA_AUDIO_TRACK: i32 = 222;

    /// 键码常量：睡眠键。使设备进入睡眠状态。行为类似于 KEYCODE_POWER，但如果设备已处于睡眠状态，则不起作用。
    pub const KEYCODE_SLEEP: i32 = 223;

    /// 键码常量：唤醒键。唤醒设备。行为有点类似于 KEYCODE_POWER，但如果设备已唤醒，则不起作用。
    pub const KEYCODE_WAKEUP: i32 = 224;

    /// 键码常量：音乐特殊功能键。用于启动音乐播放器应用程序。
    pub const KEYCODE_MUSIC: i32 = 209;

    /// 键码常量：计算器特殊功能键。用于启动计算器应用程序。
    pub const KEYCODE_CALCULATOR: i32 = 210;

    /// 键码常量：日本假名/平假名键。
    pub const KEYCODE_KATAKANA_HIRAGANA: i32 = 215;

    /// 键码常量：日本日元键。
    pub const KEYCODE_YEN: i32 = 216;

    /// 键码常量：日本Ro键。
    pub const KEYCODE_RO: i32 = 217;

    /// 键码常量：日本假名键。
    pub const KEYCODE_KANA: i32 = 218;

    /// 键码常量：辅助键。启动全局辅助活动。不会传递给应用程序。
    pub const KEYCODE_ASSIST: i32 = 219;

    /// 键码常量：亮度减小键。降低屏幕亮度。
    pub const KEYCODE_BRIGHTNESS_DOWN: i32 = 220;

    /// 键码常量：亮度增大键。提高屏幕亮度。
    pub const KEYCODE_BRIGHTNESS_UP: i32 = 221;

    /// 键码常量：配对键。启动外设配对模式。对于配对遥控器或游戏控制器特别有用，尤其是如果没有其他输入模式可用时。
    pub const KEYCODE_PAIRING: i32 = 225;

    /// 键码常量：媒体顶层菜单键。跳转到媒体菜单的顶部。
    pub const KEYCODE_MEDIA_TOP_MENU: i32 = 226;

    /// 键码常量：‘11’键。
    pub const KEYCODE_11: i32 = 227;

    /// 键码常量：“12”键。
    pub const KEYCODE_12: i32 = 228;

    /// 键码常量：上一个频道键。跳转到最后一个观看的频道。
    pub const KEYCODE_LAST_CHANNEL: i32 = 229;

    /// 键码常量：电视数据服务键。显示数据服务，如天气、体育等。
    pub const KEYCODE_TV_DATA_SERVICE: i32 = 230;

    /// 键码常量：语音助手键。启动全局语音助手活动。不会传递给应用程序。
    pub const KEYCODE_VOICE_ASSIST: i32 = 231;

    /// 键码常量：收音机键。切换电视服务/收音机服务。
    pub const KEYCODE_TV_RADIO_SERVICE: i32 = 232;

    /// 键码常量：电视图文键。显示电视图文服务。
    pub const KEYCODE_TV_TELETEXT: i32 = 233;

    /// 键码常量：数字输入键。当每个数字键被分配用于选择单独的频道时，启动输入多位频道号。对应于 CEC 用户控制代码的数字输入模式 (0x1D)。
    pub const KEYCODE_TV_NUMBER_ENTRY: i32 = 234;

    /// 键码常量：模拟地面广播键。切换到模拟地面广播服务。
    pub const KEYCODE_TV_TERRESTRIAL_ANALOG: i32 = 235;

    /// 键码常量：数字地面广播键。切换到数字地面广播服务。
    pub const KEYCODE_TV_TERRESTRIAL_DIGITAL: i32 = 236;

    /// 键码常量：卫星键。切换到数字卫星广播服务。
    pub const KEYCODE_TV_SATELLITE: i32 = 237;

    /// 键码常量：BS键。切换到日本可用的BS数字卫星广播服务。
    pub const KEYCODE_TV_SATELLITE_BS: i32 = 238;

    /// 键码常量：CS键。切换到日本可用的CS数字卫星广播服务。
    pub const KEYCODE_TV_SATELLITE_CS: i32 = 239;

    /// 键码常量：BS/CS键。在BS和CS数字卫星服务之间切换。
    pub const KEYCODE_TV_SATELLITE_SERVICE: i32 = 240;

    /// 键码常量：切换网络键。切换选择广播服务。
    pub const KEYCODE_TV_NETWORK: i32 = 241;

    /// 键码常量：天线/电缆键。在天线和电缆之间切换广播输入源。
    pub const KEYCODE_TV_ANTENNA_CABLE: i32 = 242;

    /// 键码常量：HDMI #1 键。切换到 HDMI 输入 #1。
    pub const KEYCODE_TV_INPUT_HDMI_1: i32 = 243;

    /// 键码常量：HDMI #2 键。切换到 HDMI 输入 #2。
    pub const KEYCODE_TV_INPUT_HDMI_2: i32 = 244;

    /// 键码常量：HDMI #3 键。切换到 HDMI 输入 #3。
    pub const KEYCODE_TV_INPUT_HDMI_3: i32 = 245;

    /// 键码常量：HDMI #4 键。切换到 HDMI 输入 #4。
    pub const KEYCODE_TV_INPUT_HDMI_4: i32 = 246;

    /// 键码常量：复合 #1 键。切换到复合视频输入 #1。
    pub const KEYCODE_TV_INPUT_COMPOSITE_1: i32 = 247;

    /// 键码常量：复合 #2 键。切换到复合视频输入 #2。
    pub const KEYCODE_TV_INPUT_COMPOSITE_2: i32 = 248;

    /// 键码常量：分量 #1 键。切换到分量视频输入 #1。
    pub const KEYCODE_TV_INPUT_COMPONENT_1: i32 = 249;

    /// 键码常量：分量 #2 键。切换到分量视频输入 #2。
    pub const KEYCODE_TV_INPUT_COMPONENT_2: i32 = 250;

    /// 键码常量：VGA #1 键。切换到 VGA（模拟 RGB）输入 #1。
    pub const KEYCODE_TV_INPUT_VGA_1: i32 = 251;

    /// 键码常量：音频描述键。开启/关闭音频描述。
    pub const KEYCODE_TV_AUDIO_DESCRIPTION: i32 = 252;

    /// 键码常量：音频描述混合音量调高键。与正常音频音量相比，增大音频描述音量。
    pub const KEYCODE_TV_AUDIO_DESCRIPTION_MIX_UP: i32 = 253;

    /// 键码常量：音频描述混音音量减小键。与正常音频音量相比，降低音频描述音量。
    pub const KEYCODE_TV_AUDIO_DESCRIPTION_MIX_DOWN: i32 = 254;

    /// 键码常量：缩放模式键。更改缩放模式（正常、全屏、缩放、宽缩放等）
    pub const KEYCODE_TV_ZOOM_MODE: i32 = 255;

    /// 键码常量：内容菜单键。进入标题列表。对应于CEC用户控制代码的“内容菜单”（0x0B）
    pub const KEYCODE_TV_CONTENTS_MENU: i32 = 256;

    /// 键码常量：媒体上下文菜单键。进入媒体内容的上下文菜单。对应于CEC用户控制代码的“媒体上下文相关菜单”（0x11）。
    pub const KEYCODE_TV_MEDIA_CONTEXT_MENU: i32 = 257;

    /// 键码常量：定时器编程键。进入定时器录制菜单。对应于CEC用户控制代码的“定时器编程”（0x54）。
    pub const KEYCODE_TV_TIMER_PROGRAMMING: i32 = 258;

    /// 键码常量：帮助键。
    pub const KEYCODE_HELP: i32 = 259;

    /// 键码常量：导航到上一个键。在有序的项目集合中向后移动一个项目。
    pub const KEYCODE_NAVIGATE_PREVIOUS: i32 = 260;

    /// 键码常量：导航到下一个键。在有序的项目集合中前进到下一个项目。
    pub const KEYCODE_NAVIGATE_NEXT: i32 = 261;

    /// 键码常量：导航进入键。激活当前具有焦点的项目或展开到导航层级的下一个级别。
    pub const KEYCODE_NAVIGATE_IN: i32 = 262;

    /// 键码常量：导航退出键。退出导航层级的一个级别或折叠当前具有焦点的项目。
    pub const KEYCODE_NAVIGATE_OUT: i32 = 263;

    /// 键码常量：Wear手表上的主要电源/重置按钮的主要茎键。
    pub const KEYCODE_STEM_PRIMARY: i32 = 264;

    /// 键码常量：Wear的通用茎键1
    pub const KEYCODE_STEM_1: i32 = 265;

    //noinspection SpellCheckingInspection
    /// 键码常量：方向键向上-向左
    pub const KEYCODE_DPAD_UP_LEFT: i32 = 268;

    //noinspection SpellCheckingInspection
    /// 键码常量：方向键向下向左
    pub const KEYCODE_DPAD_DOWN_LEFT: i32 = 269;

    //noinspection SpellCheckingInspection
    /// 键码常量：方向键右上
    pub const KEYCODE_DPAD_UP_RIGHT: i32 = 270;

    //noinspection SpellCheckingInspection
    /// 键码常量：方向键右下
    pub const KEYCODE_DPAD_DOWN_RIGHT: i32 = 271;

    /// 键码常量：Wear 的通用茎键 2
    pub const KEYCODE_STEM_2: i32 = 266;

    /// 键码常量：Wear 的通用茎键 3
    pub const KEYCODE_STEM_3: i32 = 267;

    /// 键码常量：跳过向前的媒体键。
    pub const KEYCODE_MEDIA_SKIP_FORWARD: i32 = 272;

    /// 键码常量：跳过向后的媒体键。
    pub const KEYCODE_MEDIA_SKIP_BACKWARD: i32 = 273;

    /// 键码常量：逐帧向前媒体键。每次向前移动一帧媒体。
    pub const KEYCODE_MEDIA_STEP_FORWARD: i32 = 274;

    /// 键码常量：逐帧向后媒体键。每次向后移动一帧媒体。
    pub const KEYCODE_MEDIA_STEP_BACKWARD: i32 = 275;

    /// 键码常量：除非持有唤醒锁，否则使设备进入休眠状态。
    pub const KEYCODE_SOFT_SLEEP: i32 = 276;

    /// 键码常量：剪切键。
    pub const KEYCODE_CUT: i32 = 277;

    /// 键码常量：复制键。
    pub const KEYCODE_COPY: i32 = 278;

    /// 键码常量：粘贴键。
    pub const KEYCODE_PASTE: i32 = 279;

    /// 键码常量：由系统用于向上导航
    pub const KEYCODE_SYSTEM_NAVIGATION_UP: i32 = 280;

    /// 键码常量：由系统用于向下导航
    pub const KEYCODE_SYSTEM_NAVIGATION_DOWN: i32 = 281;

    /// 键码常量：由系统用于向左导航
    pub const KEYCODE_SYSTEM_NAVIGATION_LEFT: i32 = 282;

    /// 键码常量：由系统用于向右导航
    pub const KEYCODE_SYSTEM_NAVIGATION_RIGHT: i32 = 283;

    /// 键码常量：显示所有应用
    pub const KEYCODE_ALL_APPS: i32 = 284;

    /// 键码常量：刷新键。
    pub const KEYCODE_REFRESH: i32 = 285;

    /// 键码常量：点赞键。应用可以使用此键让用户对内容进行点赞。
    pub const KEYCODE_THUMBS_UP: i32 = 286;

    /// 键码常量：反对键。应用可利用此功能让用户反对内容。
    pub const KEYCODE_THUMBS_DOWN: i32 = 287;

    /// 键码常量：用于切换当前正在使用内容的 android.accounts.Account。系统可能会使用该代码来全局设置账户。
    pub const KEYCODE_PROFILE_SWITCH: i32 = 288;

    /// 键码常量：视频应用键 #1。
    pub const KEYCODE_VIDEO_APP_1: i32 = 289;

    /// 键码常量：视频应用键 #2。
    pub const KEYCODE_VIDEO_APP_2: i32 = 290;

    /// 键码常量：视频应用键 #3。
    pub const KEYCODE_VIDEO_APP_3: i32 = 291;

    /// 键码常量：视频应用键 #4。
    pub const KEYCODE_VIDEO_APP_4: i32 = 292;

    /// 键码常量：视频应用键 #5。
    pub const KEYCODE_VIDEO_APP_5: i32 = 293;

    /// 键码常量：视频应用键 #6。
    pub const KEYCODE_VIDEO_APP_6: i32 = 294;

    /// 键码常量：视频应用键 #7。
    pub const KEYCODE_VIDEO_APP_7: i32 = 295;

    /// 键码常量：视频应用键 #8。
    pub const KEYCODE_VIDEO_APP_8: i32 = 296;

    /// 键码常量：特色应用键 #1。
    pub const KEYCODE_FEATURED_APP_1: i32 = 297;

    /// 键码常量：特色应用键 #2。
    pub const KEYCODE_FEATURED_APP_2: i32 = 298;

    /// 键码常量：特色应用键 #3。
    pub const KEYCODE_FEATURED_APP_3: i32 = 299;

    /// 键码常量：特色应用键 #4。
    pub const KEYCODE_FEATURED_APP_4: i32 = 300;

    /// 键码常量：演示应用键 #1。
    pub const KEYCODE_DEMO_APP_1: i32 = 301;

    /// 键码常量：演示应用键 #2。
    pub const KEYCODE_DEMO_APP_2: i32 = 302;

    /// 键码常量：演示应用键 #3。
    pub const KEYCODE_DEMO_APP_3: i32 = 303;

    /// 键码常量：演示应用键 #4。
    pub const KEYCODE_DEMO_APP_4: i32 = 304;

    /// 键码常量：键盘背光调暗
    pub const KEYCODE_KEYBOARD_BACKLIGHT_DOWN: i32 = 305;

    /// 键码常量：键盘背光调亮
    pub const KEYCODE_KEYBOARD_BACKLIGHT_UP: i32 = 306;

    /// 键码常量：键盘背光切换
    pub const KEYCODE_KEYBOARD_BACKLIGHT_TOGGLE: i32 = 307;

    /// 键码常量：触控笔笔杆上的主要按钮。这通常是最靠近触控笔尖的按钮。
    pub const KEYCODE_STYLUS_BUTTON_PRIMARY: i32 = 308;

    /// 键码常量：触控笔笔杆上的第二个按钮。这通常是从触控笔尖算起的第二个按钮。
    pub const KEYCODE_STYLUS_BUTTON_SECONDARY: i32 = 309;

    /// 键码常量：触控笔笔杆上的第三个按钮。这通常是从触控笔尖开始的第三个按钮。
    pub const KEYCODE_STYLUS_BUTTON_TERTIARY: i32 = 310;

    /// 键码常量：触控笔尾部的按钮。此按钮的使用通常与橡皮擦的功能无关。
    pub const KEYCODE_STYLUS_BUTTON_TAIL: i32 = 311;

    /// 键码常量：打开最近使用的应用程序视图（又称概览）。此键由框架处理，永远不会传递给应用程序。
    pub const KEYCODE_RECENT_APPS: i32 = 312;

    /// 键码常量：用户可以通过系统自定义其用途的按钮。用户可自定义键 #1。
    pub const KEYCODE_MACRO_1: i32 = 313;

    /// 键码常量：用户可以通过系统自定义其用途的按钮。用户可自定义键 #2。
    pub const KEYCODE_MACRO_2: i32 = 314;

    /// 键码常量：用户可以通过系统自定义其用途的按钮。用户可自定义键 #3。
    pub const KEYCODE_MACRO_3: i32 = 315;

    /// 键码常量：用户可以通过系统自定义其用途的按钮。用户可自定义键 #4。
    pub const KEYCODE_MACRO_4: i32 = 316;

    /// 最后一个 KEYCODE 的整数值。随着新的键码添加到 KeyEvent，该值会增加。
    pub const LAST_KEYCODE: i32 = Self::KEYCODE_MACRO_4;

    #[doc(hidden)]
    #[deprecated(note = "现在键码数量已超过 MAX_KEYCODE。请使用 getMaxKeyCode()。")]
    pub const MAX_KEYCODE: i32 = 84;

    /// getAction值：该键已被按下。
    pub const ACTION_DOWN: i32 = 0;

    /// getAction 值：按键已被释放。
    pub const ACTION_UP: i32 = 1;

    #[doc(hidden)]
    #[deprecated(
        note = "输入系统不再使用。getAction 值：连续发生多个重复按键事件，或者正在传递复杂字符串。如果按键代码不是 KEYCODE_UNKNOWN，则 getRepeatCount() 方法返回应执行给定按键代码的次数。否则，如果按键代码是 KEYCODE_UNKNOWN，则这是 getCharacters 返回的字符序列。"
    )]
    pub const ACTION_MULTIPLE: i32 = 2;

    /// SHIFT 键在 CAPS 模式下锁定。保留供 MetaKeyKeyListener 用于其 API 中已发布的常量。
    pub const META_CAP_LOCKED: i32 = 0x100;

    /// ALT 键已锁定。保留供 MetaKeyKeyListener 用于其 API 中已发布的常量。
    pub const META_ALT_LOCKED: i32 = 0x200;

    /// SYM 键已锁定。保留供 MetaKeyKeyListener 用于其 API 中已发布的常量。
    pub const META_SYM_LOCKED: i32 = 0x400;

    /// 文本处于选择模式。保留供 MetaKeyKeyListener 使用，用于其 API 中未发布的私有常量，目前由于遗留原因而保留。
    pub const META_SELECTING: i32 = 0x800;

    /// 此掩码用于检查是否按下了 ALT 元键之一。
    pub const META_ALT_ON: i32 = 0x02;

    /// 此掩码用于检查左边 ALT 元键是否被按下。
    pub const META_ALT_LEFT_ON: i32 = 0x10;

    /// 此掩码用于检查是否按下了右边 ALT 元键。
    pub const META_ALT_RIGHT_ON: i32 = 0x20;

    /// 此掩码用于检查是否按下了 SHIFT 元键之一。
    pub const META_SHIFT_ON: i32 = 0x1;

    /// 此掩码用于检查左 SHIFT 元键是否被按下。
    pub const META_SHIFT_LEFT_ON: i32 = 0x40;

    /// 此掩码用于检查是否按下了右 SHIFT 元键。
    pub const META_SHIFT_RIGHT_ON: i32 = 0x80;

    /// 此掩码用于检查 SYM 元键是否被按下。
    pub const META_SYM_ON: i32 = 0x4;

    /// 此掩码用于检查 FUNCTION 元键是否被按下。
    pub const META_FUNCTION_ON: i32 = 0x8;

    /// 此掩码用于检查是否按下了 CTRL 元键之一。
    pub const META_CTRL_ON: i32 = 0x1000;

    /// 此掩码用于检查左边 CTRL 元键是否被按下。
    pub const META_CTRL_LEFT_ON: i32 = 0x2000;

    /// 此掩码用于检查是否按下了右边 CTRL 元键。
    pub const META_CTRL_RIGHT_ON: i32 = 0x4000;

    /// 此掩码用于检查是否按下了 META 元键之一。
    pub const META_META_ON: i32 = 0x10000;

    /// 该掩码用于检查左META键是否按下。
    pub const META_META_LEFT_ON: i32 = 0x20000;

    /// 此掩码用于检查是否按下了正确的 META 元键。
    pub const META_META_RIGHT_ON: i32 = 0x40000;

    /// 此掩码用于检查 CAPS LOCK 元键是否打开。
    pub const META_CAPS_LOCK_ON: i32 = 0x100000;

    /// 此掩码用于检查 NUM LOCK 元键是否打开。
    pub const META_NUM_LOCK_ON: i32 = 0x200000;

    /// 此掩码用于检查 SCROLL LOCK 元键是否打开。
    pub const META_SCROLL_LOCK_ON: i32 = 0x400000;

    /// 此掩码是 META_SHIFT_ON、META_SHIFT_LEFT_ON 和 META_SHIFT_RIGHT_ON 的组合。
    pub const META_SHIFT_MASK: i32 =
        Self::META_SHIFT_ON | Self::META_SHIFT_LEFT_ON | Self::META_SHIFT_RIGHT_ON;

    /// 此掩码是 META_ALT_ON、META_ALT_LEFT_ON 和 META_ALT_RIGHT_ON 的组合。
    pub const META_ALT_MASK: i32 =
        Self::META_ALT_ON | Self::META_ALT_LEFT_ON | Self::META_ALT_RIGHT_ON;

    /// 此掩码是 META_CTRL_ON、META_CTRL_LEFT_ON 和 META_CTRL_RIGHT_ON 的组合。
    pub const META_CTRL_MASK: i32 =
        Self::META_CTRL_ON | Self::META_CTRL_LEFT_ON | Self::META_CTRL_RIGHT_ON;

    /// 此掩码是 META_META_ON、META_META_LEFT_ON 和 META_META_RIGHT_ON 的组合。
    pub const META_META_MASK: i32 =
        Self::META_META_ON | Self::META_META_LEFT_ON | Self::META_META_RIGHT_ON;

    /// 如果设备由于该键事件而被唤醒，则设置此掩码。
    #[deprecated(note = "由于系统本身会消耗所有唤醒键，因此系统永远不会设置此标志。")]
    pub const FLAG_WOKE_HERE: u32 = 0x1;

    /// 如果键事件是由软件键盘生成的，则设置此掩码。
    pub const FLAG_SOFT_KEYBOARD: u32 = 0x2;

    /// 如果我们不希望按键事件导致我们离开触摸模式，则设置此掩码。
    pub const FLAG_KEEP_TOUCH_MODE: u32 = 0x4;

    /// 如果已知事件来自系统的可信部分，则设置此掩码。也就是说，已知事件来自用户，并且不可能被第三方组件欺骗。
    pub const FLAG_FROM_SYSTEM: u32 = 0x8;

    /// 此掩码用于兼容性，以识别来自 IME 的输入键，该 IME 的输入键已自动标记为“下一个”或“完成”。这允许 TextView 将这些输入键作为旧应用程序的正常输入键进行分派，但在收到这些输入键时仍会执行适当的操作。
    pub const FLAG_EDITOR_ACTION: u32 = 0x10;

    /// 当与向上键事件相关联时，这表示按键已被取消。这通常用于虚拟触摸屏按键，用户可以从虚拟按键区域滑动到显示屏上：在这种情况下，应用程序将收到已取消的向上事件，并且不应执行通常与按键相关联的操作。
    /// 请注意，要使此功能正常工作，应用程序在收到向上事件或长按超时已过期之前不能对按键执行操作。
    pub const FLAG_CANCELED: u32 = 0x20;

    /// 此按键事件由虚拟（屏幕上）硬键区域生成。通常，这是触摸屏上常规显示屏之外的区域，专用于“硬件”按钮。
    pub const FLAG_VIRTUAL_HARD_KEY: u32 = 0x40;

    /// 此标志是为长按超时后发生的第一次按键重复设置的。
    pub const FLAG_LONG_PRESS: u32 = 0x80;

    /// 当按键事件因按下时执行长按动作而设置了 FLAG_CANCELED 时设置。
    pub const FLAG_CANCELED_LONG_PRESS: u32 = 0x100;

    /// 当此事件的按键代码从其初始按下时仍在被跟踪时，设置为 ACTION_UP。也就是说，有人请求在按键按下时开始跟踪，并且长按不会导致跟踪被取消。
    pub const FLAG_TRACKING: u32 = 0x200;

    /// 当合成按键事件以实现应用未处理的事件的默认行为时设置。后备按键事件由未处理的轨迹球运动（用于模拟方向键盘）和按键映射中声明的某些未处理的按键（例如，当 NumLock 关闭时的特殊功能数字键盘键）生成。
    pub const FLAG_FALLBACK: u32 = 0x400;

    /// 此标志表示此事件由无障碍服务修改或生成。值 = 0x800
    pub const FLAG_IS_ACCESSIBILITY_EVENT: u32 = 0x800;

    //noinspection SpellCheckingInspection
    /// 表示密钥正在被预先分派。
    pub const FLAG_PREDISPATCH: u32 = 0x20000000;

    /// 私人控制，用于确定应用程序何时跟踪按键序列。
    pub const FLAG_START_TRACKING: u32 = 0x40000000;

    /// 私有标志，指示系统何时检测到此按键事件可能与先前传送的按键事件的顺序不一致，例如，当发送了按键释放事件但按键并未按下时。
    pub const FLAG_TAINTED: u32 = 0x80000000;

    /// 返回最大键码。
    #[java_method]
    pub fn get_max_key_code() -> i32 {}

    /// 获取在字符 c 上添加重音后生成的字符。例如，getDeadChar('`', 'e') 返回 è。
    #[java_method]
    pub fn get_dead_char(accent: i32, c: i32) -> i32 {}

    /**
     * 创建一个新的按键事件。
     * `action` 操作代码：ACTION_DOWN、ACTION_UP 或 ACTION_MULTIPLE。
     * `code` 按键代码。
     * */
    #[java_constructor]
    pub fn new(action: i32, code: i32) -> Self {}

    /**
     * 创建一个新的按键事件。
     * `down_time` 此按键代码最初按下的时间（以 android.os.SystemClock.uptimeMillis 为单位）。
     * `event_time` 此事件发生的时间（以 android.os.SystemClock.uptimeMillis 为单位）。
     * `action` 操作代码：ACTION_DOWN、ACTION_UP 或 ACTION_MULTIPLE。
     * `code` 按键代码。
     * `repeat` 按下事件的重复计数（如果是在首次按下之后，则 > 0）或多个事件的事件计数。
     * */
    #[java_constructor]
    pub fn new_with_time(
        down_time: i64,
        event_time: i64,
        action: i32,
        code: i32,
        repeat: i32,
    ) -> Self {
    }

    /// 获取（可能回收的）按键事件。
    #[java_method]
    pub fn obtain(
        down_time: i64,
        event_time: i64,
        action: i32,
        code: i32,
        repeat: i32,
        meta_state: i32,
        device_id: i32,
        scan_code: i32,
        flags: u32,
        source: i32,
        display_id: i32,
        characters: String,
    ) -> Self {
    }

    /// 获取另一个关键事件的副本（可能被回收）。
    #[java_method]
    pub fn obtain_other(other: &Self) -> Self {}

    /**
     * 创建与给定事件相同的新按键事件，但其事件时间和重复次数将替换为给定值。
     * `event` 要复制的现有事件。这不会被修改。
     * `event_time` 事件的新事件时间（以 android.os.SystemClock.uptimeMillis 为单位）。
     * `new_repeat` 事件的新重复次数。
     * */
    #[java_method]
    pub fn change_time_repeat(event: &Self, event_time: i64, new_repeat: i32) -> Self {}

    /**
     * 创建与给定事件相同的新键事件，但其事件时间和重复次数将替换为给定值。
     * `event` 要复制的现有事件。这不会被修改。
     * `event_time` 事件的新事件时间（以 android.os.SystemClock.uptimeMillis 为单位）。
     * `new_repeat` 事件的新重复次数。
     * `new_flags` 事件的新标志，替换原始事件中的整个值。
     * */
    #[java_method]
    pub fn change_time_repeat_with_flags(
        event: &Self,
        event_time: i64,
        new_repeat: i32,
        new_flags: u32,
    ) -> Self {
    }

    /**
     * 创建一个与给定事件相同的新键事件，但其操作将替换为给定值。
     * `event` 要复制的现有事件。这不会被修改。
     * `action` 事件的新操作代码。
     * */
    #[java_method]
    pub fn change_action(event: &Self, action: i32) -> Self {}

    /**
     * 创建一个与给定事件相同的新键事件，但其标志将被替换为给定值。
     * `event` 要复制的现有事件。这不会被修改。
     * `flags` 新的标志常量。
     * */
    #[java_method]
    pub fn change_flags(event: &Self, flags: u32) -> Self {}

    /**
     * 返回：如果操作是 ACTION_DOWN，则返回 true；否则返回 false。
     * */
    #[deprecated(note = "不要在新代码中使用，而是明确检查 getAction()。")]
    #[java_method]
    pub fn is_down(&self) -> bool {}

    /**
     * 这是系统键吗？系统键不能用作菜单快捷键。
     * */
    #[java_method]
    pub fn is_system(&self) -> bool {}

    #[doc(hidden)]
    #[java_method(overload = isWakeKey)]
    pub fn is_wake(&self) -> bool {}

    /**
     * 如果指定的键码是游戏手柄按钮，则返回 true。
     * 返回：如果键码是游戏手柄按钮（例如 KEYCODE_BUTTON_A），则返回 True。
     * */
    #[java_method]
    pub fn is_gamepad_button(key_code: i32) -> bool {}

    /**
     * 默认情况下，按键是否会触发对焦点视图的点击。
     * */
    #[java_method]
    pub fn is_confirm_key(key_code: i32) -> bool {}

    /**
     * 返回此键是否将被发送到android.media.session.MediaSession。若未处理则回调。
     * */
    #[java_method]
    pub fn is_media_session_key(key_code: i32) -> bool {}

    /// 这是系统键吗？系统键不能用作菜单快捷键。
    #[java_method]
    pub fn is_system_key(key_code: i32) -> bool {}

    #[doc(hidden)]
    #[java_method]
    pub fn is_wake_key(key_code: i32) -> bool {}

    #[doc(hidden)]
    #[java_method]
    pub fn is_meta_key(key_code: i32) -> bool {}

    #[doc(hidden)]
    #[java_method]
    pub fn is_alt_key(key_code: i32) -> bool {}

    #[doc(hidden)]
    #[java_method]
    pub fn get_device_id(&self) -> i32 {}

    #[doc(hidden)]
    #[java_method]
    pub fn get_source(&self) -> i32 {}

    #[doc(hidden)]
    #[java_method]
    pub fn set_source(&self, source: i32) {}

    #[doc(hidden)]
    #[java_method]
    pub fn get_display_id(&self) -> i32 {}

    #[doc(hidden)]
    #[java_method]
    pub fn set_display_id(&self, display_id: i32) {}

    /// 返回元键的状态。
    /// 返回：一个整数，其中每个设置为 1 的位代表按下的元键
    #[java_method]
    pub fn get_meta_state(&self) -> i32 {}

    /**
     * 返回修饰键的状态。就此函数而言，KEYCODE_CAPS_LOCK、KEYCODE_SCROLL_LOCK 和 KEYCODE_NUM_LOCK 不被视为修饰键。因此，此函数专门屏蔽了 META_CAPS_LOCK_ON、META_SCROLL_LOCK_ON 和 META_NUM_LOCK_ON。返回的值由使用 normalizeMetaState(int) 规范化的元状态（来自 getMetaState）组成，然后使用 getModifierMetaStateMask 进行屏蔽，以便仅保留有效的修饰位。
     * 返回：一个整数，其中设置为 1 的每个位代表按下的修饰键。
     * */
    #[java_method]
    pub fn get_modifiers(&self) -> i32 {}

    /**
     * 修改事件的标志。
     * `new_flags` 事件的新标志，替换整个值。
     * */
    #[java_method]
    pub fn set_flags(&self, new_flags: u32) {}

    /**
     * 返回此键事件的标志。
     * */
    #[java_method]
    pub fn get_flags(&self) -> i32 {}

    /// 获取包含所有有效修饰键元状态位的掩码。就此函数而言，KEYCODE_CAPS_LOCK、KEYCODE_SCROLL_LOCK 和 KEYCODE_NUM_LOCK 不被视为修饰键。因此，掩码明确排除 META_CAPS_LOCK_ON、META_SCROLL_LOCK_ON 和 META_NUM_LOCK_ON。
    /// 返回：修饰键元状态掩码，它是 META_SHIFT_ON、META_SHIFT_LEFT_ON、META_SHIFT_RIGHT_ON、META_ALT_ON、META_ALT_LEFT_ON、META_ALT_RIGHT_ON、META_CTRL_ON、META_CTRL_LEFT_ON、META_CTRL_RIGHT_ON、META_META_ON、META_META_LEFT_ON、META_META_RIGHT_ON、META_SYM_ON、META_FUNCTION_ON 的组合。
    #[java_method]
    pub fn get_modifier_meta_state_mask() -> i32 {}

    /**
     * 如果此键码是修饰键，则返回 true。就此函数而言，KEYCODE_CAPS_LOCK、KEYCODE_SCROLL_LOCK 和 KEYCODE_NUM_LOCK 不被视为修饰键。因此，此函数对这些键返回 false。
     * 返回：如果键码是 KEYCODE_SHIFT_LEFT、KEYCODE_SHIFT_RIGHT、KEYCODE_ALT_LEFT、KEYCODE_ALT_RIGHT、KEYCODE_CTRL_LEFT、KEYCODE_CTRL_RIGHT、KEYCODE_META_LEFT 或 KEYCODE_META_RIGHT、KEYCODE_SYM、KEYCODE_NUM、KEYCODE_FUNCTION 之一，则返回 True。
     * */
    #[java_method]
    pub fn is_modifier_key(key_code: i32) -> bool {}

    /**
     * 规范化指定的元状态。元状态被规范化，这样如果设置了左或右修饰符元状态位，则结果还将包括该修饰符的通用位。如果指定的元状态包含 META_ALT_LEFT_ON，则结果除了 META_ALT_LEFT_ON 和输入中指定的其他位之外，还将包含 META_ALT_ON。
     * 对 shift、control 和 meta 执行相同的过程。如果指定的元状态包含 MetaKeyKeyListener 定义的合成元状态，则这些状态将在此处转换，并从结果中删除原始合成元状态。MetaKeyKeyListener.META_CAP_LOCKED 转换为 META_CAPS_LOCK_ON。MetaKeyKeyListener.META_ALT_LOCKED 转换为 META_ALT_ON。MetaKeyKeyListener.META_SYM_LOCKED 转换为 META_SYM_ON。未定义的元状态位被删除。
     * 返回：规范化的元状态。
     * `meta_state` 元状态。
     * */
    #[java_method]
    pub fn normalize_meta_state(meta_state: i32) -> i32 {}

    /**
     * 如果根据指定的元状态未按下任何修饰键，则返回 true。就此函数而言，KEYCODE_CAPS_LOCK、KEYCODE_SCROLL_LOCK 和 KEYCODE_NUM_LOCK 不被视为修饰键。因此，此函数忽略 META_CAPS_LOCK_ON、META_SCROLL_LOCK_ON 和 META_NUM_LOCK_ON。使用 normalizeMetaState(int) 比较之前，元状态已标准化。
     * 返回：如果没有按下任何修饰键，则返回 True。
     * `meta_state` 要考虑的元状态。
     * */
    #[java_method]
    pub fn meta_state_has_no_modifiers(meta_state: i32) -> bool {}

    /**
     * 如果根据指定的元状态仅按下了指定的修饰键，则返回 true。如果按下了不同的修饰键组合，则返回 false。就此函数而言，KEYCODE_CAPS_LOCK、KEYCODE_SCROLL_LOCK 和 KEYCODE_NUM_LOCK 不被视为修饰键。因此，此函数忽略 META_CAPS_LOCK_ON、META_SCROLL_LOCK_ON 和 META_NUM_LOCK_ON。
     * 如果指定的修饰符掩码包括方向修饰符（例如 META_SHIFT_LEFT_ON），则此方法可确保在该侧按下修饰符。如果指定的修饰符掩码包括非方向修饰符（例如 META_SHIFT_ON），则此方法可确保在任一侧按下修饰符。如果指定的修饰符掩码包含同一类型键的方向和非方向修饰符，例如 META_SHIFT_ON 和 META_SHIFT_LEFT_ON，则此方法将抛出非法参数异常。
     * 返回：如果仅按下了指定的修饰键，则返回 True。
     * 抛出：IllegalArgumentException – 如果 modifiers 参数包含无效修饰符
     * `meta_state` 要考虑的元状态。
     * `modifiers` 要检查的修饰键的元状态。可能是 getModifierMetaStateMask() 定义的修饰符元状态的组合。可以为 0，以确保没有按下修饰键。
     * */
    #[java_method]
    pub fn meta_state_has_modifiers(
        meta_state: i32,
        modifiers: i32,
    ) -> Result<bool, <Self as JType>::Error> {
    }

    /**
     * 如果没有按下修饰键，则返回 true。就此函数而言，KEYCODE_CAPS_LOCK、KEYCODE_SCROLL_LOCK 和 KEYCODE_NUM_LOCK 不被视为修饰键。因此，此函数忽略 META_CAPS_LOCK_ON、META_SCROLL_LOCK_ON 和 META_NUM_LOCK_ON。
     * 使用 normalizeMetaState(int) 进行比较之前，元状态已标准化。
     * 返回：如果没有按下修饰键，则返回 True。
     * */
    #[java_method]
    pub fn has_no_modifiers(&self) -> bool {}

    /**
     * 如果仅按下了指定的修饰键，则返回 true。如果按下了不同的修饰键组合，则返回 false。就此函数而言，KEYCODE_CAPS_LOCK、KEYCODE_SCROLL_LOCK 和 KEYCODE_NUM_LOCK 不被视为修饰键。因此，此函数忽略 META_CAPS_LOCK_ON、META_SCROLL_LOCK_ON 和 META_NUM_LOCK_ON。
     * 如果指定的修饰符掩码包括方向修饰符（例如 META_SHIFT_LEFT_ON），则此方法可确保在该侧按下修饰符。如果指定的修饰符掩码包括非方向修饰符（例如 META_SHIFT_ON），则此方法可确保在任一侧按下修饰符。如果指定的修饰符掩码包括同一类型键的方向修饰符和非方向修饰符（例如 META_SHIFT_ON 和 META_SHIFT_LEFT_ON），则此方法将引发非法参数异常。
     * 返回：如果仅按下了指定的修饰键，则为 True。
     * 抛出：IllegalArgumentException – 如果 modifiers 参数包含无效修饰键
     * `modifiers` 要检查的修饰键的元状态。可能是 getModifierMetaStateMask() 定义的修饰键元状态的组合。可能为 0，以确保未按下任何修饰键。
     * */
    #[java_method]
    pub fn has_modifiers(&self, modifiers: i32) -> Result<bool, <Self as JType>::Error> {}

    /**
     * 返回 ALT 元键的按下状态。如果按下了 ALT 键，则返回 true，否则返回 false
     * */
    #[java_method]
    pub fn is_alt_pressed(&self) -> bool {}

    /**
     * 返回 SHIFT 元键的按下状态。如果按下了 SHIFT 键，则返回 true，否则返回 false
     * */
    #[java_method]
    pub fn is_shift_pressed(&self) -> bool {}

    /**
     * 返回 SYM 元键的按下状态。
     * 返回：如果 SYM 键被按下，则返回 true，否则返回 false
     * */
    #[java_method]
    pub fn is_sym_pressed(&self) -> bool {}

    /**
     * 返回 CTRL 元键的按下状态。如果按下了 CTRL 键，则返回 true，否则返回 false
     * */
    #[java_method]
    pub fn is_ctrl_pressed(&self) -> bool {}

    /**
     * 返回 META 元键的按下状态。如果按下 META 键则返回 true，否则返回 false
     * */
    #[java_method]
    pub fn is_meta_pressed(&self) -> bool {}

    /**
     * 返回 FUNCTION 元键的按下状态。如果 FUNCTION 键被按下，则返回 true，否则返回 false
     * */
    #[java_method]
    pub fn is_function_pressed(&self) -> bool {}

    /**
     * 返回 CAPS LOCK 元键的锁定状态。如果 CAPS LOCK 键处于打开状态，则返回 true，否则返回 false
     * */
    #[java_method]
    pub fn is_caps_lock_on(&self) -> bool {}

    /**
     * 返回 SCROLL LOCK 元键的锁定状态。如果 SCROLL LOCK 键处于打开状态，则返回 true，否则返回 false
     * */
    #[java_method]
    pub fn is_num_lock_on(&self) -> bool {}

    /**
     * 查询此按键事件的操作。可能是 ACTION_DOWN、ACTION_UP 或 ACTION_MULTIPLE。
     * 返回：事件操作：ACTION_DOWN、ACTION_UP 或 ACTION_MULTIPLE。
     * */
    #[java_method]
    pub fn get_action(&self) -> i32 {}

    /// 对于 ACTION_UP 事件，表示事件已根据 FLAG_CANCELED 取消。
    #[java_method]
    pub fn is_canceled(&self) -> bool {}

    /**
     * 为按键事件设置 FLAG_CANCELED 标志。
     * */
    #[java_method]
    pub fn cancel(&self) {}

    /// 在 KeyEvent.Callback.onKeyDown 期间调用此方法，让系统跟踪按键直至其最后按下（可能包括长按）。请注意，一次只能跟踪一个按键 - 如果在跟踪前一个按键时收到另一个按键按下事件，则跟踪前一个事件时会停止。
    #[java_method]
    pub fn start_tracking(&self) {}

    /**
     * 对于 ACTION_UP 事件，表示该事件仍按照 FLAG_TRACKING 从其初始向下事件进行跟踪。
     * */
    #[java_method]
    pub fn is_tracking(&self) -> bool {}

    /// 对于 ACTION_DOWN 事件，表示该事件已根据 FLAG_LONG_PRESS 取消。
    #[java_method]
    pub fn is_long_press(&self) -> bool {}

    /// 检索按键事件的按键代码。这是按下的物理按键，而不是 Unicode 字符。
    /// 返回：事件的按键代码。
    #[java_method]
    pub fn get_key_code(&self) -> i32 {}

    /// 对于 ACTION_MULTIPLE 事件（其键代码为 KEYCODE_UNKNOWN）的特殊情况，这是与该事件关联的原始字符串。在所有其他情况下，它为空。
    /// 返回：返回与该事件关联的 1 个或多个字符的字符串。
    #[deprecated(note = "不再由输入系统使用。")]
    #[java_method]
    pub fn get_characters(&self) -> Option<String> {}

    /// 查询此按键事件的硬件按键 ID。这些值不可靠，且因设备而异。@more 主要用于调试目的。
    #[java_method]
    pub fn get_scan_code(&self) -> i32 {}

    /// 查询事件的重复计数。对于按键按下事件，这是按键重复的次数，第一次按下从 0 开始，然后从那里开始计数。对于按键弹起事件，此值始终等于零。对于多个按键事件，这是发生的按下/弹起对的数量。
    /// 返回：按键重复的次数。
    #[java_method]
    pub fn get_repeat_count(&self) -> i32 {}

    /**
     * 修改事件的按下时间和事件时间。
     * `down_time` 事件的新按下时间（以 android.os.SystemClock.uptimeMillis 为单位）。
     * `event_time` 事件的新事件时间（以 android.os.SystemClock.uptimeMillis 为单位）。
     * */
    #[java_method]
    pub fn set_time(down_time: i64, event_time: i64) {}

    /**
     * 查询最近一次按键按下事件的时间，以 android.os.SystemClock.uptimeMillis 时间为基准。如果这是按下事件，则此方法与 getEventTime() 相同。
     * 请注意，当按下按键时，此值是最近按下的按键的按下时间，该按键可能不是此事件的同一物理按键。
     * 返回：返回最近一次按键按下时间，以 android.os.SystemClock.uptimeMillis 时间为基准
     * */
    #[java_method]
    pub fn get_down_time(&self) -> i64 {}

    /**
     * 以 android.os.SystemClock.uptimeMillis 时间基准检索此事件发生的时间。
     * 返回：以 android.os.SystemClock.uptimeMillis 时间基准返回此事件发生的时间。
     * */
    #[java_method]
    pub fn get_event_time(&self) -> i64 {}

    /**
     * 查询此事件发生的时间，以 android.os.SystemClock.uptimeMillis 时间为基础，但精度为纳秒（而不是毫秒）。该值以纳秒为精度，但可能不具有纳秒的精度。
     * 返回：返回此事件发生的时间，以 android.os.SystemClock.uptimeMillis 时间为基础，但精度为纳秒（而不是毫秒）。
     * */
    #[java_method]
    pub fn get_event_time_nanos(&self) -> i64 {}

    /**
     * 已重命名为 getDeviceId。
     * */
    #[deprecated(note = "请改用 getDeviceId()。")]
    #[java_method]
    pub fn get_keyboard_device(&self) -> i32 {}

    /**
     * 获取此键的主要字符。换句话说，就是实际打印在其上的标签。
     * 返回：显示标签字符，如果没有则返回 0（例如，对于非打印键）。
     * */
    #[java_method]
    pub fn get_display_label(&self) -> char {}

    /**
     * 获取由指定键和元键状态组合生成的 Unicode 字符。返回当指定元位（参见 MetaKeyKeyListener）处于活动状态时，指定键将生成的 Unicode 字符。
     * 如果该键不是用于键入 Unicode 字符的键，则返回 0。如果返回值已设置位 KeyCharacterMap.COMBINING_ACCENT，则该键是“死键”，在使用 KeyCharacterMap.COMBINING_ACCENT_MASK 进行屏蔽后，应将其与另一个键组合以实际生成字符（参见 KeyCharacterMap.getDeadChar）。
     * 返回：相关字符或组合重音符，如果没有，则返回 0。
     * */
    #[java_method]
    pub fn get_unicode_char(&self) -> i32 {}

    /**
     * 获取由指定键和元键状态组合生成的 Unicode 字符。返回当指定元位（参见 MetaKeyKeyListener）处于活动状态时，指定键将生成的 Unicode 字符。如果该键不是用于键入 Unicode 字符的键，则返回 0。
     * 如果返回值已设置 KeyCharacterMap.COMBINING_ACCENT 位，则该键是“死键”，在使用 KeyCharacterMap.COMBINING_ACCENT_MASK 屏蔽后，应与另一个键组合以实际生成字符（参见 KeyCharacterMap.getDeadChar）。
     * 返回：相关字符或组合重音符，如果没有，则返回 0。
     * `meta_state` 元键修饰符状态。
     * */
    #[java_method(overload=getUnicodeChar)]
    pub fn get_unicode_char_from(meta_state: i32) -> i32 {}

    /**
     * 获取与该键关联的数字或符号。返回的是字符值，而不是数值。如果该键不是数字，而是符号，则返回该符号。
     * 此方法旨在支持键盘上的拨号盘和其他数字或符号输入，其中某些键兼具字母和符号键的功能。此方法返回与该键关联的数字或符号，与用户是否按下了所需的修饰键无关。
     * 例如，在一个特定的键盘上，按下 ALT 时，QWERTY 顶部行上的键会生成数字，因此 ALT-Q 会映射到“1”。因此，对于该键盘，当使用 KEYCODE_Q 调用 getNumber 时，它会返回“1”，以便用户可以在有意义时不按 ALT 来输入数字。
     * 返回：关联的数字或符号字符，如果没有，则返回 0。
     * */
    #[java_method]
    pub fn get_number(&self) -> char {}

    /**
     * 如果此键产生字形，则返回 true。
     * 如果此键是打印键，则返回 true。
     * */
    #[java_method]
    pub fn is_printing_key(&self) -> bool {}

    /**
     * 返回表示指定操作的符号名称的字符串，例如“ACTION_DOWN”，如果未知，则返回等效的数字常量，例如“35”。
     * 返回：指定操作的符号名称。
     * `action` 操作。
     * */
    #[java_method]
    pub fn action_to_string(action: i32) -> String {}

    //noinspection SpellCheckingInspection
    /**
     * 返回表示指定键码的符号名称的字符串，例如“KEYCODE_A”、“KEYCODE_DPAD_UP”或等效数字常量（如“1001”（如果未知）。
     * 此函数主要用于调试、日志记录和测试。它不特定于语言环境，也不旨在以面向用户的方式使用。
     * 返回：指定键码的符号名称。
     * `key_code` 键码。
     * */
    #[java_method]
    pub fn key_code_to_string(key_code: i32) -> String {}

    /**
     * 通过其符号名称（例如“KEYCODE_A”）或等效数字常量（例如“29”）获取键码。对于符号名称，从 Build.VERSION_CODES.Q 开始，前缀“KEYCODE_”是可选的。
     * 返回：键码，如果未找到，则返回 KEYCODE_UNKNOWN。
     * `symbolic_name` 键码的符号名称。
     * */
    #[java_method]
    pub fn key_code_from_string(symbolic_name: String) -> i32 {}

    /**
     * 返回一个字符串，该字符串表示指定的组合元键修饰符状态标志的符号名称，例如“0”、“META_SHIFT_ON”、“META_ALT_ON|META_SHIFT_ON”或等效数字常量，例如“0x10000000”（如果未知）。
     * 返回：指定的组合元状态标志的符号名称。
     * `meta_state` 元状态。
     * */
    #[java_method]
    pub fn meta_state_to_string(meta_state: i32) -> String {}
}

/// 当视图被点击并保持时调用的回调的接口定义。
#[allow(non_camel_case_types)]
#[java_interface(name = "android/view/View$OnLongClickListener")]
pub trait View_OnLongClickListener {
    /**
     * 当单击并按住某个视图时调用。
     * 返回：如果回调消耗了长按，则返回 true，否则返回 false。
     * `v` 被单击并按住的视图。
     * */
    fn on_long_click(&self, v: View) -> bool;
}

#[doc(hidden)]
#[allow(non_camel_case_types)]
#[java_class(name = "android/view/View$OnLongClickListenerImpl")]
pub struct View_OnLongClickListenerImpl(Box<dyn Fn(View) -> bool + Send + Sync>);

impl Default for View_OnLongClickListenerImplDefault {
    fn default() -> Self {
        Self(Box::new(|v| unimplemented!("{:?}", v)))
    }
}

#[java_implement]
impl View_OnLongClickListener for View_OnLongClickListenerImpl {
    fn on_long_click(&self, v: View) -> bool {
        self.0(v)
    }
}

/// 测试android.view
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
    view.set_context_clickable(true);
    assert_eq!(true, view.is_context_clickable());
    view.set_clickable(true);
    assert_eq!(true, view.is_clickable());
    view.set_long_clickable(true);
    assert_eq!(true, view.is_long_clickable());
    view.set_allow_click_when_disabled(true);
    view.perform_click();
    assert_eq!(None, view.get_parent::<ViewGroup>());

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
    let key_event = KeyEvent::new(KeyEvent::ACTION_DOWN, KeyEvent::KEYCODE_0);
    assert_eq!(KeyEvent::ACTION_DOWN, key_event.get_action());
    assert_eq!(KeyEvent::KEYCODE_0, key_event.get_key_code());
}
