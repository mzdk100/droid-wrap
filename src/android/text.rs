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
    java::lang::{CharSequence, CharSequenceImpl},
    JObjNew, JObjRef, JProxy, JType,
};
use droid_wrap_derive::{java_class, java_implement, java_interface, java_method};
use std::sync::Arc;

/**
整数的位定义，定义可编辑对象中保存的文本的基本内容类型。支持的类可以与变体和标志组合以指示所需的行为。

示例
- 密码字段，密码对用户可见： inputType = TYPE_CLASS_TEXT | TYPE_TEXT_VARIATION_VISIBLE_PASSWORD
- 多行邮箱地址，自动大写： inputType = TYPE_CLASS_TEXT | TYPE_TEXT_VARIATION_POSTAL_ADDRESS | TYPE_TEXT_FLAG_MULTI_LINE
- 时间字段： inputType = TYPE_CLASS_DATETIME | TYPE_DATETIME_VARIATION_TIME
*/
#[java_interface(name = "android/text/InputType")]
pub trait InputType {
    /// 确定给定文本总体类别的位掩码。当前支持的类别有：TYPE_CLASS_TEXT、TYPE_CLASS_NUMBER、TYPE_CLASS_PHONE、TYPE_CLASS_DATETIME。IME 作者：如果您不了解该类别，则假定 TYPE_CLASS_TEXT 不带任何变体或标志。
    const TYPE_MASK_CLASS: i32 = 0x0000000f;

    /// 确定基础内容类的变化的位掩码。
    const TYPE_MASK_VARIATION: i32 = 0x00000ff0;

    /// 提供选项的附加位标志的位掩码。
    const TYPE_MASK_FLAGS: i32 = 0x00fff000;

    /// 未指定显式类型时的特殊内容类型。这应解释为目标输入连接不够丰富，无法处理和显示候选文本之类的内容，也无法检索当前文本，因此输入法将需要在有限的“生成按键事件”模式下运行（如果支持）。
    /// 请注意，某些输入法可能不支持该模式，例如，即使设置了此标志，基于语音的输入法也可能无法生成按键事件。
    const TYPE_NULL: i32 = 0x00000000;

    /// 普通文本类。该类支持以下标志（只能设置其中一个）：TYPE_TEXT_FLAG_CAP_CHARACTERS、TYPE_TEXT_FLAG_CAP_WORDS 和 TYPE_TEXT_FLAG_CAP_SENTENCES。
    /// 它还支持以下变体：TYPE_TEXT_VARIATION_NORMAL 和 TYPE_TEXT_VARIATION_URI。如果您无法识别变体，则应假定为普通。
    const TYPE_CLASS_TEXT: i32 = 0x00000001;

    /// TYPE_CLASS_TEXT 的标志：将所有字符大写。覆盖 TYPE_TEXT_FLAG_CAP_WORDS 和 TYPE_TEXT_FLAG_CAP_SENTENCES。此值明确定义为与 TextUtils#CAP_MODE_CHARACTERS 相同。当然，这只影响有大写和小写字母的语言。
    const TYPE_TEXT_FLAG_CAP_CHARACTERS: i32 = 0x00001000;

    /// TYPE_CLASS_TEXT 的标志：将每个单词的第一个字符大写。覆盖 TYPE_TEXT_FLAG_CAP_SENTENCES。此值明确定义为与 TextUtils#CAP_MODE_WORDS 相同。当然，这只影响有大写和小写字母的语言。
    const TYPE_TEXT_FLAG_CAP_WORDS: i32 = 0x00002000;

    /// TYPE_CLASS_TEXT 的标志：将每个句子的第一个字符大写。此值明确定义为与 TextUtils#CAP_MODE_SENTENCES 相同。例如，在英语中，它意味着在句号和空格后大写（请注意，其他语言可能对句号有不同的字符，或者不使用空格，或者使用不同的语法规则）。当然，这只影响有大写和小写字母的语言。
    const TYPE_TEXT_FLAG_CAP_SENTENCES: i32 = 0x00004000;

    /// TYPE_CLASS_TEXT 的标志：用户正在输入自由格式的文本，该文本应应用自动更正。如果没有此标志，IME 将不会尝试更正拼写错误。除非您真的希望用户在此字段中输入非单词，例如为游戏中的角色选择名称，否则您应该始终设置此标志。与 TYPE_TEXT_FLAG_AUTO_COMPLETE 和 TYPE_TEXT_FLAG_NO_SUGGESTIONS 形成对比：`` 表示 IME 将在用户输入时尝试自动更正拼写错误，但未定义 IME 是否提供显示建议的界面。
    const TYPE_TEXT_FLAG_AUTO_CORRECT: i32 = 0x00008000;

    //noinspection SpellCheckingInspection
    /// TYPE_CLASS_TEXT 的标志：文本编辑器（即应用程序）正在根据其自身的语义自动完成输入的文本，并在用户输入时将其呈现给用户。这通常意味着输入法不应显示候选词，但可以期望编辑器从 android.view.inputmethod.InputMethodSession#displayCompletions InputMethodSession.displayCompletions() 提供自己的完成/候选词，这是编辑器调用 android.view.inputmethod.InputMethodManager#displayCompletions InputMethodManager.displayCompletions() 的结果。
    /// 请注意与 TYPE_TEXT_FLAG_AUTO_CORRECT 和 TYPE_TEXT_FLAG_NO_SUGGESTIONS 的对比：`` 表示编辑器应显示一个用于显示建议的界面，但它不提供自己的界面，而是依靠编辑器传递完成/更正。
    const TYPE_TEXT_FLAG_AUTO_COMPLETE: i32 = 0x00010000;

    /// TYPE_CLASS_TEXT 的标志：可在字段中输入多行文本。如果未设置此标志，则文本字段将限制为一行。当未设置此标志时，IME 还可以选择不显示回车键，因为不需要创建新行。
    const TYPE_TEXT_FLAG_MULTI_LINE: i32 = 0x00020000;

    /// TYPE_CLASS_TEXT 标志：与此相关的常规文本视图不应该是多行，但是当全屏输入法提供文本时，如果可以的话它应该使用多行。
    const TYPE_TEXT_FLAG_IME_MULTI_LINE: i32 = 0x00040000;

    /// TYPE_CLASS_TEXT 的标志：输入法不需要显示任何基于字典的候选项。这对于不包含该语言单词且不会从任何基于字典的 补 全 或 更正 中受益的文本视图很有用。设置后，它会覆盖 TYPE_TEXT_FLAG_AUTO_CORRECT 值。请避免使用此选项，除非您确定这是您想要的。许多输入法需要建议才能正常工作，例如基于手势输入的建议。如果您只是不想让 IME 更正拼写错误，请考虑清除 TYPE_TEXT_FLAG_AUTO_CORRECT。
    /// 请注意与 TYPE_TEXT_FLAG_AUTO_CORRECT 和 TYPE_TEXT_FLAG_AUTO_COMPLETE 的对比：` ` 表示 IME 不需要显示界面来显示建议。大多数 IME 也会认为这意味着它们不需要尝试自动更正用户正在输入的内容。
    const TYPE_TEXT_FLAG_NO_SUGGESTIONS: i32 = 0x00080000;

    /// TYPE_CLASS_TEXT 标志：让 IME 知道应用程序需要文本转换建议。文本转换建议适用于具有发音字符和目标字符的音译语言。当用户输入发音字符时，IME 可以向用户提供可能的目标字符。设置此标志后，IME 应通过 Builder#setTextConversionSuggestions(List) 插入文本转换建议，并且 IME 将使用文本转换建议初始化的 TextAttribute 提供给应用程序。要接收附加信息，应用程序需要实现 InputConnection#setComposingText(CharSequence, int, TextAttribute)、InputConnection#setComposingRegion(int, int, TextAttribute) 和 InputConnection#commitText(CharSequence, int, TextAttribute)。
    const TYPE_TEXT_FLAG_ENABLE_TEXT_CONVERSION_SUGGESTIONS: i32 = 0x00100000;

    /// TYPE_CLASS_TEXT 的默认变体：普通的旧文本。
    const TYPE_TEXT_VARIATION_NORMAL: i32 = 0x00000000;

    /// type_class_text的变体：输入URI。
    const TYPE_TEXT_VARIATION_URI: i32 = 0x00000010;

    /// TYPE_CLASS_TEXT 的变体：输入电子邮件地址。
    const TYPE_TEXT_VARIATION_EMAIL_ADDRESS: i32 = 0x00000020;

    /// TYPE_CLASS_TEXT 的变体：输入电子邮件的主题行。
    const TYPE_TEXT_VARIATION_EMAIL_SUBJECT: i32 = 0x00000030;

    /// TYPE_CLASS_TEXT 的变体：输入简短、可能非正式的消息，例如即时消息或文本消息。
    const TYPE_TEXT_VARIATION_SHORT_MESSAGE: i32 = 0x00000040;

    /// TYPE_CLASS_TEXT 的变体：输入较长的、可能正式的消息的内容，例如电子邮件的正文。
    const TYPE_TEXT_VARIATION_LONG_MESSAGE: i32 = 0x00000050;

    /// TYPE_CLASS_TEXT 的变体：输入一个人的姓名。
    const TYPE_TEXT_VARIATION_PERSON_NAME: i32 = 0x00000060;

    /// TYPE_CLASS_TEXT 的变体：输入邮寄地址。
    const TYPE_TEXT_VARIATION_POSTAL_ADDRESS: i32 = 0x00000070;

    /// TYPE_CLASS_TEXT 的变体：输入密码。
    const TYPE_TEXT_VARIATION_PASSWORD: i32 = 0x00000080;

    /// TYPE_CLASS_TEXT 的变体：输入密码，该密码应该对用户可见。
    const TYPE_TEXT_VARIATION_VISIBLE_PASSWORD: i32 = 0x00000090;

    /// TYPE_CLASS_TEXT 的变体：在网络表单内输入文本。
    const TYPE_TEXT_VARIATION_WEB_EDIT_TEXT: i32 = 0x000000a0;

    /// TYPE_CLASS_TEXT 的变体：输入文本来过滤列表的内容等。
    const TYPE_TEXT_VARIATION_FILTER: i32 = 0x000000b0;

    /// TYPE_CLASS_TEXT 的变体：输入语音发音的文本，例如联系人中的语音姓名字段。这最适用于一种拼写可能有多种语音读法的语言，例如日语。
    const TYPE_TEXT_VARIATION_PHONETIC: i32 = 0x000000c0;

    //noinspection SpellCheckingInspection
    /// TYPE_CLASS_TEXT 的变体：在 Web 表单中输入电子邮件地址。此功能已添加到 android.os.Build.VERSION_CODES#HONEYCOMB。IME 必须以此 API 版本或更高版本为目标才能看到此输入类型；如果不是，则当通过 android.view.inputmethod.EditorInfo#makeCompatible(int) EditorInfo.makeCompatible(int) 传递时，此类型的请求将被视为 TYPE_TEXT_VARIATION_EMAIL_ADDRESS。
    const TYPE_TEXT_VARIATION_WEB_EMAIL_ADDRESS: i32 = 0x000000d0;

    //noinspection SpellCheckingInspection
    /// TYPE_CLASS_TEXT 的变体：在 Web 表单中输入密码。此功能已添加到 android.os.Build.VERSION_CODES#HONEYCOMB。IME 必须以此 API 版本或更高版本为目标才能看到此输入类型；如果不是，则当通过 android.view.inputmethod.EditorInfo#makeCompatible(int) EditorInfo.makeCompatible(int) 传递时，此类型的请求将被视为 TYPE_TEXT_VARIATION_PASSWORD。
    const TYPE_TEXT_VARIATION_WEB_PASSWORD: i32 = 0x000000e0;

    /// 数字文本类。此类支持以下标志：TYPE_NUMBER_FLAG_SIGNED 和 TYPE_NUMBER_FLAG_DECIMAL。它还支持以下变体：TYPE_NUMBER_VARIATION_NORMAL 和 TYPE_NUMBER_VARIATION_PASSWORD。IME 作者：如果您无法识别变体，则应假定为正常。
    const TYPE_CLASS_NUMBER: i32 = 0x00000002;

    /// TYPE_CLASS_NUMBER 标志：该数字是有符号的，允许在开头使用正号或负号。
    const TYPE_NUMBER_FLAG_SIGNED: i32 = 0x00001000;

    /// TYPE_CLASS_NUMBER 标志：数字是十进制，允许小数点提供分数值。
    const TYPE_NUMBER_FLAG_DECIMAL: i32 = 0x00002000;

    //noinspection SpellCheckingInspection
    /// TYPE_CLASS_NUMBER 的默认变体：普通数字文本。此功能已添加到 android.os.Build.VERSION_CODES#HONEYCOMB。IME 必须以此 API 版本或更高版本为目标才能看到此输入类型；否则，在通过 android.view.inputmethod.EditorInfo#makeCompatible(int) EditorInfo.makeCompatible(int) 传递时，此类型的请求将被丢弃。
    const TYPE_NUMBER_VARIATION_NORMAL: i32 = 0x00000000;

    //noinspection SpellCheckingInspection
    /// TYPE_CLASS_NUMBER 的变体：输入数字密码。此功能已添加到 android.os.Build.VERSION_CODES#HONEYCOMB。IME 必须以此 API 版本或更高版本为目标才能看到此输入类型；否则，在通过 android.view.inputmethod.EditorInfo#makeCompatible(int) EditorInfo.makeCompatible(int) 传递时，此类型的请求将被丢弃。
    const TYPE_NUMBER_VARIATION_PASSWORD: i32 = 0x00000010;

    /// 电话号码类。此类目前不支持任何变体或标志。
    const TYPE_CLASS_PHONE: i32 = 0x00000003;

    /// 日期和时间类。它支持以下变体：TYPE_DATETIME_VARIATION_NORMAL TYPE_DATETIME_VARIATION_DATE 和 TYPE_DATETIME_VARIATION_TIME。
    const TYPE_CLASS_DATETIME: i32 = 0x00000004;

    /// TYPE_CLASS_DATETIME 的默认变体：允许输入日期和时间。
    const TYPE_DATETIME_VARIATION_NORMAL: i32 = 0x00000000;

    /// TYPE_CLASS_DATETIME 的默认变体：仅允许输入日期。
    const TYPE_DATETIME_VARIATION_DATE: i32 = 0x00000010;

    /// TYPE_CLASS_DATETIME 的默认变体：只允许输入时间。
    const TYPE_DATETIME_VARIATION_TIME: i32 = 0x00000020;
}

#[doc(hidden)]
#[java_class(name = "android/text/InputTypeImpl")]
pub struct InputTypeImpl;

impl InputType for InputTypeImpl {}

/**
这是文本的接口，其内容和标记可以更改（与字符串等不可变文本相反）。如果您创建可编辑的 DynamicLayout，则布局将随着文本的更改而重新排列。
*/
#[java_interface(name = "android/text/Editable")]
pub trait Editable: CharSequence {
    #[doc(hidden)]
    type Cs: CharSequence;
    #[doc(hidden)]
    type E: Editable;

    /**
    用源切片 start…end 的副本替换此 Editable 中指定范围 (st…en) 的文本。目标切片可能为空，在这种情况下操作为插入；源切片可能为空，在这种情况下操作为删除。
    在提交更改之前，使用 setFilters 设置的每个过滤器都有机会修改源文本。如果源是 Spanned，则来自源的跨度将保留到 Editable 中。Editable 中完全覆盖替换范围的现有跨度将被保留，但严格在替换范围内的任何跨度将被删除。
    如果源包含带有 Spanned.SPAN_PARAGRAPH 标志的跨度，并且它不满足段落边界约束，则不会保留它。作为特殊情况，即使替换了光标所在的整个范围，光标位置也会保留。
    返回：对此对象的引用。
    */
    fn replace(&self, st: i32, en: i32, source: Self::Cs, start: i32, end: i32) -> Self::E;

    /**
    方便 replace(st, en, text, 0, text.length())
    */
    fn replace_convenience(&self, st: i32, en: i32, text: Self::Cs) -> Self::E;

    /**
    方便 replace(where, where, text, start, end)
    */
    fn insert(&self, r#where: i32, text: Self::Cs, start: i32, end: i32) -> Self::E;

    /**
    方便 replace(where, where, text, 0, text.length());
    */
    fn insert_convenience(&self, r#where: i32, text: Self::Cs) -> Self::E;

    /**
    方便 replace(st, en, "", 0, 0)
    */
    fn delete(&self, st: i32, en: i32) -> Self::E;

    /// 方便 replace(length(), length(), text, 0, text.length())
    fn append_convenience(&self, text: Self::Cs) -> Self::E;

    /// 方便 replace(length(), length(), text, start, end)
    fn append(&self, text: Self::Cs, start: i32, end: i32) -> Self::E;

    /// 方便 append(String.valueOf(text))
    fn append_char(&self, text: char) -> Self::E;

    /// 方便 replace(0, length(), "", 0, 0).
    /// 请注意，这将清除文本，而不是跨度；如果需要，请使用 clearSpans。
    fn clear(&self);

    /// 从可编辑对象中移除所有跨度，就像在每个跨度上调用 removeSpan 一样。
    fn clear_spans(&self);
}

#[doc(hidden)]
#[java_class(name = "android/text/EditableImpl", extends=CharSequenceImpl)]
pub struct EditableImpl;

impl CharSequence for EditableImpl {
    fn length(&self) -> i32 {
        self._based.length()
    }

    fn char_at(&self, index: i32) -> Result<char, droid_wrap_utils::Error> {
        self._based.char_at(index)
    }
}

#[java_implement]
impl Editable for EditableImpl {
    type Cs = CharSequenceImpl;
    type E = Self;

    #[java_method(type_bound=(<Self as Editable>::Cs, CharSequence), type_bound=(Self::E, Editable))]
    fn replace(
        &self,
        st: i32,
        en: i32,
        source: <Self as Editable>::Cs,
        start: i32,
        end: i32,
    ) -> Self::E {
    }

    #[java_method(overload=replace, type_bound=(<Self as Editable>::Cs, CharSequence), type_bound=(Self::E, Editable))]
    fn replace_convenience(&self, st: i32, en: i32, text: <Self as Editable>::Cs) -> Self::E {}

    #[java_method(type_bound=(<Self as Editable>::Cs, CharSequence), type_bound=(Self::E, Editable))]
    fn insert(&self, r#where: i32, text: <Self as Editable>::Cs, start: i32, end: i32) -> Self::E {}

    #[java_method(overload=insert, type_bound=(<Self as Editable>::Cs, CharSequence), type_bound=(Self::E, Editable))]
    fn insert_convenience(&self, r#where: i32, text: <Self as Editable>::Cs) -> Self::E {}

    #[java_method(type_bound=(Self::E, Editable))]
    fn delete(&self, st: i32, en: i32) -> Self::E {}

    #[java_method(overload=append, type_bound=(<Self as Editable>::Cs, CharSequence),type_bound=(Self::E, Editable))]
    fn append_convenience(&self, text: <Self as Editable>::Cs) -> Self::E {}

    #[java_method(type_bound=(<Self as Editable>::Cs, CharSequence),type_bound=(Self::E, Editable))]
    fn append(&self, text: <Self as Editable>::Cs, start: i32, end: i32) -> Self::E {}

    #[java_method(type_bound=(Self::E, Editable))]
    fn append_char(&self, text: char) -> Self::E {}

    #[java_method]
    fn clear(&self) {}

    #[java_method]
    fn clear_spans(&self) {}
}

/**
当此类型的对象附加到 Editable 时，其方法将在文本改变时被调用。
*/
#[java_interface(name = "android/text/TextWatcher")]
pub trait TextWatcher {
    #[doc(hidden)]
    type Cs: CharSequence;
    #[doc(hidden)]
    type E: Editable;

    /// 调用此方法是为了通知您，在 s 中，从 start 开始的 count 个字符即将被长度为 after 的新文本替换。
    /// 尝试从此回调更改 s 是错误的。
    fn before_text_changed(&self, s: Self::Cs, start: i32, count: i32, after: i32);

    /// 调用此方法是为了通知您，在 s 中，从 start 开始的 count 个字符刚刚替换了之前长度为 before 的旧文本。
    /// 尝试通过此回调更改 s 是错误的。
    fn on_text_changed(&self, s: Self::Cs, start: i32, before: i32, count: i32);

    //noinspection SpellCheckingInspection
    /// 调用此方法是为了通知您，在 s 中的某个地方，文本已发生更改。从此回调对 s 进行进一步更改是合法的，但请注意不要陷入无限循环，因为您所做的任何更改都会导致此方法再次递归调用。
    /// （您不会被告知更改发生的位置，因为其他 afterTextChanged() 方法可能已经进行了其他更改并使偏移量无效。但如果您需要在此处知道，您可以在 onTextChanged 中使用 Spannable.setSpan 来标记您的位置，然后从此处查找 span 结束的位置。
    fn after_text_changed(&self, s: Self::E);
}

#[doc(hidden)]
#[java_class(name = "android/text/TextWatcherImpl")]
pub struct TextWatcherImpl {
    before_text_changed:
        Box<dyn Fn(<TextWatcherImpl as TextWatcher>::Cs, i32, i32, i32) + Send + Sync>,
    on_text_changed: Box<dyn Fn(<TextWatcherImpl as TextWatcher>::Cs, i32, i32, i32) + Send + Sync>,
    after_text_changed: Box<dyn Fn(<TextWatcherImpl as TextWatcher>::E) + Send + Sync>,
}

impl TextWatcherImpl {
    pub fn from_fn(
        before_text_changed: impl Fn(
                /* s */ <Self as TextWatcher>::Cs,
                /* start */ i32,
                /* count */ i32,
                /* after */ i32,
            ) + Send
            + Sync
            + 'static,
        on_text_changed: impl Fn(
                /* s */ <Self as TextWatcher>::Cs,
                /* start */ i32,
                /* before */ i32,
                /* count */ i32,
            ) + Send
            + Sync
            + 'static,
        after_text_changed: impl Fn(/* s */ <Self as TextWatcher>::E) + Send + Sync + 'static,
    ) -> Arc<Self> {
        Self::new(TextWatcherImplDefault {
            before_text_changed: Box::new(before_text_changed),
            on_text_changed: Box::new(on_text_changed),
            after_text_changed: Box::new(after_text_changed),
        })
    }
}

impl Default for TextWatcherImplDefault {
    fn default() -> Self {
        Self {
            before_text_changed: Box::new(|_, _, _, _| ()),
            on_text_changed: Box::new(|_, _, _, _| ()),
            after_text_changed: Box::new(|_| ()),
        }
    }
}

#[java_implement]
impl TextWatcher for TextWatcherImpl {
    type Cs = CharSequenceImpl;
    type E = EditableImpl;

    fn before_text_changed(
        &self,
        s: <Self as TextWatcher>::Cs,
        start: i32,
        count: i32,
        after: i32,
    ) {
        (self.before_text_changed)(s, start, count, after)
    }

    fn on_text_changed(&self, s: <Self as TextWatcher>::Cs, start: i32, before: i32, count: i32) {
        (self.on_text_changed)(s, start, before, count)
    }

    fn after_text_changed(&self, s: <Self as TextWatcher>::E) {
        (self.after_text_changed)(s)
    }
}

/// 测试android.text
#[cfg(feature = "test_android_text")]
pub fn test() {
    let watcher = TextWatcherImpl::from_fn(
        |_s, _start, _count, _after| (),
        |_s, _start, _before, _count| (),
        |_s| (),
    );
    dbg!(watcher);
}
