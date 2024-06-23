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

use droid_wrap_derive::{java_class, java_constructor, java_method};

use crate::{android::content::Context, java::lang::CharSequence, JObjNew, JObjRef, JType};

/**
 * 用于输入和修改文本的用户界面元素。定义编辑文本小部件时，必须指定 android.R.styleable.TextView_inputType 属性。例如，对于纯文本输入，将 inputType 设置为“text”：
 * ```xml
 *   <EditText
 *    android:id="@+id/ plain_text_input"
 *    android:layout_height="wrap_content"
 *    android:layout_width="match_parent"
 *    android:inputType="text"/>
 * ```
 * 选择输入类型可配置显示的键盘类型、可接受的字符以及编辑文本的外观。例如，如果您想接受秘密数字（如唯一 PIN 码或序列号），可以将 inputType 设置为“numericPassword”。inputType 为“numericPassword”会导致编辑文本仅接受数字，聚焦时显示数字键盘，并屏蔽输入的文本以保护隐私。有关其他 android.R.styleable.TextView_inputType 设置的示例，请参阅文本字段指南。
 * 您还可以通过在编辑文本中添加 android.text.TextWatcher 来接收用户更改文本时的回调。例如，当您想在进行更改时添加自动保存功能或验证用户输入的格式时，这很有用。您可以使用 TextView.addTextChangedListener 方法添加文本观察器。此小部件不支持自动调整文本大小。 XML 属性请参阅 EditText 属性、TextView 属性、View 属性
 * */
#[java_class(name = "android/widget/EditText", extends=TextView)]
pub struct EditText;

impl EditText {
    #[java_constructor]
    pub fn new(context: &Context) -> Self {}

    /**
     * 方便选择。选择所有。
     * */
    #[java_method]
    pub fn select_all(&self) {}
}

/**
 * 向用户显示文本的用户界面元素。要提供用户可编辑的文本，请参阅 EditText。
 * 以下代码示例展示了一种典型用法，其中包含 XML 布局和用于修改文本视图内容的代码：
 * ```xml
 *   <LinearLayout
 *     xmlns:android="http:// schemas. android. com/ apk/ res/ android"
 *     android:layout_width="match_parent"
 *     android:layout_height="match_parent">
 *   <TextView
 *       android:id="@+id/ text_view_id"
 *       android:layout_height="wrap_content"
 *       android:layout_width="wrap_content"
 *       android:text="@string/ hello" />
 * </ LinearLayout>
 * ```
 * 此代码示例演示了如何修改上一个 XML 布局中定义的文本视图的内容：
 * ```java
 * public class MainActivity extends Activity {
 * protected void onCreate(Bundle savedInstanceState) {
 * super.onCreate(savedInstanceState);
 * setContentView(R.layout.activity_main);
 * final TextView helloTextView = (TextView) findViewById(R.id.text_view_id);
 * helloTextView.setText(R.string.user_greeting);
 * }
 * }
 * ```
 * 要自定义 TextView 的外观，请参阅样式和主题。 XML 属性请参阅 TextView 属性、视图属性
 * */
#[java_class(name = "android/widget/TextView", extends=super::view::View)]
pub struct TextView;

impl TextView {
    #[java_constructor]
    pub fn new(context: &Context) -> Self {}

    /**
     * 设置要显示的文本。TextView 不接受类似 HTML 的格式，但您可以使用 XML 资源文件中的文本字符串进行这种格式设置。要设置字符串的样式，请将 android.text.style.* 对象附加到 android.text.SpannableString，或参阅可用资源类型文档，了解在 XML 资源文件中设置格式化文本的示例。必要时，TextView 将使用 Spannable.Factory 创建最终或中间 Spannable。同样，它将使用 Editable.Factory 创建最终或中间 Editable。如果传递的文本是 PrecomputedText，但用于创建 PrecomputedText 的参数与此 TextView 不匹配，则会引发 IllegalArgumentException。要确保参数匹配，您可以在调用此方法之前调用 setTextMetricsParams。
     * 可能抛出IllegalArgumentException 如果传递的文本是 PrecomputedText，但用于创建 PrecomputedText 的参数与此 TextView 不匹配。
     * `text` 要显示的文本
     * */
    #[java_method]
    pub fn set_text<CS: CharSequence>(&self, text: &CS) {}

    /**
     * 返回 TextView 正在显示的文本。如果使用 BufferType.SPANNABLE 或 BufferType.EDITABLE 参数调用 setText(CharSequence)，则可以将此方法的返回值分别转换为 Spannable 或 Editable。返回值的内容不应修改。如果您想要一个可修改的内容，您应该先制作自己的副本。
     * 返回：文本视图显示的文本。
     * */
    #[java_method]
    pub fn get_text<CS: CharSequence>(&self) -> CS {}

    /**
     * 设置 TextView 的文本为空时显示的文本。Null 表示使用普通的空文本。Hint 目前不参与确定视图的大小。
     * `hint` 要显示的提示文字。
     * */
    #[java_method]
    pub fn set_hint<CS: CharSequence>(&self, hint: &CS) {}

    /**
     * 返回TextView的文本为空时显示的提示。
     * */
    #[java_method]
    pub fn get_hint<CS: CharSequence>(&self) -> CS {}
}

#[cfg(feature = "test_android_widget")]
pub fn test() {
    use crate::{
        android::app::Activity,
        java::lang::{CharSequenceExt, CharSequenceImpl},
    };
    let ctx: Context = (&Activity::fetch()).into();
    let edit = EditText::new(&ctx);
    assert!(edit.to_string().starts_with("android.widget.EditText"));
    edit.select_all();
    let text = TextView::new(&ctx);
    assert!(text.to_string().starts_with("android.widget.TextView"));
    let cs = "你好".to_char_sequence::<CharSequenceImpl>();
    text.set_text(&cs);
    assert_eq!(cs, text.get_text::<CharSequenceImpl>());
    let cs = "世界".to_char_sequence::<CharSequenceImpl>();
    text.set_hint(&cs);
    assert_eq!(cs, text.get_hint::<CharSequenceImpl>());
}
