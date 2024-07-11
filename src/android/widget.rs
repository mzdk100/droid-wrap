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

use droid_wrap_derive::{
    java_class, java_constructor, java_field, java_implement, java_interface, java_method,
};
use std::sync::Arc;

use crate::{
    android::{
        content::Context,
        text::TextWatcher,
        view::{KeyEvent, ViewGroup, ViewGroup_LayoutParams, ViewGroup_MarginLayoutParams},
    },
    java::lang::CharSequence,
    JObjNew, JObjRef, JProxy, JType,
};

/**
 * 用于输入和修改文本的用户界面元素。定义编辑文本小部件时，必须指定 android.R.styleable.TextView_inputType 属性。例如，对于纯文本输入，将 inputType 设置为 "text"：
 *   &lt;EditText
 *    android:id="@+id/plain_text_input"
 *    android:layout_height="wrap_content"
 *    android:layout_width="match_parent"
 *    android:inputType="text"/&gt;
 * 选择输入类型可配置显示的键盘类型、可接受的字符以及编辑文本的外观。例如，如果您想接受秘密数字（如唯一 PIN 码或序列号），可以将 inputType 设置为 "numericPassword"。inputType 为 "numericPassword" 会导致编辑文本仅接受数字，聚焦时显示数字键盘，并屏蔽输入的文本以保护隐私。有关其他 android.R.styleable.TextView_inputType 设置的示例，请参阅文本字段指南。
 * 您还可以通过在编辑文本中添加 android.text.TextWatcher 来接收用户更改文本时的回调。例如，当您想在进行更改时添加自动保存功能或验证用户输入的格式时，这很有用。您可以使用 TextView.addTextChangedListener 方法添加文本观察器。此小部件不支持自动调整文本大小。 XML 属性请参阅 EditText 属性、TextView 属性、View 属性
 * */
#[java_class(name = "android/widget/EditText", extends=TextView)]
pub struct EditText;

impl EditText {
    #[doc(hidden)]
    #[java_constructor]
    pub fn new(context: &Context) -> Self {}

    /**
     * 方便选择。选择所有。
     * */
    #[java_method]
    pub fn select_all(&self) {}

    /**
     * 方便选择。
     * setSelection(Spannable, int, int)。
     * */
    #[java_method]
    pub fn set_selection(&self, start: i32, stop: i32) -> Result<(), droid_wrap_utils::Error> {}

    /**
     * 子类会重写此功能以指定它们默认具有 KeyListener，即使在 XML 选项中没有特别调用。
     * */
    pub fn get_default_editable(&self) -> bool {
        self._based.get_default_editable()
    }
}

/**
 * 向用户显示文本的用户界面元素。要提供用户可编辑的文本，请参阅 EditText。
 * 以下代码示例展示了一种典型用法，其中包含 XML 布局和用于修改文本视图内容的代码：
 *   &lt;LinearLayout
 *     xmlns:android="http://schemas.android.com/apk/res/android"
 *     android:layout_width="match_parent"
 *     android:layout_height="match_parent"&gt;
 *   &lt;TextView
 *       android:id="@+id/text_view_id"
 *       android:layout_height="wrap_content"
 *       android:layout_width="wrap_content"
 *       android:text="@string/hello" /&gt;
 * &lt;/LinearLayout&gt;
 * 此代码示例演示了如何修改上一个 XML 布局中定义的文本视图的内容：
 * public class MainActivity extends Activity {
 * protected void onCreate(Bundle savedInstanceState) {
 * super.onCreate(savedInstanceState);
 * setContentView(R.layout.activity_main);
 * final TextView helloTextView = (TextView) findViewById(R.id.text_view_id);
 * helloTextView.setText(R.string.user_greeting);
 * }
 * }
 * 要自定义 TextView 的外观，请参阅样式和主题。 XML 属性请参阅 TextView 属性、视图属性
 * */
#[java_class(name = "android/widget/TextView", extends=super::view::View)]
pub struct TextView;

impl TextView {
    #[doc(hidden)]
    #[java_constructor]
    pub fn new(context: &Context) -> Self {}

    /**
     * 设置要显示的文本。TextView 不接受类似 HTML 的格式，但您可以使用 XML 资源文件中的文本字符串进行这种格式设置。要设置字符串的样式，请将 android.text.style.* 对象附加到 android.text.SpannableString，或参阅可用资源类型文档，了解在 XML 资源文件中设置格式化文本的示例。必要时，TextView 将使用 Spannable.Factory 创建最终或中间 Spannable。同样，它将使用 Editable.Factory 创建最终或中间 Editable。如果传递的文本是 PrecomputedText，但用于创建 PrecomputedText 的参数与此 TextView 不匹配，则会引发 IllegalArgumentException。要确保参数匹配，您可以在调用此方法之前调用 setTextMetricsParams。
     * 可能抛出IllegalArgumentException 如果传递的文本是 PrecomputedText，但用于创建 PrecomputedText 的参数与此 TextView 不匹配。
     * `text` 要显示的文本
     * */
    #[java_method]
    pub fn set_text<CS: CharSequence>(&self, text: Option<CS>) {}

    /**
     * 返回 TextView 正在显示的文本。如果使用 BufferType.SPANNABLE 或 BufferType.EDITABLE 参数调用 setText(CharSequence)，则可以将此方法的返回值分别转换为 Spannable 或 Editable。返回值的内容不应修改。如果您想要一个可修改的内容，您应该先制作自己的副本。
     * 返回：文本视图显示的文本。
     * */
    #[java_method]
    pub fn get_text<CS: CharSequence>(&self) -> Option<CS> {}

    /**
     * 设置 TextView 的文本为空时显示的文本。Null 表示使用普通的空文本。Hint 目前不参与确定视图的大小。
     * `hint` 要显示的提示文字。
     * */
    #[java_method]
    pub fn set_hint<CS: CharSequence>(&self, hint: Option<CS>) {}

    /**
     * 返回TextView的文本为空时显示的提示。
     * */
    #[java_method]
    pub fn get_hint<CS: CharSequence>(&self) -> Option<CS> {}

    /**
     * 子类会重写此功能以指定它们默认具有 KeyListener，即使在 XML 选项中没有特别调用。
     * */
    #[java_method]
    pub fn get_default_editable(&self) -> bool {}

    /**
     * 使用为 EditorInfo.inputType 定义的常量设置内容的类型。这将通过调用 setKeyListener(KeyListener) 来更改键侦听器，以匹配给定的内容类型。如果给定的内容类型是 EditorInfo.TYPE_NULL，则不会为此文本视图显示软键盘。
     * 请注意，如果更改输入类型的 EditorInfo.TYPE_TEXT_FLAG_MULTI_LINE 标志，则显示的最大行数（请参阅 setMaxLines(int)）将被修改。
     * `type` 输入类型。
     * */
    #[java_method]
    pub fn set_input_type(&self, r#type: i32) {}

    /**
     * 获取可编辑内容的类型。
     * */
    #[java_method]
    pub fn get_input_type(&self) -> i32 {}

    /**
     * 将 TextWatcher 添加到此 TextView 的文本发生变化时调用其方法的列表中。
     * 在 1.0 中，TextWatcher.afterTextChanged 方法在 setText 调用后被错误地未调用。现在，如果有任何文本更改侦听器执行 setText 会强制缓冲区类型为可编辑（否则不会为可编辑）并调用此方法。
     * `watcher` 文字监视器。
     * */
    #[java_method]
    pub fn add_text_changed_listener<TW: TextWatcher>(&self, watcher: &TW) {}

    /**
     * 从TextView的文本改变时会被调用其方法的TextWatcher列表中移除指定的TextWatcher。
     * `watcher` 文字监视器。
     * */
    #[java_method]
    pub fn remove_text_changed_listener<TW: TextWatcher>(&self, watcher: &TW) {}

    /**
     * 设置一个特殊侦听器，在文本视图上执行操作时调用。当按下回车键或用户选择提供给 IME 的操作时，将调用此侦听器。
     * 设置此项意味着普通硬键事件不会在文本视图中插入换行符，即使它是多行的；但是，按住 ALT 修饰键将允许用户插入换行符。
     * */
    #[java_method]
    pub fn set_on_editor_action_listener<L: TextView_OnEditorActionListener>(&self, l: &L) {}
}

/**
 * 在编辑器上执行操作时调用的回调的接口定义。
 * */
#[allow(non_camel_case_types)]
#[java_interface(name = "android/widget/TextView$OnEditorActionListener")]
pub trait TextView_OnEditorActionListener {
    /**
     * 执行操作时调用。
     * 返回:如果您已使用该操作，则返回 true，否则为 false。
     * `v` 被点击的视图。
     * `action_id` 操作的标识符。这将是您提供的标识符，或 EditorInfo。如果由于按下 Enter 键而调用，则为 IME_NULL。从 Android 14 开始，如果输入限制为一行，则在由 Enter 键触发时还将包含操作标识符。
     * `event` 如果由 Enter 键触发，则这是事件；否则，这是 null。
     * */
    fn on_editor_action(&self, v: TextView, action_id: i32, event: Option<KeyEvent>) -> bool;
}

#[doc(hidden)]
#[allow(non_camel_case_types)]
#[java_class(name = "android/widget/TextView$OnEditorActionListenerImpl")]
pub struct TextView_OnEditorActionListenerImpl(
    Box<dyn Fn(TextView, i32, Option<KeyEvent>) -> bool + Send + Sync>,
);

impl Default for TextView_OnEditorActionListenerImplDefault {
    fn default() -> Self {
        Self(Box::new(|v, action_id, event| {
            unimplemented!("{:?}, {}, {:?}", v, action_id, event)
        }))
    }
}

impl TextView_OnEditorActionListenerImpl {
    pub fn from_fn(
        func: impl Fn(
                /* v */ TextView,
                /* action_id */ i32,
                /* event */ Option<KeyEvent>,
            ) -> bool
            + Send
            + Sync
            + 'static,
    ) -> Arc<Self> {
        Self::new(TextView_OnEditorActionListenerImplDefault(Box::new(func)))
    }
}

#[java_implement]
impl TextView_OnEditorActionListener for TextView_OnEditorActionListenerImpl {
    fn on_editor_action(&self, v: TextView, action_id: i32, event: Option<KeyEvent>) -> bool {
        self.0(v, action_id, event)
    }
}

/// 要在活动中显示按钮，请将按钮添加到活动的布局 XML 文件：
///   &lt;Button
///       android:id="@+id/button_id"
///       android:layout_height="wrap_content"
///       android:layout_width="wrap_content"
///       android:text="@string/self_destruct" /&gt;
/// 要指定按下按钮时的操作，请在相应的活动代码中对按钮对象设置单击监听器：
///   public class MyActivity extends Activity {
///       protected void onCreate(Bundle savedInstanceState) {
///           super. onCreate(savedInstanceState);
///           setContentView(R. layout. content_layout_id);
///           final Button button = findViewById(R. id. button_id);
///           button.setOnClickListener(new View.OnClickListener() {
///               public void onClick(View v) {
///                   // 用户按下按钮后，此处的代码在主线程上执行
///               }
///           });
///       }
///   }
/// 上面的代码片段创建了一个 android.view.View.OnClickListener 实例，并使用 setOnClickListener(View.OnClickListener) 将侦听器连接到按钮。因此，系统会在用户按下按钮后执行您在 onClick(View) 中编写的代码。
/// 系统在主线程上执行 onClick 中的代码。这意味着您的 onClick 代码必须快速执行，以避免延迟您的应用对进一步用户操作的响应。有关更多详细信息，请参阅保持您的应用响应迅速。
/// 每个按钮都使用系统的默认按钮背景进行样式设置，该背景通常因平台的不同版本而异。如果您对默认按钮样式不满意，可以对其进行自定义。有关更多详细信息和代码示例，请参阅按钮样式指南。有关按钮上可用的所有 XML 样式属性，请参阅按钮属性、TextView 属性、视图属性。请参阅样式和主题指南，了解如何实现和组织与样式相关的属性的覆盖。
#[java_class(name = "android/widget/Button", extends=TextView)]
pub struct Button;

impl Button {
    /**
     * 从代码创建按钮时要使用的简单构造函数。
     * `context` 按钮正在运行的上下文，它可以访问当前主题，资源等。
     * */
    #[java_constructor]
    pub fn new(context: &Context) -> Self {}
}

/**
 * 将其他视图水平排列在一列中或垂直排列在一行中的布局。
 * 以下代码片段展示了如何在布局 XML 文件中包含线性布局：
 * &lt;LinearLayout xmlns:android="<http://schemas.android.com/apk/res/android>"
 *     android:layout_width="match_parent"
 *     android:layout_height="match_parent"
 *     android:paddingLeft="16dp"
 *     android:paddingRight="16dp"
 *     android:orientation="horizontal"
 *     android:gravity="center"&gt;
 *     &lt;!-- 在此处添加其他小部件或布局标签.这些标签被视为线性布局的 "子视图" 或 "子项" --&gt;
 *   &lt;/LinearLayout&gt;
 * 设置 android:orientation 以指定子视图是否显示在行或列中。要控制线性布局如何对齐其包含的所有视图，请为 android:gravity 设置一个值。例如，上面的代码片段将 android:gravity 设置为 "center"。
 * 您设置的值会影响单行或单列内所有子视图的水平和垂直对齐。您可以在各个子视图上设置 android:layout_weight 以指定线性布局如何在其包含的视图之间划分剩余空间。有关示例，请参阅线性布局指南。请参阅 LinearLayout.LayoutParams 以了解您可以在子视图上设置的其他属性，以影响其在包含的线性布局中的位置和大小。
 * */
#[java_class(name = "android/widget/LinearLayout", extends=ViewGroup)]
pub struct LinearLayout;

impl LinearLayout {
    #[doc(hidden)]
    pub const HORIZONTAL: i32 = 0;

    #[doc(hidden)]
    pub const VERTICAL: i32 = 1;

    #[doc(hidden)]
    #[java_constructor]
    pub fn new(context: &Context) -> Self {}

    /**
     * 布局应该是一列还是一行。
     * `orientation` 传递水平或垂直。默认值为水平。
     * */
    #[java_method]
    pub fn set_orientation(&self, orientation: i32) {}

    /**
     * 返回当前方向。
     * 返回：水平或垂直
     * */
    #[java_method]
    pub fn get_orientation(&self) -> i32 {}
}

/**
 * 与 ViewLinearLayout 相关的每个子布局信息。
 * */
#[allow(non_camel_case_types)]
#[java_class(name = "android/widget/LinearLayout$LayoutParams", extends=ViewGroup_MarginLayoutParams)]
pub struct LinearLayout_LayoutParams;

impl LinearLayout_LayoutParams {
    /// 指示 LinearLayout 中有多少额外空间将分配给与这些 LayoutParams 关联的视图。如果视图不应拉伸，请指定 0。否则，额外的像素将在权重大于 0 的所有视图之间按比例分配。
    #[java_field]
    pub fn get_weight(&self) -> f32 {}

    /// 指示 LinearLayout 中有多少额外空间将分配给与这些 LayoutParams 关联的视图。如果视图不应拉伸，请指定 0。否则，额外的像素将在权重大于 0 的所有视图之间按比例分配。
    #[java_field]
    pub fn set_weight(&self, value: f32) {}

    /// 与这些 LayoutParams 关联的视图的重力。
    #[java_field]
    pub fn get_gravity(&self) -> i32 {}

    /// 与这些 LayoutParams 关联的视图的重力。
    #[java_field]
    pub fn set_gravity(&self, value: i32) {}

    #[doc(hidden)]
    #[java_constructor]
    pub fn new(width: i32, height: i32) -> Self {}

    /**
     * 创建一组具有指定宽度、高度和权重的新布局参数。
     * `width` 宽度，可以是 MATCH_PARENT、WRAP_CONTENT 或固定大小（以像素为单位）
     * `height` 高度，可以是 MATCH_PARENT、WRAP_CONTENT 或固定大小（以像素为单位）
     * `weight` 权重
     * */
    #[java_constructor]
    pub fn new_with_weight(width: i32, height: i32, weight: f32) -> Self {}

    #[doc(hidden)]
    #[allow(unused_qualifications)]
    #[java_constructor]
    pub fn from_layout_params(p: &ViewGroup_LayoutParams) -> Self {}

    #[doc(hidden)]
    #[java_constructor]
    pub fn from_margin_layout_params(source: &ViewGroup_MarginLayoutParams) -> Self {}
}

/// 测试android.widget
#[cfg(feature = "test_android_widget")]
pub fn test() {
    use crate::{
        android::{
            app::Activity,
            text::{InputType, InputTypeImpl, TextWatcherImpl},
        },
        java::lang::{CharSequenceExt, CharSequenceImpl},
    };
    let context: Context = (&Activity::fetch()).into();
    let edit = EditText::new(&context);
    assert!(edit.to_string().starts_with("android.widget.EditText"));
    edit.select_all();
    // let _ = edit.set_selection(0,2);
    let editor_listener = TextView_OnEditorActionListenerImpl::from_fn(|_, _, _| true);
    edit.set_on_editor_action_listener(editor_listener.as_ref());

    let text = TextView::new(&context);
    assert!(text.to_string().starts_with("android.widget.TextView"));
    text.set_text(Some("你好".to_char_sequence::<CharSequenceImpl>()));
    assert_eq!(
        Some("你好".to_char_sequence()),
        text.get_text::<CharSequenceImpl>()
    );
    text.set_hint(Some("世界".to_char_sequence::<CharSequenceImpl>()));
    assert_eq!(
        Some("世界".to_char_sequence::<CharSequenceImpl>()),
        text.get_hint()
    );
    text.set_input_type(InputTypeImpl::TYPE_CLASS_DATETIME);
    assert_eq!(InputTypeImpl::TYPE_CLASS_DATETIME, text.get_input_type());
    let button = Button::new(&context);
    button.set_text(Some("测试按钮".to_char_sequence::<CharSequenceImpl>()));
    let layout = LinearLayout::new(&context);
    layout.set_orientation(LinearLayout::VERTICAL);
    assert_eq!(LinearLayout::VERTICAL, layout.get_orientation());
    let params = LinearLayout_LayoutParams::new_with_weight(
        ViewGroup_LayoutParams::MATCH_PARENT,
        ViewGroup_LayoutParams::MATCH_PARENT,
        1.0,
    );
    assert_eq!(1.0, params.get_weight());
    let watcher = TextWatcherImpl::from_fn(
        |s, start, count, after| {
            dbg!((s, start, count, after));
        },
        |s, start, before, count| {
            dbg!((s, start, before, count));
        },
        |s| {
            dbg!(s);
        },
    );
    button.add_text_changed_listener(watcher.as_ref());
    button.remove_text_changed_listener(watcher.as_ref());
}
