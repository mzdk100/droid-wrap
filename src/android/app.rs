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

use droid_wrap_derive::{java_class, java_method};
use droid_wrap_utils::{android_context, vm_attach};

use crate::{
    android::view::{View, WindowManager},
    java::lang::{CharSequence, Runnable},
    JObjNew, JObjRef, JType,
};

//noinspection SpellCheckingInspection
/// 活动是用户可以执行的单一、集中的操作。几乎所有活动都会与用户交互，因此 Activity 类会负责为您创建一个窗口，您可以使用 setContentView 将 UI 放置在该窗口中。
/// 虽然活动通常以全屏窗口的形式呈现给用户，但它们也可以以其他方式使用：浮动窗口（通过设置了 android.R.attr.windowIsFloating 的主题）、多窗口模式或嵌入到其他窗口中。几乎所有 Activity 子类都会实现两种方法： onCreate 是您初始化活动的地方。
/// 最重要的是，在这里您通常会使用定义 UI 的布局资源调用 setContentView(int)，并使用 findViewById 检索该 UI 中您需要以编程方式与之交互的小部件。
/// onPause 是您处理用户暂停与活动进行活动交互的地方。此时应提交用户所做的任何更改（通常提交给保存数据的 android.content.ContentProvider）。在此状态下，活动在屏幕上仍然可见。与 Context 一起使用。startActivity()，所有活动类都必须在其包的 AndroidManifest.xml 中具有相应的 声明。
/// 此处涵盖的主题：片段、活动生命周期、配置更改、启动活动和获取结果、保存持久状态、权限、流程生命周期
///
/// 开发者指南
/// Activity 类是应用程序整个生命周期的重要组成部分，而活动的启动和组合方式是平台应用程序模型的基本组成部分。如需详细了解 Android 应用程序的结构以及活动的行为方式，请阅读应用程序基础知识和任务和返回堆栈开发者指南。您还可以在活动开发者指南中找到有关如何创建活动的详细讨论。
///
/// Fragments
/// androidx.fragment.app.FragmentActivity 子类可以利用 androidx.fragment.app.Fragment 类来更好地模块化其代码，为更大的屏幕构建更复杂的用户界面，并帮助在小屏幕和大屏幕之间扩展其应用程序。有关使用 fragments 的更多信息，请阅读 Fragments 开发者指南。
///
/// 活动生命周期
/// 系统中的活动作为活动堆栈进行管理。当启动新的活动时，通常将其放置在当前堆栈的顶部，并成为运行活动 - 先前的活动始终保持在堆栈中的下方，并且直到新活动退出之前，它不会再次进入前景。屏幕上可以看到一个或多个活动堆栈。
/// 一项活动本质上有四个状态：如果活动在屏幕的前景（最高堆栈的最高位置），则它是活动的或运行的。这通常是用户当前与之交互的活动。如果一项活动失去了焦点，但仍会显示给用户，则可以看到。如果新的非填充大小或透明活动集中在您的活动之上，则可能在多窗口模式下具有较高的位置，或者活动本身在当前窗口模式下不可集中。此类活动完全活着（它维护所有州和会员信息，并且仍然附加到窗口管理器上）。
/// 如果一个活动完全被另一个活动掩盖，则将其停止或隐藏。它仍然保留所有状态和成员信息，但是，用户不再可见，因此其窗口被隐藏了，当其他地方需要内存时，它通常会被系统杀死。该系统可以通过要求完成或简单地杀死其过程，从而使其从内存中删除活动，从而摧毁其过程。当它再次显示给用户时，必须将其完全重新启动并恢复为先前的状态。
/// 下图显示了活动的重要状态路径。正方形矩形表示您可以实现的回调方法，以执行活动在状态之间移动时执行操作。彩色椭圆形是活动中的主要状态。
/// 您可能有兴趣在 Activity 中监控三个关键循环： Activity 的整个生命周期发生在第一次调用 onCreate 到最后一次调用 onDestroy 之间。Activity 将在 onCreate() 中设置“全局”状态，并在 onDestroy() 中释放所有剩余资源。
/// 例如，如果它有一个线程在后台运行以从网络下载数据，它可能会在 onCreate() 中创建该线程，然后在 onDestroy() 中停止该线程。 Activity 的可见生命周期发生在调用 onStart 到相应的 onStop 调用之间。在此期间，用户可以在屏幕上看到 Activity，尽管它可能不在前台并与用户交互。在这两种方法之间，您可以维护向用户显示 Activity 所需的资源。例如，您可以在 onStart() 中注册一个 android.content.BroadcastReceiver 来监控影响 UI 的更改，并在用户不再看到您正在显示的内容时在 onStop() 中取消注册它。
/// onStart() 和 onStop() 方法可以多次调用，因为活动对用户可见或隐藏。活动的前台生命周期发生在对 onResume 的调用和对 onPause 的相应调用之间。在此期间，活动可见、处于活动状态并与用户交互。活动可以频繁地在恢复和暂停状态之间切换 - 例如当设备进入睡眠状态时、当活动结果传递时、当新意图传递时 - 因此这些方法中的代码应该相当轻量。活动的整个生命周期由以下活动方法定义。所有这些都是钩子，您可以覆盖它们以在活动更改状态时执行适当的工作。
/// 所有活动都将实现 onCreate 以进行初始设置；许多活动还将实现 onPause 以提交数据更改并准备暂停与用户的交互，以及 onStop 以处理不再显示在屏幕上的情况。在实现这些方法时，您应该始终调用您的超类。
///   public class Activity extends ApplicationContext {
///       protected void onCreate(Bundle savedInstanceState);
///
///       protected void onStart();
///
///       protected void onRestart();
///
///       protected void onResume();
///
///       protected void onPause();
///
///       protected void onStop();
///
///       protected void onDestroy();
///    }
/// 一般来说，活动生命周期的流程如下：
/// 方法 | 描述 | 可终止？| 下一步
/// onCreate() | 首次创建活动时调用。您应该在此处进行所有常规静态设置：创建视图、将数据绑定到列表等。此方法还为您提供了一个包含活动先前冻结状态的 Bundle（如果有）。始终紧随 onStart()。 | 否 | onStart()
/// onRestart() | 在您的活动停止后、再次启动之前调用。始终紧随 onStart() | 否 | onStart()
/// onStart() | 当活动对用户可见时调用。如果活动进入前台，则紧随 onResume()，如果活动隐藏，则紧随 onStop()。 | 否 | onResume() 或 onStop()
/// onResume() | 当活动开始与用户交互时调用。此时您的活动位于其活动堆栈的顶部，用户输入将发送到它。始终紧随 onPause()。| 否 | onPause()
/// onPause() |当 Activity 失去前台状态、不再可聚焦或在转换为停止/隐藏或销毁状态之前调用。Activity 对用户仍然可见，因此建议保持其可见活动并继续更新 UI。此方法的实现必须非常快，因为下一个 Activity 将不会恢复，直到此方法返回。如果 Activity 返回到前台，则后面跟着 onResume()，如果它对用户不可见，则后面跟着 onStop()。 | Pre-Build.VERSION_CODES.HONEYCOMB | onResume() 或 onStop()
/// onStop() | 当 Activity 对用户不再可见时调用。这可能是因为在顶部启动了新 Activity、将现有 Activity 置于此 Activity 前面或此 Activity 被销毁。这通常用于停止动画和刷新 UI 等。如果此 Activity 要回来与用户交互，则后面跟着 onRestart()，如果此 Activity 要消失，则后面跟着 onDestroy()。 | 是 | onRestart() 或 onDestroy()
/// onDestroy() | 您的活动被销毁之前收到的最后一个调用。发生这种情况的原因可能是活动正在结束（有人在其上调用了 finish），也可能是因为系统暂时销毁了此活动实例以节省空间。您可以使用 isFinishing 方法区分这两种情况。 | 是 | 无
/// 请注意上表中的“可杀死”列 - 对于标记为可杀死的方法，在该方法返回后，系统可能会随时杀死承载活动的进程，而无需执行其另一行代码。因此，您应该使用 onPause 方法将任何持久数据（例如用户编辑）写入存储。此外，在将活动置于此类后台状态之前会调用方法 onSaveInstanceState(Bundle)，允许您将活动中的任何动态实例状态保存到给定的 Bundle 中，如果需要重新创建活动，则稍后在 onCreate 中接收。有关进程生命周期如何与其托管的活动相关联的更多信息，请参阅进程生命周期部分。
/// 请注意，在 onPause 而不是 onSaveInstanceState 中保存持久数据非常重要，因为后者不是生命周期回调的一部分，因此不会在其文档中描述的每种情况下都调用它。
/// 请注意，这些语义在针对 Build.VERSION_CODES.HONEYCOMB 开头的平台的应用程序与针对先前平台的应用程序之间会略有不同。从 Honeycomb 开始，应用程序在其 onStop 返回之前不处于可终止状态。这会影响何时调用 onSaveInstanceState(Bundle)（可以在 onPause() 之后安全地调用它），并允许应用程序安全地等到 onStop() 保存持久状态。对于针对 Build.VERSION_CODES.P 开头的平台的应用程序，onSaveInstanceState(Bundle) 将始终在 onStop 之后调用，因此应用程序可以在 onStop 中安全地执行片段事务，并能够在以后保存持久状态。对于那些未标记为可终止的方法，从调用该方法开始并在该方法返回后继续，系统将不会终止活动的进程。因此，活动处于可终止状态，例如，在 onStop() 之后到 onResume() 开始之间。请记住，在极端内存压力下，系统可以随时终止应用程序进程。
///
/// 配置更改
/// 如果设备的配置（由 Resources.Configuration 类定义）发生变化，则显示用户界面的任何内容都需要更新以匹配该配置。由于 Activity 是与用户交互的主要机制，因此它包含处理配置更改的特殊支持。除非您另行指定，否则配置更改（例如屏幕方向、语言、输入设备等的更改）将导致当前活动被销毁，并根据需要经历正常的活动生命周期过程 onPause、onStop 和 onDestroy。
/// 如果活动处于前台或对用户可见，则在该实例中调用 onDestroy 后，将创建该活动的新实例，并使用上一个实例从 onSaveInstanceState 生成的任何 savedInstanceState。这样做是因为任何应用程序资源（包括布局文件）都可以根据任何配置值进行更改。因此，处理配置更改的唯一安全方法是重新检索所有资源，包括布局、可绘制对象和字符串。因为活动必须已经知道如何保存其状态并从该状态重新创建自身，所以这是一种让活动使用新配置重新启动的便捷方法。在某些特殊情况下，您可能希望根据一种或多种类型的配置更改绕过活动重新启动。这可以通过其清单中的 android:configChanges 属性完成。对于您在此处声明处理的任何类型的配置更改，您将收到对当前活动的 onConfigurationChanged 方法的调用，而不是重新启动。但是，如果配置更改涉及您未处理的任何内容，则活动仍将重新启动，并且不会调用 onConfigurationChanged。
/// 启动活动并获取结果 startActivity 方法用于启动新活动，新活动将放置在活动堆栈的顶部。它需要一个参数，即 Intent，用于描述要执行的活动。有时，您希望在活动结束时从活动中获取结果。例如，您可以启动一个活动，让用户在联系人列表中选择一个人；当活动结束时，它会返回所选的人。为此，您可以调用 startActivityForResult(Intent, int) 版本，并使用第二个整数参数标识调用。结果将通过您的 onActivityResult 方法返回。当活动退出时，它可以调用 setResult(int) 将数据返回给其父级。它必须始终提供结果代码，可以是标准结果 RESULT_CANCELED、RESULT_OK 或从 RESULT_FIRST_USER 开始的任何自定义值。此外，它可以选择返回包含其想要的任何其他数据的 Intent。所有这些信息都会出现在父级的活动中。 onActivityResult()，以及它最初提供的整数标识符。如果子活动因任何原因（例如崩溃）失败，则父活动将收到带有代码 RESULT_CANCELED 的结果。
///   public class MyActivity extends Activity {
///       ...
///
///       static final int PICK_CONTACT_REQUEST = 0;
///
///       public boolean onKeyDown(int keyCode, KeyEvent event) {
///          if (keyCode == KeyEvent. KEYCODE_DPAD_CENTER) {
///               // When the user center presses, let them pick a contact.
///               startActivityForResult(
///                   new Intent(Intent. ACTION_PICK,
///                   new Uri("content:// contacts")),
///                   PICK_CONTACT_REQUEST);
///              return true;
///           }
///           return false;
///       }
///
///       protected void onActivityResult(int requestCode, int resultCode,
///               Intent data) {
///           if (requestCode == PICK_CONTACT_REQUEST) {
///               if (resultCode == RESULT_OK) {
///                   // A contact was picked.  Here we will just display it
///                   // to the user.
///                   startActivity(new Intent(Intent. ACTION_VIEW, data));
///               }
///           }
///       }
///   }
///
/// 保存持久状态
/// 活动通常要处理两种持久状态：共享文档类数据（通常使用内容提供程序存储在 SQLite 数据库中）和内部状态（例如用户偏好）。对于内容提供程序数据，我们建议活动使用“就地编辑”用户模型。也就是说，用户所做的任何编辑都会立即生效，无需额外的确认步骤。支持此模型通常只需遵循两个规则即可：创建新文档时，会立即为其创建后备数据库条目或文件。例如，如果用户选择编写新电子邮件，则在他们开始输入数据时会立即为该电子邮件创建一个新条目，这样，如果他们在此之后转到任何其他活动，此电子邮件现在将出现在草稿列表中。当调用活动的 onPause() 方法时，它应该将用户所做的任何更改提交给后备内容提供程序或文件。这可确保即将运行的任何其他活动都可以看到这些更改。
/// 您可能希望在活动生命周期的关键时刻更积极地提交数据：例如，在开始新活动之前、在完成您自己的活动之前、用户在输入字段之间切换时等。此模型旨在防止用户在活动之间导航时丢失数据，并允许系统在活动停止后（或在 Build.VERSION_CODES.HONEYCOMB 之前的平台版本上暂停）随时安全地终止活动（因为其他地方需要系统资源）。请注意，这意味着用户按下活动中的 BACK 并不意味着“取消”——它意味着离开活动并保存其当前内容。取消活动中的编辑必须通过其他机制提供，例如明确的“恢复”或“撤消”选项。有关内容提供者的更多信息，请参阅内容包。这些是不同活动如何在它们之间调用和传播数据的关键方面。
/// Activity 类还提供了用于管理与活动相关的内部持久状态的 API。例如，这可用于记住用户在日历中首选的初始显示（日视图或周视图）或用户在 Web 浏览器中的默认主页。活动持久状态通过方法 getPreferences 进行管理，允许您检索和修改与活动相关的一组名称/值对。要使用在多个应用程序组件（活动、接收器、服务、提供程序）之间共享的首选项，您可以使用底层 Context.getSharedPreferences() 方法来检索存储在特定名称下的首选项对象。
/// （请注意，无法在应用程序包之间共享设置数据 - 为此，您需要一个内容提供程序。）以下是日历活动的摘录，该活动在其持久设置中存储了用户的首选视图模式：
///   public class CalendarActivity extends Activity {
///      ...
///
///       static final int DAY_VIEW_MODE = 0;
///       static final int WEEK_VIEW_MODE = 1;
///
///       private SharedPreferences mPrefs;
///       private int mCurViewMode;
///
///       protected void onCreate(Bundle savedInstanceState) {
///           super. onCreate(savedInstanceState);
///
///           mPrefs = getSharedPreferences(getLocalClassName(), MODE_PRIVATE);
///           mCurViewMode = mPrefs. getInt("view_mode", DAY_VIEW_MODE);
///       }
///
///       protected void onPause() {
///           super. onPause();
///
///           SharedPreferences. Editor ed = mPrefs. edit();
///           ed. putInt("view_mode", mCurViewMode);
///           ed. commit();
///       }
///   }
///
/// 权限
/// 当在清单的 标记中声明某个 Activity 时，可以强制执行启动该 Activity 的能力。这样，其他应用程序将需要在自己的清单中声明相应的 元素，才能启动该 Activity。启动 Activity 时，您可以在 Intent 上设置 Intent.FLAG_GRANT_READ_URI_PERMISSION 和/或 Intent.FLAG_GRANT_WRITE_URI_PERMISSION。这将授予 Activity 访问 Intent 中特定 URI 的权限。
/// 访问权限将一直保留，直到 Activity 完成（它将在托管进程被终止和其他临时销毁期间保留）。从 Build.VERSION_CODES.GINGERBREAD 开始，如果 Activity 已创建并且正在向 onNewIntent(Intent) 传递新的 Intent，则任何新授予的 URI 权限都将添加到其拥有的现有权限中。有关权限和安全性的更多信息，请参阅“安全和权限”文档。
///
/// 进程生命周期
/// Android 系统会尝试尽可能长时间地保留应用程序进程，但最终当内存不足时，将需要删除旧进程。如活动生命周期中所述，删除哪个进程的决定与用户与其交互的状态密切相关。一般而言，根据进程中运行的活动，进程可以处于四种状态，此处按重要性顺序列出。系统将先杀死不太重要的进程（最后一个），然后再杀死更重要的进程（第一个）。前台活动（用户当前正在与之交互的屏幕顶部的活动）被认为是最重要的。只有在其使用的内存超过设备可用内存时，才会将其进程作为最后的手段被杀死。通常此时设备已达到内存分页状态，因此这是保持用户界面响应所必需的。
/// 可见活动（用户可见但不在前台的活动，例如位于前台对话框后面的活动或多窗口模式下其他活动旁边的活动）被认为极其重要，除非需要保持前台活动运行，否则不会将其终止。后台活动（用户不可见且已停止的活动）不再重要，因此系统可以安全地终止其进程以回收内存用于其他前台或可见进程。如果需要终止其进程，当用户导航回活动时（使其再次在屏幕上可见），将使用其先前在 onSaveInstanceState 中提供的 savedInstanceState 调用其 onCreate 方法，以便它可以在与用户上次离开时​​相同的状态下重新启动。空进程是指不承载任何活动或其他应用程序组件（例如 Service 或 android.content.BroadcastReceiver 类）的进程。当内存不足时，系统会很快终止这些进程。因此，您在 Activity 之外执行的任何后台操作都必须在 Activity BroadcastReceiver 或 Service 的上下文中执行，以确保系统知道它需要保留您的进程。有时，Activity 可能需要执行独立于 Activity 生命周期本身的长期运行操作。例如，相机应用程序允许您将图片上传到网站。上传可能需要很长时间，应用程序应允许用户在执行时离开应用程序。为此，您的 Activity 应启动一个用于上传的服务。这样，系统就可以在上传期间正确确定您的进程的优先级（认为它比其他不可见的应用程序更重要），而不管原始 Activity 是暂停、停止还是完成。
#[java_class(name = "android/app/Activity")]
pub struct Activity;

impl Activity {
    /**
    当您的活动完成后并应被关闭时，请调用此方法。活动结果将通过 onActivityResult() 方法传回给启动者。
    */
    #[java_method]
    pub fn finish(&self) {}

    /**
    检查此活动是否正在完成，可能是因为您调用了 finish 或其他人请求它完成。这通常在 onPause 中用于确定活动是暂停还是完全完成。
    返回：如果活动正在完成，则返回 true；否则返回 false。
    */
    #[java_method]
    pub fn is_finishing(&self) -> bool {}

    /**
    更改与此活动关联的标题。如果这是顶级活动，其窗口的标题将会更改。如果这是嵌入活动，则父级可以对其执行任何操作。
    `title` 标题。
    */
    #[java_method]
    pub fn set_title<CS: CharSequence>(&self, title: &CS) {}

    /**
    获取与此活动关联的标题。
    */
    #[java_method]
    pub fn get_title<CS: CharSequence>(&self) -> CS {}

    /**
    将活动内容设置为显式视图。此视图直接放入活动的视图层次结构中。它本身可以是一个复杂的视图层次结构。调用此方法时，将忽略指定视图的布局参数。视图的宽度和高度默认设置为 ViewGroup.LayoutParams.MATCH_PARENT。要使用您自己的布局参数，请改为调用 setContentView(View, ViewGroup.LayoutParams)。
    `view` 要显示的所需内容。
    */
    #[java_method]
    pub fn set_content_view(&self, view: &View) {}

    /**
    在 UI 线程上运行指定的操作。如果当前线程是 UI 线程，则立即执行该操作。如果当前线程不是 UI 线程，则将操作发布到 UI 线程的事件队列。
    `action` 在 UI 线程上运行的操作
    */
    #[java_method]
    pub fn run_on_ui_thread<R: Runnable>(&self, action: &R) {}

    /// 查询用于显示自定义窗口的窗口管理器。
    #[java_method]
    pub fn get_window_manager<WM: WindowManager>(&self) -> WM {}

    /**
    获取实例。
    */
    pub fn fetch() -> Self {
        let ctx = android_context();
        vm_attach!(mut env);
        if let Ok(obj) = env.new_global_ref(&ctx) {
            Self::_new(&obj, ())
        } else {
            Self::null()
        }
    }
}

/// 测试android.app
#[cfg(feature = "test_android_app")]
pub fn test() {
    use crate::{
        android::{view::WindowManagerImpl, widget::EditText},
        java::lang::{CharSequenceExt, CharSequenceImpl, RunnableImpl},
    };
    let act = std::sync::Arc::new(Activity::fetch());
    assert!(act.to_string().starts_with("android.app.NativeActivity"));
    let cs = "我的应用".to_char_sequence::<CharSequenceImpl>();
    act.set_title(&cs);
    assert_eq!(cs, act.get_title());
    assert_eq!(false, act.is_finishing());
    let edit = EditText::new(&act.as_ref().into());
    let act2 = act.clone();
    let runnable = RunnableImpl::from_fn(move || {
        act2.set_content_view(&edit);
    });
    act.run_on_ui_thread(runnable.as_ref());
    let wm: WindowManagerImpl = act.get_window_manager();
    assert!(wm.to_string().starts_with("android.view.WindowManagerImpl"));
}
