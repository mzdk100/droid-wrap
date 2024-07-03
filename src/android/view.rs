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
    pub fn get_content_description<CS: CharSequence>(&self) -> Option<CS>
    where
        <CS as JObjNew>::Fields: Default,
    {
    }

    /**
     * 设置视图的内容描述。内容描述简要描述视图，主要用于辅助功能支持，以确定应如何向用户呈现视图。对于没有文本表示的视图（如 android.widget.ImageButton），有用的内容描述会解释视图的作用。例如，用于拨打电话的带有电话图标的图像按钮可以使用“呼叫”作为其内容描述。用于保存文件的软盘图像可以使用“保存”。这应该省略角色或状态。角色是指视图的用户界面元素类型，例如按钮或复选框。状态是指视图经常变化的属性，例如按钮的开/关状态或音量滑块的音频级别。内容描述更新并不频繁，并且在元素的语义内容（而不是状态）发生变化时使用。例如，在音乐播放期间，播放按钮可能会更改为暂停按钮。
     * `content_description` 内容描述。
     * */
    #[java_method]
    pub fn set_content_description<CS: CharSequence>(&self, content_description: Option<CS>)
    where
        <CS as JObjNew>::Fields: Default,
    {
    }

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
    pub fn set_on_click_listener<L: OnClickListener + JProxy>(&self, l: &L) {}

    /**
     * 获取与此视图关联的 LayoutParams。所有视图都应具有布局参数。这些参数为此视图的父级提供参数，指定应如何排列。
     * ViewGroup.LayoutParams 有许多子类，这些子类对应于负责排列其子级的 ViewGroup 的不同子类。如果此视图未附加到父 ViewGroup 或 setLayoutParams(ViewGroup.LayoutParams) 未成功调用，则此方法可能返回 null。当视图附加到父 ViewGroup 时，此方法不得返回 null。
     * 返回：与此视图关联的 LayoutParams，如果尚未设置参数，则返回 null
     * */
    #[java_method]
    pub fn get_layout_params(&self) -> Option<LayoutParams> {}

    /**
     * 设置与此视图相关的布局参数。这些参数为该视图的父级提供参数，指定应如何排列。ViewGroup 有许多子类。LayoutParams，这些对应于负责排列其子级的 ViewGroup 的不同子类。
     * `params` 此视图的布局参数，不能为空
     * */
    #[java_method]
    pub fn set_layout_params(&self, params: &LayoutParams) {}
}

/// 当视图被点击时调用的回调的接口定义。
#[java_interface(name = "android/view/View$OnClickListener")]
pub trait OnClickListener {
    /**
     * 当单击某个视图时调用。
     * `v` 被单击的视图。
     * */
    fn on_click(&self, v: View);
}

#[java_class(name = "android/view/View$OnClickListenerImpl")]
pub struct OnClickListenerImpl(Box<dyn Fn(View) + Sync + Send>);

impl OnClickListenerImpl {
    pub fn from_fn(func: impl Fn(View) + Sync + Send + 'static) -> Arc<Self> {
        Self::new(OnClickListenerImplDefault(Box::new(func)))
    }
}

impl Default for OnClickListenerImplDefault {
    fn default() -> Self {
        Self(Box::new(|_| ()))
    }
}

#[java_implement]
impl OnClickListener for OnClickListenerImpl {
    fn on_click(&self, v: View) {
        self.0(v)
    }
}

/**
 * 接口允许您向 Activity 添加和删除子视图。要获取此类的实例，请调用 Context.getSystemService()。
 * */
#[java_interface(name = "android/view/ViewManager")]
pub trait ViewManager {
    fn remove_view(&self, view: &View);
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
#[java_class(name = "android/view/ViewGroup$LayoutParams")]
pub struct LayoutParams;

impl LayoutParams {
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
#[java_class(name = "android/view/ViewGroup$MarginLayoutParams", extends=LayoutParams)]
pub struct MarginLayoutParams;

impl MarginLayoutParams {
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
    pub fn from_layout_params(source: &LayoutParams) -> Self {}
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
    let l = OnClickListenerImpl::from_fn(|_| {
        println!("View is clicked.");
    });
    view.set_on_click_listener(l.as_ref());
    let params = LayoutParams::new(LayoutParams::MATCH_PARENT, LayoutParams::MATCH_PARENT);
    params.set_width(40);
    assert_eq!(40, params.get_width());
    params.set_height(50);
    assert_eq!(50, params.get_height());
    let params = MarginLayoutParams::from_layout_params(&params);
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
}
