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
    android::{content::Context, os::Bundle},
    JObjNew, JObjRef, JType,
};
use droid_wrap_derive::{java_class, java_method};

//noinspection SpellCheckingInspection
/**
 * 总体输入法框架 (IMF) 架构的中央系统 API，用于仲裁应用程序与当前输入法之间的交互。此处涵盖的主题：
 * 架构概述 应用程序 输入法 安全性
 *
 * 架构概述
 * 输入法框架 (IMF) 架构涉及三个主要方面：
 * 此类表示的输入法管理器是系统管理所有其他部分之间交互的中心点。它在此处表示为客户端 API，存在于每个应用程序上下文中，并与管理所有进程之间交互的全局系统服务进行通信。
 * 输入法 (IME) 实现一种特定的交互模型，允许用户生成文本。系统绑定到当前正在使用的输入法，从而创建并运行它，并告诉它何时隐藏和显示其 UI。一次只有一个 IME 在运行。
 * 多个客户端应用程序与输入法管理器仲裁输入焦点并控制 IME 的状态。一次只有一个这样的客户端处于活动状态（使用 IME）。
 *
 * 应用程序
 * 在大多数情况下，使用标准 android.widget.TextView 或其子类的应用程序几乎不需要做任何事情就可以很好地与软输入法配合使用。您需要注意的主要事项是：
 * 正确设置可编辑文本视图中的 android.R.attr.inputType，以便输入法具有足够的上下文来帮助用户在其中输入文本。在显示输入法时妥善处理丢失的屏幕空间。理想情况下，应用程序应该处理其窗口被调整得更小的情况，但它可以依赖系统在需要时执行窗口平移。您应该在活动上设置 android.R.attr.windowSoftInputMode 属性或在您创建的窗口上设置相应的值，以帮助系统确定是平移还是调整大小（它会尝试自动确定，但可能会出错）。
 * 您还可以使用相同的 android.R.attr.windowSoftInputMode 属性控制窗口的首选软输入状态（打开、关闭等）。通过此处的 API 可实现更细粒度的控制，以便直接与 IMF 及其 IME 交互 - 显示或隐藏输入区域、让用户选择输入法等。对于我们当中少数编写自己的文本编辑器的人来说，您需要实现 View.onCreateInputConnection 以返回您自己的 InputConnection 接口的新实例，从而允许 IME 与您的编辑器交互。
 *
 * 输入法
 * 输入法 (IME) 实现为 android.app.Service，通常从 InputMethodService 派生。它必须提供核心 InputMethod 接口，尽管这通常由 InputMethodService 处理，并且实现者只需要处理那里的高级 API。有关实现 IME 的更多信息，请参阅 InputMethodService 类。
 *
 * 安全性
 * 输入法存在许多安全问题，因为它们本质上可以自由地完全驱动 UI 并监视用户输入的所有内容。Android 输入法框架还允许任意第三方 IME，因此必须小心限制它们的选择和交互。以下是有关 IMF 背后的安全架构的一些要点：
 * 仅允许系统通过 Manifest.permission.BIND_INPUT_METHOD 权限直接访问 IME 的 InputMethod 接口。通过不绑定到不需要此权限的输入法服务，系统强制执行此操作，因此系统可以保证没有其他不受信任的客户端在其控制之外访问当前输入法。 IMF 可能有许多客户端进程，但一次只能有一个处于活动状态。
 * 非活动客户端无法通过下面描述的机制与 IMF 的关键部分进行交互。输入法的客户端仅被授予访问其 InputMethodSession 接口的权限。为每个客户端创建此接口的一个实例，并且当前 IME 只会处理来自与活动客户端关联的会话的调用。对于普通 IME，此操作由 android.inputmethodservice.AbstractInputMethodService 强制执行，但必须由自定义原始 InputMethodSession 实现的 IME 明确处理。
 * 只有活动客户端的 InputConnection 才会接受操作。 IMF 会告知每个客户端进程是否处于活动状态，并且框架会强制在非活动进程中忽略对当前 InputConnection 的调用。这可确保当前 IME 只能向用户认为处于焦点的 UI 传递事件和文本编辑。 IME 在屏幕关闭时永远无法与 InputConnection 交互。这是通过在屏幕关闭时使所有客户端处于非活动状态来强制执行的，并防止恶意 IME 在用户无法意识到其行为时驱动 UI。
 * 客户端应用程序可以要求系统让用户选择新的 IME，但不能以编程方式自行切换到新 IME。这可避免恶意应用程序将用户切换到自己的 IME，当用户导航到另一个应用程序时，该 IME 仍保持运行。另一方面，IME 被允许以编程方式将系统切换到另一个 IME，因为它已经完全控制了用户输入。用户必须先在设置中明确启用新 IME，然后才能切换到新 IME，以向系统确认他们知道该 IME 并希望将其启用。如果您的应用以 Android 11（API 级别 30）或更高版本为目标，则此类中的方法均会根据软件包可见性规则返回过滤结果，当前连接的 IME 除外。具有 InputMethod.SERVICE_INTERFACE 查询的应用会查看所有 IME。
 * */
#[java_class(name = "android/view/inputmethod/InputMethodManager")]
pub struct InputMethodManager;

impl InputMethodManager {
    pub const DISPATCH_IN_PROGRESS: i32 = -1;

    pub const DISPATCH_NOT_HANDLED: i32 = 0;

    pub const DISPATCH_HANDLED: i32 = 1;

    pub const SHOW_IM_PICKER_MODE_AUTO: i32 = 0;

    pub const SHOW_IM_PICKER_MODE_INCLUDE_AUXILIARY_SUBTYPES: i32 = 1;

    pub const SHOW_IM_PICKER_MODE_EXCLUDE_AUXILIARY_SUBTYPES: i32 = 2;

    /**
     * 当下一个 IME 焦点应用程序发生改变时，清除 SHOW_FORCED 标志。
     * 请注意，当在服务器端启用此标志时，SHOW_FORCED 将不再影响下一个焦点应用程序继续显示 IME，以防当下一个焦点应用程序不是 IME 请求者时出现意外的 IME 可见。
     * */
    pub const CLEAR_SHOW_FORCED_FLAG_WHEN_LEAVING: i64 = 214016041; // 这是一个错误ID。

    /// showSoftInput 的标志表明这是显示输入窗口的隐式请求，而不是用户直接请求的结果。在这种情况下，窗口可能不会显示。
    pub const SHOW_IMPLICIT: i32 = 0x0001;

    /// showSoftInput 的标志表示用户已强制打开输入法（例如通过长按菜单），因此在用户明确这样做之前不应关闭输入法。
    #[deprecated(
        note = "请改用不带此标志的 showSoftInput。使用此标志可能会导致软输入即使在调用应用程序关闭时仍然可见。使用此标志可以使软输入在全局范围内保持可见。从 Android T 开始，此标志仅在调用者当前处于焦点时才有效。"
    )]
    pub const SHOW_FORCED: i32 = 0x0002;

    /// showSoftInput(View, int, ResultReceiver) 和 hideSoftInputFromWindow(IBinder, int, ResultReceiver) 的 ResultReceiver 结果代码标志：软输入窗口的状态保持不变并保持显示。
    pub const RESULT_UNCHANGED_SHOWN: i32 = 0;

    /// showSoftInput(View, int, ResultReceiver) 和 hideSoftInputFromWindow(IBinder, int, ResultReceiver) 的 ResultReceiver 结果代码标志：软输入窗口的状态保持不变且保持隐藏。
    pub const RESULT_UNCHANGED_HIDDEN: i32 = 1;

    /// showSoftInput(View, int, ResultReceiver) 和 hideSoftInputFromWindow(IBinder, int, ResultReceiver) 的 ResultReceiver 结果代码的标志：软输入窗口的状态从隐藏更改为显示。
    pub const RESULT_SHOWN: i32 = 2;

    /// showSoftInput(View, int, ResultReceiver) 和 hideSoftInputFromWindow(IBinder, int, ResultReceiver) 的 ResultReceiver 结果代码的标志：软输入窗口的状态从显示更改为隐藏。
    pub const RESULT_HIDDEN: i32 = 3;

    /// hideSoftInputFromWindow 和 InputMethodService.requestHideSelf(int) 的标志表明仅当用户未明确显示软输入窗口时才应隐藏。
    pub const HIDE_IMPLICIT_ONLY: i32 = 0x0001;

    /// hideSoftInputFromWindow 和 InputMethodService.requestShowSelf(int) 的标志表明软输入窗口通常应该被隐藏，除非它最初是用 SHOW_FORCED 显示的。
    pub const HIDE_NOT_ALWAYS: i32 = 0x0002;

    /**
     * 确保对于通过反射或类似方式直接或间接依赖 sInstance 的应用程序，sInstance 变为非空。以下是我们知道的场景，可能还有更多我们目前还不知道的场景。
     * 通过反射直接访问 sInstance 的应用程序（由于 UnsupportedAppUsage 注释，目前是允许的）。目前，android.view.WindowManagerGlobal.getWindowSession() 很可能保证当这样的应用程序访问它时 sInstance 不为空，但从 android.view.WindowManagerGlobal.getWindowSession() 中删除该代码可能会在其应用程序中发现未经测试的代码路径，这可能发生在该应用程序的早期启动时。
     * 通过反射直接访问 peekInstance() 的应用程序（由于 UnsupportedAppUsage 注释，目前是允许的）。目前，android.view.WindowManagerGlobal.getWindowSession() 很可能保证当这样的应用程序调用 peekInstance() 时 peekInstance() 返回非空对象，但从 android.view.WindowManagerGlobal.getWindowSession() 中删除该代码。 getWindowSession() 可能会显示其应用中未经测试的代码路径，这可能发生在该应用的早期启动时间。
     * 好消息是，与 sInstance 的情况不同，我们至少可以通过更改 peekInstance() 的语义来解决此情况，目前 peekInstance() 的语义定义为“检索全局 InputMethodManager 实例（如果存在）”，并将其更改为始终返回非空 InputMethodManager 的语义。但是，如果在 android.view.WindowManagerGlobal.getWindowSession() 之前调用 peekInstance()，并且它期望 peekInstance() 返回 null（如 JavaDoc 中所述），则引入这种解决方法也可能会触发不同的兼容性问题。
     * 由于这纯粹是一种兼容性黑客攻击，因此必须仅从 android.view.WindowManagerGlobal.getWindowSession() 和 getInstance() 使用此方法。 TODO（Bug 116157766）：清理 UnsupportedAppUsage 后删除此方法。
     * */
    #[java_method]
    pub fn ensure_default_instance_for_default_display_if_necessary() {}

    /**
     * 检查活动输入连接（如果有）是否适用于给定的视图。
     * 请注意，view 参数不考虑 View.checkInputConnectionProxy(View)。当且仅当指定的视图是连接到 IME 的实际 View 实例时，此方法才返回 true。
     * `view` 要检查的视图。如果视图当前正在与 IME 交互，则返回 true。
     * */
    #[java_method]
    pub fn has_active_input_connection(&self, view: Option<super::View>) -> bool {}

    /**
     * 查询给定上下文的实例，如果尚不存在则创建它。
     * 返回：InputMethodManager 实例
     * `context` IME API 需要工作的上下文
     * */
    #[java_method]
    pub fn for_context(context: &Context) -> Self {}

    /**
     * 返回：全局 InputMethodManager 实例
     * */
    #[deprecated(
        note = "已弃用。请勿使用。请改用 Context.getSystemService(Class)。此方法无法完全支持多显示器场景。"
    )]
    #[java_method]
    pub fn get_instance() -> Self {}

    /// 返回：sInstance
    #[deprecated(
        note = "已弃用。请勿使用。请改用 Context.getSystemService(Class)。此方法无法完全支持多显示器场景。"
    )]
    #[java_method]
    pub fn peek_instance() -> Self {}

    /**
     * 如果当前选定的 IME 支持触控笔手写且已启用，则返回 true。如果该方法返回 false，则不应调用 startStylusHandwriting(View)，触控笔触摸应继续作为正常触摸输入。
     * */
    #[java_method]
    pub fn is_stylus_handwriting_available(&self) -> bool {}

    /**
     * 如果当前选定的 IME 支持触控笔手写且已为给定的 user_id 启用，则返回 true。如果该方法返回 false，则不应调用 startStylusHandwriting(View)，触控笔触控应继续作为正常触控输入。当且仅当 user_id 与当前进程的用户 ID 不同时，才需要 Manifest.permission.INTERACT_ACROSS_USERS_FULL。
     * `user_id` 要查询的用户 ID。
     * */
    #[java_method]
    pub fn is_stylus_handwriting_available_as_user(&self, user_id: i32) -> bool {}

    /// 允许您发现附加的输入法是否在全屏模式下运行。如果是全屏模式（完全覆盖您的 UI），则返回 true，否则返回 false。
    #[java_method]
    pub fn is_fullscreen_mode(&self) -> bool {}

    /**
     * 如果给定的视图是输入法的当前活动视图，则返回 true。
     * `view` 要检测的view。
     * */
    #[java_method(overload = isActive)]
    pub fn is_active_with_view(&self, view: &super::View) -> bool {}

    /// 如果当前有任何视图为输入方法，则返回true。
    #[java_method]
    pub fn is_active(&self) -> bool {}

    /**
     * 如果给定视图的 ViewRootImpl 是 InputMethodManager 的当前活动视图，则返回 true。
     * `attached_view` 要检测的view。
     * */
    #[java_method]
    pub fn is_current_root_view(&self, attached_view: &super::View) -> bool {}

    /// 如果当前服务的视图正在接受全文编辑，则返回 true。如果返回 false，则表示它没有输入连接，因此只能处理原始按键事件。
    #[java_method]
    pub fn is_accepting_text(&self) -> bool {}

    /// 如果输入法抑制系统拼写检查器，则返回 true。
    #[java_method]
    pub fn is_input_method_suppressing_spell_checker(&self) -> bool {}

    /**
     * 与没有结果接收器的 showSoftInput(View, int, ResultReceiver) 同义：明确请求向用户显示当前输入法的软输入区域（如果需要）。
     * `view` 当前聚焦的视图，希望接收软键盘输入。请注意，只有当此视图本身具有视图焦点并且其包含窗口具有窗口焦点时，此视图才被视为聚焦。否则调用失败并返回 false。
     * `flags` 提供其他操作标志。当前可能为 0 或设置了 SHOW_IMPLICIT 位。
     * */
    #[java_method]
    pub fn show_soft_input(&self, view: &super::View, flags: i32) -> bool {}

    /**
     * 启动手写笔手写会话。如果当前输入法支持，则在给定的 View 上启动手写笔手写会话，捕获所有手写笔输入并将其转换为 InputConnection 命令。如果 IME 成功启动手写模式，则任何当前调度的手写笔指针都将被取消。如果 IME 不支持手写笔手写模式或由于任何原因无法满足该模式，则请求将被忽略，并且手写笔触摸将继续作为正常触摸输入。
     * 理想情况下，应首先调用 isStylusHandwritingAvailable() 来确定 IME 是否支持手写笔手写。
     * `view` 请求手写笔手写的 View。它及其窗口必须处于焦点状态。
     * */
    #[java_method]
    pub fn start_stylus_handwriting(&self, view: &super::View) {}

    /**
     * 准备将启动手写笔手写会话的委托委托给与检测到初始手写笔迹的视图相同或不同的窗口中的其他编辑器。委托可用于在启动编辑器视图或其 InputConnection 之前启动手写笔手写会话。调用此方法将开始缓冲手写笔动作事件，直到调用 acceptStylusHandwritingDelegation(View)，此时可以启动手写会话并将缓冲的手写笔动作事件传送到 IME。
     * 例如，当初始手写笔迹位于伪编辑器（如小部件）（没有 InputConnection）上，但实际编辑器位于不同的窗口上时，可以使用委托。注意：如果使用手写笔在能够进行 InputConnection 的实际编辑器上书写，请改用 startStylusHandwriting(View)。
     * `delegator_view` 接收初始手写笔迹并将其委托给实际编辑器的视图。其窗口必须具有焦点。
     * */
    #[java_method]
    pub fn prepare_stylus_handwriting_delegation(&self, delegator_view: &super::View) {}

    /**
     * 准备将启动手写笔会话的委托委托给与检测到初始手写笔迹的视图相同或不同的窗口中的不同包中的其他编辑器。委托可用于在启动编辑器视图或其 InputConnection 之前启动手写笔会话。调用此方法将开始缓冲手写笔动作事件，直到调用 acceptStylusHandwritingDelegation(View, String)，此时可以启动手写会话并将缓冲的手写笔动作事件传送到 IME。
     * 例如，当初始手写笔迹位于伪编辑器（如小部件）（没有 InputConnection）上，但实际编辑器位于给定包中的不同窗口上时，可以使用委托。注意：如果委托者和委托者位于同一个包中，请改用 prepareStylusHandwritingDelegation(View)。
     * `delegator_view` 接收初始手写笔迹并将其委托给实际编辑器的视图。其窗口必须具有焦点。
     * `delegate_package_name` 包含实际编辑器的包名称，该编辑器应通过调用 acceptStylusHandwritingDelegation 来启动手写笔会话。
     * */
    #[java_method(overload = prepareStylusHandwritingDelegation)]
    pub fn prepare_stylus_handwriting_delegation_with_package(
        &self,
        delegator_view: &super::View,
        delegate_package_name: String,
    ) {
    }

    /**
     * 如果之前使用委托者的 prepareStylusHandwritingDelegation(View) 请求了手写启动委托，则在委托视图上接受并启动手写笔手写会话。注意：如果委托者和委托者位于不同的应用程序包中，则改用 acceptStylusHandwritingDelegation(View, String)。
     * 如果视图与 prepareStylusHandwritingDelegation(View) 中使用的视图属于同一应用程序包并且可以启动手写会话，则返回 true。
     * `delegate_view` 委托视图能够通过 InputConnection 接收输入，将在其上调用 startStylusHandwriting(View)。
     * */
    #[java_method]
    pub fn accept_stylus_handwriting_delegation(&self, delegate_view: &super::View) -> bool {}

    /**
     * 如果先前使用委托者的 prepareStylusHandwritingDelegation(View, String) 请求了手写启动委托，并且该视图属于指定的委托包，则接受并在委托视图上启动手写笔手写会话。注意：如果委托者和委托者位于同一应用程序包中，则改用 acceptStylusHandwritingDelegation(View)。
     * 如果视图属于 prepareStylusHandwritingDelegation(View, String) 中声明的允许委托包并且可以启动手写会话，则返回 true。
     * `delegate_view` 能够通过 InputConnection 接收输入的委托视图，将在其上调用 startStylusHandwriting(View)。
     * `delegator_package_name` 处理初始手写笔笔迹的委托人的包名称。
     * */
    #[java_method(overload = acceptStylusHandwritingDelegation)]
    pub fn accept_stylus_handwriting_delegation_with_package(
        &self,
        delegate_view: &super::View,
        delegator_package_name: String,
    ) -> bool {
    }

    /**
     * 此方法切换输入法窗口显示。如果输入窗口已显示，则隐藏。如果没有，则显示输入窗口。
     * `show_flags` 提供其他操作标志。可以为 0 或设置 SHOW_IMPLICIT、SHOW_FORCED 位。
     * `hide_flags` 提供其他操作标志。可以为 0 或设置 HIDE_IMPLICIT_ONLY、HIDE_NOT_ALWAYS 位。
     * */
    #[deprecated(
        note = "明确使用 showSoftInput(View, int) 或 hideSoftInputFromWindow(IBinder, int)。特别是在焦点变化期间，IME 的当前可见性定义不明确。从 Android S 开始，只有当调用应用是当前 IME 焦点时，这才会产生效果。"
    )]
    #[java_method]
    pub fn toggle_soft_input(&self, show_flags: i32, hide_flags: i32) {}

    /**
     * 如果输入法当前已连接到给定视图，则使用其新内容重新启动它。当视图中的文本在正常输入法或按键输入流之外发生变化时，您应该调用此方法，例如当应用程序调用 TextView.setText() 时。
     * `view` 文本已更改的视图。
     * */
    #[java_method]
    pub fn restart_input(&self, view: &super::View) {}

    /**
     * 向系统提示与 view 关联的文本是由非输入法编辑器 (IME) 更新的，这样系统就可以取消来自 IME 的任何待处理的文本编辑请求，直到它收到新的编辑上下文（例如 InputConnection.takeSnapshot() 提供的周围文本）。
     * 当 view 不支持 InputConnection.takeSnapshot() 协议时，调用此方法可能会触发 View.onCreateInputConnection(EditorInfo)。与 restartInput(View) 不同，此 API 不会立即与 InputConnection 交互。相反，应用程序可能会在以后根据需要接收 InputConnection.takeSnapshot()，以便系统可以捕获 IME 的新编辑上下文。
     * 例如，可以将此 API 的连续调用强制转换为 InputConnection.takeSnapshot() 的单个（或零个）回调。
     * `view` 文本已更改的视图。
     * */
    #[java_method]
    pub fn invalidate_input(&self, view: &super::View) {}

    #[java_method]
    pub fn add_virtual_stylus_id_for_test_session(&self) {}

    /**
     * 设置触控笔空闲超时时间，超过此时间后，手写 InkWindow 将被删除。此 API 仅用于测试。
     * `timeout` 以毫秒为单位设置。要重置为默认值，请使用 <= 零的值。
     * */
    #[java_method]
    pub fn set_stylus_window_idle_timeout_for_test(&self, timeout: i64) {}

    /**
     * 请注意，不应在 mH 锁内调用此方法，以防止启动输入后台线程可能被 mH 锁内已有的其他方法阻塞。
     * */
    #[java_method]
    pub fn check_focus(&self) {}

    /**
     * 报告当前选择范围。
     * 编辑器作者，每当光标在编辑器中移动时，您都需要调用此方法。请记住，除了这样做之外，每次调用 View.onCreateInputConnection(EditorInfo) 时，您的编辑器都需要始终在 EditorInfo.initialSelStart 和 EditorInfo.initialSelEnd 中提供当前光标值，每当键盘出现或焦点更改为文本字段时，都会发生这种情况，等等。
     * `view` `sel_start` `sel_end` `candidates_start` `candidates_end`
     * */
    #[java_method]
    pub fn update_selection(
        &self,
        view: &super::View,
        sel_start: i32,
        sel_end: i32,
        candidates_start: i32,
        candidates_end: i32,
    ) {
    }

    /**
     * 当用户轻按或单击文本视图时，通知事件。
     * `view` 要单击的view。
     * */
    #[deprecated(
        note = "对于充当巨大“画布”的复合视图，此方法的语义永远无法得到很好的定义，复合视图可以承载自己的 UI 层次结构和子焦点状态。android.webkit.WebView 就是一个很好的例子。应用程序/IME 开发人员不应依赖此方法。"
    )]
    #[java_method]
    pub fn view_clicked(&self, view: &super::View) {}

    /**
     * 如果当前输入法想要监视输入编辑器在其窗口中的光标位置，则返回 true。
     * */
    #[deprecated(note = "请改用 InputConnection.requestCursorUpdates(int)。")]
    #[java_method]
    pub fn is_watching_cursor(&self, view: &super::View) -> bool {}

    /// 如果当前输入法希望在光标/锚点位置改变时收到通知，则返回 true。
    #[deprecated(note = "此方法保留用于 UnsupportedAppUsage。不得使用。")]
    #[java_method]
    pub fn is_cursor_anchor_info_enabled(&self) -> bool {}

    /**
     * 设置 updateCursorAnchorInfo(View, CursorAnchorInfo) 的请求模式。
     * `flags` 标志。
     * */
    #[deprecated(note = "此方法保留用于 UnsupportedAppUsage。不得使用。")]
    #[java_method]
    pub fn set_update_cursor_anchor_info_mode(&self, flags: i32) {}

    /**
     * 报告窗口中的当前光标位置。
     * `view` `left` `top` `right` `bottom`
     * */
    #[deprecated(note = "请改用 updateCursorAnchorInfo(View, CursorAnchorInfo)。")]
    #[java_method]
    pub fn update_cursor(&self, view: &super::View, left: i32, top: i32, right: i32, bottom: i32) {}

    /**
     * 在当前输入法上调用 InputMethodSession.appPrivateCommand()。
     * `view` 发送命令的可选视图，如果要发送命令而不管附加到输入法的视图是什么，则为 null。
     * `action` 要执行的命令的名称。这必须是范围名称，即以您拥有的包名称为前缀，这样不同的开发人员就不会创建冲突的命令。
     * `data` 包含在命令中的任何数据。
     * */
    #[java_method]
    pub fn send_app_private_command(&self, view: &super::View, action: String, data: &Bundle) {}

    /**
     * 显示 IME 选择器弹出窗口。需要 PackageManager.FEATURE_INPUT_METHODS 功能，可使用 PackageManager.hasSystemFeature(String) 检测。
     * */
    #[java_method]
    pub fn show_input_method_picker(&self) {}

    /**
     * 显示系统的输入法选择器对话框。
     * `show_auxiliary_subtypes` 设置为 true 以显示辅助输入法。
     * `display_id` – 应显示选择器对话框的显示器的 ID。
     * */
    #[java_method]
    pub fn show_input_method_picker_from_system(
        &self,
        show_auxiliary_subtypes: bool,
        display_id: i32,
    ) {
    }

    /**
     * CTS 的测试 API，用于确保 showInputMethodPicker() 能够按预期工作。自定义 showInputMethodPicker() API 的实现时，请确保此测试 API 在 showInputMethodPicker() 显示 UI 时返回。否则您的操作系统实现可能无法通过 CTS。
     * 返回：在 showInputMethodPicker() 显示 UI 时返回 true。
     * */
    #[java_method]
    pub fn is_input_method_picker_shown(&self) -> bool {}

    /**
     * CTS 的测试 API，用于检查是否有任何待处理的 IME 可见性请求。
     * 返回：当且仅当存在待处理的 IME 可见性请求时才返回 true。
     * */
    #[java_method]
    pub fn has_pending_ime_visibility_requests(&self) -> bool {}

    /**
     * 显示指定输入法的子类型启用设置。
     * `imi_id` 将显示其子类型设置的输入法。如果 imiId 为空，则将显示所有输入法的子类型。
     * */
    #[java_method]
    pub fn show_input_method_and_subtype_enabler(imi_id: Option<String>) {}

    /// 通知用户已使用此输入法采取了某些操作。
    #[deprecated(note = "只是为了避免可能出现的应用兼容性问题。")]
    #[java_method]
    pub fn notify_user_action(&self) {}

    /**
     * 由于 android.compat.annotation.UnsupportedAppUsage 而保留了此值。
     * TODO（Bug 113914148）： 检查是否可以删除此值。我们意外地将 WindowManagerInternal#getInputMethodWindowVisibleHeight 暴露给了应用开发者，其中一些开发者开始依赖它。
     * 返回：定义不明确的内容。
     * */
    #[java_method]
    pub fn get_input_method_window_visible_height(&self) -> i32 {}

    /**
     * 控制 RemoteInputConnectionImpl.requestCursorUpdatesInternal(int, int, int) 中的显示 ID 不匹配验证。当 IME 客户端和 IME 在不同的显示器中运行时，updateCursorAnchorInfo(View, CursorAnchorInfo) 不能保证正常工作。这就是为什么当显示 ID 不匹配时，RemoteInputConnectionImpl.requestCursorUpdatesInternal(int, int, int) 默认返回 false。
     * 此方法允许特殊应用在确定应该有效时覆盖此行为。默认情况下，验证处于启用状态。
     * `enabled` false 表示禁用显示 ID 验证。
     * */
    #[java_method]
    pub fn set_request_cursor_update_display_id_check(&self, enabled: bool) {}

    /**
     * 如果当前显示器具有要应用的转换矩阵，则返回内部 API。如果设置了将虚拟显示屏坐标转换为主机屏幕坐标的矩阵，则返回 true。
     * */
    #[java_method]
    pub fn has_virtual_display_to_screen_matrix(&self) -> bool {}

    /**
     * 这仅用于 CTS 测试。请勿在 CTS 包之外使用此方法。
     * 返回：此 InputMethodManager 所在的显示器的 ID
     * */
    #[java_method]
    pub fn get_display_id(&self) -> i32 {}
}

//noinspection SpellCheckingInspection
#[cfg(feature = "test_android_view_inputmethod")]
pub fn test() {
    use crate::{
        android::{app::Activity, content::Context},
        java::lang::ObjectExt,
    };
    let context: Context = (&Activity::fetch()).into();
    let imm: InputMethodManager = context
        .get_system_service(Context::INPUT_METHOD_SERVICE.to_string())
        .unwrap()
        .cast();
    assert!(imm
        .to_string()
        .starts_with("android.view.inputmethod.InputMethodManager"));
}
