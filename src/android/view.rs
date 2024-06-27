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

use crate::{android::content::Context, java::lang::CharSequence, JObjNew, JObjRef, JProxy, JType};
use droid_wrap_derive::{
    java_class, java_constructor, java_implement, java_interface, java_method,
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

#[cfg(feature = "test_android_view")]
pub fn test() {
    use crate::{
        android::app::Activity,
        java::lang::{CharSequenceExt, CharSequenceImpl},
    };
    let ctx: Context = (&Activity::fetch()).into();
    let view = View::new(&ctx);
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
}
