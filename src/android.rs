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
use droid_wrap_derive::java_class;

/**
包含封装整个 Android 应用程序模型的高级类。
*/
#[cfg(feature = "android_app")]
pub mod app;

/**
包含用于访问和发布设备上的数据的类。
*/
#[cfg(feature = "android_content")]
pub mod content;

/**
提供低级图形工具，如画布、颜色过滤器、点和矩形，让您直接处理屏幕绘图。
*/
#[cfg(feature = "android_graphics")]
pub mod graphics;

/**
提供对硬件功能（例如摄像头和其他传感器）的支持。
*/
#[cfg(feature = "android_hardware")]
pub mod hardware;

/**
提供管理音频和视频中各种媒体接口的类。
*/
#[cfg(feature = "android_media")]
pub mod media;

/**
提供设备上的基本操作系统服务、消息传递和进程间通信。
*/
#[cfg(feature = "android_os")]
pub mod os;

/**
提供便利类，以访问Android提供的内容提供者。
*/
#[cfg(feature = "android_provider")]
pub mod provider;

//noinspection SpellCheckingInspection
/// 自 API 级别 31 起，Renderscript 已被弃用。请参阅迁移指南以了解替代方案。
#[cfg(feature = "android_renderscript")]
pub mod renderscript;

/**
语音能力。
*/
#[cfg(feature = "android_speech")]
pub mod speech;

/**
提供用于呈现或跟踪屏幕上的文本和文本跨度的类。
*/
#[cfg(feature = "android_text")]
pub mod text;

/**
提供一些类，这些类公开处理屏幕布局和与用户交互的基本用户界面类。
*/
#[cfg(feature = "android_view")]
pub mod view;

/**
小部件包包含可在应用程序屏幕上使用的（大部分是视觉的）UI 元素。
*/
#[cfg(feature = "android_widget")]
pub mod widget;

use crate::{JObjNew, JObjRef, JType};

/// 安卓应用权限定义
#[allow(non_camel_case_types)]
#[java_class(name = "android/Manifest$permission")]
pub struct Manifest_permission;

impl Manifest_permission {
    /**
    允许调用应用程序继续在另一个应用程序中发起的通话。例如，一个视频通话应用程序希望继续在用户的移动网络上进行语音通话。
    当通话从一个应用程序转移到另一个应用程序时，涉及两个参与交接的设备：发起设备和接收设备。发起设备是请求交接通话的起始点，而接收设备是另一方确认交接请求的地方。
    此权限保护对接收到TelecomManager.acceptHandover(Uri, int, PhoneAccountHandle)方法的访问，接收方使用该方法来接受通话交接。
    保护级别：危险
    */
    pub const ACCEPT_HANDOVER: &'static str = "android.permission.ACCEPT_HANDOVER";

    /**
    允许应用程序在后台访问位置信息。如果您请求此权限，还必须请求ACCESS_COARSE_LOCATION（大致位置访问）或ACCESS_FINE_LOCATION（精确位置访问）权限之一。仅请求此权限本身不会获得位置访问权限。
    保护级别：危险
    这是一个严格受限的权限，除非记录中的安装程序将该权限列入白名单，否则应用程序无法持有此权限。更多详细信息，请参阅PackageInstaller.SessionParams.setWhitelistedRestrictedPermissions(Set)。
    */
    pub const ACCESS_BACKGROUND_LOCATION: &'static str =
        "android.permission.ACCESS_BACKGROUND_LOCATION";

    /**
    允许应用程序跨用户访问数据块。
    */
    pub const ACCESS_BLOBS_ACROSS_USERS: &'static str =
        "android.permission.ACCESS_BLOBS_ACROSS_USERS";

    /**
    允许对签入数据库中的“properties”表进行读写访问，以更改要上传的值。
    不由第三方应用程序使用。
    */
    pub const ACCESS_CHECKIN_PROPERTIES: &'static str =
        "android.permission.ACCESS_CHECKIN_PROPERTIES";

    /**
    允许应用程序访问大致位置信息。或者，您可能需要ACCESS_FINE_LOCATION（精确位置访问）权限。
    保护级别：危险
    */
    pub const ACCESS_COARSE_LOCATION: &'static str = "android.permission.ACCESS_COARSE_LOCATION";

    /**
    允许应用访问精确位置。或者，您可能需要ACCESS_COARSE_LOCATION权限。
    保护级别：危险
    */
    pub const ACCESS_FINE_LOCATION: &'static str = "android.permission.ACCESS_FINE_LOCATION";

    /**
    允许应用访问具有android.content.pm.UserProperties#PROFILE_API_VISIBILITY_HIDDEN用户属性的配置文件，例如UserManager.USER_TYPE_PROFILE_PRIVATE。
    保护级别：正常
    */
    pub const ACCESS_HIDDEN_PROFILES: &'static str = "android.permission.ACCESS_HIDDEN_PROFILES";

    /**
    允许应用访问额外的位置提供者命令。
    保护级别：正常
    */
    pub const ACCESS_LOCATION_EXTRA_COMMANDS: &'static str =
        "android.permission.ACCESS_LOCATION_EXTRA_COMMANDS";

    /**
    允许应用访问用户共享集合中保存的任何地理位置。
    保护级别：危险
    */
    pub const ACCESS_MEDIA_LOCATION: &'static str = "android.permission.ACCESS_MEDIA_LOCATION";

    /**
    允许应用访问有关网络的信息。
    保护级别：正常
    */
    pub const ACCESS_NETWORK_STATE: &'static str = "android.permission.ACCESS_NETWORK_STATE";

    /**
    希望访问通知策略的应用的标记权限。此权限在受管理的配置文件中不受支持。
    保护级别：正常
    */
    pub const ACCESS_NOTIFICATION_POLICY: &'static str =
        "android.permission.ACCESS_NOTIFICATION_POLICY";

    /**
    允许应用程序访问有关Wi-Fi网络的信息。
    保护级别：普通
    */
    pub const ACCESS_WIFI_STATE: &'static str = "android.permission.ACCESS_WIFI_STATE";

    /**
    允许应用程序调用账户认证器。
    不由第三方应用程序使用。
    */
    pub const ACCOUNT_MANAGER: &'static str = "android.permission.ACCOUNT_MANAGER";

    /**
    允许应用程序识别物理活动。
    保护级别：危险
    */
    pub const ACTIVITY_RECOGNITION: &'static str = "android.permission.ACTIVITY_RECOGNITION";

    /**
    允许应用程序将语音邮件添加到系统中。
    保护级别：危险
    */
    pub const ADD_VOICEMAIL: &'static str = "com.android.voicemail.permission.ADD_VOICEMAIL";

    /**
    允许应用接听来电。
    保护级别：危险
    */
    pub const ANSWER_PHONE_CALLS: &'static str = "android.permission.ANSWER_PHONE_CALLS";

    /**
    允许应用程序收集电池统计信息。
    保护级别：签名|特权|开发
    */
    pub const BATTERY_STATS: &'static str = "android.permission.BATTERY_STATS";

    /**
    必须由无障碍服务所需，以确保只有系统可以绑定到它。
    保护级别：签名
    */
    pub const BIND_ACCESSIBILITY_SERVICE: &'static str =
        "android.permission.BIND_ACCESSIBILITY_SERVICE";

    //noinspection SpellCheckingInspection
    /**
    允许应用程序告知AppWidget服务哪个应用程序可以访问AppWidget的数据。正常的用户流程是用户选择一个AppWidget放入特定的宿主中，从而允许该宿主应用程序访问来自AppWidget应用的私有数据。拥有此权限的应用程序应遵守该约定。
    非第三方应用程序使用。
    */
    pub const BIND_APPWIDGET: &'static str = "android.permission.BIND_APPWIDGET";

    /**
    必须由AutofillService所需，以确保只有系统可以绑定到它。
    保护级别：签名
    */
    pub const BIND_AUTOFILL_SERVICE: &'static str = "android.permission.BIND_AUTOFILL_SERVICE";

    /**
    必须由CallRedirectionService所需，以确保只有系统可以绑定到它。
    保护级别：签名|特权
    */
    pub const BIND_CALL_REDIRECTION_SERVICE: &'static str =
        "android.permission.BIND_CALL_REDIRECTION_SERVICE";

    /**
    CarrierMessagingClientService的子类必须使用此权限进行保护。
    保护级别：签名
    */
    pub const BIND_CARRIER_MESSAGING_CLIENT_SERVICE: &'static str =
        "android.permission.BIND_CARRIER_MESSAGING_CLIENT_SERVICE";

    /**
    错误
    此常量在API级别23中已被弃用。
    */
    #[deprecated(note = "请使用BIND_CARRIER_SERVICES代替")]
    pub const BIND_CARRIER_MESSAGING_SERVICE: &'static str =
        "android.permission.BIND_CARRIER_MESSAGING_SERVICE";

    /**
    允许绑定到运营商应用中的服务的系统进程将具有此权限。运营商应用应使用此权限来保护其服务，仅允许系统绑定到这些服务。
    保护级别：签名|特权
    */
    pub const BIND_CARRIER_SERVICES: &'static str = "android.permission.BIND_CARRIER_SERVICES";

    /**
    错误
    此常量在API级别30中已被弃用。
    必须由ChooserTargetService所需，以确保只有系统可以绑定到它。
    保护级别：签名
    */
    #[deprecated(
        note = "若要发布直接共享目标，请遵循https://developer.android.com/training/sharing/receive.html#providing-direct-share-targets中的说明。"
    )]
    pub const BIND_CHOOSER_TARGET_SERVICE: &'static str =
        "android.permission.BIND_CHOOSER_TARGET_SERVICE";

    /**
    任何CompanionDeviceService都必须要求此权限，以确保只有系统可以绑定到它。
    */
    pub const BIND_COMPANION_DEVICE_SERVICE: &'static str =
        "android.permission.BIND_COMPANION_DEVICE_SERVICE"; // 注意：原文中的"COMPANION"可能是"COMPANION"的拼写错误，正常应为"COMPANION_DEVICE_SERVICE"的前缀部分，但这里保留原文拼写以匹配实际代码或文档。

    /**
    必须由ConditionProviderService所需，以确保只有系统可以绑定到它。
    保护级别：签名
    */
    pub const BIND_CONDITION_PROVIDER_SERVICE: &'static str =
        "android.permission.BIND_CONDITION_PROVIDER_SERVICE";

    /**
    允许SystemUI请求第三方控件。
    仅应由系统请求，并由ControlsProviderService声明所需。
    */
    pub const BIND_CONTROLS: &'static str = "android.permission.BIND_CONTROLS";

    /**
    必须由CredentialProviderService所需，以确保只有系统可以绑定到它。
    保护级别：签名
    */
    pub const BIND_CREDENTIAL_PROVIDER_SERVICE: &'static str =
        "android.permission.BIND_CREDENTIAL_PROVIDER_SERVICE";

    /**
    必须由设备管理接收器所需，以确保只有系统可以与其交互。
    保护级别：签名
    */
    pub const BIND_DEVICE_ADMIN: &'static str = "android.permission.BIND_DEVICE_ADMIN";

    /**
    必须由DreamService要求，以确保只有系统可以绑定到它。
    保护级别：签名
    */
    pub const BIND_DREAM_SERVICE: &'static str = "android.permission.BIND_DREAM_SERVICE";

    //noinspection SpellCheckingInspection
    /**
    必须由InCallService要求，以确保只有系统可以绑定到它。
    保护级别：签名|特权
    */
    pub const BIND_INCALL_SERVICE: &'static str = "android.permission.BIND_INCALL_SERVICE";

    /**
    必须由InputMethodService要求，以确保只有系统可以绑定到它。
    保护级别：签名
    */
    pub const BIND_INPUT_METHOD: &'static str = "android.permission.BIND_INPUT_METHOD";

    /**
    必须由MidiDeviceService要求，以确保只有系统可以绑定到它。
    保护级别：签名
    */
    pub const BIND_MIDI_DEVICE_SERVICE: &'static str =
        "android.permission.BIND_MIDI_DEVICE_SERVICE";

    /**
    必须由HostApduService或OffHostApduService要求，以确保只有系统可以绑定到它。
    保护级别：签名
    */
    pub const BIND_NFC_SERVICE: &'static str = "android.permission.BIND_NFC_SERVICE";

    /**
    必须由NotificationListenerService要求，以确保只有系统可以绑定到它。
    保护级别：签名
    */
    pub const BIND_NOTIFICATION_LISTENER_SERVICE: &'static str =
        "android.permission.BIND_NOTIFICATION_LISTENER_SERVICE";

    /**
    必须由PrintService要求，以确保只有系统可以绑定到它。
    保护级别：签名
    */
    pub const BIND_PRINT_SERVICE: &'static str = "android.permission.BIND_PRINT_SERVICE";

    /**
    必须由QuickAccessWalletService要求以确保只有系统可以绑定到它。
    保护级别：签名
    */
    pub const BIND_QUICK_ACCESS_WALLET_SERVICE: &'static str =
        "android.permission.BIND_QUICK_ACCESS_WALLET_SERVICE";

    /**
    允许应用程序绑定到第三方快速设置磁贴。
    仅应由系统请求，应由TileService声明要求。
    */
    pub const BIND_QUICK_SETTINGS_TILE: &'static str =
        "android.permission.BIND_QUICK_SETTINGS_TILE";

    //noinspection SpellCheckingInspection
    /**
    必须由RemoteViewsService要求，以确保只有系统可以绑定到它。
    保护级别：签名|特权
    */
    pub const BIND_REMOTEVIEWS: &'static str = "android.permission.BIND_REMOTEVIEWS";

    /**
    必须由CallScreeningService要求，以确保只有系统可以绑定到它。
    保护级别：签名|特权
    */
    pub const BIND_SCREENING_SERVICE: &'static str = "android.permission.BIND_SCREENING_SERVICE";

    /**
    必须由ConnectionService要求，以确保只有系统可以绑定到它。
    保护级别：签名|特权
    */
    pub const BIND_TELECOM_CONNECTION_SERVICE: &'static str =
        "android.permission.BIND_TELECOM_CONNECTION_SERVICE";

    /**
    必须由文本服务（例如拼写检查服务）要求，以确保只有系统可以绑定到它。
    保护级别：签名
    */
    pub const BIND_TEXT_SERVICE: &'static str = "android.permission.BIND_TEXT_SERVICE";

    /**
    电视输入服务（TvInputService）必须要求此权限，以确保只有系统可以绑定到它。
    保护级别：签名|特权
    */
    pub const BIND_TV_INPUT: &'static str = "android.permission.BIND_TV_INPUT";

    /**
    电视交互应用服务（TvInteractiveAppService）必须要求此权限，以确保只有系统可以绑定到它。
    保护级别：签名|特权
    */
    pub const BIND_TV_INTERACTIVE_APP: &'static str = "android.permission.BIND_TV_INTERACTIVE_APP";

    /**
    视觉语音邮件服务（VisualVoicemailService）的链接必须要求此权限，以确保只有系统可以绑定到它。
    保护级别：签名|特权
    */
    pub const BIND_VISUAL_VOICEMAIL_SERVICE: &'static str =
        "android.permission.BIND_VISUAL_VOICEMAIL_SERVICE";

    /**
    语音交互服务（VoiceInteractionService）必须要求此权限，以确保只有系统可以绑定到它。
    保护级别：签名
    */
    pub const BIND_VOICE_INTERACTION: &'static str = "android.permission.BIND_VOICE_INTERACTION";

    /**
    虚拟专用网络服务（VpnService）必须要求此权限，以确保只有系统可以绑定到它。
    保护级别：签名
    */
    pub const BIND_VPN_SERVICE: &'static str = "android.permission.BIND_VPN_SERVICE";

    /**
    虚拟现实监听服务（VrListenerService）必须要求此权限，以确保只有系统可以绑定到它。
    保护级别：签名
    */
    pub const BIND_VR_LISTENER_SERVICE: &'static str =
        "android.permission.BIND_VR_LISTENER_SERVICE";

    /**
    壁纸服务（WallpaperService）必须要求此权限，以确保只有系统可以绑定到它。
    保护级别：签名|特权
    */
    pub const BIND_WALLPAPER: &'static str = "android.permission.BIND_WALLPAPER";

    /**
    允许应用程序连接到已配对的蓝牙设备。
    保护级别：普通
    */
    pub const BLUETOOTH: &'static str = "android.permission.BLUETOOTH";

    /**
    允许应用程序发现和配对蓝牙设备。
    保护级别：普通
    */
    pub const BLUETOOTH_ADMIN: &'static str = "android.permission.BLUETOOTH_ADMIN";

    /**
    需要此权限才能向附近的蓝牙设备发送广告。
    保护级别：危险
    */
    pub const BLUETOOTH_ADVERTISE: &'static str = "android.permission.BLUETOOTH_ADVERTISE";

    /**
    需要此权限才能连接到已配对的蓝牙设备。
    保护级别：危险
    */
    pub const BLUETOOTH_CONNECT: &'static str = "android.permission.BLUETOOTH_CONNECT";

    /**
    允许应用程序无需用户交互即可配对蓝牙设备，并允许或拒绝访问电话簿或消息。
    第三方应用程序不得使用。
    */
    pub const BLUETOOTH_PRIVILEGED: &'static str = "android.permission.BLUETOOTH_PRIVILEGED";

    /**
    需要此权限才能发现和配对附近的蓝牙设备。
    保护级别：危险
    */
    pub const BLUETOOTH_SCAN: &'static str = "android.permission.BLUETOOTH_SCAN";

    /**
    允许应用程序访问用户用于测量其体内情况（如心率）的传感器数据。
    保护级别：危险
    */
    pub const BODY_SENSORS: &'static str = "android.permission.BODY_SENSORS";

    /**
    允许应用程序在后台访问用户用于测量其体内情况（如心率）的传感器数据。如果您请求此权限，还必须请求BODY_SENSORS权限。仅请求此权限本身不会授予您访问身体传感器的权限。
    保护级别：危险
    这是一个严格限制的权限，除非记录中的安装程序将此权限加入白名单，否则应用程序无法获得此权限。更多详细信息，请参阅PackageInstaller.SessionParams.setWhitelistedRestrictedPermissions(Set)。
    */
    pub const BODY_SENSORS_BACKGROUND: &'static str = "android.permission.BODY_SENSORS_BACKGROUND";

    /**
    允许应用程序广播应用程序包已被移除的通知。
    第三方应用程序不得使用。
    */
    pub const BROADCAST_PACKAGE_REMOVED: &'static str =
        "android.permission.BROADCAST_PACKAGE_REMOVED";

    /**
    允许应用程序广播短信接收通知。
    第三方应用程序不得使用。
    */
    pub const BROADCAST_SMS: &'static str = "android.permission.BROADCAST_SMS";

    /**
    允许应用程序广播粘性意图（Sticky Intent）。这些广播在系统完成后，其数据会被系统保留，以便客户端能够快速检索数据，而无需等待下一次广播。
    保护级别：正常
    */
    pub const BROADCAST_STICKY: &'static str = "android.permission.BROADCAST_STICKY";

    /**
    允许应用程序广播WAP推送接收通知。
    第三方应用程序不得使用。
    */
    pub const BROADCAST_WAP_PUSH: &'static str = "android.permission.BROADCAST_WAP_PUSH";

    /**
    允许实现了InCallService API的应用有资格被设置为呼叫伴侣应用。这意味着，当有活跃通话时，电信框架将绑定到该应用的InCallService实现。应用可以使用InCallService API来查看系统上的通话信息并控制这些通话。
    保护级别：普通
    */
    pub const CALL_COMPANION_APP: &'static str = "android.permission.CALL_COMPANION_APP"; // 注意：原文中的"CALL_COMPANION_APP"可能是一个拼写错误，正确的应该是"CALL_COMPANION_APP"的某种正确形式，比如"CALL_COMPANION_APP"如果考虑英文习惯应为"CALL_COMPANION_APP"（如果指“伴侣”的话），但在此保留原文拼写。

    /**
    允许应用无需通过拨号器用户界面让用户确认即可发起电话呼叫。
    注意：持有此权限的应用还可以调用运营商的MMI代码来更改设置，如呼叫转移或呼叫等待偏好。
    保护级别：危险
    */
    pub const CALL_PHONE: &'static str = "android.permission.CALL_PHONE";

    /**
    允许应用无需通过拨号器用户界面让用户确认即可拨打任何电话号码，包括紧急号码。
    不适用于第三方应用。
    */
    pub const CALL_PRIVILEGED: &'static str = "android.permission.CALL_PRIVILEGED";

    /**
    访问相机设备所必需。
    这将自动为所有相机功能强制执行uses-feature清单元素。如果您不需要所有相机功能或在没有相机的情况下也能正常操作，那么您必须相应地修改清单，以便在不支持所有相机功能的设备上安装。
    保护级别：危险
    */
    pub const CAMERA: &'static str = "android.permission.CAMERA";

    /**
    允许应用捕获音频输出。如果仅打算捕获USAGE_UNKNOWN（未知用途）、USAGE_MEDIA（媒体用途）或USAGE_GAME（游戏用途）的音频，请使用CAPTURE_MEDIA_OUTPUT权限。
    不适用于第三方应用。
    */
    pub const CAPTURE_AUDIO_OUTPUT: &'static str = "android.permission.CAPTURE_AUDIO_OUTPUT";

    /**
    允许一个应用程序更改另一个（非自身的）应用程序组件的启用状态。
    第三方应用程序不得使用。
    */
    pub const CHANGE_COMPONENT_ENABLED_STATE: &'static str =
        "android.permission.CHANGE_COMPONENT_ENABLED_STATE";

    /**
    允许应用程序修改当前配置，如区域设置。
    保护级别：签名|特权|开发
    */
    pub const CHANGE_CONFIGURATION: &'static str = "android.permission.CHANGE_CONFIGURATION";

    /**
    允许应用程序更改网络连接状态。
    保护级别：正常
    */
    pub const CHANGE_NETWORK_STATE: &'static str = "android.permission.CHANGE_NETWORK_STATE";

    /**
    允许应用程序进入Wi-Fi多播模式。
    保护级别：正常
    */
    pub const CHANGE_WIFI_MULTICAST_STATE: &'static str =
        "android.permission.CHANGE_WIFI_MULTICAST_STATE";

    /**
    允许应用程序更改Wi-Fi连接状态。
    保护级别：正常
    */
    pub const CHANGE_WIFI_STATE: &'static str = "android.permission.CHANGE_WIFI_STATE";

    /**
    允许应用程序清除设备上所有已安装应用程序的缓存。
    保护级别：签名|特权
    */
    pub const CLEAR_APP_CACHE: &'static str = "android.permission.CLEAR_APP_CACHE";

    /**
    允许应用程序配置并连接到Wifi显示器
    */
    pub const CONFIGURE_WIFI_DISPLAY: &'static str = "android.permission.CONFIGURE_WIFI_DISPLAY";

    /**
    允许启用/禁用来自无线电的位置更新通知。
    第三方应用程序不得使用。
    */
    pub const CONTROL_LOCATION_UPDATES: &'static str =
        "android.permission.CONTROL_LOCATION_UPDATES";

    /**
    允许浏览器调用查询API集，以获取在CredentialManager.prepareGetCredential API准备期间生成的凭据候选者的元数据。
    保护级别：普通
    */
    pub const CREDENTIAL_MANAGER_QUERY_CANDIDATE_CREDENTIALS: &'static str =
        "android.permission.CREDENTIAL_MANAGER_QUERY_CANDIDATE_CREDENTIALS";

    /**
    允许在凭据管理器获取流程中指定要查询的凭据提供者候选者，或者在凭据管理器创建流程中将其设为首选默认项。
    保护级别：普通
    */
    pub const CREDENTIAL_MANAGER_SET_ALLOWED_PROVIDERS: &'static str =
        "android.permission.CREDENTIAL_MANAGER_SET_ALLOWED_PROVIDERS";

    /**
    允许浏览器代表另一个RP（依赖方）调用凭据管理器API。
    保护级别：普通
    */
    pub const CREDENTIAL_MANAGER_SET_ORIGIN: &'static str =
        "android.permission.CREDENTIAL_MANAGER_SET_ORIGIN";

    /**
    删除应用程序缓存文件的旧权限，现已不再使用，但表示我们应静默地忽略调用，而不是抛出异常。
    保护级别：签名|特权
    */
    pub const DELETE_CACHE_FILES: &'static str = "android.permission.DELETE_CACHE_FILES";

    /**
    允许应用程序删除软件包。
    第三方应用程序不得使用。
    从Build.VERSION_CODES.N版本开始，如果删除软件包的应用程序不是安装该软件包的应用程序，则会请求用户确认。
    */
    pub const DELETE_PACKAGES: &'static str = "android.permission.DELETE_PACKAGES";

    /**
    允许应用程序向系统发送配套消息。
    */
    pub const DELIVER_COMPANION_MESSAGES: &'static str =
        "android.permission.DELIVER_COMPANION_MESSAGES"; // 注意：这里可能是一个拼写错误，通常应为 "DELIVER_COMPANION_MESSAGES" 或类似的名称，但Android官方API中可能没有这个权限，或此为特定应用定义的权限。若基于Android官方文档，此权限名称可能需要验证或更正。

    /**
    允许应用程序在尝试对其窗口进行屏幕截图时收到通知。
    保护级别：正常
    */
    pub const DETECT_SCREEN_CAPTURE: &'static str = "android.permission.DETECT_SCREEN_CAPTURE";

    /**
    允许应用程序在正在被录制时收到通知。
    保护级别：正常
    */
    pub const DETECT_SCREEN_RECORDING: &'static str = "android.permission.DETECT_SCREEN_RECORDING";

    /**
    允许应用程序对诊断资源进行读写操作。
    第三方应用程序不得使用。
    */
    pub const DIAGNOSTIC: &'static str = "android.permission.DIAGNOSTIC"; // 注意：在Android官方API中，此权限可能不是标准权限，或者其名称有所不同。如果需要，请查阅最新的Android官方文档以确认正确的权限名称。

    /**
    如果键盘锁不安全，则允许应用程序禁用键盘锁。
    保护级别：正常
    */
    pub const DISABLE_KEYGUARD: &'static str = "android.permission.DISABLE_KEYGUARD";

    /**
    允许应用程序从系统服务检索状态转储信息。
    第三方应用程序不得使用。
    */
    pub const DUMP: &'static str = "android.permission.DUMP";

    /**
    允许应用程序通过PackageInstaller.SessionParams.setRequestUpdateOwnership(boolean)指示其有意成为更新所有者。
    保护级别：普通
    */
    pub const ENFORCE_UPDATE_OWNERSHIP: &'static str =
        "android.permission.ENFORCE_UPDATE_OWNERSHIP";

    /**
    允许辅助应用程序在应用程序内部代表用户执行操作。
    目前，此权限仅授予用户选择的助理应用程序。
    保护级别：内部|角色
    */
    pub const EXECUTE_APP_ACTION: &'static str = "android.permission.EXECUTE_APP_ACTION";

    /**
    允许应用程序展开或折叠状态栏。
    保护级别：普通
    */
    pub const EXPAND_STATUS_BAR: &'static str = "android.permission.EXPAND_STATUS_BAR";

    /**
    以制造商测试应用程序的身份运行，以root用户身份运行。仅在设备处于制造商测试模式时可用。
    第三方应用程序不得使用。
    */
    pub const FACTORY_TEST: &'static str = "android.permission.FACTORY_TEST";

    /**
    允许普通应用程序使用Service.startForeground。
    保护级别：普通
    */
    pub const FOREGROUND_SERVICE: &'static str = "android.permission.FOREGROUND_SERVICE";

    /**
    允许普通应用程序以“camera”（相机）类型使用Service.startForeground方法。
    保护级别：normal（普通）| instant（即时）
    */
    pub const FOREGROUND_SERVICE_CAMERA: &'static str =
        "android.permission.FOREGROUND_SERVICE_CAMERA";

    /**
    允许普通应用程序以“connectedDevice”（连接设备）类型使用Service.startForeground方法。
    保护级别：normal（普通）| instant（即时）
    */
    pub const FOREGROUND_SERVICE_CONNECTED_DEVICE: &'static str =
        "android.permission.FOREGROUND_SERVICE_CONNECTED_DEVICE";

    /**
    允许普通应用程序以“dataSync”（数据同步）类型使用Service.startForeground方法。
    保护级别：normal（普通）| instant（即时）
    */
    pub const FOREGROUND_SERVICE_DATA_SYNC: &'static str =
        "android.permission.FOREGROUND_SERVICE_DATA_SYNC";

    /**
    允许普通应用程序以“health”（健康）类型使用Service.startForeground方法。
    保护级别：normal（普通）| instant（即时）
    */
    pub const FOREGROUND_SERVICE_HEALTH: &'static str =
        "android.permission.FOREGROUND_SERVICE_HEALTH";

    /**
    允许普通应用程序以“location”（位置）类型使用Service.startForeground方法。
    保护级别：normal（普通）| instant（即时）
    */
    pub const FOREGROUND_SERVICE_LOCATION: &'static str =
        "android.permission.FOREGROUND_SERVICE_LOCATION";

    /**
    允许普通应用程序以“mediaPlayback”（媒体播放）类型使用Service.startForeground方法。
    保护级别：normal（普通）| instant（即时）
    */
    pub const FOREGROUND_SERVICE_MEDIA_PLAYBACK: &'static str =
        "android.permission.FOREGROUND_SERVICE_MEDIA_PLAYBACK";

    /**
    允许普通应用程序使用Service.startForeground方法，并指定类型为"mediaProcessing"。
    保护级别：normal|instant
    */
    pub const FOREGROUND_SERVICE_MEDIA_PROCESSING: &'static str =
        "android.permission.FOREGROUND_SERVICE_MEDIA_PROCESSING";

    /**
    允许普通应用程序使用Service.startForeground方法，并指定类型为"mediaProjection"。
    保护级别：normal|instant
    */
    pub const FOREGROUND_SERVICE_MEDIA_PROJECTION: &'static str =
        "android.permission.FOREGROUND_SERVICE_MEDIA_PROJECTION";

    /**
    允许普通应用程序使用Service.startForeground方法，并指定类型为"microphone"。
    保护级别：normal|instant
    */
    pub const FOREGROUND_SERVICE_MICROPHONE: &'static str =
        "android.permission.FOREGROUND_SERVICE_MICROPHONE";

    /**
    允许普通应用程序使用Service.startForeground方法，并指定类型为"phoneCall"。
    保护级别：normal|instant
    */
    pub const FOREGROUND_SERVICE_PHONE_CALL: &'static str =
        "android.permission.FOREGROUND_SERVICE_PHONE_CALL";

    /**
    允许普通应用程序使用Service.startForeground方法，并指定类型为"remoteMessaging"。
    保护级别：normal|instant
    */
    pub const FOREGROUND_SERVICE_REMOTE_MESSAGING: &'static str =
        "android.permission.FOREGROUND_SERVICE_REMOTE_MESSAGING";

    //noinspection SpellCheckingInspection
    /**
    允许普通应用程序使用Service.startForeground方法，并指定类型为"specialUse"。
    保护级别：normal|appop|instant
    */
    pub const FOREGROUND_SERVICE_SPECIAL_USE: &'static str =
        "android.permission.FOREGROUND_SERVICE_SPECIAL_USE";

    /**
    允许普通应用程序使用Service.startForeground方法，并指定类型为"systemExempted"。仅当应用程序符合ServiceInfo.FOREGROUND_SERVICE_TYPE_SYSTEM_EXEMPTED中列出的用例时，才允许使用此类型。
    保护级别：normal|instant
    */
    pub const FOREGROUND_SERVICE_SYSTEM_EXEMPTED: &'static str =
        "android.permission.FOREGROUND_SERVICE_SYSTEM_EXEMPTED";

    /**
    允许访问账户服务中的账户列表。
    注意：从Android 6.0（API级别23）开始，如果应用程序与管理账户的身份验证器具有相同的 签名，则无需“GET_ACCOUNTS”权限即可读取该账户的信息。在Android 5.1及以下版本中，所有应用程序都需要“GET_ACCOUNTS”权限才能读取任何账户的信息。
    保护级别：危险
    */
    pub const GET_ACCOUNTS: &'static str = "android.permission.GET_ACCOUNTS";

    /**
    允许访问账户服务中的账户列表。
    保护级别：签名|特权
    */
    pub const GET_ACCOUNTS_PRIVILEGED: &'static str = "android.permission.GET_ACCOUNTS_PRIVILEGED";

    /**
    允许应用程序查询任何软件包所使用的空间。
    保护级别：正常
    */
    pub const GET_PACKAGE_SIZE: &'static str = "android.permission.GET_PACKAGE_SIZE";

    /**
    错误
    此常量在API级别21中已被弃用。
    */
    #[deprecated(note = "不再强制执行。")]
    pub const GET_TASKS: &'static str = "android.permission.GET_TASKS";

    /**
    此权限可用于内容提供者，以允许全局搜索系统访问其数据。通常，当提供者具有某些保护其数据的权限（而全局搜索预计不会持有这些权限）时，将此权限作为只读权限添加到提供者中执行全局搜索查询的路径。普通应用程序无法持有此权限；它用于应用程序保护自己免受除全局搜索之外的其他所有人的访问。
    保护级别：签名|特权
    */
    pub const GLOBAL_SEARCH: &'static str = "android.permission.GLOBAL_SEARCH";

    /**
    允许应用程序阻止非系统覆盖窗口在其上方绘制
    */
    pub const HIDE_OVERLAY_WINDOWS: &'static str = "android.permission.HIDE_OVERLAY_WINDOWS";

    /**
    允许应用以大于200 Hz的采样率访问传感器数据。
    保护级别：普通
    */
    pub const HIGH_SAMPLING_RATE_SENSORS: &'static str =
        "android.permission.HIGH_SAMPLING_RATE_SENSORS";

    /**
    允许应用程序在位置管理器中安装位置提供程序。
    不适用于第三方应用程序。
    */
    pub const INSTALL_LOCATION_PROVIDER: &'static str =
        "android.permission.INSTALL_LOCATION_PROVIDER";

    /**
    允许应用程序安装包。
    不适用于第三方应用程序。
    */
    pub const INSTALL_PACKAGES: &'static str = "android.permission.INSTALL_PACKAGES";

    /**
    允许应用程序在启动器中安装快捷方式。
    在Android O（API级别26）及更高版本中，INSTALL_SHORTCUT广播对您的应用程序不再有任何影响，因为它是私有隐式广播。相反，您应该使用ShortcutManager类的requestPinShortcut()方法来创建应用程序快捷方式。
    保护级别：普通
    */
    pub const INSTALL_SHORTCUT: &'static str = "com.android.launcher.permission.INSTALL_SHORTCUT";

    /**
    允许即时应用创建前台服务。
    保护级别：签名|开发|即时应用|应用操作
    */
    pub const INSTANT_APP_FOREGROUND_SERVICE: &'static str =
        "android.permission.INSTANT_APP_FOREGROUND_SERVICE";

    /**
    允许在同一配置文件组中的配置文件之间交互。
    */
    pub const INTERACT_ACROSS_PROFILES: &'static str =
        "android.permission.INTERACT_ACROSS_PROFILES";

    /**
    允许应用程序打开网络套接字。
    保护级别：普通
    */
    pub const INTERNET: &'static str = "android.permission.INTERNET";

    /**
    允许应用程序调用ActivityManager.killBackgroundProcesses(String)方法。
    从Android版本Build.VERSION_CODES.UPSIDE_DOWN_CAKE（即Android 10，代号“Q”的某个甜点名称前的占位符，实际未使用）开始，ActivityManager.killBackgroundProcesses(String)方法不再对第三方应用程序开放。为了向后兼容，当调用此API时，调用者自己包的后台进程仍然会被终止。如果调用者拥有系统权限KILL_ALL_BACKGROUND_PROCESSES，其他进程也会被终止。
    保护级别：普通
    */
    pub const KILL_BACKGROUND_PROCESSES: &'static str =
        "android.permission.KILL_BACKGROUND_PROCESSES";

    /**
    允许应用程序使用Intent.ACTION_LAUNCH_CAPTURE_CONTENT_ACTIVITY_FOR_NOTE意图动作捕获屏幕内容以执行截图。
    保护级别：内部|角色
    仅供ROLE_NOTES角色使用。
    */
    pub const LAUNCH_CAPTURE_CONTENT_ACTIVITY_FOR_NOTE: &'static str =
        "android.permission.LAUNCH_CAPTURE_CONTENT_ACTIVITY_FOR_NOTE";

    /**
    应用程序需要此权限，以便Settings.ACTION_SETTINGS_EMBED_DEEP_LINK_ACTIVITY能够在设置应用程序中嵌入其Activity。
    */
    pub const LAUNCH_MULTI_PANE_SETTINGS_DEEP_LINK: &'static str =
        "android.permission.LAUNCH_MULTI_PANE_SETTINGS_DEEP_LINK";

    //noinspection SpellCheckingInspection
    /**
    允许数据加载器读取一个包的访问日志。访问日志包含随时间变化的页面引用集合。
    声明此权限意味着打算使用该API，并且设备用户可以通过设置应用程序授予权限。
    保护级别：签名|特权|appop
    数据加载器必须是为安装应用程序提供数据的程序。
    数据加载器必须同时拥有LOADER_USAGE_STATS权限和appop:LOADER_USAGE_STATS授权，才能访问读取日志。
    */
    pub const LOADER_USAGE_STATS: &'static str = "android.permission.LOADER_USAGE_STATS";

    /**
    允许应用程序使用硬件中的位置功能，例如地理围栏API。
    非第三方应用程序使用。
    */
    pub const LOCATION_HARDWARE: &'static str = "android.permission.LOCATION_HARDWARE";

    /**
    允许融资设备自助服务终端应用程序对设备锁定服务执行操作
    保护级别：内部|角色
    仅供FINANCED_DEVICE_KIOSK角色使用。
    */
    pub const MANAGE_DEVICE_LOCK_STATE: &'static str =
        "android.permission.MANAGE_DEVICE_LOCK_STATE";

    /**
    允许应用程序管理与无障碍功能相关的策略。
    若要对不同于调用用户的用户调用受此权限保护的API，需要Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL权限。
    保护级别：内部|角色
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_ACCESSIBILITY: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_ACCESSIBILITY";

    /**
    允许应用程序设置与帐户管理相关的策略。
    若要对不同于调用用户的用户调用受此权限保护的API，需要Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS权限。
    保护级别：内部|角色
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_ACCOUNT_MANAGEMENT: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_ACCOUNT_MANAGEMENT";

    /**
    允许应用程序为当前用户之外的设备设置策略，这些策略对于确保设备所有权安全而不访问用户数据是必需的。
    持有此权限允许在所有用户设备上使用其他持有的MANAGE_DEVICE_POLICY_*权限（前提是它们不授予访问用户数据的权限）。
    保护级别：内部|角色
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_ACROSS_USERS: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_ACROSS_USERS";

    /**
    允许应用程序为当前用户之外的设备设置策略。
    MANAGE_DEVICE_POLICY_ACROSS_USERS的完整形式，移除了访问用户数据的限制。
    持有此权限允许在所有用户的设备上使用任何其他已持有的MANAGE_DEVICE_POLICY_*权限。
    保护级别：内部|角色
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL";

    /**
    允许应用程序为当前用户之外的设备设置策略，这些策略对于保护当前用户内的数据至关重要。
    持有此权限允许在所有用户的设备上使用其他已持有的MANAGE_DEVICE_POLICY_*权限，前提是这些权限对于保护当前用户内的数据是必需的。
    保护级别：内部|角色
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_ACROSS_USERS_SECURITY_CRITICAL: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_ACROSS_USERS_SECURITY_CRITICAL";

    /**
    允许应用程序设置与飞行模式相关的策略。
    对于不同于调用用户的用户，调用受此权限保护的API需要Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS权限。
    保护级别：内部|角色
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_AIRPLANE_MODE: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_AIRPLANE_MODE";

    /**
    允许应用程序管理有关修改应用程序的策略。
    对于不同于调用用户的用户，调用受此权限保护的API需要Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL权限。
    保护级别：内部|角色
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_APPS_CONTROL: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_APPS_CONTROL";

    /**
    允许应用程序管理应用程序限制。
    若要对不同于调用用户的用户调用受此权限保护的API，需要Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL权限。
    保护级别：internal|role
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_APP_RESTRICTIONS: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_APP_RESTRICTIONS";

    /**
    允许应用程序管理与应用程序用户数据相关的策略。
    若要对不同于调用用户的用户调用受此权限保护的API，需要`Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL`权限。
    保护级别：internal|role
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_APP_USER_DATA: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_APP_USER_DATA";

    /**
    允许应用程序设置与向特权应用程序（如助手应用程序）发送辅助内容相关的策略。
    保护级别：internal|role
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_ASSIST_CONTENT: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_ASSIST_CONTENT";

    /**
    允许应用程序设置与音频输出相关的策略。
    若要对不同于调用用户的用户调用受此权限保护的API，需要`Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL`权限。
    保护级别：internal|role
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_AUDIO_OUTPUT: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_AUDIO_OUTPUT";

    /**
    允许应用程序设置与自动填充相关的策略。
    若要对不同于调用用户的用户调用受此权限保护的API，需要Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL权限。
    保护级别：内部|角色
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_AUTOFILL: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_AUTOFILL";

    /**
    允许应用程序管理备份服务策略。
    若要对不同于调用用户的用户调用受此权限保护的API，需要Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL权限。
    保护级别：内部|角色
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_BACKUP_SERVICE: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_BACKUP_SERVICE";

    /**
    允许应用程序管理阻止应用程序卸载的策略。
    保护级别：内部|角色
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_BLOCK_UNINSTALL: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_BLOCK_UNINSTALL";

    /**
    允许应用程序设置与蓝牙相关的策略。
    若要对不同于调用用户的用户调用受此权限保护的API，需要Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL权限。
    保护级别：内部|角色
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_BLUETOOTH: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_BLUETOOTH";

    /**
    允许应用程序在用户同意的情况下请求错误报告。
    若要对不同于调用用户的用户调用受此权限保护的API，需要Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL权限。
    保护级别：内部|角色
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_BUGREPORT: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_BUGREPORT";

    /**
    允许应用程序管理呼叫策略。
    若要对不同于调用用户的用户调用受此权限保护的API，需要Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL权限。
    保护级别：internal|role
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_CALLS: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_CALLS";

    /**
    允许应用程序设置与限制用户使用或启用/禁用相机能力相关的策略。
    若要对不同于调用用户的用户调用受此权限保护的API，需要`Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS`权限。
    保护级别：internal|role
    仅供`DEVICE_POLICY_MANAGEMENT`角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_CAMERA: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_CAMERA";

    /**
    允许应用程序管理与相机切换相关的策略。
    保护级别：internal|role
    仅供`DEVICE_POLICY_MANAGEMENT`角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_CAMERA_TOGGLE: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_CAMERA_TOGGLE";

    /**
    允许应用程序设置与证书相关的策略。
    若要对不同于调用用户的用户调用受此权限保护的API，需要`Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL`权限。
    保护级别：internal|role
    仅供`DEVICE_POLICY_MANAGEMENT`角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_CERTIFICATES: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_CERTIFICATES";

    /**
    允许应用程序管理与通用准则模式相关的策略。
    若要对不同于调用用户的用户调用受此权限保护的API，需要Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL权限。
    保护级别：内部|角色
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_COMMON_CRITERIA_MODE: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_COMMON_CRITERIA_MODE";

    /**
    允许应用程序管理与内容保护相关的策略。
    保护级别：内部|角色
    */
    pub const MANAGE_DEVICE_POLICY_CONTENT_PROTECTION: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_CONTENT_PROTECTION";

    /**
    允许应用程序管理调试功能策略。
    若要对不同于调用用户的用户调用受此权限保护的API，需要`Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL`权限。
    保护级别：内部|角色
    仅供`DEVICE_POLICY_MANAGEMENT`角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_DEBUGGING_FEATURES: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_DEBUGGING_FEATURES";

    /**
    允许应用程序设置与默认短信应用程序相关的策略。
    若要对不同于调用用户的用户调用受此权限保护的API，需要`Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS`权限。
    保护级别：内部|角色
    仅供`DEVICE_POLICY_MANAGEMENT`角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_DEFAULT_SMS: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_DEFAULT_SMS";

    /**
    允许应用程序管理与设备标识符相关的策略。
    保护级别：内部|角色
    仅供`DEVICE_POLICY_MANAGEMENT`角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_DEVICE_IDENTIFIERS: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_DEVICE_IDENTIFIERS";

    /**
    允许应用程序设置与显示相关的策略。
    若要对不同于调用用户的用户调用受此权限保护的API，需要Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL权限。
    保护级别：internal|role
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_DISPLAY: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_DISPLAY";

    /**
    允许应用程序设置与恢复出厂设置相关的策略。
    若要对不同于调用用户的用户调用受此权限保护的API，需要Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL权限。
    保护级别：internal|role
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_FACTORY_RESET: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_FACTORY_RESET";

    /**
    允许应用程序设置与“fun”相关的策略。
    若要对不同于调用用户的用户调用受此权限保护的API，需要Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL权限。
    保护级别：internal|role
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_FUN: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_FUN";

    /**
    允许应用程序设置与输入方法相关的策略。
    若要对不同于调用用户的用户调用受此权限保护的API，需要Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS权限。
    保护级别：internal|role
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_INPUT_METHODS: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_INPUT_METHODS";

    /**
    允许应用程序管理从未知来源安装的策略。
    若要对不同于调用用户的用户调用受此权限保护的API，需要拥有MANAGE_SECURITY_CRITICAL_DEVICE_POLICY_ACROSS_USERS权限。
    保护级别：内部|角色
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_INSTALL_UNKNOWN_SOURCES: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_INSTALL_UNKNOWN_SOURCES";

    /**
    允许应用程序设置与保留已卸载软件包相关的策略。
    若要对不同于调用用户的用户调用受此权限保护的API，需要拥有Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL权限。
    保护级别：内部|角色
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_KEEP_UNINSTALLED_PACKAGES: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_KEEP_UNINSTALLED_PACKAGES";

    /**
    允许应用程序管理与键盘锁（Keyguard）相关的策略。
    若要对不同于调用用户的用户调用受此权限保护的API，需要拥有Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_SECURITY_CRITICAL权限。
    保护级别：内部|角色
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_KEYGUARD: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_KEYGUARD";

    /**
    允许应用程序设置与区域设置（Locale）相关的策略。
    若要对不同于调用用户的用户调用受此权限保护的API，需要拥有Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL权限。
    保护级别：内部|角色
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_LOCALE: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_LOCALE";

    /**
    允许应用程序设置与位置相关的策略。
    若要对不同于调用用户的用户调用受此权限保护的API，需要Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL权限。
    保护级别：internal|role
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_LOCATION: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_LOCATION";

    /**
    允许应用程序使用适当的跨用户权限锁定某个用户配置文件或设备。
    若要对不同于调用用户的用户调用受此权限保护的API，需要`Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL`权限。
    保护级别：internal|role
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_LOCK: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_LOCK";

    /**
    允许应用程序设置与锁定凭据相关的策略。
    若要对不同于调用用户的用户调用受此权限保护的API，需要`Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_SECURITY_CRITICAL`权限。
    保护级别：internal|role
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_LOCK_CREDENTIALS: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_LOCK_CREDENTIALS";

    /**
    允许应用程序管理锁定任务策略。
    若要对不同于调用用户的用户调用受此权限保护的API，需要`Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL`权限。
    保护级别：internal|role
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_LOCK_TASK: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_LOCK_TASK";

    /**
    允许应用程序设置与管理员下载的订阅相关的策略。
    若要对不同于调用用户的用户调用受此权限保护的API，需要Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL权限。
    保护级别：内部|角色
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_MANAGED_SUBSCRIPTIONS: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_MANAGED_SUBSCRIPTIONS";

    /**
    允许应用程序管理与计量数据相关的策略。
    若要对不同于调用用户的用户调用受此权限保护的API，需要`Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL`权限。
    保护级别：内部|角色
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_METERED_DATA: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_METERED_DATA";

    /**
    允许应用程序设置与限制用户使用或启用/禁用麦克风能力相关的策略。
    若要对不同于调用用户的用户调用受此权限保护的API，需要`Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS`权限。
    保护级别：内部|角色
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_MICROPHONE: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_MICROPHONE";

    /**
    允许应用程序管理与麦克风切换相关的策略。
    保护级别：内部|角色
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_MICROPHONE_TOGGLE: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_MICROPHONE_TOGGLE";

    /**
    允许应用程序设置与移动网络相关的策略。
    若要对不同于调用用户的用户调用受此权限保护的API，需要Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL权限。
    保护级别：内部|角色
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_MOBILE_NETWORK: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_MOBILE_NETWORK";

    /**
    允许应用程序管理防止用户修改用户的策略。
    若要对不同于调用用户的用户调用受此权限保护的API，需要`Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL`权限。
    保护级别：内部|角色
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_MODIFY_USERS: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_MODIFY_USERS";

    /**
    允许应用程序管理与内存标记扩展（MTE）相关的策略。
    保护级别：内部|角色
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_MTE: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_MTE";

    /**
    允许应用程序设置与附近通信（例如Beam和附近流）相关的策略。
    若要对不同于调用用户的用户调用受此权限保护的API，需要`Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL`权限。
    保护级别：内部|角色
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_NEARBY_COMMUNICATION: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_NEARBY_COMMUNICATION";

    /**
    允许应用程序设置与网络日志记录相关的策略。
    若要对不同于调用用户的用户调用受此权限保护的API，需要Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL权限。
    保护级别：内部|角色
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_NETWORK_LOGGING: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_NETWORK_LOGGING";

    /**
    允许应用程序管理管理组织的身份。
    若要对不同于调用用户的用户调用受此权限保护的API，需要Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL权限。
    保护级别：内部|角色
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_ORGANIZATION_IDENTITY: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_ORGANIZATION_IDENTITY";

    /**
    允许应用程序设置与覆盖APN（接入点名称）相关的策略。
    若要对不同于调用用户的用户调用受此权限保护的API，需要Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL权限。
    保护级别：内部|角色
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_OVERRIDE_APN: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_OVERRIDE_APN";

    /**
    允许应用程序设置与隐藏和挂起应用程序包相关的策略。
    若要对不同于调用用户的用户调用受此权限保护的API，需要Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS权限。
    保护级别：内部|角色
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_PACKAGE_STATE: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_PACKAGE_STATE";

    /**
    允许应用程序设置与物理媒体相关的策略。
    若要对不同于调用用户的其他用户调用受此权限保护的API，需要Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL权限。
    保护级别：internal|role
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_PHYSICAL_MEDIA: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_PHYSICAL_MEDIA";

    /**
    允许应用程序设置与打印相关的策略。
    若要对不同于调用用户的其他用户调用受此权限保护的API，需要Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL权限。
    保护级别：internal|role
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_PRINTING: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_PRINTING";

    /**
    允许应用程序设置与私有DNS相关的策略。
    若要对不同于调用用户的其他用户调用受此权限保护的API，需要Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL权限。
    保护级别：internal|role
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_PRIVATE_DNS: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_PRIVATE_DNS";

    /**
    允许应用程序设置与配置文件相关的策略。
    若要对不同于调用用户的其他用户调用受此权限保护的API，需要Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL权限。
    保护级别：internal|role
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_PROFILES: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_PROFILES";

    /**
    允许应用程序设置与配置文件交互相关的策略（例如，禁止跨配置文件的复制和粘贴）。
    若要对不同于调用用户的用户调用受此权限保护的API，需要Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL权限。
    保护级别：内部|角色
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_PROFILE_INTERACTION: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_PROFILE_INTERACTION";

    /**
    允许应用程序设置与网络无关的全局HTTP代理。
    若要对不同于调用用户的用户调用受此权限保护的API，需要`Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL`权限。
    保护级别：内部|角色
    仅供`DEVICE_POLICY_MANAGEMENT`角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_PROXY: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_PROXY";

    /**
    允许应用程序查询系统更新。
    若要对不同于调用用户的用户调用受此权限保护的API，需要`Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL`权限。
    保护级别：内部|角色
    仅供`DEVICE_POLICY_MANAGEMENT`角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_QUERY_SYSTEM_UPDATES: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_QUERY_SYSTEM_UPDATES";

    /**
    允许应用程序强制为当前用户设置新的设备解锁密码或管理配置文件挑战。
    若要对不同于调用用户的用户调用受此权限保护的API，需要`Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL`权限。
    保护级别：内部|角色
    仅供`DEVICE_POLICY_MANAGEMENT`角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_RESET_PASSWORD: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_RESET_PASSWORD";

    /**
    允许应用程序设置与用户配置私有DNS相关的策略限制。
    若要对不同于调用用户的用户调用受此权限保护的API，需要Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS权限。
    保护级别：internal|role
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_RESTRICT_PRIVATE_DNS: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_RESTRICT_PRIVATE_DNS";

    /**
    允许应用程序设置软件包上运行时权限的授予状态。
    若要对不同于调用用户的用户调用受此权限保护的API，需要Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL权限。
    保护级别：internal|role
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_RUNTIME_PERMISSIONS: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_RUNTIME_PERMISSIONS";

    /**
    允许应用程序设置与后台运行用户相关的策略。
    若要对不同于调用用户的用户调用受此权限保护的API，需要Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL权限。
    保护级别：internal|role
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_RUN_IN_BACKGROUND: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_RUN_IN_BACKGROUND";

    /**
    允许应用程序管理安全启动策略。
    若要对不同于调用用户的用户调用受此权限保护的API，需要Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL权限。
    保护级别：internal|role
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_SAFE_BOOT: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_SAFE_BOOT";

    /**
    允许应用程序设置与屏幕截图相关的策略。
    若要对不同于调用用户的用户调用受此权限保护的API，需要Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS权限。
    保护级别：internal|role
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_SCREEN_CAPTURE: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_SCREEN_CAPTURE";

    /**
    允许应用程序设置与屏幕内容使用相关的策略。
    若要对不同于调用用户的用户调用受此权限保护的API，需要Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL权限。
    保护级别：internal|role
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_SCREEN_CONTENT: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_SCREEN_CONTENT";

    /**
    允许应用程序设置与安全日志记录相关的策略。
    若要对不同于调用用户的用户调用受此权限保护的API，需要Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL权限。
    保护级别：internal|role
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_SECURITY_LOGGING: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_SECURITY_LOGGING";

    /**
    允许应用程序设置与设置相关的策略。
    若要对不同于调用用户的用户调用受此权限保护的API，需要Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL权限。
    保护级别：internal|role
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_SETTINGS: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_SETTINGS";

    /**
    允许应用程序设置与短信相关的策略。
    若要对不同于调用用户的用户调用受此权限保护的API，需要Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL权限。
    保护级别：internal|role
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_SMS: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_SMS";

    /**
    允许应用程序设置与状态栏相关的策略。
    保护级别：internal|role
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_STATUS_BAR: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_STATUS_BAR";

    /**
    允许应用程序为受活动策略影响的用户操作设置支持消息。
    若要对不同于调用用户的用户调用受此权限保护的API，需要`Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL`权限。
    保护级别：internal|role
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_SUPPORT_MESSAGE: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_SUPPORT_MESSAGE";

    /**
    允许应用程序设置与暂停个人应用程序相关的策略。
    若要对不同于调用用户的用户调用受此权限保护的API，需要`Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL`权限。
    保护级别：internal|role
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_SUSPEND_PERSONAL_APPS: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_SUSPEND_PERSONAL_APPS";

    /**
    允许应用程序管理系统应用相关的策略。
    若要在不同于调用用户的用户上调用受此权限保护的API，需要Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL权限。
    保护级别：internal|role
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_SYSTEM_APPS: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_SYSTEM_APPS";

    /**
    允许应用程序设置与系统对话框相关的策略。
    若要在不同于调用用户的用户上调用受此权限保护的API，需要Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL权限。
    保护级别：internal|role
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_SYSTEM_DIALOGS: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_SYSTEM_DIALOGS";

    /**
    允许应用程序设置与系统更新相关的策略。
    若要在不同于调用用户的用户上调用受此权限保护的API，需要Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL权限。
    保护级别：internal|role
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_SYSTEM_UPDATES: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_SYSTEM_UPDATES";

    /**
    允许应用程序管理与时间相关的设备策略。
    若要在不同于调用用户的用户上调用受此权限保护的API，需要Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL权限。
    保护级别：internal|role
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_TIME: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_TIME";

    /**
    允许应用程序设置与USB数据信号相关的策略。
    保护级别：内部|角色
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_USB_DATA_SIGNALLING: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_USB_DATA_SIGNALLING";

    /**
    允许应用程序设置与USB文件传输相关的策略。
    若要对不同于调用用户的用户调用受此权限保护的API，需要Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL权限。
    保护级别：内部|角色
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_USB_FILE_TRANSFER: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_USB_FILE_TRANSFER";

    /**
    允许应用程序设置与用户相关的策略。
    若要对不同于调用用户的用户调用受此权限保护的API，需要Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL权限。
    保护级别：内部|角色
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_USERS: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_USERS";

    /**
    允许应用程序设置与VPN相关的策略。
    若要对不同于调用用户的用户调用受此权限保护的API，需要Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL权限。
    保护级别：内部|角色
    仅供DEVICE_POLICY_MANAGEMENT角色使用。
    */
    pub const MANAGE_DEVICE_POLICY_VPN: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_VPN";

    /**
    允许应用程序设置与壁纸相关的策略。
    若要对不同于调用用户的用户调用受此权限保护的API，需要Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL权限。
    保护级别：内部|角色
    仅供具有DEVICE_POLICY_MANAGEMENT角色的应用使用。
    */
    pub const MANAGE_DEVICE_POLICY_WALLPAPER: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_WALLPAPER";

    /**
    允许应用程序设置与Wifi相关的策略。
    若要对不同于调用用户的用户调用受此权限保护的API，需要`Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS`权限。
    保护级别：内部|角色
    仅供具有DEVICE_POLICY_MANAGEMENT角色的应用使用。
    */
    pub const MANAGE_DEVICE_POLICY_WIFI: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_WIFI";

    /**
    允许应用程序设置与窗口相关的策略。
    若要对不同于调用用户的用户调用受此权限保护的API，需要`Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS_FULL`权限。
    保护级别：内部|角色
    仅供具有DEVICE_POLICY_MANAGEMENT角色的应用使用。
    */
    pub const MANAGE_DEVICE_POLICY_WINDOWS: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_WINDOWS";

    /**
    允许应用程序管理与数据擦除相关的策略。
    若要对不同于调用用户的用户调用受此权限保护的API，需要`Manifest.permission#MANAGE_DEVICE_POLICY_ACROSS_USERS`权限。
    保护级别：内部|角色
    仅供具有DEVICE_POLICY_MANAGEMENT角色的应用使用。
    */
    pub const MANAGE_DEVICE_POLICY_WIPE_DATA: &'static str =
        "android.permission.MANAGE_DEVICE_POLICY_WIPE_DATA";

    /**
    允许应用程序管理对文档的访问，通常作为文档选择器的一部分。
    此权限仅应由平台文档管理应用程序请求。第三方应用程序无法被授予此权限。
    */
    pub const MANAGE_DOCUMENTS: &'static str = "android.permission.MANAGE_DOCUMENTS";

    /**
    在作用域存储中允许应用程序广泛访问外部存储。旨在供少数需要代表用户管理文件的应用程序使用。
    保护级别：签名|应用操作|预安装
    */
    pub const MANAGE_EXTERNAL_STORAGE: &'static str = "android.permission.MANAGE_EXTERNAL_STORAGE";

    /**
    允许应用程序修改和删除此设备或任何连接存储设备上的媒体文件，而无需用户确认。要使此权限生效，应用程序必须已被授予READ_EXTERNAL_STORAGE或MANAGE_EXTERNAL_STORAGE权限。
    即使应用程序被授予此权限，如果它们想要修改或删除媒体文件，也必须通过调用MediaStore.createWriteRequest(ContentResolver, Collection)、MediaStore.createDeleteRequest(ContentResolver, Collection)或MediaStore.createTrashRequest(ContentResolver, Collection, boolean)来获得访问权限。
    此权限不直接提供读取或写入访问权限。它只是阻止这些请求的用户确认对话框。
    如果应用程序未被授予ACCESS_MEDIA_LOCATION权限，系统在写入请求时也会弹出用户确认对话框。
    保护级别：签名|应用操作|预安装
    */
    pub const MANAGE_MEDIA: &'static str = "android.permission.MANAGE_MEDIA";

    /**
    允许查询正在进行的呼叫的详细信息并管理这些呼叫。
    保护级别：签名|应用操作
    */
    pub const MANAGE_ONGOING_CALLS: &'static str = "android.permission.MANAGE_ONGOING_CALLS";

    /**
    允许调用应用程序通过自管理的ConnectionService API管理其自身的通话。有关自管理的ConnectionService API的更多信息，请参阅PhoneAccount.CAPABILITY_SELF_MANAGED。
    保护级别：普通
    */
    pub const MANAGE_OWN_CALLS: &'static str = "android.permission.MANAGE_OWN_CALLS";

    /**
    允许应用程序在无法满足Wi-Fi接口请求而不拆除一个或多个其他接口时收到通知，并提供是否批准该请求或拒绝它的决定。
    第三方应用程序不得使用。
    */
    pub const MANAGE_WIFI_INTERFACES: &'static str = "android.permission.MANAGE_WIFI_INTERFACES";

    /**
    此权限用于让原始设备制造商（OEM）为其受信任的应用程序授予访问特权Wi-Fi API子集的权限，以提高Wi-Fi性能。允许应用程序管理Wi-Fi网络选择相关功能，如启用或禁用全局自动连接、修改连接扫描间隔以及批准Wi-Fi Direct连接。
    第三方应用程序不得使用。
    */
    pub const MANAGE_WIFI_NETWORK_SELECTION: &'static str =
        "android.permission.MANAGE_WIFI_NETWORK_SELECTION";

    /**
    第三方应用程序不得使用。
    */
    pub const MASTER_CLEAR: &'static str = "android.permission.MASTER_CLEAR";

    /**
    允许应用程序了解正在播放的内容并控制其播放。
    由于媒体消费的隐私性，第三方应用程序不得使用。
    */
    pub const MEDIA_CONTENT_CONTROL: &'static str = "android.permission.MEDIA_CONTENT_CONTROL";

    /**
    允许应用程序控制媒体应用的路由。
    仅供角色为COMPANION_DEVICE_WATCH的设备使用。
    */
    pub const MEDIA_ROUTING_CONTROL: &'static str = "android.permission.MEDIA_ROUTING_CONTROL";

    /**
    允许应用程序修改全局音频设置。
    保护级别：正常
    */
    pub const MODIFY_AUDIO_SETTINGS: &'static str = "android.permission.MODIFY_AUDIO_SETTINGS";

    /**
    允许修改电话状态，如开机、MMI等，但不包括拨打电话。
    不允许第三方应用程序使用。
    */
    pub const MODIFY_PHONE_STATE: &'static str = "android.permission.MODIFY_PHONE_STATE";

    /**
    允许格式化可移动存储的文件系统。
    不允许第三方应用程序使用。
    */
    pub const MOUNT_FORMAT_FILESYSTEMS: &'static str =
        "android.permission.MOUNT_FORMAT_FILESYSTEMS";

    /**
    允许挂载和卸载可移动存储的文件系统。
    不允许第三方应用程序使用。
    */
    pub const MOUNT_UNMOUNT_FILESYSTEMS: &'static str =
        "android.permission.MOUNT_UNMOUNT_FILESYSTEMS";

    /**
    允许通过Wi-Fi广播和连接到附近的设备。
    保护级别：危险
    */
    pub const NEARBY_WIFI_DEVICES: &'static str = "android.permission.NEARBY_WIFI_DEVICES";

    /**
    允许应用程序通过NFC执行输入/输出操作。
    保护级别：正常
    */
    pub const NFC: &'static str = "android.permission.NFC";

    /**
    允许应用程序接收NFC首选支付服务信息。
    保护级别：正常
    */
    pub const NFC_PREFERRED_PAYMENT_INFO: &'static str =
        "android.permission.NFC_PREFERRED_PAYMENT_INFO";

    /**
    允许应用接收NFC交易事件。
    保护级别：普通
    */
    pub const NFC_TRANSACTION_EVENT: &'static str = "android.permission.NFC_TRANSACTION_EVENT";

    /**
    允许应用修改任何WiFi配置，即使这些配置是由其他应用创建的。一旦重新配置，原始创建者将无法再进行任何修改。
    第三方应用不得使用。
    */
    pub const OVERRIDE_WIFI_CONFIG: &'static str = "android.permission.OVERRIDE_WIFI_CONFIG";

    /**
    允许应用收集组件使用统计信息
    声明此权限意味着打算使用相关API，并且设备用户可以通过“设置”应用授予权限。
    保护级别：签名|特权|开发|应用操作|零售演示
    */
    pub const PACKAGE_USAGE_STATS: &'static str = "android.permission.PACKAGE_USAGE_STATS";

    /**
    错误
    此常量在API级别15中已被弃用。
    允许应用使其活动持久化。
    */
    #[deprecated(note = "此功能将在未来版本中移除；请勿使用。")]
    pub const PERSISTENT_ACTIVITY: &'static str = "android.permission.PERSISTENT_ACTIVITY";

    /**
    允许应用发布通知
    保护级别：危险
    */
    pub const POST_NOTIFICATIONS: &'static str = "android.permission.POST_NOTIFICATIONS";

    /**
    错误
    此常量在API级别29中已被弃用。
    允许应用在拨出电话期间查看正在拨打的号码，并可以选择将呼叫重定向到另一个号码或完全取消呼叫。
    保护级别：危险
    这是一个严格受限的权限，在记录的安装程序将权限加入白名单之前，应用无法持有此权限。更多详细信息，请参阅PackageInstaller.SessionParams.setWhitelistedRestrictedPermissions(Set)。
    */
    #[deprecated(
        note = "应用应使用CallRedirectionService代替Intent.ACTION_NEW_OUTGOING_CALL广播。"
    )]
    pub const PROCESS_OUTGOING_CALLS: &'static str = "android.permission.PROCESS_OUTGOING_CALLS";

    /**
    允许应用使用自动填充框架显示其建议。
    目前，此权限仅授予浏览器应用。
    保护级别：internal|role
    */
    pub const PROVIDE_OWN_AUTOFILL_SUGGESTIONS: &'static str =
        "android.permission.PROVIDE_OWN_AUTOFILL_SUGGESTIONS";

    /**
    允许应用能够从远程设备存储和检索凭据。
    保护级别：signature|privileged|role
    */
    pub const PROVIDE_REMOTE_CREDENTIALS: &'static str =
        "android.permission.PROVIDE_REMOTE_CREDENTIALS";

    /**
    允许查询设备上的任何普通应用，无论其清单声明如何。
    保护级别：normal
    */
    pub const QUERY_ALL_PACKAGES: &'static str = "android.permission.QUERY_ALL_PACKAGES";

    /**
    允许应用查询AppSearch中可见于ASSISTANT角色的全局数据。
    */
    pub const READ_ASSISTANT_APP_SEARCH_DATA: &'static str =
        "android.permission.READ_ASSISTANT_APP_SEARCH_DATA";

    /**
    允许以非危险权限只读访问电话状态，包括蜂窝网络类型、软件版本等信息。
    */
    pub const READ_BASIC_PHONE_STATE: &'static str = "android.permission.READ_BASIC_PHONE_STATE";

    /**
    允许应用读取用户的日历数据。
    保护级别：dangerous（危险）
    */
    pub const READ_CALENDAR: &'static str = "android.permission.READ_CALENDAR";

    /**
    允许应用读取用户的通话记录。
    注意：如果您的应用使用READ_CONTACTS权限，并且您的minSdkVersion和targetSdkVersion值都设置为15或更低，系统将隐式授予您的应用此权限。如果您不需要此权限，请确保您的targetSdkVersion为16或更高。
    保护级别：dangerous（危险）
    这是一个严格限制的权限，除非记录中的安装程序将此权限加入白名单，否则应用无法持有此权限。更多详细信息，请参阅PackageInstaller.SessionParams.setWhitelistedRestrictedPermissions(Set)。
    */
    pub const READ_CALL_LOG: &'static str = "android.permission.READ_CALL_LOG";

    /**
    允许应用程序读取用户的联系人数据。
    保护级别：危险
    */
    pub const READ_CONTACTS: &'static str = "android.permission.READ_CONTACTS";

    /**
    允许应用程序访问Dropbox中的数据。
    第三方应用程序不得使用。
    */
    pub const READ_DROPBOX_DATA: &'static str = "android.permission.READ_DROPBOX_DATA";

    /**
    允许应用程序从外部存储读取数据。
    注意：从API级别33开始，此权限不再生效。如果您的应用程序访问其他应用程序的媒体文件，请请求以下一个或多个权限：READ_MEDIA_IMAGES、READ_MEDIA_VIDEO、READ_MEDIA_AUDIO。了解更多关于与媒体文件相关的存储权限。
    从API级别19开始，此权限被强制执行。在API级别19之前，此权限不强制执行，所有应用程序仍然可以访问外部存储。您可以通过在运行Android 4.1或更高版本的设备上启用设置应用程序中的开发者选项下的保护USB存储来测试您的应用程序，以强制执行此权限。
    从API级别19开始，此权限不再需要，以读取或写入由Context.getExternalFilesDir(String)和Context.getExternalCacheDir()返回的应用程序特定目录中的文件。
    从API级别29开始，应用程序不需要请求此权限来访问外部存储中的应用程序特定目录或MediaStore中的自己的文件。应用程序不应该请求此权限，除非它们需要访问MediaStore中的其他应用程序的文件。有关这些更改的更多信息，请参阅开发者文档中的范围存储部分。
    如果您的minSdkVersion和targetSdkVersion值都设置为3或更低，系统会隐式授予您的应用程序此权限。如果您不需要此权限，请确保您的targetSdkVersion为4或更高。
    这是一个软限制权限，除非记录的安装程序允许列表中包含此权限，否则应用程序无法以完整形式持有此权限。具体来说，如果权限被允许列表，持有应用程序可以访问外部存储和视觉和听觉媒体集合，如果权限未被允许列表，持有应用程序只能访问视觉和听觉媒体集合。此外，此权限是不可变的限制，这意味着允许列表状态只能在安装时指定，直到应用程序安装后才能更改。有关更多详细信息，请参见PackageInstaller.SessionParams.setWhitelistedRestrictedPermissions(Set)。
    保护级别：危险
    */
    pub const READ_EXTERNAL_STORAGE: &'static str = "android.permission.READ_EXTERNAL_STORAGE";

    /**
    允许应用程序查询AppSearch中HOME角色可见的全局数据。
    */
    pub const READ_HOME_APP_SEARCH_DATA: &'static str =
        "android.permission.READ_HOME_APP_SEARCH_DATA";

    /**
    错误
    此常量在API级别16中已被弃用。
    允许应用程序检索按键和开关的当前状态。
    第三方应用程序不得使用。
    */
    #[deprecated(note = "使用该权限的API已被移除。")]
    pub const READ_INPUT_STATE: &'static str = "android.permission.READ_INPUT_STATE";

    /**
    允许应用程序读取低级别的系统日志文件。
    第三方应用程序不得使用，因为日志条目可能包含用户的私人信息。
    */
    pub const READ_LOGS: &'static str = "android.permission.READ_LOGS";

    /**
    允许应用程序从外部存储读取音频文件。
    此权限从API级别Build.VERSION_CODES.TIRAMISU开始强制执行。针对Build.VERSION_CODES.TIRAMISU或更高版本且需要从外部存储读取音频文件的应用程序必须持有此权限；无需READ_EXTERNAL_STORAGE权限。对于targetSdkVersion为Build.VERSION_CODES.S_V2或更低版本的应用程序，则需要READ_EXTERNAL_STORAGE权限来读取音频文件。
    保护级别：危险
    */
    pub const READ_MEDIA_AUDIO: &'static str = "android.permission.READ_MEDIA_AUDIO";

    /**
    允许应用程序从外部存储读取图像文件。
    此权限从API级别Build.VERSION_CODES.TIRAMISU开始强制执行。针对Build.VERSION_CODES.TIRAMISU或更高版本且需要从外部存储读取图像文件的应用程序必须持有此权限；无需READ_EXTERNAL_STORAGE权限。对于targetSdkVersion为Build.VERSION_CODES.S_V2或更低版本的应用程序，则需要READ_EXTERNAL_STORAGE权限来读取图像文件。
    保护级别：危险
    */
    pub const READ_MEDIA_IMAGES: &'static str = "android.permission.READ_MEDIA_IMAGES";

    /**
    允许应用程序从外部存储读取视频文件。
    此权限从API级别Build.VERSION_CODES.TIRAMISU开始强制执行。针对Build.VERSION_CODES.TIRAMISU或更高版本且需要从外部存储读取视频文件的应用程序必须持有此权限；不需要READ_EXTERNAL_STORAGE权限。对于targetSdkVersion为Build.VERSION_CODES.S_V2或更低版本的应用程序，要读取视频文件，则需要READ_EXTERNAL_STORAGE权限。
    保护级别：危险
    */
    pub const READ_MEDIA_VIDEO: &'static str = "android.permission.READ_MEDIA_VIDEO";

    /**
    允许应用程序读取用户通过权限提示的照片选择器选择的外部存储中的图像或视频文件。应用程序可以检查此权限，以验证用户是否决定使用照片选择器，而不是授予READ_MEDIA_IMAGES或READ_MEDIA_VIDEO的访问权限。它不会阻止应用程序手动访问标准的照片选择器。根据所需的媒体类型，应与此权限一起请求READ_MEDIA_IMAGES和/或READ_MEDIA_VIDEO。
    如果应用程序请求READ_MEDIA_IMAGES、READ_MEDIA_VIDEO或ACCESS_MEDIA_LOCATION，则无论target SDK版本如何，此权限都会自动添加到应用程序的清单中。如果应用程序不请求此权限，则对于READ_MEDIA_IMAGES和/或READ_MEDIA_VIDEO，授权对话框将返回`PERMISSION_GRANTED`，但应用程序只能访问用户选择的媒体。这种虚假的授予状态将持续到应用程序进入后台为止。
    保护级别：危险
    */
    pub const READ_MEDIA_VISUAL_USER_SELECTED: &'static str =
        "android.permission.READ_MEDIA_VISUAL_USER_SELECTED";

    /**
    允许应用读取附近的流式传输策略。该策略控制是否允许设备将其通知和应用流式传输到附近的其他设备。非设备所有者的应用需要此权限才能调用 DevicePolicyManager.getNearbyNotificationStreamingPolicy() 或 DevicePolicyManager.getNearbyAppStreamingPolicy()。
    */
    pub const READ_NEARBY_STREAMING_POLICY: &'static str =
        "android.permission.READ_NEARBY_STREAMING_POLICY";

    /**
    允许读取设备的电话号码。这是 READ_PHONE_STATE 权限授予功能的一个子集，但对即时应用开放。
    保护级别：危险|即时
    */
    pub const READ_PHONE_NUMBERS: &'static str = "android.permission.READ_PHONE_NUMBERS";

    /**
    允许只读访问电话状态，包括当前蜂窝网络信息、任何正在进行的通话的状态以及设备上注册的所有 PhoneAccounts 的列表。
    注意：如果您的 minSdkVersion 和 targetSdkVersion 值都设置为 3 或更低，系统将隐式授予您的应用此权限。如果您不需要此权限，请确保您的 targetSdkVersion 为 4 或更高。
    保护级别：危险
    */
    pub const READ_PHONE_STATE: &'static str = "android.permission.READ_PHONE_STATE";

    /**
    允许只读访问精确的电话状态。允许读取电话状态的详细信息，适用于特殊用途的应用，如拨号器、运营商应用或 IMS 应用。
    */
    pub const READ_PRECISE_PHONE_STATE: &'static str =
        "android.permission.READ_PRECISE_PHONE_STATE";

    /**
    允许应用读取 SMS 消息。
    保护级别：危险
    这是一个严格受限的权限，除非记录中的安装程序将此权限加入白名单，否则应用无法获得此权限。更多详细信息，请参阅 PackageInstaller.SessionParams.setWhitelistedRestrictedPermissions(Set)。
    */
    pub const READ_SMS: &'static str = "android.permission.READ_SMS";

    /**
    允许应用程序读取同步设置。
    保护级别：普通
    */
    pub const READ_SYNC_SETTINGS: &'static str = "android.permission.READ_SYNC_SETTINGS";

    /**
    允许应用程序读取同步统计信息。
    保护级别：普通
    */
    pub const READ_SYNC_STATS: &'static str = "android.permission.READ_SYNC_STATS";

    /**
    允许应用程序读取系统中的语音邮件。
    保护级别：签名|特权|角色
    */
    pub const READ_VOICEMAIL: &'static str = "com.android.voicemail.permission.READ_VOICEMAIL";

    /**
    需要此权限才能重启设备。
    第三方应用程序不得使用。
    */
    pub const REBOOT: &'static str = "android.permission.REBOOT";

    /**
    允许应用程序接收系统启动完成后广播的Intent.ACTION_BOOT_COMPLETED。如果不请求此权限，则在该时刻无法接收到此广播。尽管持有此权限本身没有安全隐患，但它可能会对用户体验产生负面影响，因为它会增加系统启动所需的时间，并允许应用程序在用户不知情的情况下运行。因此，您必须明确声明使用此功能，以便让用户知晓。
    保护级别：普通
    */
    pub const RECEIVE_BOOT_COMPLETED: &'static str = "android.permission.RECEIVE_BOOT_COMPLETED";

    /**
    允许应用程序监视接收到的MMS消息。
    保护级别：危险
    这是一个严格的受限权限，在记录中的安装程序将该权限加入白名单之前，应用程序无法持有此权限。更多详细信息，请参阅PackageInstaller.SessionParams.setWhitelistedRestrictedPermissions(Set)。
    */
    pub const RECEIVE_MMS: &'static str = "android.permission.RECEIVE_MMS";

    /**
    允许应用程序接收SMS消息。
    保护级别：危险
    这是一个严格的受限权限，在记录中的安装程序将该权限加入白名单之前，应用程序无法持有此权限。更多详细信息，请参阅PackageInstaller.SessionParams.setWhitelistedRestrictedPermissions(Set)。
    */
    pub const RECEIVE_SMS: &'static str = "android.permission.RECEIVE_SMS";

    /**
    允许应用程序接收WAP推送消息。
    保护级别：危险
    这是一个严格的受限权限，在记录中的安装程序将该权限加入白名单之前，应用程序无法持有此权限。更多详细信息，请参阅PackageInstaller.SessionParams.setWhitelistedRestrictedPermissions(Set)。
    */
    pub const RECEIVE_WAP_PUSH: &'static str = "android.permission.RECEIVE_WAP_PUSH";

    /**
    允许应用程序录制音频。
    保护级别：危险
    */
    pub const RECORD_AUDIO: &'static str = "android.permission.RECORD_AUDIO";

    /**
    允许应用程序更改任务的Z轴顺序（即前后顺序）。
    保护级别：正常
    */
    pub const REORDER_TASKS: &'static str = "android.permission.REORDER_TASKS";

    /**
    允许应用程序通过CompanionDeviceManager请求与能够流式传输Android应用的虚拟显示器关联（AssociationRequest.DEVICE_PROFILE_APP_STREAMING）。
    非第三方应用程序使用。
    */
    pub const REQUEST_COMPANION_PROFILE_APP_STREAMING: &'static str =
        "android.permission.REQUEST_COMPANION_PROFILE_APP_STREAMING";

    /**
    允许应用程序通过CompanionDeviceManager请求与能够进行车载投影的车辆抬头显示器（HUD）关联（AssociationRequest.DEVICE_PROFILE_AUTOMOTIVE_PROJECTION）。
    非第三方应用程序使用。
    */
    pub const REQUEST_COMPANION_PROFILE_AUTOMOTIVE_PROJECTION: &'static str =
        "android.permission.REQUEST_COMPANION_PROFILE_AUTOMOTIVE_PROJECTION";

    /**
    允许应用程序通过CompanionDeviceManager请求与计算机关联，以与其他设备共享功能和/或数据，如通知、照片和媒体（AssociationRequest.DEVICE_PROFILE_COMPUTER）。
    非第三方应用程序使用。
    */
    pub const REQUEST_COMPANION_PROFILE_COMPUTER: &'static str =
        "android.permission.REQUEST_COMPANION_PROFILE_COMPUTER";

    /**
    允许应用程序通过CompanionDeviceManager请求以“眼镜”的身份与设备关联
    保护级别：普通
    */
    pub const REQUEST_COMPANION_PROFILE_GLASSES: &'static str =
        "android.permission.REQUEST_COMPANION_PROFILE_GLASSES";

    /**
    允许应用程序通过CompanionDeviceManager请求从Android主机向附近设备流式传输内容（AssociationRequest.DEVICE_PROFILE_NEARBY_DEVICE_STREAMING）。
    非第三方应用程序使用。
    */
    pub const REQUEST_COMPANION_PROFILE_NEARBY_DEVICE_STREAMING: &'static str =
        "android.permission.REQUEST_COMPANION_PROFILE_NEARBY_DEVICE_STREAMING";

    /**
    允许应用程序通过CompanionDeviceManager请求以“手表”的身份与设备关联
    保护级别：普通
    */
    pub const REQUEST_COMPANION_PROFILE_WATCH: &'static str =
        "android.permission.REQUEST_COMPANION_PROFILE_WATCH";

    /**
    允许伴侣应用在后台运行。此权限意味着拥有 REQUEST_COMPANION_START_FOREGROUND_SERVICES_FROM_BACKGROUND 权限，并允许从后台启动前台服务。如果应用无需在后台运行，而只是需要从后台启动前台服务，请考虑使用权限较小的 REQUEST_COMPANION_START_FOREGROUND_SERVICES_FROM_BACKGROUND。
    保护级别：普通
    */
    pub const REQUEST_COMPANION_RUN_IN_BACKGROUND: &'static str =
        "android.permission.REQUEST_COMPANION_RUN_IN_BACKGROUND";

    /**
    允许应用创建“自管理”关联。
    */
    pub const REQUEST_COMPANION_SELF_MANAGED: &'static str =
        "android.permission.REQUEST_COMPANION_SELF_MANAGED";

    /**
    允许伴侣应用从后台启动前台服务。
    保护级别：普通
    另请参阅：
    REQUEST_COMPANION_RUN_IN_BACKGROUND
    */
    pub const REQUEST_COMPANION_START_FOREGROUND_SERVICES_FROM_BACKGROUND: &'static str =
        "android.permission.REQUEST_COMPANION_START_FOREGROUND_SERVICES_FROM_BACKGROUND";

    /**
    允许伴侣应用在后台使用数据。
    保护级别：普通
    */
    pub const REQUEST_COMPANION_USE_DATA_IN_BACKGROUND: &'static str =
        "android.permission.REQUEST_COMPANION_USE_DATA_IN_BACKGROUND";

    /**
    允许应用请求删除软件包。以 API 级别 Build.VERSION_CODES.P 或更高版本为目标的应用必须持有此权限，才能使用 Intent.ACTION_UNINSTALL_PACKAGE 或 PackageInstaller.uninstall(VersionedPackage, IntentSender)。
    保护级别：普通
    */
    pub const REQUEST_DELETE_PACKAGES: &'static str = "android.permission.REQUEST_DELETE_PACKAGES";

    /**
    应用程序必须持有的权限，才能使用Settings.ACTION_REQUEST_IGNORE_BATTERY_OPTIMIZATIONS。
    保护级别：普通
    */
    pub const REQUEST_IGNORE_BATTERY_OPTIMIZATIONS: &'static str =
        "android.permission.REQUEST_IGNORE_BATTERY_OPTIMIZATIONS";

    /**
    允许应用程序请求安装包。面向API级别大于25的应用程序必须持有此权限，才能使用Intent.ACTION_INSTALL_PACKAGE。
    保护级别：签名
    */
    pub const REQUEST_INSTALL_PACKAGES: &'static str =
        "android.permission.REQUEST_INSTALL_PACKAGES";

    /**
    允许应用程序订阅其关联伴侣设备存在状态更改的通知。
    */
    pub const REQUEST_OBSERVE_COMPANION_DEVICE_PRESENCE: &'static str =
        "android.permission.REQUEST_OBSERVE_COMPANION_DEVICE_PRESENCE";

    /**
    允许应用程序请求屏幕锁定复杂度，并提示用户将屏幕锁定更新到一定的复杂度级别。
    保护级别：普通
    */
    pub const REQUEST_PASSWORD_COMPLEXITY: &'static str =
        "android.permission.REQUEST_PASSWORD_COMPLEXITY";

    /**
    错误
    此常量在API级别15中已被弃用。
    */
    #[deprecated(note = "ActivityManager.restartPackage(String) API不再受支持。")]
    pub const RESTART_PACKAGES: &'static str = "android.permission.RESTART_PACKAGES";

    /**
    允许应用程序使用用户发起的任务API。更多详细信息，请参阅JobInfo.Builder.setUserInitiated(boolean)。
    保护级别：普通
    */
    pub const RUN_USER_INITIATED_JOBS: &'static str = "android.permission.RUN_USER_INITIATED_JOBS";

    /**
    允许应用程序使用精确闹钟API。
    这是一个可以被系统或用户撤销的特殊访问权限。它仅应用于启用需要精确闹钟的用户界面功能。更多详细信息，请参阅相关的开发者文档。
    应用程序需要针对API版本Build.VERSION_CODES.S或更高版本才能请求此权限。请注意，针对较低API级别的应用程序无需此权限即可使用精确闹钟API。
    持有此权限并且针对API版本Build.VERSION_CODES.TIRAMISU及以下版本的应用程序将始终保留在WORKING_SET或更低的待机桶中。
    如果您的应用程序的核心功能依赖于精确闹钟，则可以在针对API版本Build.VERSION_CODES.TIRAMISU时请求USE_EXACT_ALARM权限。所有使用精确闹钟作为次要功能（这些功能仍然应该面向用户）的应用程序应继续使用此权限。
    保护级别：签名|特权|应用操作
    */
    pub const SCHEDULE_EXACT_ALARM: &'static str = "android.permission.SCHEDULE_EXACT_ALARM";

    /**
    允许一个应用程序（电话）在来电期间向其他应用程序发送请求，以处理通过消息回复的操作。
    第三方应用程序不得使用。
    */
    pub const SEND_RESPOND_VIA_MESSAGE: &'static str =
        "android.permission.SEND_RESPOND_VIA_MESSAGE";

    /**
    允许应用程序发送短信。
    保护级别：危险
    这是一个严格受限的权限，在记录的安装程序将权限加入白名单之前，应用程序无法持有此权限。更多详细信息，请参阅PackageInstaller.SessionParams.setWhitelistedRestrictedPermissions(Set)。
    */
    pub const SEND_SMS: &'static str = "android.permission.SEND_SMS";

    /**
    允许应用向用户广播一个设置闹钟的Intent。
    保护级别：正常
    */
    pub const SET_ALARM: &'static str = "com.android.alarm.permission.SET_ALARM";

    /**
    允许应用控制当活动被置于后台时是否立即结束。
    第三方应用不可使用。
    */
    pub const SET_ALWAYS_FINISH: &'static str = "android.permission.SET_ALWAYS_FINISH";

    /**
    修改全局动画缩放比例。
    第三方应用不可使用。
    */
    pub const SET_ANIMATION_SCALE: &'static str = "android.permission.SET_ANIMATION_SCALE";

    /**
    允许应用在BiometricDialog（系统UI）上设置高级功能，包括徽标、徽标描述和带有更多选项按钮的内容视图。
    第三方应用不可使用。
    */
    pub const SET_BIOMETRIC_DIALOG_ADVANCED: &'static str =
        "android.permission.SET_BIOMETRIC_DIALOG_ADVANCED";

    /**
    为应用配置调试。
    第三方应用不可使用。
    */
    pub const SET_DEBUG_APP: &'static str = "android.permission.SET_DEBUG_APP";

    /**
    错误
    此常量在API级别15中已被弃用。
    */
    #[deprecated(note = "不再有用，详情请参见PackageManager.addPackageToPreferred(String)。")]
    pub const SET_PREFERRED_APPLICATIONS: &'static str =
        "android.permission.SET_PREFERRED_APPLICATIONS";

    /**
    允许应用程序设置可以同时运行的最大数量（非必需）的应用程序进程。
    第三方应用程序不得使用。
    */
    pub const SET_PROCESS_LIMIT: &'static str = "android.permission.SET_PROCESS_LIMIT";

    /**
    允许应用程序直接设置系统时间。
    第三方应用程序不得使用。
    */
    pub const SET_TIME: &'static str = "android.permission.SET_TIME";

    /**
    允许应用程序直接设置系统时区。
    第三方应用程序不得使用。
    */
    pub const SET_TIME_ZONE: &'static str = "android.permission.SET_TIME_ZONE";

    /**
    允许应用程序设置壁纸。
    保护级别：正常
    */
    pub const SET_WALLPAPER: &'static str = "android.permission.SET_WALLPAPER";

    /**
    允许应用程序设置壁纸提示。
    保护级别：正常
    */
    pub const SET_WALLPAPER_HINTS: &'static str = "android.permission.SET_WALLPAPER_HINTS";

    /**
    允许应用程序请求向所有持久进程发送信号。
    第三方应用程序不得使用。
    */
    pub const SIGNAL_PERSISTENT_PROCESSES: &'static str =
        "android.permission.SIGNAL_PERSISTENT_PROCESSES";

    //noinspection SpellCheckingInspection
    /**
    错误
    此常量在API级别31中已被弃用。
    允许金融类应用程序读取经过筛选的短信消息。保护级别：signature|appop
    */
    #[deprecated(note = "使用该权限的API已不再有效。")]
    pub const SMS_FINANCIAL_TRANSACTIONS: &'static str =
        "android.permission.SMS_FINANCIAL_TRANSACTIONS";

    /**
    允许应用程序在任何时候从后台启动前台服务。此权限不供第三方应用程序使用，唯一例外是当应用程序是默认的短信应用程序时。否则，它仅可由特权应用程序、应用验证器应用程序以及具有任何EMERGENCY或SYSTEM GALLERY角色的应用程序使用。
    */
    pub const START_FOREGROUND_SERVICES_FROM_BACKGROUND: &'static str =
        "android.permission.START_FOREGROUND_SERVICES_FROM_BACKGROUND";

    /**
    允许持有者启动显示应用程序功能列表的屏幕。
    保护级别：signature|installer
    */
    pub const START_VIEW_APP_FEATURES: &'static str = "android.permission.START_VIEW_APP_FEATURES";

    /**
    允许持有者启动显示应用程序权限使用情况的屏幕。
    保护级别：signature|installer
    */
    pub const START_VIEW_PERMISSION_USAGE: &'static str =
        "android.permission.START_VIEW_PERMISSION_USAGE";

    /**
    允许应用程序打开、关闭或禁用状态栏及其图标。
    第三方应用程序不得使用。
    */
    pub const STATUS_BAR: &'static str = "android.permission.STATUS_BAR";

    /**
    允许应用程序订阅锁屏（即显示）状态。
    保护级别：签名|角色
    仅供ROLE_ASSISTANT角色和签名应用使用。
    */
    pub const SUBSCRIBE_TO_KEYGUARD_LOCKED_STATE: &'static str =
        "android.permission.SUBSCRIBE_TO_KEYGUARD_LOCKED_STATE";

    /**
    允许应用使用WindowManager.LayoutParams.TYPE_APPLICATION_OVERLAY类型创建窗口，这些窗口会显示在所有其他应用之上。很少有应用应该使用此权限；这些窗口旨在与用户进行系统级交互。
    注意：如果应用针对的是API级别23或更高版本，则应用用户必须通过权限管理屏幕明确授予此权限给应用。应用通过发送带有Settings.ACTION_MANAGE_OVERLAY_PERMISSION操作的Intent来请求用户的批准。应用可以通过调用Settings.canDrawOverlays()来检查它是否已获得此授权。
    保护级别：签名|设置|应用操作|安装程序|pre23|开发
    */
    pub const SYSTEM_ALERT_WINDOW: &'static str = "android.permission.SYSTEM_ALERT_WINDOW";

    /**
    如果设备可用，则允许使用设备的红外发射器。
    保护级别：普通
    */
    pub const TRANSMIT_IR: &'static str = "android.permission.TRANSMIT_IR";

    /**
    允许应用打开屏幕，例如使用PowerManager.ACQUIRE_CAUSES_WAKEUP。
    仅供家庭自动化应用使用。
    */
    pub const TURN_SCREEN_ON: &'static str = "android.permission.TURN_SCREEN_ON";

    /**
    错误
    不要在您的应用中使用此权限。
    此权限已不再受支持。
    */
    pub const UNINSTALL_SHORTCUT: &'static str =
        "com.android.launcher.permission.UNINSTALL_SHORTCUT";

    /**
    允许应用程序更新设备统计信息。
    第三方应用不得使用。
    */
    pub const UPDATE_DEVICE_STATS: &'static str = "android.permission.UPDATE_DEVICE_STATS";

    /**
    允许应用程序通过PackageInstaller.SessionParams.setRequireUserAction(int)指示应用更新不需要用户操作。
    保护级别：普通
    */
    pub const UPDATE_PACKAGES_WITHOUT_USER_ACTION: &'static str =
        "android.permission.UPDATE_PACKAGES_WITHOUT_USER_ACTION";

    /**
    允许应用程序使用设备支持的生物识别模式。
    保护级别：普通
    */
    pub const USE_BIOMETRIC: &'static str = "android.permission.USE_BIOMETRIC";

    /**
    允许应用程序使用精确闹钟，就像使用SCHEDULE_EXACT_ALARM一样，但无需向用户请求此权限。
    这仅供那些核心功能依赖于精确闹钟的应用程序使用。如果您的应用程序需要精确闹钟来实现用户可能使用也可能不使用的次要功能，则应继续使用SCHEDULE_EXACT_ALARM。
    请记住，这是一个强大的权限，应用商店可能会执行策略来审核和审查此权限的使用情况。此类审核如果发现应用程序滥用此权限，可能会导致应用从应用商店中移除。
    应用程序需要针对API版本Build.VERSION_CODES.TIRAMISU或更高版本才能请求此权限。请注意，在设备上应只请求USE_EXACT_ALARM或SCHEDULE_EXACT_ALARM其中之一。如果您的应用程序在旧版SDK中已经使用SCHEDULE_EXACT_ALARM，但在SDK 33及更高版本上需要使用USE_EXACT_ALARM，则应将SCHEDULE_EXACT_ALARM声明为具有max-sdk属性，如下所示：
    `<uses-permission android:name="android.permission.SCHEDULE_EXACT_ALARM" android:maxSdkVersion="32" />`
    持有此权限的应用程序始终保持在WORKING_SET或更低的待机分组中。
    */
    pub const USE_EXACT_ALARM: &'static str = "android.permission.USE_EXACT_ALARM";

    /**
    （已弃用）
    此常量在API级别28中已被弃用。
    允许应用程序使用指纹硬件。
    保护级别：普通
    */
    #[deprecated(note = "应用程序应请求USE_BIOMETRIC代替")]
    pub const USE_FINGERPRINT: &'static str = "android.permission.USE_FINGERPRINT";

    /**
    对于针对Build.VERSION_CODES.Q版本且希望使用通知全屏Intent的应用是必需的。
    保护级别：普通
    */
    pub const USE_FULL_SCREEN_INTENT: &'static str = "android.permission.USE_FULL_SCREEN_INTENT";

    /**
    允许读取设备标识符并使用基于ICC的身份验证，如EAP-AKA。通常在访问运营商服务器和管理订户服务时的身份验证中需要。
    保护级别：签名|应用操作
    */
    pub const USE_ICC_AUTH_WITH_DEVICE_IDENTIFIER: &'static str =
        "android.permission.USE_ICC_AUTH_WITH_DEVICE_IDENTIFIER";

    /**
    允许应用程序使用SIP服务。
    保护级别：危险
    */
    pub const USE_SIP: &'static str = "android.permission.USE_SIP";

    /**
    为了能够使用超宽带技术测距到设备是必需的。
    保护级别：危险
    */
    pub const UWB_RANGING: &'static str = "android.permission.UWB_RANGING";

    /**
    允许访问振动器。
    保护级别：普通
    */
    pub const VIBRATE: &'static str = "android.permission.VIBRATE";

    /**
    允许使用PowerManager WakeLocks来防止处理器进入休眠状态或屏幕变暗。
    保护级别：普通
    */
    pub const WAKE_LOCK: &'static str = "android.permission.WAKE_LOCK";

    /**
    允许应用程序写入APN（接入点名称）设置，并读取现有APN设置的敏感字段，如用户名和密码。
    第三方应用程序不得使用。
    */
    pub const WRITE_APN_SETTINGS: &'static str = "android.permission.WRITE_APN_SETTINGS";

    /**
    允许应用程序写入用户的日历数据。
    保护级别：危险
    */
    pub const WRITE_CALENDAR: &'static str = "android.permission.WRITE_CALENDAR";

    /**
    允许应用程序写入和读取用户的通话记录数据。
    注意：如果您的应用使用了WRITE_CONTACTS权限，并且您的minSdkVersion和targetSdkVersion值都设置为15或更低，系统将隐式授予您的应用此权限。如果您不需要此权限，请确保您的targetSdkVersion为16或更高。
    保护级别：危险
    这是一个严格受限的权限，除非记录中的安装程序将此权限加入白名单，否则应用程序无法获得此权限。更多详情，请参阅PackageInstaller.SessionParams.setWhitelistedRestrictedPermissions(Set)。
    */
    pub const WRITE_CALL_LOG: &'static str = "android.permission.WRITE_CALL_LOG";

    /**
    允许应用程序写入用户的联系人数据。
    保护级别：危险
    */
    pub const WRITE_CONTACTS: &'static str = "android.permission.WRITE_CONTACTS";

    /**
    允许应用程序写入外部存储。
    注意：如果您的应用以Build.VERSION_CODES.R或更高版本为目标，则此权限无效。
    如果您的应用在运行API级别19或更高版本的设备上，则无需声明此权限即可在Context.getExternalFilesDir(String)和Context.getExternalCacheDir()返回的应用特定目录中读写文件。
    了解更多有关如何修改您的应用不拥有的媒体文件以及非媒体文件的信息。
    如果您的应用是文件管理器并且需要广泛访问外部存储文件，则系统必须将您的应用添加到允许列表中，以便您可以成功请求MANAGE_EXTERNAL_STORAGE权限。了解更多有关minSdkVersion和targetSdkVersion值的适当用例的信息。如果它们的值设置为3或更低，系统将隐式授予您的应用此权限。如果您不需要此权限，请确保您的targetSdkVersion为4或更高。
    保护级别：危险
    */
    pub const WRITE_EXTERNAL_STORAGE: &'static str = "android.permission.WRITE_EXTERNAL_STORAGE";

    //noinspection SpellCheckingInspection
    /**
    允许应用程序修改Google服务地图。
    第三方应用程序不得使用。
    */
    pub const WRITE_GSERVICES: &'static str = "android.permission.WRITE_GSERVICES";

    /**
    允许应用程序读取或写入安全的系统设置。
    第三方应用程序不得使用。
    */
    pub const WRITE_SECURE_SETTINGS: &'static str = "android.permission.WRITE_SECURE_SETTINGS";

    //noinspection SpellCheckingInspection
    /**
    允许应用读取或写入系统设置。
    注意：如果应用以API级别23或更高版本为目标，应用用户必须通过在权限管理屏幕中明确授予此权限给应用。应用通过发送一个带有Settings.ACTION_MANAGE_WRITE_SETTINGS动作的intent来请求用户的批准。应用可以通过调用Settings.System.canWrite()来检查是否已获得此授权。
    保护级别：signature|preinstalled|appop|pre23
    */
    pub const WRITE_SETTINGS: &'static str = "android.permission.WRITE_SETTINGS";

    /**
    允许应用写入同步设置。
    保护级别：normal
    */
    pub const WRITE_SYNC_SETTINGS: &'static str = "android.permission.WRITE_SYNC_SETTINGS";

    /**
    允许应用修改和删除系统中现有的语音邮件。
    保护级别：signature|privileged|role
    */
    pub const WRITE_VOICEMAIL: &'static str = "com.android.voicemail.permission.WRITE_VOICEMAIL";
}
