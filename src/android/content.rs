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

/// 包含用于访问有关应用程序包的信息的类，包括有关其活动、权限、服务、签名和提供程序的信息。
#[cfg(feature = "android_content_pm")]
pub mod pm;

use droid_wrap_derive::{java_class, java_constructor, java_interface, java_method};

use crate::{
    android::os::Bundle,
    java::{
        io::{File, Serializable},
        lang::{CharSequence, ClassLoader, Comparable, Object},
    },
    JObjNew, JObjRef, JType,
};

/**
与应用程序环境相关的全局信息的接口。这是一个抽象类，其实现由 Android 系统提供。它允许访问特定于应用程序的资源和类，以及对应用程序级操作（如启动活动、广播和接收意图等）的向上调用。
*/
#[java_class(name = "android/content/Context")]
pub struct Context;

impl Context {
    /// 文件创建模式：默认模式，其中创建的文件只能由调用应用程序（或共享同一用户 ID 的所有应用程序）访问。
    pub const MODE_PRIVATE: i32 = 0x0000;

    /// 文件创建模式：允许所有其他应用程序对创建的文件具有读取权限。从 android.os.Build.VERSION_CODES#N 开始，尝试使用此模式会引发 SecurityException。
    #[deprecated(
        note = "创建全球可读文件非常危险，并且可能导致应用程序出现安全漏洞。强烈反对这样做；相反，应用程序应该使用更正式的机制进行交互，例如 ContentProvider、BroadcastReceiver 和 android.app.Service。无法保证此访问模式会保留在文件上，例如在备份和恢复时。"
    )]
    pub const MODE_WORLD_READABLE: i32 = 0x0001;

    /// 文件创建模式：允许所有其他应用程序对创建的文件具有写访问权限。从 android.os.Build.VERSION_CODES#N 开始，尝试使用此模式将引发 SecurityException。
    #[deprecated(
        note = "创建全球可写文件非常危险，并且可能导致应用程序出现安全漏洞。强烈反对这样做；相反，应用程序应该使用更正式的机制进行交互，例如 ContentProvider、BroadcastReceiver 和 android.app.Service。无法保证此访问模式会保留在文件上，例如在备份和恢复时。"
    )]
    pub const MODE_WORLD_WRITEABLE: i32 = 0x0002;

    /// 文件创建模式：与 openFileOutput 一起使用，如果文件已经存在，则将数据写入现有文件的末尾而不是擦除它。
    pub const MODE_APPEND: i32 = 0x8000;

    /**
    SharedPreference 加载标志：设置后，即使共享首选项实例已在此进程中加载​​，也会检查磁盘上的文件是否被修改。当应用程序有多个进程时，有时需要此行为，所有进程都写入同一个 SharedPreferences 文件。不过，进程之间通常有更好的通信形式。
    这是 Gingerbread（Android 2.3）及之前的遗留（但未记录）行为，当针对此类版本时，此标志是隐含的。对于针对高于 Android 2.3 的 SDK 版本的应用程序，如果需要，必须明确设置此标志。
    */
    #[deprecated(
        note = "MODE_MULTI_PROCESS 在某些 Android 版本中无法可靠地工作，而且不提供任何跨进程协调并发修改的机制。应用程序不应尝试使用它。相反，它们应该使用显式跨进程数据管理方法，例如 android.content.ContentProvider ContentProvider。"
    )]
    pub const MODE_MULTI_PROCESS: i32 = 0x0004;

    /// 数据库打开标志：设置后，数据库打开时默认启用预写日志功能。
    pub const MODE_ENABLE_WRITE_AHEAD_LOGGING: i32 = 0x0008;

    /// 数据库打开标志：设置后，数据库打开时不支持本地化排序器。
    pub const MODE_NO_LOCALIZED_COLLATORS: i32 = 0x0010;

    /// 与 getSystemService(String) 一起使用来查询 android.os.PowerManager 来控制电源管理，包括“唤醒锁”，它可让您在运行长时间任务时保持设备开启。
    pub const POWER_SERVICE: &'static str = "power";

    //noinspection SpellCheckingInspection
    /// 与 getSystemService(String) 一起使用来查询 android.os.PowerStatsService 以访问电力统计服务。
    pub const POWER_STATS_SERVICE: &'static str = "powerstats";

    /// 与 getSystemService(String) 一起使用来查询 android.os.RecoverySystem 以访问恢复系统服务。
    pub const RECOVERY_SERVICE: &'static str = "recovery";

    /// 与 getSystemService(String) 一起使用来查询 android.os.SystemUpdateManager 以访问系统更新管理器服务。
    pub const SYSTEM_UPDATE_SERVICE: &'static str = "system_update";

    /// 与GetSystemService(String)一起查询android.view.WindowManager访问系统的窗口管理器。
    pub const WINDOW_SERVICE: &'static str = "window";

    //noinspection SpellCheckingInspection
    /// 与 getSystemService(String) 一起使用来查询 android.view.LayoutInflater 以在此上下文中扩充布局资源。
    pub const LAYOUT_INFLATER_SERVICE: &'static str = "layout_inflater";

    /// 与 getSystemService(String) 一起使用来查询 android.accounts.AccountManager，以便在您选择的时间接收意图。
    pub const ACCOUNT_SERVICE: &'static str = "account";

    /// 与 getSystemService(String) 一起使用来查询 android.app.ActivityManager 以便与全局系统状态进行交互。
    pub const ACTIVITY_SERVICE: &'static str = "activity";

    /// 与 getSystemService(String) 一起使用来查询 android.app.ActivityTaskManager 以便与全局系统状态进行交互。
    pub const ACTIVITY_TASK_SERVICE: &'static str = "activity_task";

    /// 与 getSystemService(String) 一起使用来查询 android.app.UriGrantsManager 以便与全局系统状态进行交互。
    pub const URI_GRANTS_SERVICE: &'static str = "uri_grants";

    /// 与GetSystemService(String)一起查询android.app.AlarmManager在您选择时接收意图。
    pub const ALARM_SERVICE: &'static str = "alarm";

    /// 与GetSystemService(String)一起查询android.app.NotificationManager，以告知用户背景事件。
    pub const NOTIFICATION_SERVICE: &'static str = "notification";

    /// 与 getSystemService(String) 一起使用来查询 android.view.accessibility.AccessibilityManager，以便通过注册的事件监听器向用户提供 UI 事件的反馈。
    pub const ACCESSIBILITY_SERVICE: &'static str = "accessibility";

    /// 与 getSystemService(String) 一起使用来查询 android.view.accessibility.CaptioningManager，以获取字幕属性并监听字幕偏好设置的变化。
    pub const CAPTIONING_SERVICE: &'static str = "captioning";

    /// 与 getSystemService(String) 一起使用来查询用于控制键盘保护的 android.app.KeyguardManager。
    pub const KEYGUARD_SERVICE: &'static str = "keyguard";

    /// 与 getSystemService(String) 一起使用来查询 android.location.LocationManager 来控制位置更新。
    pub const LOCATION_SERVICE: &'static str = "location";

    /// 与 getSystemService(String) 一起使用来查询 android.location.CountryDetector 来检测用户所在的国家/地区。
    pub const COUNTRY_DETECTOR: &'static str = "country_detector";

    /**
    与 getSystemService(String) 一起使用来查询 android.app.SearchManager 来处理搜索。
    配置＃ui_mode_type_watch不支持android.app.SearchManager。
    */
    pub const SEARCH_SERVICE: &'static str = "search";

    /// 与 getSystemService(String) 一起使用来查询用于访问传感器的 android.hardware.SensorManager。
    pub const SENSOR_SERVICE: &'static str = "sensor";

    /// 与 getSystemService(String) 一起使用来查询 android.hardware.SensorPrivacyManager，以访问传感器隐私功能。
    pub const SENSOR_PRIVACY_SERVICE: &'static str = "sensor_privacy";

    /// 与 getSystemService(String) 一起使用来查询 android.os.storage.StorageManager 来访问系统存储功能。
    pub const STORAGE_SERVICE: &'static str = "storage";

    //noinspection SpellCheckingInspection
    /// 与 getSystemService(String) 一起使用来查询 android.app.usage.StorageStatsManager 以访问系统存储统计信息。
    pub const STORAGE_STATS_SERVICE: &'static str = "storagestats";

    /// 与 getSystemService(String) 一起使用来查询 com.android.server.WallpaperService 以访问壁纸。
    pub const WALLPAPER_SERVICE: &'static str = "wallpaper";

    /// 与 getSystemService(String) 一起使用来查询 android.os.VibratorManager，以访问设备振动器、与单个振动器交互以及在多个振动器上播放同步效果。
    pub const VIBRATOR_MANAGER_SERVICE: &'static str = "vibrator_manager";

    /// 与 getSystemService(String) 一起使用来查询 android.os.Vibrator 以便与振动硬件进行交互。
    #[deprecated(note = "使用 android.os.VibratorManager 来查询默认系统振动器。")]
    pub const VIBRATOR_SERVICE: &'static str = "vibrator";

    /// 与 getSystemService(String) 一起使用来查询 android.app.StatusBarManager，以便与状态栏和快速设置进行交互。
    pub const STATUS_BAR_SERVICE: &'static str = "statusbar";

    /// 与 getSystemService(String) 一起使用来查询 android.net.ConnectivityManager 来处理网络连接管理。
    pub const CONNECTIVITY_SERVICE: &'static str = "connectivity";

    /// 与 getSystemService(String) 一起使用来查询 android.net.PacProxyManager，以处理 pac 代理信息的管理。
    pub const PAC_PROXY_SERVICE: &'static str = "pac_proxy";

    /// 与 getSystemService(String) 一起使用来查询用于管理虚拟运营商网络的 android.net.vcn.VcnManager
    pub const VCN_MANAGEMENT_SERVICE: &'static str = "vcn_management";

    //noinspection SpellCheckingInspection
    /// 与 getSystemService(String) 一起使用来查询 android.net.INetd 以便与网络堆栈进行通信
    pub const NETD_SERVICE: &'static str = "netd";

    /// 与 android.os.ServiceManager.getService() 一起使用来查询 INetworkStackConnector IBinder 以便与网络堆栈进行通信
    pub const NETWORK_STACK_SERVICE: &'static str = "network_stack";

    /// 与 getSystemService(String) 一起使用来查询 android.net.TetheringManager 来管理网络共享功能。
    pub const TETHERING_SERVICE: &'static str = "tethering";

    /// 与 getSystemService(String) 一起使用来查询 android.net.IpSecManager，以便使用 IPSec 加密套接字或网络。
    pub const IPSEC_SERVICE: &'static str = "ipsec";

    /// 与GetSystemService(String)一起查询android.net.VpnManager来管理平台内置VPN的配置文件。
    pub const VPN_MANAGEMENT_SERVICE: &'static str = "vpn_management";

    /// 与 getSystemService(String) 一起使用来查询 android.net.ConnectivityDiagnosticsManager，以执行网络连接诊断以及从系统接收网络连接信息。
    pub const CONNECTIVITY_DIAGNOSTICS_SERVICE: &'static str = "connectivity_diagnostics";

    /// 与 getSystemService(String) 一起使用来查询 android.net.TestNetworkManager 以构建 TUN 和有限使用网络
    pub const TEST_NETWORK_SERVICE: &'static str = "test_network";

    //noinspection SpellCheckingInspection
    /// 与 getSystemService(String) 一起使用来查询 android.os.IUpdateLock，用于管理不得被无头 OTA 应用程序或类似应用程序中断的运行时序列。
    pub const UPDATE_LOCK_SERVICE: &'static str = "updatelock";

    //noinspection SpellCheckingInspection
    /// 对于内部网络管理服务来说是常量，并不是真正的上下文服务。
    pub const NETWORKMANAGEMENT_SERVICE: &'static str = "network_management";

    /// 与 getSystemService(String) 一起使用来查询用于管理切片的 com.android.server.slice.SliceManagerService。
    pub const SLICE_SERVICE: &'static str = "slice";

    //noinspection SpellCheckingInspection
    /// 与GetSystemService(String)一起查询android.app.usage.NetworkStatsManager进行查询网络使用统计信息。
    pub const NETWORK_STATS_SERVICE: &'static str = "netstats";

    //noinspection SpellCheckingInspection
    #[doc(hidden)]
    pub const NETWORK_POLICY_SERVICE: &'static str = "netpolicy";

    #[doc(hidden)]
    pub const NETWORK_WATCHLIST_SERVICE: &'static str = "network_watchlist";

    /// 与 getSystemService(String) 一起使用来查询 android.net.wifi.WifiManager 来处理 Wi-Fi 访问的管理。
    pub const WIFI_SERVICE: &'static str = "wifi";

    //noinspection SpellCheckingInspection
    /**
    与 getSystemService(String) 一起使用来查询 android.net.wifi.wificond.WifiNl80211Manager，用于处理 Wi-Fi nl802.11 守护进程（wificond）的管理。
    @see android.net.wifi.wificond.WifiNl80211Manager
    */
    pub const WIFI_NL80211_SERVICE: &'static str = "wifinl80211";

    //noinspection SpellCheckingInspection
    /**
    与 getSystemService(String) 一起使用来查询 android.net.wifi.p2p.WifiP2pManager 来处理 Wi-Fi 对等连接的管理。
    @see android.net.wifi.p2p.WifiP2pManager
    */
    pub const WIFI_P2P_SERVICE: &'static str = "wifip2p";

    //noinspection SpellCheckingInspection
    /// 与 getSystemService(String) 一起使用来查询 android.net.wifi.aware.WifiAwareManager 来处理 Wi-Fi Aware 的管理。
    pub const WIFI_AWARE_SERVICE: &'static str = "wifiaware";

    //noinspection SpellCheckingInspection
    /// 与 getSystemService(String) 一起使用来查询 android.net.wifi.WifiScanner 以扫描 wifi 世界
    pub const WIFI_SCANNING_SERVICE: &'static str = "wifiscanner";

    //noinspection SpellCheckingInspection
    /// 与 getSystemService(String) 一起使用来查询用于测距 wifi 设备的 android.net.wifi.RttManager
    #[deprecated]
    pub const WIFI_RTT_SERVICE: &'static str = "rttmanager";

    //noinspection SpellCheckingInspection
    /// 与 getSystemService(String) 一起使用来查询用于测量带有 wifi 的设备范围的 android.net.wifi.rtt.WifiRttManager。
    pub const WIFI_RTT_RANGING_SERVICE: &'static str = "wifirtt";

    //noinspection SpellCheckingInspection
    /// 与 getSystemService(String) 一起使用来查询 android.net.lowpan.LowpanManager，以处理 LoWPAN 访问的管理。
    pub const LOWPAN_SERVICE: &'static str = "lowpan";

    /// 与 getSystemService(String) 一起使用来查询 android.net.EthernetManager 来处理以太网访问的管理。
    pub const ETHERNET_SERVICE: &'static str = "ethernet";

    //noinspection SpellCheckingInspection
    /// 与 getSystemService(String) 一起使用来查询 android.net.nsd.NsdManager，用于处理网络服务发现的管理
    pub const NSD_SERVICE: &'static str = "servicediscovery";

    /// 与 getSystemService(String) 一起使用来查询 android.media.AudioManager，用于处理音量、铃声模式和音频路由的管理。
    pub const AUDIO_SERVICE: &'static str = "audio";

    /// 与 getSystemService(String) 一起使用来查询 android.media.AudioDeviceVolumeManager，以处理音频设备（例如扬声器、USB 耳机）音量的管理。
    pub const AUDIO_DEVICE_VOLUME_SERVICE: &'static str = "audio_device_volume";

    /// 与 getSystemService(String) 一起使用来查询 android.media.MediaTranscodingManager 以进行媒体转码。
    pub const MEDIA_TRANSCODING_SERVICE: &'static str = "media_transcoding";

    /// AuthService 负责协调生物特征识别和 PIN/图案/密码身份验证。BiometricService 被拆分为两个服务，AuthService 和 BiometricService，其中 AuthService 是协调所有类型身份验证的高级服务，而 BiometricService 是仅负责生物特征身份验证的下层服务。
    /// 理想情况下，我们应该将 BiometricManager 重命名为 AuthManager，因为它在逻辑上与 AuthService 相对应。但是，由于 BiometricManager 是一个公共 API，我们保留了旧名称，但更改了内部实现以使用 AuthService。
    /// 截至目前，AUTH_SERVICE 常量仅用于在 SystemServiceRegistry 和 SELinux 中标识服务。要获取 AUTH_SERVICE 的管理器，应使用 BIOMETRIC_SERVICE 和 getSystemService(String) 来检索 android.hardware.biometrics.BiometricManager 两个服务及其管理器的映射：
    /// Service | Manager
    /// AuthService BiometricManager
    /// BiometricService N/A
    pub const AUTH_SERVICE: &'static str = "auth";

    /// 与 getSystemService(String) 一起使用来查询 android.hardware.fingerprint.FingerprintManager 来处理指纹管理。
    pub const FINGERPRINT_SERVICE: &'static str = "fingerprint";

    /// 与 getSystemService(String) 一起使用来查询 android.hardware.face.FaceManager 来处理面部身份验证的管理。
    pub const FACE_SERVICE: &'static str = "face";

    /// 与 getSystemService(String) 一起使用来查询 android.hardware.iris.IrisManager，用于处理虹膜认证的管理。
    pub const IRIS_SERVICE: &'static str = "iris";

    /// 与 getSystemService(String) 一起使用来查询 android.hardware.biometrics.BiometricManager，用于处理生物识别和 PIN/模式/密码身份验证。
    pub const BIOMETRIC_SERVICE: &'static str = "biometric";

    /// 与 getSystemService(String) 一起使用来查询用于管理 android.media.MediaSession2 的 android.media.MediaCommunicationManager。
    pub const MEDIA_COMMUNICATION_SERVICE: &'static str = "media_communication";

    /// 与getSystemService一起查询android.media.MediaRouter来控制和管理媒体路由。
    pub const MEDIA_ROUTER_SERVICE: &'static str = "media_router";

    /// 与 getSystemService(String) 一起使用来查询 android.media.session.MediaSessionManager 来管理媒体会话。
    pub const MEDIA_SESSION_SERVICE: &'static str = "media_session";

    /// 与 getSystemService(String) 一起使用来查询 android.telephony.TelephonyManager，用于处理设备的电话功能管理。
    pub const TELEPHONY_SERVICE: &'static str = "phone";

    /// 与 getSystemService(String) 一起使用来查询 android.telephony.SubscriptionManager，以处理设备的电话订阅管理。
    pub const TELEPHONY_SUBSCRIPTION_SERVICE: &'static str = "telephony_subscription_service";

    /// 与 getSystemService(String) 一起使用来查询 android.telecom.TelecomManager 来管理设备的电信相关功能。
    pub const TELECOM_SERVICE: &'static str = "telecom";

    /// 与 getSystemService(String) 一起使用来查询 android.telephony.CarrierConfigManager，以读取运营商配置值。
    pub const CARRIER_CONFIG_SERVICE: &'static str = "carrier_config";

    //noinspection SpellCheckingInspection
    /// 与 getSystemService(String) 一起使用来查询 android.telephony.euicc.EuiccManager 来管理设备 eUICC（嵌入式 SIM）。
    pub const EUICC_SERVICE: &'static str = "euicc";

    //noinspection SpellCheckingInspection
    /// 与GetSystemService(String)一起查询android.telephony.euicc.EuiccCardManager访问设备EUICC（嵌入式SIM）。
    pub const EUICC_CARD_SERVICE: &'static str = "euicc_card";

    /// 与 getSystemService(String) 一起使用来查询 android.telephony.MmsManager 来发送/接收 MMS 消息。
    pub const MMS_SERVICE: &'static str = "mms";

    /// 与 getSystemService(String) 一起使用来查询 android.content.ClipboardManager，以访问和修改全局剪贴板的内容。
    pub const CLIPBOARD_SERVICE: &'static str = "clipboard";

    //noinspection SpellCheckingInspection
    /// 与 getSystemService(String) 一起使用来查询用于文本分类服务的 TextClassificationManager。
    pub const TEXT_CLASSIFICATION_SERVICE: &'static str = "textclassification";

    //noinspection SpellCheckingInspection
    /// 与GetSystemService(String)一起查询android.view.selectiontoolbar.SelectionToolBarmanager进行选择工具栏服务。
    pub const SELECTION_TOOLBAR_SERVICE: &'static str = "selection_toolbar";

    /// 与 getSystemService(String) 一起使用来查询字体服务的 android.graphics.fonts.FontManager。
    pub const FONT_SERVICE: &'static str = "font";

    /// 与 getSystemService(String) 一起使用来查询 com.android.server.attention.AttentionManagerService 以获得注意服务。
    pub const ATTENTION_SERVICE: &'static str = "attention";

    /**
    (内部) 旋转解析器服务的官方发布名称。
    // TODO(b/178151184): 释放前，将其更改为旋转解析器。
    */
    pub const ROTATION_RESOLVER_SERVICE: &'static str = "resolver";

    //noinspection SpellCheckingInspection
    /// 与 getSystemService(String) 一起使用来查询 android.view.inputmethod.InputMethodManager 来访问输入法。
    pub const INPUT_METHOD_SERVICE: &'static str = "input_method";

    //noinspection SpellCheckingInspection
    /// 与 getSystemService(String) 一起使用来查询 android.view.textservice.TextServicesManager 来访问文本服务。
    pub const TEXT_SERVICES_MANAGER_SERVICE: &'static str = "textservices";

    //noinspection SpellCheckingInspection
    /// 与 getSystemService(String) 一起使用来查询 android.appwidget.AppWidgetManager 来访问 AppWidgets。
    pub const APPWIDGET_SERVICE: &'static str = "appwidget";

    //noinspection SpellCheckingInspection
    /// (内部)语音交互管理器服务的官方发布名称。
    pub const VOICE_INTERACTION_MANAGER_SERVICE: &'static str = "voiceinteraction";

    /// （内部）自动填充服务的官方出版名称。
    pub const AUTOFILL_MANAGER_SERVICE: &'static str = "autofill";

    //noinspection SpellCheckingInspection
    /// （内部）文本到语音管理服务的官方发布名称。
    pub const TEXT_TO_SPEECH_MANAGER_SERVICE: &'static str = "texttospeech";

    /// 内容捕获服务的官方发布名称。
    pub const CONTENT_CAPTURE_MANAGER_SERVICE: &'static str = "content_capture";

    /// 翻译服务的官方出版名称。
    pub const TRANSLATION_MANAGER_SERVICE: &'static str = "translation";

    /// 支持ui翻译功能的翻译服务的官方发布名称。
    pub const UI_TRANSLATION_SERVICE: &'static str = "ui_translation";

    /// 用于获取任务快照的内容选择和分类。
    pub const CONTENT_SUGGESTIONS_SERVICE: &'static str = "content_suggestions";

    /**
    应用程序预测服务的官方发布名称。
    注意：此服务是可选的；“ ”的调用者。
    */
    pub const APP_PREDICTION_SERVICE: &'static str = "app_prediction";

    /**
    搜索 UI 服务的官方发布名称。
    注意：此服务是可选的；“ ”的调用者。
    */
    pub const SEARCH_UI_SERVICE: &'static str = "search_ui";

    //noinspection SpellCheckingInspection
    /**
    用于获取智能空间服务。
    注意：此服务是可选的；“ ”的调用者。
    */
    pub const SMARTSPACE_SERVICE: &'static str = "smartspace";

    /**
    用于获取云搜索服务。
    注意：此服务是可选的；“ ”的调用者。
    */
    pub const CLOUDSEARCH_SERVICE: &'static str = "cloudsearch";

    //noinspection SpellCheckingInspection
    /// 与 getSystemService(String) 一起使用来访问 com.android.server.voiceinteraction.SoundTriggerService。
    pub const SOUND_TRIGGER_SERVICE: &'static str = "soundtrigger";

    //noinspection SpellCheckingInspection
    /// 与 getSystemService(String) 一起使用来访问 com.android.server.soundtrigger_middleware.SoundTriggerMiddlewareService。
    pub const SOUND_TRIGGER_MIDDLEWARE_SERVICE: &'static str = "soundtrigger_middleware";

    /**
    用于获取壁纸效果生成服务。
    注意：此服务是可选的；“ ”的调用者。
    */
    pub const WALLPAPER_EFFECTS_GENERATION_SERVICE: &'static str = "wallpaper_effects_generation";

    /// 用于访问MusicRecognitionManagerService。
    pub const MUSIC_RECOGNITION_SERVICE: &'static str = "music_recognition";

    /// （内部）许可服务的官方出版名称。
    pub const PERMISSION_SERVICE: &'static str = "permission";

    /// 旧版（内部）权限服务的官方发布名称。
    //@SystemApi(client = SystemApi.Client.MODULE_LIBRARIES)
    pub const LEGACY_PERMISSION_SERVICE: &'static str = "legacy_permission";

    /// (内部)权限控制器服务的官方发布名称。
    pub const PERMISSION_CONTROLLER_SERVICE: &'static str = "permission_controller";

    /// (内部)权限检查服务的官方发布名称。
    pub const PERMISSION_CHECKER_SERVICE: &'static str = "permission_checker";

    /// (内部) 权限执行服务的官方发布名称。
    pub const PERMISSION_ENFORCER_SERVICE: &'static str = "permission_enforcer";

    //noinspection SpellCheckingInspection
    /// 与 getSystemService(String) 一起使用来查询 android.apphibernation.AppHibernationManager 以便与休眠服务进行通信。
    pub const APP_HIBERNATION_SERVICE: &'static str = "app_hibernation";

    /// 与 getSystemService(String) 一起使用来查询 android.app.backup.IBackupManager IBackupManager 以便与备份机制进行通信。
    pub const BACKUP_SERVICE: &'static str = "backup";

    /// 与 getSystemService(String) 一起使用来查询 android.content.rollback.RollbackManager 以便与回滚管理器进行通信
    pub const ROLLBACK_SERVICE: &'static str = "rollback";

    /// 与 getSystemService(String) 一起使用来查询 android.scheduling.RebootReadinessManager，以便与重启准备就绪检测器进行通信。
    pub const REBOOT_READINESS_SERVICE: &'static str = "reboot_readiness";

    /// 与 getSystemService(String) 一起使用来查询用于记录诊断日志的 android.os.DropBoxManager 实例。
    pub const DROPBOX_SERVICE: &'static str = "dropbox";

    /// BackgroundInstallControlService 的系统服务名称。此服务负责监督设备上的 MBA 并提供 MBA 的相关元数据。
    pub const BACKGROUND_INSTALL_CONTROL_SERVICE: &'static str = "background_install_control";

    /// BinaryTransparencyService 的系统服务名称。它用于查询与设备上各种预安装和系统二进制文件有关的测量值，以便为用户提供透明度。
    pub const BINARY_TRANSPARENCY_SERVICE: &'static str = "transparency";

    //noinspection SpellCheckingInspection
    /// DeviceIdleManager 的系统服务名称。
    pub const DEVICE_IDLE_CONTROLLER: &'static str = "deviceidle";

    /// PowerWhitelistManager 的系统服务名称。
    #[deprecated]
    pub const POWER_WHITELIST_MANAGER: &'static str = "power_whitelist";

    /// PowerExemptionManager 的系统服务名称。
    pub const POWER_EXEMPTION_SERVICE: &'static str = "power_exemption";

    /// 与 getSystemService(String) 一起使用来查询 android.app.admin.DevicePolicyManager 来进行全局设备策略管理。
    pub const DEVICE_POLICY_SERVICE: &'static str = "device_policy";

    //noinspection SpellCheckingInspection
    /// 与 getSystemService(String) 一起使用来查询用于控制 UI 模式的 android.app.UiModeManager。
    pub const UI_MODE_SERVICE: &'static str = "uimode";

    /// 与 getSystemService(String) 一起使用来查询 android.app.DownloadManager 以请求 HTTP 下载。
    pub const DOWNLOAD_SERVICE: &'static str = "download";

    //noinspection SpellCheckingInspection
    /// 与 getSystemService(String) 一起使用来查询 android.os.BatteryManager 来管理电池状态。
    pub const BATTERY_SERVICE: &'static str = "batterymanager";

    /// 与 getSystemService(String) 一起使用来查询 android.nfc.NfcManager 以使用 NFC。
    pub const NFC_SERVICE: &'static str = "nfc";

    /// 与 getSystemService(String) 一起使用来查询 android.bluetooth.BluetoothManager 以使用蓝牙。
    pub const BLUETOOTH_SERVICE: &'static str = "bluetooth";

    /// 与GetSystemService(String)一起查询android.net.sip.SipManager访问SIP相关服务。
    pub const SIP_SERVICE: &'static str = "sip";

    /// 与 getSystemService(String) 一起使用来查询 android.hardware.usb.UsbManager，以访问 USB 设备（作为 USB 主机）并控制此设备作为 USB 设备的行为。
    pub const USB_SERVICE: &'static str = "usb";

    /// 与 getSystemService 一起使用来查询 android.debug.AdbManager 以访问 ADB 调试功能。
    pub const ADB_SERVICE: &'static str = "adb";

    /// 与 getSystemService(String) 一起使用来查询 android.hardware.SerialManager 来访问串行端口。
    pub const SERIAL_SERVICE: &'static str = "serial";

    /// 与 getSystemService(String) 一起使用来查询 android.hardware.hdmi.HdmiControlManager 来控制和管理 HDMI-CEC 协议。
    pub const HDMI_CONTROL_SERVICE: &'static str = "hdmi_control";

    /// 与 getSystemService(String) 一起使用来查询 android.hardware.input.InputManager 以便与输入设备交互。
    pub const INPUT_SERVICE: &'static str = "input";

    /// 与 getSystemService(String) 一起使用来查询 android.hardware.display.DisplayManager 以便与显示设备交互。
    pub const DISPLAY_SERVICE: &'static str = "display";

    /// 与 getSystemService(String) 一起使用来查询 android.hardware.display.ColorDisplayManager 来控制颜色转换。
    pub const COLOR_DISPLAY_SERVICE: &'static str = "color_display";

    /// 与 getSystemService(String) 一起使用来查询 android.os.UserManager，以便在支持多用户的设备上管理用户。
    pub const USER_SERVICE: &'static str = "user";

    //noinspection SpellCheckingInspection
    /// 与 getSystemService(String) 一起使用来查询 android.content.pm.LauncherApps，以便查询和监控用户配置文件中可启动的应用程序。
    pub const LAUNCHER_APPS_SERVICE: &'static str = "launcherapps";

    /// 与 getSystemService(String) 一起使用来查询 android.content.RestrictionsManager，以查询应用程序限制并请求受限操作的权限。
    pub const RESTRICTIONS_SERVICE: &'static str = "restrictions";

    //noinspection SpellCheckingInspection
    /// 与GetSystemService(String)一起查询android.app.AppopsManager，以跟踪设备上的应用程序操作。
    pub const APP_OPS_SERVICE: &'static str = "appops";

    /// 与 getSystemService(String) 一起使用来查询用于管理角色的 android.app.role.RoleManager。
    pub const ROLE_SERVICE: &'static str = "role";

    /**
    与 getSystemService(String) 一起使用来查询 android.hardware.camera2.CameraManager 以便与相机设备交互。
    @see android.hardware.camera2.CameraManager
    */
    pub const CAMERA_SERVICE: &'static str = "camera";

    /// android.print.PrintManager用于打印和管理打印机和打印任务。
    pub const PRINT_SERVICE: &'static str = "print";

    //noinspection SpellCheckingInspection
    /// 与 getSystemService(String) 一起使用来查询 android.companion.CompanionDeviceManager 来管理配套设备
    pub const COMPANION_DEVICE_SERVICE: &'static str = "companiondevice";

    //noinspection SpellCheckingInspection
    /**
    与 getSystemService(String) 一起使用来查询用于管理虚拟设备的 android.companion.virtual.VirtualDeviceManager。
    在没有 PackageManager#FEATURE_COMPANION_DEVICE_SETUP 系统功能的设备上，getSystemService(String) 将返回“ ”.
    */
    pub const VIRTUAL_DEVICE_SERVICE: &'static str = "virtualdevice";

    /// 与 getSystemService(String) 一起使用来查询 android.hardware.ConsumerIrManager，以便从设备传输红外信号。
    pub const CONSUMER_IR_SERVICE: &'static str = "consumer_ir";

    /// android.app.trust.TrustManager 用于管理信任代理。
    pub const TRUST_SERVICE: &'static str = "trust";

    /// 与 getSystemService(String) 一起使用来查询 android.media.tv.interactive.TvInteractiveAppManager，以便与设备上的电视交互应用程序进行交互。
    pub const TV_INTERACTIVE_APP_SERVICE: &'static str = "tv_interactive_app";

    /// 与 getSystemService(String) 一起使用来查询 android.media.tv.TvInputManager，以便与设备上的电视输入进行交互。
    pub const TV_INPUT_SERVICE: &'static str = "tv_input";

    /// 与 getSystemService(String) 一起使用来查询 android.media.tv.TunerResourceManager，以便与设备上的电视调谐器资源进行交互。
    pub const TV_TUNER_RESOURCE_MGR_SERVICE: &'static str = "tv_tuner_resource_mgr";

    /// android.net.NetworkScoreManager 用于管理网络得分。
    #[deprecated(note = "请参阅 Wi-Fi 建议 API 来了解建议 WiFi 网络的替代 API。")]
    pub const NETWORK_SCORE_SERVICE: &'static str = "network_score";

    //noinspection SpellCheckingInspection
    /// 与 getSystemService(String) 一起使用来查询 android.app.usage.UsageStatsManager 来查询设备使用情况统计信息。
    pub const USAGE_STATS_SERVICE: &'static str = "usagestats";

    //noinspection SpellCheckingInspection
    /// 与GetSystemService(String)一起查询android.app.job.JobScheduler实例，以管理偶尔的背景任务。
    pub const JOB_SCHEDULER_SERVICE: &'static str = "jobscheduler";

    /// 与 getSystemService(String) 一起使用来查询 android.app.tare.EconomyManager 实例以了解经济状况。
    pub const RESOURCE_ECONOMY_SERVICE: &'static str = "tare";

    //noinspection SpellCheckingInspection
    /// 与GetSystemService(String)一起查询android.service.persistentdata.PersistentDatablockManager实例，以与跨工厂重置的存储设备进行交互。
    pub const PERSISTENT_DATA_BLOCK_SERVICE: &'static str = "persistent_data_block";

    //noinspection SpellCheckingInspection
    /// 与 getSystemService(String) 一起使用来查询用于管理 OEM 锁的 android.service.oemlock.OemLockManager 实例。
    pub const OEM_LOCK_SERVICE: &'static str = "oem_lock";

    /// 与 getSystemService(String) 一起使用来查询用于管理媒体投影会话的 android.media.projection.MediaProjectionManager 实例。
    pub const MEDIA_PROJECTION_SERVICE: &'static str = "media_projection";

    /// 与 getSystemService(String) 一起使用来查询 android.media.midi.MidiManager 来访问 MIDI 服务。
    pub const MIDI_SERVICE: &'static str = "midi";

    //noinspection SpellCheckingInspection
    /// 与 getSystemService(String) 一起使用来查询 android.hardware.radio.RadioManager 以访问广播电台服务。
    pub const RADIO_SERVICE: &'static str = "broadcastradio";

    /// 与 getSystemService(String) 一起使用来查询 android.os.HardwarePropertiesManager 来访问硬件属性服务。
    pub const HARDWARE_PROPERTIES_SERVICE: &'static str = "hardware_properties";

    //noinspection SpellCheckingInspection
    /// 与 getSystemService(String) 一起使用来查询 android.os.ThermalService 以访问热服务。
    pub const THERMAL_SERVICE: &'static str = "thermalservice";

    /// 与 getSystemService(String) 一起使用来查询 android.os.PerformanceHintManager 以访问性能提示服务。
    pub const PERFORMANCE_HINT_SERVICE: &'static str = "performance_hint";

    /// 与 getSystemService(String) 一起使用来查询 android.content.pm.ShortcutManager 以访问启动器快捷方式服务。
    pub const SHORTCUT_SERVICE: &'static str = "shortcut";

    //noinspection SpellCheckingInspection
    /// 与 getSystemService(String) 一起使用来查询 android.hardware.location.ContextHubManager 来访问上下文中心。
    pub const CONTEXTHUB_SERVICE: &'static str = "contexthub";

    //noinspection SpellCheckingInspection
    /// 与 getSystemService(String) 一起使用来查询 android.os.health.SystemHealthManager，以访问系统健康（电池、电源、内存等）指标。
    pub const SYSTEM_HEALTH_SERVICE: &'static str = "systemhealth";

    /// 守门人 服务。
    pub const GATEKEEPER_SERVICE: &'static str = "android.service.gatekeeper.IGateKeeperService";

    /// 定义访问设备标识符的策略的服务。
    pub const DEVICE_IDENTIFIERS_SERVICE: &'static str = "device_identifiers";

    /// 报告系统健康“事件”的服务
    pub const INCIDENT_SERVICE: &'static str = "incident";

    //noinspection SpellCheckingInspection
    /// 协助 incident 和 dumpstated 向用户报告状态并确认授权进行事件报告或错误报告的服务
    pub const INCIDENT_COMPANION_SERVICE: &'static str = "incidentcompanion";

    //noinspection SpellCheckingInspection
    /// 协助位于系统服务器中的 android.app.StatsManager 的服务。
    pub const STATS_MANAGER_SERVICE: &'static str = "statsmanager";

    //noinspection SpellCheckingInspection
    /// 协助 statsd 获取一般统计数据的服务。
    pub const STATS_COMPANION_SERVICE: &'static str = "statscompanion";

    //noinspection SpellCheckingInspection
    /// 协助 statsd 从引导原子记录原子的服务。
    pub const STATS_BOOTSTRAP_ATOM_SERVICE: &'static str = "statsbootstrap";

    /// 与 getSystemService(String) 一起使用来查询 android.app.StatsManager。
    pub const STATS_MANAGER: &'static str = "stats";

    /// 与 android.os.ServiceManager.getService() 一起使用来查询 IPlatformCompat IBinder，以便与平台兼容服务进行通信。
    pub const PLATFORM_COMPAT_SERVICE: &'static str = "platform_compat";

    /// 与 android.os.ServiceManager.getService() 一起使用来查询与平台兼容服务通信的本机代码的 IPlatformCompatNative IBinder。
    pub const PLATFORM_COMPAT_NATIVE_SERVICE: &'static str = "platform_compat_native";

    /// 用于捕获错误报告的服务。
    pub const BUGREPORT_SERVICE: &'static str = "bugreport";

    //noinspection GrazieInspection
    /// 与 getSystemService(String) 一起使用来查询用于管理覆盖包的 android.content.om.OverlayManager。
    pub const OVERLAY_SERVICE: &'static str = "overlay";

    /// 与 getSystemService(String) 一起使用来管理资源。
    pub const RESOURCES_SERVICE: &'static str = "resources";

    //noinspection SpellCheckingInspection
    /// 与 getSystemService(String) 一起使用来查询 android.os.IIdmap2 来管理 idmap 文件（由覆盖包使用）。
    pub const IDMAP_SERVICE: &'static str = "idmap";

    //noinspection SpellCheckingInspection
    /// 与 getSystemService(String) 一起使用来查询用于访问 VR 服务的 VrManager。
    pub const VR_SERVICE: &'static str = "vrmanager";

    //noinspection SpellCheckingInspection
    /// 与 getSystemService(String) 一起使用来查询 android.content.pm.CrossProfileApps 以进行跨配置文件操作。
    pub const CROSS_PROFILE_APPS_SERVICE: &'static str = "crossprofileapps";

    //noinspection SpellCheckingInspection
    /// 与 getSystemService 一起使用来查询 android.se.omapi.ISecureElementService 以访问 SecureElementService。
    pub const SECURE_ELEMENT_SERVICE: &'static str = "secure_element";

    //noinspection SpellCheckingInspection
    /// 与 getSystemService(String) 一起使用来查询 android.app.timedetector.TimeDetector。
    pub const TIME_DETECTOR_SERVICE: &'static str = "time_detector";

    //noinspection SpellCheckingInspection
    /// 与 getSystemService(String) 一起使用来查询 android.app.timezonedetector.TimeZoneDetector。
    pub const TIME_ZONE_DETECTOR_SERVICE: &'static str = "time_zone_detector";

    /// 与 getSystemService(String) 一起使用来查询 TimeManager。
    pub const TIME_MANAGER_SERVICE: &'static str = "time_manager";

    /// AppBindingService 的 Binder 服务名称。
    pub const APP_BINDING_SERVICE: &'static str = "app_binding";

    /// 与 getSystemService(String) 一起使用来查询 android.telephony.ims.ImsManager。
    pub const TELEPHONY_IMS_SERVICE: &'static str = "telephony_ims";

    /// 与 getSystemService(String) 一起使用来查询 android.os.SystemConfigManager。
    pub const SYSTEM_CONFIG_SERVICE: &'static str = "system_config";

    //noinspection SpellCheckingInspection
    /// 与 getSystemService(String) 一起使用来查询 android.telephony.ims.RcsMessageManager。
    pub const TELEPHONY_RCS_MESSAGE_SERVICE: &'static str = "ircsmessage";

    /// 与 getSystemService(String) 一起使用来查询 android.os.image.DynamicSystemManager。
    pub const DYNAMIC_SYSTEM_SERVICE: &'static str = "dynamic_system";

    /// 与 getSystemService(String) 一起使用来查询 android.app.blob.BlobStoreManager，以便从系统维护的 Blob 存储中贡献和访问数据 Blob。
    pub const BLOB_STORE_SERVICE: &'static str = "blob_store";

    /// 与 getSystemService(String) 一起使用来查询 TelephonyRegistryManager。
    pub const TELEPHONY_REGISTRY_SERVICE: &'static str = "telephony_registry";

    //noinspection SpellCheckingInspection
    /// 与 getSystemService(String) 一起使用来查询 android.os.BatteryStatsManager。
    pub const BATTERY_STATS_SERVICE: &'static str = "batterystats";

    //noinspection SpellCheckingInspection
    /// 与 getSystemService(String) 一起使用来查询 android.app.appsearch.AppSearchManager，以便索引和查询系统管理的应用程序数据。
    pub const APP_SEARCH_SERVICE: &'static str = "app_search";

    /// 与 getSystemService(String) 一起使用来查询 android.content.integrity.AppIntegrityManager。
    pub const APP_INTEGRITY_SERVICE: &'static str = "app_integrity";

    //noinspection SpellCheckingInspection
    /// 与 getSystemService(String) 一起使用来查询 android.content.pm.DataLoaderManager。
    pub const DATA_LOADER_MANAGER_SERVICE: &'static str = "dataloader_manager";

    /// 与 getSystemService(String) 一起使用来查询 android.os.incremental.IncrementalManager。
    pub const INCREMENTAL_SERVICE: &'static str = "incremental";

    //noinspection SpellCheckingInspection
    /// 与 getSystemService(String) 一起使用来查询 android.security.attestationverification.AttestationVerificationManager。
    pub const ATTESTATION_VERIFICATION_SERVICE: &'static str = "attestation_verification";

    /// 与 getSystemService(String) 一起使用来查询 android.security.FileIntegrityManager。
    pub const FILE_INTEGRITY_SERVICE: &'static str = "file_integrity";

    /// 用于远程密钥配置的绑定服务。
    pub const REMOTE_PROVISIONING_SERVICE: &'static str = "remote_provisioning";

    /// 与 getSystemService(String) 一起使用来查询用于控制设备灯的 android.hardware.lights.LightsManager。
    pub const LIGHTS_SERVICE: &'static str = "lights";

    /// 与 getSystemService(String) 一起使用来查询 android.uwb.UwbManager。
    pub const UWB_SERVICE: &'static str = "uwb";

    /// 与 getSystemService(String) 一起使用来查询用于控制梦境状态的 android.app.DreamManager。
    pub const DREAM_SERVICE: &'static str = "dream";

    /// 与 getSystemService(String) 一起使用来查询 android.telephony.SmsManager 来访问短信功能。
    pub const SMS_SERVICE: &'static str = "sms";

    /// 与 getSystemService(String) 一起使用来访问 PeopleManager 来与您发布的对话进行交互。
    pub const PEOPLE_SERVICE: &'static str = "people";

    /// 与 getSystemService(String) 一起使用来访问设备状态服务。
    pub const DEVICE_STATE_SERVICE: &'static str = "device_state";

    /// 与 getSystemService(String) 一起使用来查询 android.media.metrics.MediaMetricsManager，以便与设备上的媒体指标进行交互。
    pub const MEDIA_METRICS_SERVICE: &'static str = "media_metrics";

    /// 与 getSystemService(String) 一起使用来访问系统语音识别服务。
    pub const SPEECH_RECOGNITION_SERVICE: &'static str = "speech_recognition";

    /// 与 getSystemService(String) 一起使用来查询 GameManager。
    pub const GAME_SERVICE: &'static str = "game";

    /// 与 getSystemService(String) 一起使用来访问 android.content.pm.verify.domain.DomainVerificationManager 来查询已声明的 Web 域的批准和用户状态。
    pub const DOMAIN_VERIFICATION_SERVICE: &'static str = "domain_verification";

    //noinspection SpellCheckingInspection
    /// 与 getSystemService(String) 一起使用来访问 android.view.displayhash.DisplayHashManager 来处理显示哈希。
    pub const DISPLAY_HASH_SERVICE: &'static str = "display_hash";

    /// 与 getSystemService(String) 一起使用来查询 android.app.LocaleManager。
    pub const LOCALE_SERVICE: &'static str = "locale";

    //noinspection SpellCheckingInspection
    /// 与 getSystemService(String) 一起使用来查询 android.safetycenter.SafetyCenterManager 实例，以便与安全中心进行交互。
    pub const SAFETY_CENTER_SERVICE: &'static str = "safety_center";

    /// 与 getSystemService(String) 一起使用来查询 android.nearby.NearbyManager 来发现附近的设备。
    pub const NEARBY_SERVICE: &'static str = "nearby";

    //noinspection SpellCheckingInspection
    /// 与 getSystemService(String) 一起使用来查询 android.app.ambientcontext.AmbientContextManager。
    pub const AMBIENT_CONTEXT_SERVICE: &'static str = "ambient_context";

    /// 与 getSystemService(String) 一起使用来查询 android.app.wearable.WearableSensingManager。
    pub const WEARABLE_SENSING_SERVICE: &'static str = "wearable_sensing";

    //noinspection SpellCheckingInspection
    /// 与GetSystemService(String)一起查询android.health.connect.HealthConnectManager。
    pub const HEALTHCONNECT_SERVICE: &'static str = "healthconnect";

    /// 与 getSystemService(String) 一起使用来查询 android.credentials.CredentialManager 来向您的应用程序验证用户身份。
    pub const CREDENTIAL_SERVICE: &'static str = "credential";

    //noinspection SpellCheckingInspection
    /// 与 getSystemService(String) 一起使用来查询 android.devicelock.DeviceLockManager。
    pub const DEVICE_LOCK_SERVICE: &'static str = "device_lock";

    //noinspection SpellCheckingInspection
    /**
    与 getSystemService(String) 一起使用来查询 android.system.virtualmachine.VirtualMachineManager。
    在没有 PackageManager#FEATURE_VIRTUALIZATION_FRAMEWORK 系统功能的设备上，getSystemService(String) 将返回“ ”。
    */
    pub const VIRTUALIZATION_SERVICE: &'static str = "virtualization";

    /// 与 getSystemService(String) 一起使用来查询 GrammaticalInflectionManager。
    pub const GRAMMATICAL_INFLECTION_SERVICE: &'static str = "grammatical_inflection";

    /// 与 getSystemService(String) 一起使用来查询 android.telephony.satellite.SatelliteManager 来访问卫星功能。
    pub const SATELLITE_SERVICE: &'static str = "satellite";

    //noinspection SpellCheckingInspection
    /// 与 getSystemService(String) 一起使用来查询 android.net.wifi.sharedconnectivity.app.SharedConnectivityManager 来访问共享连接服务。
    pub const SHARED_CONNECTIVITY_SERVICE: &'static str = "shared_connectivity";

    /**
    返回可用于查询此包中的类的类加载器。
    */
    #[java_method]
    pub fn get_class_loader(&self) -> ClassLoader {}

    /**
    将给定的意图广播给所有感兴趣的 BroadcastReceiver。此调用是异步的；它会立即返回，您将在接收器运行时继续执行。接收器不会传播任何结果，接收器也无法中止广播。如果您想允许接收器传播结果或中止广播，则必须使用 sendOrderedBroadcast(Intent, String) 发送有序广播。有关 Intent 广播的更多信息，请参阅 BroadcastReceiver。
    `intent` 要广播的 Intent；与此 Intent 匹配的所有接收器都将收到广播。
    */
    #[java_method]
    pub fn send_broadcast(&self, intent: &Intent) {}

    /**
    与 startActivity(Intent, Bundle) 相同，但未指定任何选项。
    `intent` 要启动的活动的描述。
    抛出：ActivityNotFoundException – `
    */
    #[java_method]
    pub fn start_activity(&self, intent: &Intent) {}

    /**
    返回此应用程序包的名称。
    */
    #[java_method]
    pub fn get_package_name(&self) -> String {}

    /**
    返回此上下文所派生自的基础上下文的名称。这与 getOpPackageName() 相同，除非系统组件加载到其他应用进程中，在这种情况下 getOpPackageName() 将是该进程中主软件包的名称（以便应用操作 uid 验证可以使用该名称）。
    */
    #[java_method]
    pub fn get_base_package_name(&self) -> String {}

    /**
    返回应用于此上下文中的 android.app.AppOpsManager 调用的包名称，以便 app ops manager 的 uid 验证可以使用该名称。这通常不适用于第三方应用程序开发人员。
    */
    #[java_method]
    pub fn get_op_package_name(&self) -> String {}

    /**
    归因可用于复杂的应用中，以在逻辑上区分应用的各个部分。例如，博客应用可能还内置有即时通讯应用。在这种情况下，每个子功能可以使用两个单独的标签。
    返回：此上下文所针对的归因标签，如果这是默认标签，则返回 null。
    */
    #[java_method]
    pub fn get_attribution_tag(&self) -> Option<String> {}

    /**
    返回此上下文的主要 Android 包的完整路径。Android 包是一个 ZIP 文件，其中包含应用程序的主要资源。注意：这通常对应用程序没有用，因为它们不应直接访问文件系统。
    返回：String 资源路径。
    */
    #[java_method]
    pub fn get_package_resource_path(&self) -> String {}

    /**
    返回此上下文的主要 Android 包的完整路径。Android 包是一个 ZIP 文件，其中包含应用程序的主要代码和资产。
    注意：这通常对应用程序没有用，因为它们不应直接访问文件系统。
    返回：String 代码和资产的路径。
    */
    #[java_method]
    pub fn get_package_code_path(&self) -> String {}

    //noinspection SpellCheckingInspection
    /**
    按名称返回系统级服务的句柄。返回对象的类因请求的名称而异。当前可用的名称为：
    WINDOW_SERVICE ("window")
    您可以在顶级窗口管理器中放置自定义窗口。返回的对象是 WindowManager。只能从可视上下文（例如 Activity 或使用 createWindowContext(int, Bundle) 创建的上下文）中获取，这些上下文会根据屏幕区域的配置和可视边界进行调整。
    LAYOUT_INFLATER_SERVICE ("layout_inflater")
    android.view.LayoutInflater 用于在此上下文中填充布局资源。只能从可视上下文（例如 Activity 或使用 createWindowContext(int, Bundle) 创建的上下文）获取，这些上下文会根据屏幕区域的配置和可视边界进行调整。
    ACTIVITY_SERVICE ("activity")
    用于与系统全局活动状态交互的 ActivityManager。
    WALLPAPER_SERVICE ("wallpaper")
    android.service.wallpaper.WallpaperService 用于在此上下文中访问壁纸。只能从可视上下文（例如 Activity 或使用 createWindowContext(int, Bundle) 创建的上下文）获取，这些上下文会根据屏幕区域的配置和可视边界进行调整。
    POWER_SERVICE ("power")
    一个用于控制电源管理的android.os.PowerManager。
    ALARM_SERVICE ("alarm")
    一个 Android 应用程序 AlarmManager，用于在您选择的时间接收意图。
    NOTIFICATION_SERVICE ("notification")
    android.app.NotificationManager，用于通知用户后台事件。
    KEYGUARD_SERVICE ("keyguard")
    一个用于控制键盘锁的 android.app.KeyguardManager。
    LOCATION_SERVICE ("location")
    android.location.LocationManager 用于控制位置（例如 GPS）更新。
    SEARCH_SERVICE ("search")
    一个用于处理搜索的android.app.SearchManager。
    VIBRATOR_MANAGER_SERVICE ("vibrator_manager")
    android.os.VibratorManager，用于访问设备振动器、与各个振动器交互以及在多个振动器上播放同步效果。
    VIBRATOR_SERVICE ("vibrator")
    一个android.os.Vibrator，用于与振动器硬件进行交互。
    CONNECTIVITY_SERVICE ("connectivity")
    用于处理网络连接管理的 ConnectivityManager。
    IPSEC_SERVICE ("ipsec")
    IpSecManager 用于管理套接字和网络上的 IPSec。
    WIFI_SERVICE ("wifi")
    用于管理 Wi-Fi 连接的 WifiManager。在 Android 7 之前的版本中，应仅从应用上下文中获取它，而不能从任何其他派生上下文中获取它，以避免调用过程中出现内存泄漏。
    WIFI_AWARE_SERVICE ("wifiaware")
    WifiAwareManager 用于管理 Wi-Fi Aware 发现和连接。
    WIFI_P2P_SERVICE ("wifip2p")
    WifiP2pManager 用于管理 Wi-Fi Direct 连接。
    INPUT_METHOD_SERVICE ("input_method")
    用于管理输入方法的 InputMethodManager。
    UI_MODE_SERVICE ("uimode")
    一个用于控制 UI 模式的 android.app.UiModeManager。
    DOWNLOAD_SERVICE ("download")
    用于请求 HTTP 下载的 android.app.DownloadManager
    BATTERY_SERVICE ("batterymanager")
    用于管理电池状态的 android.os.BatteryManager
    JOB_SCHEDULER_SERVICE ("taskmanager")
    一个用于管理计划任务的android.app.job.JobScheduler
    NETWORK_STATS_SERVICE ("netstats")
    用于查询网络使用情况统计数据的 NetworkStatsManager。
    HARDWARE_PROPERTIES_SERVICE ("hardware_properties")
    用于访问硬件属性的 android.os.HardwarePropertiesManager。
    DOMAIN_VERIFICATION_SERVICE ("domain_verification")
    android.content.pm.verify.domain.DomainVerificationManager，用于访问 Web 域批准状态。
    DISPLAY_HASH_SERVICE ("display_hash")
    android.view.displayhash.DisplayHashManager 用于管理显示哈希。
    注意：通过此 API 获取的系统服务可能与获取它们的上下文密切相关。一般来说，不要在各种不同的上下文（活动、应用程序、服务、提供商等）之间共享服务对象。
    注意：PackageManager.isInstantApp() 返回 true 的免安装应用无法访问以下系统服务：DEVICE_POLICY_SERVICE、FINGERPRINT_SERVICE、KEYGUARD_SERVICE、SHORTCUT_SERVICE、USB_SERVICE、WALLPAPER_SERVICE、WIFI_P2P_SERVICE、WIFI_SERVICE、WIFI_AWARE_SERVICE。对于这些服务，此方法将返回 null。通常，如果您以免安装应用的形式运行，则应始终检查此方法的结果是否为 null。
    注意：在实现此方法时，请记住可以在较新的 Android 版本中添加新服务，因此如果您只是在寻找上面提到的明确名称，请确保在您无法识别该名称时返回 null — 如果您抛出 RuntimeException 异常，您的应用可能会在新的 Android 版本上中断。
    返回：如果名称不存在则返回 null。
    `name` 所需服务的名称。
    */
    #[java_method]
    pub fn get_system_service(&self, name: String) -> Option<Object> {}

    //noinspection SpellCheckingInspection
    #[doc(hidden)]
    #[deprecated(note = "使用 getSharedPreferencesPath(String)")]
    #[java_method]
    pub fn get_shared_prefs_file(&self, name: String) -> File {}

    /**
    将现有的共享首选项文件从给定的源存储上下文移动到此上下文。这通常用于升级后在存储位置之间迁移数据，例如移至设备保护的存储。
    返回：如果动作成功，或者如果在源上下文中不存在共享偏好，则为false。
    `source_context` 包含现有共享首选项的源上下文。
    `name` 共享首选项文件的名称。
    */
    #[java_method]
    pub fn move_shared_preferences_from(&self, source_context: &Self, name: String) -> bool {}

    /**
    删除现有的共享首选项文件。
    返回：如果共享首选项文件已成功删除，则返回 true；否则返回 false。
    `name` 共享首选项文件的名称（在应用程序包中是唯一的）。
    */
    #[java_method]
    pub fn delete_shared_preferences(&self, name: String) -> bool {}

    /**
    删除与此 Context 的应用程序包关联的给定私有文件。
    返回：如果文件已成功删除，则返回 true；否则返回 false。
    `name` 要删除的文件的名称；不能包含路径分隔符。
    */
    #[java_method]
    pub fn delete_file(&self, name: String) -> bool {}

    /**
    返回文件系统中存储使用 openFileOutput 创建的文件的绝对路径。如果调用应用程序移动到采用的存储设备，则返回的路径可能会随时间而变化，因此只应保留相对路径。
    返回：给定文件的绝对路径。
    `name` 您想要获取其路径的文件的名称。
    */
    #[java_method]
    pub fn get_file_stream_path(&self, name: String) -> File {}

    /**
    返回文件系统中存储使用 getSharedPreferences(String, int) 创建的文件的绝对路径。如果调用应用程序移动到采用的存储设备，则返回的路径可能会随时间而变化，因此只应保留相对路径。
    返回：给定文件的绝对路径。
    `name` 您想要获取其路径的共享首选项的名称。
    */
    #[java_method]
    pub fn get_shared_preferences_path(&self, name: String) -> File {}

    /**
    返回文件系统中存储此应用所有私有文件的目录的绝对路径。应用不应直接使用此路径；而应使用 getFilesDir()、getCacheDir()、getDir(String, int) 或此类上的其他存储 API。
    如果调用应用移动到已采用的存储设备，则返回的路径可能会随时间而变化，因此只应保留相对路径。调用应用无需额外权限即可读取或写入返回路径下的文件。
    */
    #[java_method]
    pub fn get_data_dir(&self) -> File {}

    /**
    返回文件系统中存储使用 openFileOutput 创建的文件的目录的绝对路径。如果调用应用程序移动到采用的存储设备，则返回的路径可能会随时间而变化，因此只应保留相对路径。
    调用应用程序无需额外权限即可读取或写入返回路径下的文件。保存应用程序文件的目录的路径。
    */
    #[java_method]
    pub fn get_files_dir(&self) -> File {}

    /**
    返回与文件系统上的 crate 相关的目录的绝对路径。`crate_id` 需要经过验证的文件名。它不能包含任何“..”、"."、File.separatorChar 等。
    如果调用应用程序移动到采用的存储设备，则返回的路径可能会随时间而变化，因此只应保留相对路径。调用应用程序无需额外权限即可读取或写入返回路径下的文件。
    返回：crate 目录文件。
    `crate_id` getDataDir()/ crates 下的经过验证的相对文件名
    */
    #[java_method]
    pub fn get_crate_dir(&self, crate_id: String) -> File {}

    /**
    返回文件系统上目录的绝对路径，类似于 getFilesDir()。不同之处在于，放置在此目录下的文件将被排除在自动备份到远程存储之外。
    有关 Android 中自动备份机制的完整讨论，请参阅 BackupAgent。如果调用应用程序移动到采用的存储设备，则返回的路径可能会随时间而变化，因此只应保留相对路径。
    调用应用程序无需额外权限即可读取或写入返回路径下的文件。
    返回：包含不会自动备份到远程存储的应用程序文件的目录的路径。
    */
    #[java_method]
    pub fn get_no_backup_files_dir(&self) -> File {}

    /**
    返回主共享/外部存储设备上目录的绝对路径，应用程序可在该目录中放置其拥有的持久文件。这些文件是应用程序内部的，通常不作为媒体对用户可见。
    这与 getFilesDir() 类似，因为这些文件将在应用程序卸载时被删除，但存在一些重要区别：共享存储可能并非始终可用，因为用户可以弹出可移动媒体。可以使用 Environment.getExternalStorageState(File) 检查媒体状态。
    这些文件没有强制执行安全性。例如，任何拥有 android.Manifest.permission.WRITE_EXTERNAL_STORAGE 的应用程序都可以写入这些文件。
    如果共享存储设备是模拟的（由 Environment.isExternalStorageEmulated(File) 确定），则其内容由私有用户数据分区支持，这意味着将数据存储在此处而不是 getFilesDir() 返回的私有目录中几乎没有好处。
    从 Build.VERSION_CODES.KITKAT开始，无需任何权限即可读取或写入返回的路径；调用应用程序始终可以访问它。这仅适用于为调用应用程序的软件包名称生成的路径。
    要访问属于其他软件包的路径，需要 android.Manifest.permission.WRITE_EXTERNAL_STORAGE 和/或 android.Manifest.permission.READ_EXTERNAL_STORAGE。
    在具有多个用户的设备上（如 UserManager 所述），每个用户都有自己独立的共享存储。应用程序只能访问他们以该用户身份运行的共享存储。
    如果插入了不同的共享存储介质，则返回的路径可能会随时间而变化，因此只应保留相对路径。以下是操作应用程序共享存储中的文件的典型代码示例：
    @sample development/samples/ApiDemos/src/com/example/android/apis/content/ExternalStorage.java private_file
    如果您为此函数提供非空类型，则返回的文件将是指向给定类型的子目录的路径。虽然媒体扫描仪不会自动扫描这些文件，但您可以使用 MediaScannerConnection.scanFile 将它们明确添加到媒体数据库。
    请注意，这与 Environment.getExternalStoragePublicDirectory() 不同，后者提供所有应用程序共享的媒体目录。此处返回的目录归应用程序所有，卸载应用程序时将删除其内容。
    与 Environment.getExternalStoragePublicDirectory() 不同，此处返回的目录将自动为您创建。以下是操作应用程序共享存储中的图片并将其添加到媒体数据库的典型代码示例：
    @sample development/samples/ApiDemos/src/com/example/android/apis/content/ExternalStorage.java private_picture
    返回：应用程序特定目录的绝对路径。如果共享存储当前不可用，则可能返回 null。
    `type` 要返回的文件目录的类型。对于文件目录的根目录，可能为 null；对于子目录，可能为以下常量之一：
    Environment.DIRECTORY_MUSIC、Environment.DIRECTORY_PODCASTS、Environment.DIRECTORY_RINGTONES、Environment.DIRECTORY_ALARMS、Environment.DIRECTORY_NOTIFICATIONS、Environment.DIRECTORY_PICTURES 或 Environment.DIRECTORY_MOVIES。
    */
    #[java_method]
    pub fn get_external_files_dir(&self, r#type: Option<String>) -> Option<File> {}

    /**
    返回可找到此应用程序的 OBB 文件（如果有）的主要共享/外部存储目录。请注意，如果应用程序没有任何 OBB 文件，则此目录可能不存在。
    这与 getFilesDir() 类似，当应用程序卸载时这些文件将被删除，但也存在一些重要的区别：
    共享存储可能并不总是可用，因为用户可以弹出可移动媒体。可以使用 Environment.getExternalStorageState(File) 检查媒体状态。
    这些文件没有强制实施安全性。例如，任何持有 android.Manifest.permission.WRITE_EXTERNAL_STORAGE 的应用程序都可以写入这些文件。
    从 Build.VERSION_CODES.KITKAT 开始，无需任何权限即可读取或写入此方法返回的路径。但是，从 Build.VERSION_CODES.M 开始，要读取 OBB 扩展文件，您必须在应用清单中声明 android.Manifest.permission.READ_EXTERNAL_STORAGE 权限，并在运行时请求权限，如下所示：
    &lt;uses-permission android:name="android.permission.READ_EXTERNAL_STORAGE" android:maxSdkVersion="23" /&gt;
    从 Build.VERSION_CODES.N 开始，android.Manifest.permission.READ_EXTERNAL_STORAGE 权限不再需要，因此不要在运行时请求此权限。要处理这两种情况，您的应用必须先尝试读取 OBB 文件，如果失败，您必须在运行时请求 android.Manifest.permission.READ_EXTERNAL_STORAGE 权限。以下代码片段显示了如何执行此操作：
    File obb = new File(obb_filename);
    boolean open_failed = false;
    try {
        BufferedReader br = new BufferedReader(new FileReader(obb));
        open_failed = false;
        ReadObbFile(br);
    } catch (IOException e) {
        open_failed = true;
    }
    if (open_failed) {
        // 读取 OBB 文件之前请求 READ_EXTERNAL_STORAGE 权限
        ReadObbFileWithPermission();
    }
    在具有多个用户的设备上（如 UserManager 所述），多个用户可能共享同一个 OBB 存储位置。应用程序应确保在不同用户下运行的多个实例不会互相干扰。
    返回：应用程序特定目录的绝对路径。如果共享存储当前不可用，则可能返回 null。
    */
    #[java_method]
    pub fn get_obb_dir(&self) -> Option<File> {}

    /**
    返回文件系统上应用程序特定缓存目录的绝对路径。系统将自动删除此目录中的文件，因为设备上的其他地方需要磁盘空间。
    系统将始终首先删除较旧的文件，如 File.lastModified() 所报告的。如果需要，您可以使用 StorageManager.setCacheBehaviorGroup(File, boolean) 和 StorageManager.setCacheBehaviorTombstone(File, boolean) 对文件的删除方式进行更多控制。
    强烈建议应用程序将其缓存空间使用量保持在 StorageManager.getCacheQuotaBytes(java.util.UUID) 返回的配额以下。如果您的应用程序超出此配额，则在需要额外磁盘空间时，您的缓存文件将是第一批被删除的文件。
    相反，如果您的应用程序保持在此配额以下，则在需要额外磁盘空间时，您的缓存文件将是最后被删除的文件。请注意，您的缓存配额将随着时间的推移而变化，具体取决于用户与您的应用程序交互的频率以及系统范围磁盘空间的使用量。
    如果调用应用程序移至已采用的存储设备，则返回的路径可能会随时间而变化，因此只应保留相对路径。应用程序不需要额外的权限来读取或写入返回的路径，因为此路径位于其私有存储中。
    返回：保存应用程序缓存文件的目录的路径。
    */
    #[java_method]
    pub fn get_cache_dir(&self) -> File {}

    /**
    返回文件系统上用于存储缓存代码的应用程序特定缓存目录的绝对路径。当您的特定应用程序升级时以及整个平台升级时，系统都会删除存储在此位置的所有文件。
    此位置最适合存储应用程序在运行时生成的编译或优化代码。如果调用应用程序移动到采用的存储设备，则返回的路径可能会随时间而变化，因此只应保留相对路径。
    应用程序不需要额外的权限来读取或写入返回的路径，因为此路径位于其私有存储中。
    返回：保存应用程序代码缓存文件的目录的路径。
    */
    #[java_method]
    pub fn get_code_cache_dir(&self) -> File {}

    /**
    返回主共享/外部存储设备上应用程序特定目录的绝对路径，应用程序可将其拥有的缓存文件放置在该目录中。这些文件是应用程序内部的，通常不会作为媒体对用户可见。
    这与 getCacheDir() 类似，因为这些文件将在应用程序卸载时被删除，但存在一些重要区别：平台并不总是监控共享存储中的可用空间，因此可能不会自动删除这些文件。应用程序应始终管理此位置使用的最大空间。
    目前，平台只会在 Build.VERSION_CODES.JELLY_BEAN_MR1 或更高版本上运行且 Environment.isExternalStorageEmulated(File) 返回 true 时删除此处的文件。共享存储可能并非始终可用，因为用户可以弹出可移动媒体。
    可以使用 Environment.getExternalStorageState(File) 检查媒体状态。这些文件没有强制执行安全性。例如，任何拥有 android.Manifest.permission.WRITE_EXTERNAL_STORAGE的应用程序可以写入这些文件。
    如果共享存储设备是模拟的（由 Environment.isExternalStorageEmulated(File) 确定），则其内容由私有用户数据分区支持，这意味着将数据存储在此处而不是 getCacheDir() 返回的私有目录中几乎没有好处。
    从 Build.VERSION_CODES.KITKAT 开始，无需任何权限即可读取或写入返回的路径；调用应用程序始终可以访问它。这仅适用于为调用应用程序的软件包名称生成的路径。
    要访问属于其他软件包的路径，需要 android.Manifest.permission.WRITE_EXTERNAL_STORAGE 和/或 android.Manifest.permission.READ_EXTERNAL_STORAGE。在具有多个用户的设备上（如 UserManager 所述），每个用户都有自己独立的共享存储。
    应用程序只能访问他们以该用户身份运行的共享存储。如果插入不同的共享存储介质，则返回的路径可能会随时间而变化，因此只应保留相对路径。
    返回：应用程序特定目录的绝对路径。如果共享存储当前不可用，则可能返回 null。
    */
    #[java_method]
    pub fn get_external_cache_dir(&self) -> Option<File> {}

    /**
    返回预加载缓存中应用程序特定目录的绝对路径。当设备存储空间不足时，可以删除存储在缓存目录中的文件。无法保证何时删除这些文件。
    */
    #[java_method]
    pub fn get_preloads_file_cache(&self) -> Option<File> {}

    /**
    查询（如果需要）创建一个新目录，应用程序可以在其中放置其自己的自定义数据文件。您可以使用返回的 File 对象创建和访问此目录中的文件。
    请注意，通过 File 对象创建的文件只能由您自己的应用程序访问；您只能设置整个目录的模式，而不能设置单个文件的模式。
    如果调用应用程序移动到采用的存储设备，则返回的路径可能会随之变化，因此只应保留相对路径。应用程序不需要额外的权限来读取或写入返回的路径，因为此路径位于其私有存储中。
    返回：请求的目录的 File 对象。如果目录尚不存在，则将创建该目录。
    `name` 要查询的目录的名称。这是作为应用程序数据的一部分创建的目录。
    `mode` 操作模式。
    */
    #[java_method]
    pub fn get_dir(&self, name: String, mode: i32) -> File {}

    /**
    将现有数据库文件从给定的源存储上下文移动到此上下文。这通常用于在升级后在存储位置之间迁移数据，例如迁移到受设备保护的存储。移动数据库之前必须关闭数据库。
    返回：如果移动成功或源上下文中不存在数据库，则返回 true，否则返回 false。
    `source_context` 包含要移动的现有数据库的源上下文。
    `name` 数据库文件的名称。
    */
    #[java_method]
    pub fn move_database_from(&self, source_context: &Self, name: String) -> bool {}

    /**
    返回文件系统上存储使用 openOrCreateDatabase 创建的数据库的绝对路径。如果调用应用程序移动到采用的存储设备，则返回的路径可能会随时间而变化，因此只应保留相对路径。
    返回：给定数据库的绝对路径。
    `name` 您想要获取其路径的数据库的名称。
    */
    #[java_method]
    pub fn get_database_path(&self, name: String) -> File {}

    /**
    请求启动给定的应用服务。Intent 应包含要启动的特定服务实现的完整类名，或要定位的特定包名。
    如果 Intent 指定的内容较少，则会记录有关此情况的警告。在这种情况下，可以使用多个匹配的服务中的任何一个。
    如果此服务尚未运行，它将被实例化并启动（如果需要，为其创建一个进程）；如果它正在运行，则它将保持运行状态。
    每次调用此方法都将导致对目标服务的 android.app.Service.onStartCommand 方法的相应调用，并带有此处给出的意图。
    这提供了一种向服务提交作业的便捷方式，而无需绑定并调用其接口。使用 startService() 会覆盖 bindService 管理的默认服务生命周期：它要求服务保持运行，直到调用 stopService，无论是否有任何客户端连接到它。
    请注意，对 startService() 的调用不会嵌套：无论您调用 startService() 多少次，对 stopService 的一次调用都会停止它。
    系统会尝试尽可能地保持服务运行。只有在当前前台应用程序使用的资源太多而需要终止服务时，才应停止服务。如果服务进程中发生任何错误，它将自动重新启动。
    如果您无权启动给定的服务，此函数将抛出 SecurityException。
    注意：每次调用 startService() 都会导致系统完成大量工作来管理围绕意图处理的服务生命周期，这可能需要数毫秒的 CPU 时间。
    由于此成本，startService() 不应用于频繁向服务传递意图，而应仅用于安排重要工作。对高频调用使用绑定服务。
    从 SDK 版本 Build.VERSION_CODES.O 开始，以 SDK 版本 Build.VERSION_CODES.O 或更高版本为目标的应用不允许从后台启动后台服务。有关更多详细信息，请参阅后台执行限制。
    注意：从 SDK 版本 Build.VERSION_CODES.S 开始，以 SDK 版本 Build.VERSION_CODES.S 为目标的应用S 或更高版本的 Android 不允许从后台启动前台服务。有关更多详细信息，请参阅行为变更：针对 Android 12 的应用。
    返回:如果服务正在启动或已在运行，则返回已启动的实际服务的 ComponentName；否则，如果服务不存在，则返回 null。
    抛出:
    - SecurityException – 如果调用者无权访问服务或找不到服务。
    - IllegalStateException – Android Build.VERSION_CODES.S 之前，如果应用程序处于无法启动服务的状态（例如，在允许服务的状态下不在前台），则抛出 IllegalStateException。
    - BackgroundServiceStartNotAllowedException – Android Build.VERSION_CODES。 S 及更高版本中，如果应用程序处于无法启动服务的状态（比如在允许服务的状态下未处于前台），则会引发 android.app.BackgroundServiceStartNotAllowedException。此豁免扩展了 IllegalStateException，因此应用程序可以使用 catch(IllegalStateException) 来捕获这两者。
    `service` 标识要启动的服务。Intent 必须完全明确（提供组件名称）。Intent extras 中可以包含其他值，以提供与此特定启动调用一起的参数。
    */
    #[java_method]
    pub fn start_service(&self, service: &Intent) -> Result<ComponentName, <Self as JType>::Error> {
    }

    /**
    确定您是否被授予了特定权限。
    如果您拥有该权限，则返回：PackageManager.PERMISSION_GRANTED；如果没有，则返回：PackageManager.PERMISSION_DENIED。
    `permission` 正在检查的权限的名称。
    */
    #[java_method]
    pub fn check_self_permission(&self, permission: String) -> i32 {}

    /**
    确定您正在处理的 IPC 的调用进程是否已被授予特定权限。这与使用 android.os.Binder.getCallingPid 和 android.os.Binder.getCallingUid 返回的 pid 和 uid 调用 checkPermission(String, int, int) 基本相同。
    一个重要的区别是，如果您当前没有处理 IPC，则此函数将始终失败。这样做是为了防止意外泄露权限；您可以使用 checkCallingOrSelfPermission 来避免这种保护。
    返回：如果调用 pid/uid 被允许该权限，则返回 PackageManager.PERMISSION_GRANTED，否则返回 PackageManager.PERMISSION_DENIED。
    `permission` 正在检查的权限的名称。
    */
    #[java_method]
    pub fn check_calling_permission(&self, permission: String) -> i32 {}
}

/**
Context 的代理实现，它只是将其所有调用委托给另一个 Context。可以创建子类来修改行为，而无需更改原始 Context。
*/
#[java_class(name = "android/content/ContextWrapper", extends = Context)]
pub struct ContextWrapper;

impl ContextWrapper {
    /**
    返回可用于查询此包中的类的类加载器。
    */
    pub fn get_class_loader(&self) -> ClassLoader {
        self._based.get_class_loader()
    }

    /**
    将给定的意图广播给所有感兴趣的 BroadcastReceiver。此调用是异步的；它会立即返回，您将在接收器运行时继续执行。接收器不会传播任何结果，接收器也无法中止广播。如果您想允许接收器传播结果或中止广播，则必须使用 sendOrderedBroadcast(Intent, String) 发送有序广播。有关 Intent 广播的更多信息，请参阅 BroadcastReceiver。
    `intent` 要广播的 Intent；与此 Intent 匹配的所有接收器都将收到广播。
    */
    pub fn send_broadcast(&self, intent: &Intent) {
        self._based.send_broadcast(intent)
    }

    #[doc(hidden)]
    pub fn start_service(&self, service: &Intent) -> Result<ComponentName, <Self as JType>::Error> {
        self._based.start_service(service)
    }

    #[doc(hidden)]
    pub fn check_self_permission(&self, permission: String) -> i32 {
        self._based.check_self_permission(permission)
    }

    #[doc(hidden)]
    pub fn check_calling_permission(&self, permission: String) -> i32 {
        self._based.check_calling_permission(permission)
    }
}

//noinspection SpellCheckingInspection
/**
意图是要执行的操作的抽象描述。它可以与 startActivity 一起使用来启动活动，与 broadcastIntent 一起使用来将其发送到任何感兴趣的 BroadcastReceiver 组件，以及与 Context.startService 或 Context.bindService 一起使用来与后台 android.app.Service 通信。
意图提供了一种在不同应用程序的代码之间执行后期运行时绑定的功能。它最重要的用途是在启动活动时，可以将其视为活动之间的粘合剂。它基本上是一个被动数据结构，包含要执行的操作的抽象描述。
开发人员指南
有关如何创建和解析意图的信息，请阅读意图和意图过滤器开发人员指南。

意图结构意图中的主要信息包括：
- 动作——要执行的一般操作，例如 ACTION_VIEW、ACTION_EDIT、ACTION_MAIN 等。
- 数据——要操作的数据，例如联系人数据库中的人员记录，以 Uri 表示。
动作/数据对的一些示例如下：
- ACTION_VIEW content://contacts/people/1 -- 显示标识符为“1”的人员的信息。
- ACTION_DIAL content://contacts/people/1 -- 显示填写了联系人的电话拨号器。
- ACTION_VIEW tel:123 -- 显示填有给定号码的电话拨号器。请注意 VIEW 操作如何对特定 URI 执行被认为是最合理的事情。
- ACTION_DIAL tel:123 -- 显示填有给定号码的电话拨号器。
- ACTION_EDIT content://contacts/people/1 -- 编辑标识符为“1”的人员的信息。
- ACTION_VIEW content://contacts/people/ -- 显示联系人列表，用户可以浏览。此示例是联系人应用的典型顶级条目，向您显示联系人列表。选择要查看的特定人员将导致使用新意图 { ACTION_VIEW content://contacts/people/N } 来启动活动以显示该人员。
除了这些主要属性之外，您还可以在 Intent 中包含许多次要属性：
- category - 提供有关要执行的操作的其他信息。例如，CATEGORY_LAUNCHER 表示它应作为顶级应用程序出现在 Launcher 中，而 CATEGORY_ALTERNATIVE 表示它应包含在用户可以对数据执行的备选操作列表中。
- type - 指定 Intent 数据的显式类型（MIME 类型）。通常，类型是从数据本身推断出来的。通过设置此属性，您可以禁用该评估并强制使用显式类型。
- component - 指定用于 Intent 的组件类的显式名称。通常，这是通过查看 Intent 中的其他信息（操作、数据/类型和类别）并将其与可以处理它的组件匹配来确定的。如果设置了此属性，则不会执行任何评估，并且此组件将按原样使用。通过指定此属性，所有其他 Intent 属性都变为可选。
- extras——这是任何附加信息的 Bundle。这可用于向组件提供扩展信息。例如，如果我们有一个发送电子邮件消息的操作，我们还可以在此处包含额外的数据以提供主题、正文等。
以下是您可以使用这些附加参数指定为意图的其他操作的一些示例：
- ACTION_MAIN with category CATEGORY_HOME -- 启动主屏幕。
- ACTION_GET_CONTENT with MIME type vnd.android.cursor.item/phone -- 显示人们的电话号码列表，允许用户浏览并选择一个并将其返回到父活动。
- ACTION_GET_CONTENT with MIME type *\/\* and category CATEGORY_OPENABLE -- 显示所有可以使用ContentResolver打开的数据的选择器。 OpenInputStream（），允许用户选择其中一个，然后在其中选择一些数据，然后将结果URI返回到呼叫者。例如，可以在电子邮件应用程序中使用，以允许用户选择一些数据以作为附件。
Intent 类中定义了各种标准 Intent 操作和类别常量，但应用程序也可以定义自己的常量。这些字符串使用 Java 样式的作用域，以确保它们是唯一的 - 例如，标准 ACTION_VIEW 称为“android.intent.action.VIEW”。组合起来，操作、数据类型、类别和额外数据集合为系统定义了一种语言，允许表达诸如“呼叫约翰史密斯的牢房”之类的短语。随着应用程序添加到系统中，它们可以通过添加新的操作、类型和类别来扩展此语言，或者它们可以通过提供自己的活动来处理现有短语的行为来修改现有短语的行为。

意图解析
您将使用两种主要形式的意图。显式意图已指定组件（通过 setComponent 或 setClass），该组件提供要运行的确切类。通常，这些不会包含任何其他信息，只是应用程序在用户与应用程序交互时启动其拥有的各种内部活动的一种方式。隐式意图未指定组件；相反，它们必须包含足够的信息，以便系统确定哪个可用组件最适合运行该意图。当使用隐式意图时，给定这样一个任意意图，我们需要知道如何处理它。这是由意图解析过程处理的，它将意图映射到可以处理它的活动、广播接收器或 android.app.service（有时是两个或更多活动/接收器）。
意图解析机制基本上围绕将意图与已安装的应用程序包中的所有 描述进行匹配。 （此外，对于广播，任何 BroadcastReceiver 对象都明确注册到 Context.registerReceiver。）有关此内容的更多详细信息，请参阅 IntentFilter 类的文档。
Intent 中有三部分信息用于解析：操作、类型和类别。使用这些信息，在 PackageManager 上查询可以处理该意图的组件。根据 AndroidManifest.xml 文件中提供的意图信息确定适当的组件，如下所示：如果给出了操作，则组件必须将其列为其处理的操作。如果 Intent 中尚未提供类型，则从 Intent 的数据中查询类型。与操作一样，如果意图中包含某种类型（在其数据中明确或隐式地包含），则组件必须将其列为其处理的类型。对于不是 content: URI 的数据，并且 Intent 中未包含明确类型，则将考虑意图数据的方案（例如 http: 或 mailto:）。同样，与操作一样，如果我们要匹配一个方案，则组件必须将其列为可以处理的方案。如果提供了类别，则活动必须将其全部列为其处理的类别。也就是说，如果您包括类别 CATEGORY_LAUNCHER 和 CATEGORY_ALTERNATIVE，那么您将只解析具有列出这两个类别的意图的组件。
活动通常需要支持 CATEGORY_DEFAULT，以便 Context.startActivity() 可以找到它们。例如，考虑 Note Pad 示例应用程序，它允许用户浏览笔记数据列表并查看有关各个项目的详细信息。斜体文本表示您将用特定于您自己的包的名称替换名称的位置。
&lt;manifest xmlns:android="<http://schemas.android.com/apk/res/android>"
package="com.android.notepad"&gt;
&lt;application android:icon="@drawable/app_notes"
android:label="@string/app_name"&gt;

&lt;provider class=".NotePadProvider"
android:authorities="com.google.provider.NotePad" /&gt;

&lt;activity class=".NotesList" android:label="@string/title_notes_list"&gt;
&lt;intent-filter&gt;
&lt;action android:name="android.intent.action.MAIN" /&gt;
&lt;category android:name="android.intent.category.LAUNCHER" /&gt;
&lt;/intent-filter&gt;
&lt;intent-filter&gt;
&lt;action android:name="android.intent.action.VIEW" /&gt;
&lt;action android:name="android.intent.action.EDIT" /&gt;
&lt;action android:name="android.intent.action.PICK" /&gt;
&lt;category android:name="android.intent.category.DEFAULT" /&gt;
&lt;data android:mimeType="vnd.android.cursor.dir/vnd.google.note" /&gt;
&lt;/intent-filter&gt;
&lt;intent-filter&gt;
&lt;action android:name="android.intent.action.GET_CONTENT" /&gt;
&lt;category android:name="android.intent.category.DEFAULT" /&gt;
&lt;data android:mimeType="vnd.android.cursor.item/vnd.google.note" /&gt;
&lt;/intent-filter&gt;
&lt;/activity&gt;

&lt;activity class=".NoteEditor" android:label="@string/title_note"&gt;
&lt;intent-filter android:label="@string/resolve_edit"&gt;
&lt;action android:name="android.intent.action.VIEW" /&gt;
&lt;action android:name="android.intent.action.EDIT" /&gt;
&lt;category android:name="android.intent.category.DEFAULT" /&gt;
&lt;data android:mimeType="vnd.android.cursor.item/vnd.google.note" /&gt;
&lt;/intent-filter&gt;

&lt;intent-filter&gt;
&lt;action android:name="android.intent.action.INSERT" /&gt;
&lt;category android:name="android.intent.category.DEFAULT" /&gt;
&lt;data android:mimeType="vnd.android.cursor.dir/vnd.google.note" /&gt;
&lt;/intent-filter&gt;

&lt;/activity&gt;

&lt;activity class=".TitleEditor" android:label="@string/title_edit_title"
android:theme="@android:style/Theme.Dialog"&gt;
&lt;intent-filter android:label="@string/resolve_title"&gt;
&lt;action android:name="com.android.notepad.action.EDIT_TITLE" /&gt;
&lt;category android:name="android.intent.category.DEFAULT" /&gt;
&lt;category android:name="android.intent.category.ALTERNATIVE" /&gt;
&lt;category android:name="android.intent.category.SELECTED_ALTERNATIVE" /&gt;
&lt;data android:mimeType="vnd.android.cursor.item/vnd.google.note" /&gt;
&lt;/intent-filter&gt;
&lt;/activity&gt;

&lt;/application&gt;
&lt;/manifest&gt;
第一个活动 com.android.notepad.NotesList 是我们进入应用程序的主要入口。它可以做三件事，正如它的三个意图模板所描述的那样：
&lt;intent-filter&gt;
&lt;action android:name="android.intent.action.MAIN" /&gt;
&lt;category android:name="android.intent.category.LAUNCHER" /&gt;
&lt;/intent-filter&gt;
这为 NotePad 应用程序提供了顶级入口：标准 MAIN 操作是一个主入口点（不需要 Intent 中的任何其他信息），LAUNCHER 类别表示此入口点应在应用程序启动器中列出。
&lt;intent-filter&gt;
&lt;action android:name="android.intent.action.VIEW" /&gt;
&lt;action android:name="android.intent.action.EDIT" /&gt;
&lt;action android:name="android.intent.action.PICK" /&gt;
&lt;category android:name="android.intent.category.DEFAULT" /&gt;
&lt;data android:mimeType="vnd.android.cursor.dir/vnd.google.note" /&gt;
&lt;/intent-filter&gt;
这声明了活动可以对便笺目录执行的操作。所支持的类型由 标记指定，其中 vnd.android.cursor.dir/vnd.google.note 是一个 URI，可从中查询包含我们的记事本数据 (vnd.google.note) 的零个或多个项目的 Cursor (vnd.android.cursor.dir)。该活动允许用户查看或编辑数据目录 (通过 VIEW 和 EDIT 操作)，或选择特定便笺并将其返回给调用者 (通过 PICK 操作)。还请注意此处提供的 DEFAULT 类别：当未明确指定组件名称时，Context.startActivity 方法需要此类别来解析您的活动。
&lt;intent-filter&gt;
&lt;action android:name="android.intent.action.GET_CONTENT" /&gt;
&lt;category android:name="android.intent.category.DEFAULT" /&gt;
&lt;data android:mimeType="vnd.android.cursor.item/vnd.google.note" /&gt;
&lt;/intent-filter&gt;
此过滤器描述了向调用者返回用户选择的注释而无需知道注释来自何处的能力。数据类型 vnd.android.cursor.item/vnd.google.note 是一个 URI，可以从中查询一个包含我们的记事本数据 (vnd.google.note) 的项 (vnd.android.cursor.item)。GET_CONTENT 操作类似于 PICK 操作，其中活动将向其调用者返回用户选择的一段数据。但是，在这里，调用者指定他们想要的数据类型，而不是用户将从中选择的数据类型。鉴于这些功能，以下意图将解析为 NotesList 活动：
{ action=android.app.action.MAIN } 匹配所有可用作应用程序顶级入口点的活动。
{ action=android.app.action.MAIN, category=android.app.category.LAUNCHER } 是启动器用来填充其顶级列表的实际意图。
{ action=android.intent.action.VIEW data=content://com.google.provider.NotePad/notes } 显示“content://com.google.provider.NotePad/notes”下的所有笔记的列表，用户可以浏览并查看其详细信息。
{ action=android.app.action.PICK data=content://com.google.provider.NotePad/notes } 提供“content://com.google.provider.NotePad/notes”下的注释列表，用户可以从中挑选一个注释，并将其数据URL返回给调用者。
{ action=android.app.action.GET_CONTENT type=vnd.android.cursor.item/vnd.google.note } 与 pick 动作类似，但允许调用者指定他们想要返回的数据类型，以便系统可以找到适当的活动来选择该数据类型的内容。
第二个活动 com.android.notepad.NoteEditor 向用户显示单个笔记条目并允许他们编辑它。它可以做两件事，如其两个意图模板所述：
&lt;intent-filter android:label="@string/resolve_edit"&gt;
&lt;action android:name="android.intent.action.VIEW" /&gt;
&lt;action android:name="android.intent.action.EDIT" /&gt;
&lt;category android:name="android.intent.category.DEFAULT" /&gt;
&lt;data android:mimeType="vnd.android.cursor.item/vnd.google.note" /&gt;
&lt;/intent-filter&gt;
此活动的第一个主要目的是让用户与单个注释进行交互，如 MIME 类型 vnd.android.cursor.item/vnd.google.note 所述。此活动可以查看注释或允许用户编辑注释。我们再次支持 DEFAULT 类别，以允许在不明确指定其组件的情况下启动活动。
&lt;intent-filter&gt;
&lt;action android:name="android.intent.action.INSERT" /&gt;
&lt;category android:name="android.intent.category.DEFAULT" /&gt;
&lt;data android:mimeType="vnd.android.cursor.dir/vnd.google.note" /&gt;
&lt;/intent-filter&gt;
此活动的第二个用途是将新笔记条目插入现有笔记目录中。这在用户创建新笔记时使用：在笔记目录上执行 INSERT 操作，导致此活动运行并让用户创建新笔记数据，然后将其添加到内容提供程序。鉴于这些功能，以下意图将解析为 NoteEditor 活动：
{ action=android.intent.action.VIEW data=content://com.google.provider.NotePad/notes/{ID} } 向用户显示注释{ID}的内容。
{ action=android.app.action.EDIT data=content://com.google.provider.NotePad/notes/{ID} } 允许用户编辑注释{ID}的内容。
{ action=android.app.action.INSERT data=content://com.google.provider.NotePad/notes } 在“content://com.google.provider.NotePad/notes”的注释列表中创建一个新的空注释，并允许用户编辑它。如果他们保留更改，则将新创建的注释的 URI 返回给调用者。
最后一个活动 com.android.notepad.TitleEditor 允许用户编辑笔记的标题。这可以作为应用程序直接调用的类来实现（通过在 Intent 中明确设置其组件），但这里我们展示了一种可以在现有数据上发布替代操作的方法：
&lt;intent-filter android:label="@string/resolve_title"&gt;
&lt;action android:name="com.android.notepad.action.EDIT_TITLE" /&gt;
&lt;category android:name="android.intent.category.DEFAULT" /&gt;
&lt;category android:name="android.intent.category.ALTERNATIVE" /&gt;
&lt;category android:name="android.intent.category.SELECTED_ALTERNATIVE" /&gt;
&lt;data android:mimeType="vnd.android.cursor.item/vnd.google.note" /&gt;
&lt;/intent-filter&gt;
在这里的单个意图模板中，我们创建了自己的私有操作，称为 com.android.notepad.action.EDIT_TITLE，表示编辑笔记的标题。它必须在特定笔记（数据类型 vnd.android.cursor.item/vnd.google.note）上调用，就像以前的查看和编辑操作一样，但这里显示和编辑笔记数据中包含的标题。除了像往常一样支持默认类别外，我们的标题编辑器还支持另外两个标准类别：ALTERNATIVE 和 SELECTED_ALTERNATIVE。实现这些类别允许其他人通过 PackageManager.queryIntentActivityOptions 方法找到它提供的特殊操作而无需直接了解它，或者更常见的是使用 android.view.Menu.addIntentOptions 构建动态菜单项。
请注意，在这里的意图模板中，还为模板提供了一个明确的名称（通过 android:label="@string/resolve_title"），以便更好地控制用户在将此活动作为他们正在查看的数据的替代操作呈现时看到的内容。鉴于这些功能，以下意图将解析为 TitleEditor 活动：
{ action=com.android.notepad.action.EDIT_TITLE data=content://com.google.provider.NotePad/notes/{ID} } 显示并允许用户编辑与注释 {ID} 相关的标题。

标准活动操作
这些是 Intent 为启动活动定义的当前标准操作（通常通过 Context.startActivity）。其中最重要的，也是迄今为止最常用的是 ACTION_MAIN 和 ACTION_EDIT。
ACTION_MAIN, ACTION_VIEW, ACTION_ATTACH_DATA, ACTION_EDIT, ACTION_PICK, ACTION_CHOOSER, ACTION_GET_CONTENT, ACTION_DIAL,
ACTION_CALL, ACTION_SEND, ACTION_SENDTO, ACTION_ANSWER, ACTION_INSERT, ACTION_DELETE, ACTION_RUN, ACTION_SYNC,
ACTION_PICK_ACTIVITY, ACTION_SEARCH, ACTION_WEB_SEARCH, ACTION_FACTORY_TEST

标准广播操作
这些是 Intent 为接收广播定义的当前标准操作（通常通过 Context.registerReceiver 或清单中的 标签）。
ACTION_TIME_TICK, ACTION_TIME_CHANGED, ACTION_TIMEZONE_CHANGED, ACTION_BOOT_COMPLETED, ACTION_PACKAGE_ADDED, ACTION_PACKAGE_CHANGED, ACTION_PACKAGE_REMOVED, ACTION_PACKAGE_RESTARTED,
ACTION_PACKAGE_DATA_CLEARED, ACTION_PACKAGES_SUSPENDED, ACTION_PACKAGES_UNSUSPENDED, ACTION_UID_REMOVED, ACTION_BATTERY_CHANGED, ACTION_POWER_CONNECTED, ACTION_POWER_DISCONNECTED, ACTION_SHUTDOWN,
注意：如果您的应用以 Android 11（API 级别 30）或更高版本为目标平台，则注册包含 extras 中的软件包详细信息的广播（例如 ACTION_PACKAGES_SUSPENDED）将收到经过筛选的应用列表或什么也不会收到。详细了解如何管理软件包可见性。

标准类别
这些是当前的标准类别，可用于通过AddCategory进一步阐明意图。
CATEGORY_DEFAULT, CATEGORY_BROWSABLE, CATEGORY_TAB, CATEGORY_ALTERNATIVE, CATEGORY_SELECTED_ALTERNATIVE, CATEGORY_LAUNCHER, CATEGORY_INFO, CATEGORY_HOME,
CATEGORY_PREFERENCE, CATEGORY_TEST, CATEGORY_CAR_DOCK, CATEGORY_DESK_DOCK, CATEGORY_LE_DESK_DOCK, CATEGORY_HE_DESK_DOCK, CATEGORY_CAR_MODE, CATEGORY_APP_MARKET,
CATEGORY_VR_HOME

标准额外数据
这些是当前标准字段，可通过 putExtra 用作额外数据。
EXTRA_ALARM_COUNT, EXTRA_BCC, EXTRA_CC, EXTRA_CHANGED_COMPONENT_NAME, EXTRA_DATA_REMOVED, EXTRA_DOCK_STATE, EXTRA_DOCK_STATE_HE_DESK, EXTRA_DOCK_STATE_LE_DESK,
EXTRA_DOCK_STATE_CAR, EXTRA_DOCK_STATE_DESK, EXTRA_DOCK_STATE_UNDOCKED, EXTRA_DONT_KILL_APP, EXTRA_EMAIL, EXTRA_INITIAL_INTENTS, EXTRA_INTENT, EXTRA_KEY_EVENT,
EXTRA_ORIGINATING_URI, EXTRA_PHONE_NUMBER, EXTRA_REFERRER, EXTRA_REMOTE_INTENT_TOKEN, EXTRA_REPLACING, EXTRA_SHORTCUT_ICON, EXTRA_SHORTCUT_ICON_RESOURCE, EXTRA_SHORTCUT_INTENT,
EXTRA_STREAM, EXTRA_SHORTCUT_NAME, EXTRA_SUBJECT, EXTRA_TEMPLATE, EXTRA_TEXT, EXTRA_TITLE, EXTRA_UID, EXTRA_USER_INITIATED,

标志
这些是可以通过 setFlags 和 addFlags 在 Intent 中使用的可能标志。请参阅 setFlags 以获取所有可能标志的列表。
*/
#[java_class(name = "android/content/Intent")]
pub struct Intent;

impl Intent {
    /**
    活动操作：作为主入口点启动，不期望接收数据。
    输入：无
    输出：无
    */
    pub const ACTION_MAIN: &'static str = "android.intent.action.MAIN";

    /**
    活动操作：向用户显示数据。这是对数据执行的最常见操作 - 它是您可以对一段数据使用的通用操作，以使最合理的事情发生。例如，当用于联系人条目时，它将查看该条目；当用于 mailto: URI 时，它将弹出一个包含 URI 提供的信息的撰写窗口；当与 tel: URI 一起使用时，它将调用拨号器。
    输入：getData 是从中查询数据的 URI。
    输出：无。
    */
    pub const ACTION_VIEW: &'static str = "android.intent.action.VIEW";

    /**
    当启动子活动来管理各种类型的存储时，可以从存储 UI 发出的活动意图中包括额外内容。例如，它可以使用带有“image/\*”MIME 类型的 ACTION_VIEW 让应用在设备上显示图像，在这种情况下，还可以包括此额外内容以告知应用它来自存储 UI，因此应该可以帮助用户管理这种类型的存储。
    */
    pub const EXTRA_FROM_STORAGE: &'static str = "android.intent.extra.FROM_STORAGE";

    /// ACTION_VIEW 的同义词，对一段数据执行的“标准”操作。
    pub const ACTION_DEFAULT: &'static str = Self::ACTION_VIEW;

    /**
    活动操作：快速查看数据。启动 URI 或 URI 列表的快速查看器。处理此意图操作的活动应处理绝大多数 MIME 类型，而不仅仅是特定类型。快速查看器必须在本地呈现快速查看图像，并且不得将文件内容发送到当前设备之外。
    输入：getData 是要预览的项目的必需内容 URI。如果有多个要预览的项目，getClipData 包含可选的内容 URI 列表。EXTRA_INDEX 是剪辑数据中要首先显示的 URI 的可选索引。EXTRA_QUICK_VIEW_FEATURES 是一个可选的额外功能，指​​示可以在快速查看 UI 中显示的功能。
    输出：无。
    */
    pub const ACTION_QUICK_VIEW: &'static str = "android.intent.action.QUICK_VIEW";

    /**
    用于指示应将某些数据附加到其他位置。例如，可以将图像数据附加到联系人。应将数据附加到何处由接收者决定；意图未指定最终目的地。
    输入：getData 是要附加的数据的 URI。
    输出：无。
    */
    pub const ACTION_ATTACH_DATA: &'static str = "android.intent.action.ATTACH_DATA";

    /**
    活动操作：提供对给定数据的明确可编辑访问权限。
    输入：getData 是要编辑的数据的 URI。
    输出：无。
    */
    pub const ACTION_EDIT: &'static str = "android.intent.action.EDIT";

    /**
    活动操作：挑选一个现有项目，或插入一个新项目，然后对其进行编辑。
    输入：getType 是要创建或编辑的项目所需的 MIME 类型。extras 可以包含要传递给编辑/创建活动的类型特定数据。
    输出：挑选的项目的 URI。这必须是 content: URI，以便任何接收者都可以访问它。
    */
    pub const ACTION_INSERT_OR_EDIT: &'static str = "android.intent.action.INSERT_OR_EDIT";

    /**
    活动操作：从数据中挑选一个项目，返回所选内容。
    输入：getData 是包含数据目录 (vnd.android.cursor.dir/\*) 的 URI，可从中挑选项目。
    输出：所挑选项目的 URI。
    */
    pub const ACTION_PICK: &'static str = "android.intent.action.PICK";

    /**
    活动操作：创建提醒。
    输入：EXTRA_TITLE 将向用户显示的提醒的标题。EXTRA_TEXT 将向用户显示的提醒文本。意图至少应指定标题或文本。EXTRA_TIME 将向用户显示提醒的时间。时间以自纪元以​​来的毫秒数指定（可选）。
    输出：无。
    */
    pub const ACTION_CREATE_REMINDER: &'static str = "android.intent.action.CREATE_REMINDER";

    /**
    活动操作：创建快捷方式。
    输入：无。
    输出：表示 android.content.pm.ShortcutInfo 结果的 Intent。为了与旧版本的 Android 兼容，Intent 可能还包含三个额外内容：SHORTCUT_INTENT（值：Intent）、SHORTCUT_NAME（值：String）和 SHORTCUT_ICON（值：Bitmap）或 SHORTCUT_ICON_RESOURCE（值：ShortcutIconResource）。
    */
    pub const ACTION_CREATE_SHORTCUT: &'static str = "android.intent.action.CREATE_SHORTCUT";

    /**
    用于定义快捷方式的 Intent 的额外名称。
    */
    #[deprecated(note = "已替换为 android.content.pm.ShortcutManager#createShortcutResultIntent")]
    pub const EXTRA_SHORTCUT_INTENT: &'static str = "android.intent.extra.shortcut.INTENT";

    /**
    用于定义快捷方式名称的额外名称。
    */
    #[deprecated(note = "替换为 android.content.pm.ShortcutManager#createShortcutResultIntent")]
    pub const EXTRA_SHORTCUT_NAME: &'static str = "android.intent.extra.shortcut.NAME";

    /**
    用于定义快捷方式图标（位图）的额外名称。
    */
    #[deprecated(note = "替换为 android.content.pm.ShortcutManager#createShortcutResultIntent")]
    pub const EXTRA_SHORTCUT_ICON: &'static str = "android.intent.extra.shortcut.ICON";

    /**
    用于定义快捷方式图标的额外名称，作为 ShortcutIconResource。
    */
    #[deprecated(note = "替换为 android.content.pm.ShortcutManager#createShortcutResultIntent")]
    pub const EXTRA_SHORTCUT_ICON_RESOURCE: &'static str =
        "android.intent.extra.shortcut.ICON_RESOURCE";

    /// 提供用户界面以调整应用程序偏好设置的活动。对于所有具有设置的应用程序，这是可选但推荐的设置。
    pub const ACTION_APPLICATION_PREFERENCES: &'static str =
        "android.intent.action.APPLICATION_PREFERENCES";

    /**
    活动操作：启动显示应用信息的活动。对于安装其他应用（如应用商店）的应用，建议处理此操作以向用户提供应用信息。
    输入：EXTRA_PACKAGE_NAME 指定需要显示其信息的包。
    输出：无。
    */
    pub const ACTION_SHOW_APP_INFO: &'static str = "android.intent.action.SHOW_APP_INFO";

    /**
    活动操作：占位符，表示处理它的组件可以进行活动识别。可以放置在服务上。每个包仅支持一个服务。
    输入：无。
    输出：无
    */
    pub const ACTION_ACTIVITY_RECOGNIZER: &'static str =
        "android.intent.action.ACTIVITY_RECOGNIZER";

    /**
    活动操作：显示活动选择器，允许用户在继续操作之前选择他们想要的内容。这可以用作标准活动选择器的替代方案，当您尝试启动具有多个可能匹配项的活动时，系统会显示该标准活动选择器，但行为存在以下差异：您可以指定活动选择器中显示的标题。用户无法选择将其中一个匹配活动设为首选活动，并且所有可能的活动都将始终显示，即使其中一个活动当前被标记为首选活动。
    当用户自然希望选择一项活动以继续操作时，应使用此操作。例如，当用户单击“mailto:”链接时，不应使用此操作。他们自然希望直接转到他们的邮件应用程序，因此应直接调用 startActivity()：它将启动当前首选应用程序，或显示一个对话框，允许用户选择要使用的应用程序，并可选择将其标记为首选。相反，如果用户选择菜单项将他们正在查看的图片发送给其他人，此时他们可能想要做很多不同的事情：通过电子邮件发送、上传到网络服务等。在这种情况下，应使用 CHOOSER 操作，始终向用户显示他们可以执行的操作列表，并由调用者提供一个很好的标题，例如“发送此照片：”。
    如果您需要通过选择器授予 URI 权限，则除了内部的 EXTRA_INTENT 之外，还必须在 ACTION_CHOOSER Intent 上指定要授予的权限。这意味着使用 setClipData 指定要授予的 URI 以及 FLAG_GRANT_READ_URI_PERMISSION 和/或 FLAG_GRANT_WRITE_URI_PERMISSION（视情况而定）。为方便起见，可以使用 createChooser 函数创建此形式的 Intent。
    输入：不应指定任何数据。 get*Extra 必须有一个 EXTRA_INTENT 字段，其中包含正在执行的 Intent，并且可以选择有一个 EXTRA_TITLE 字段，其中包含要在选择器中显示的标题文本。
    输出：取决于 EXTRA_INTENT 的协议。
    */
    pub const ACTION_CHOOSER: &'static str = "android.intent.action.CHOOSER";

    //noinspection SpellCheckingInspection
    /**
    活动操作：允许用户选择特定类型的数据并返回。这与 ACTION_PICK 不同，因为这里我们只说明需要哪种数据，而不是用户可以从中选择的现有数据的 URI。ACTION_GET_CONTENT 可以允许用户在运行时创建数据（例如拍照或录音），让他们浏览网页并下载所需数据等。
    使用此操作的主要方法有两种：如果您想要特定类型的数据（例如个人联系人），则可以将 MIME 类型设置为所需的数据类型，然后使用 Context.startActivity(Intent) 启动它。然后系统将启动最佳应用程序来为您选择该类型的数据。
    您可能还对用户可以选择的一组内容类型感兴趣。例如，想要允许用户向电子邮件消息添加附件的电子邮件应用程序可以使用此操作来显示用户可以附加的所有内容类型的列表。在这种情况下，您应该使用选择器（通过 createChooser）包装 GET_CONTENT 意图，这将为用户提供适当的界面来选择如何发送数据，并允许您指定提示以表明他们正在做什么。您通常会指定广泛的 MIME 类型（例如 image/* 或 */*），从而产生广泛的内容类型供用户选择。使用如此广泛的 GET_CONTENT 操作时，通常希望仅从可以表示为流的数据中进行选择。这可以通过在 Intent 中要求 CATEGORY_OPENABLE 来实现。
    调用者可以选择指定 EXTRA_LOCAL_ONLY 以请求启动的内容选择器仅返回表示设备上本地可用的数据的结果。例如，如果将此 extra 设置为 true，则图像选择器不应显示任何可从远程服务器获得但尚未在本地设备上的图片（因此要求在打开时下载它们）。如果调用者可以处理多个返回的项目（用户执行多项选择），则可以指定 EXTRA_ALLOW_MULTIPLE 来指示这一点。
    输入：getType 是要查询的所需 MIME 类型。请注意，意图中未提供任何 URI，因为对返回数据的原始来源没有任何限制。如果您只能接受可以作为流打开的数据，您还可以包括 CATEGORY_OPENABLE。您可以使用 EXTRA_LOCAL_ONLY 将内容选择限制为本地数据。您可以使用 EXTRA_ALLOW_MULTIPLE 允许用户选择多个项目。
    输出：所选项目的 URI。这必须是 content: URI，以便任何接收者都可以访问它。
    */
    pub const ACTION_GET_CONTENT: &'static str = "android.intent.action.GET_CONTENT";

    /**
    活动操作：拨打数据指定的号码。这会显示一个带有所拨打号码的 UI，允许用户明确发起呼叫。
    输入：如果没有，则启动一个空拨号器；否则 getData 是要拨打的电话号码的 URI 或明确电话号码的 tel: URI。
    输出：无。
    */
    pub const ACTION_DIAL: &'static str = "android.intent.action.DIAL";

    /**
    活动动作：对数据指定的人进行呼叫。
    输入：如果什么也没有，将启动一个空拨号器；否则GetData是要拨打的电话号码的URI或TEL：明确电话号码的URI。
    输出：没有。
    注意：将有限制哪些申请可以启动呼叫；大多数应用程序应使用action_dial。
    注意：这种意图不能用于调用紧急电话。但是，应用程序可以使用Action_dial拨打紧急号码。
    注意：一个填充Android的应用程序。应用程序。角色。罗尔曼格。角色_Dialer角色应使用TelecomManager。 PlaceCall(URI,Bundle)进行呼叫，而不是依靠此意图。
    注意：如果应用程序针对M及以上并将其声明为使用清单。允许。 call_phone的权限未授予，然后尝试使用此操作将导致SecurityException。
    */
    pub const ACTION_CALL: &'static str = "android.intent.action.CALL";

    /**
    活动操作：拨打数据指定的紧急号码。
    输入：getData 是要拨打的电话号码的 URI 或明确电话号码的 tel: URI。
    输出：无。
    注意：不能保证呼叫将拨打到 TelecomManager 中提供的 PhoneAccount。EXTRA_PHONE_ACCOUNT_HANDLE extra（如果指定），并且可能拨打到具有 PhoneAccount 功能的另一个 PhoneAccount。CAPABILITY_PLACE_EMERGENCY_CALLS 功能，具体取决于外部因素，例如网络条件和调制解调器/SIM 状态。
    */
    pub const ACTION_CALL_EMERGENCY: &'static str = "android.intent.action.CALL_EMERGENCY";

    /**
    活动操作：拨打数据指定的紧急号码。这会显示一个带有所拨打号码的 UI，允许用户明确发起呼叫。
    输入：如果没有，则启动一个空的紧急拨号器；否则 getData 是明确的紧急电话号码的 tel: URI。
    输出：没有。
    */
    pub const ACTION_DIAL_EMERGENCY: &'static str = "android.intent.action.DIAL_EMERGENCY";

    /**
    活动操作：拨打数据指定的任何号码（紧急或非紧急）。
    输入：getData 是要拨打的电话号码的 URI 或明确电话号码的 tel: URI。
    输出：无。
    */
    pub const ACTION_CALL_PRIVILEGED: &'static str = "android.intent.action.CALL_PRIVILEGED";

    /**
    活动操作：运营商设置应用的主要入口点。提供此操作实现的运营商应用可被调用来配置运营商服务，并且通常需要运营商权限才能履行其职责。
    */
    pub const ACTION_CARRIER_SETUP: &'static str = "android.intent.action.CARRIER_SETUP";

    /**
    活动操作：向数据指定的某人发送消息。
    输入：getData 是描述目标的 URI。
    输出：无。
    */
    pub const ACTION_SENDTO: &'static str = "android.intent.action.SENDTO";

    //noinspection SpellCheckingInspection
    /**
    活动操作：向其他人提供一些数据。数据要被传送给谁尚未指定；由该操作的接收者询问用户应该将数据发送到哪里。启动 SEND 意图时，您通常应该将其包装在选择器中（通过 createChooser），这将为用户提供适当的界面来选择如何发送数据，并允许您指定提示以表明他们正在做什么。
    输入：getType 是要发送的数据的 MIME 类型。get*Extra 可以具有 EXTRA_TEXT 或 EXTRA_STREAM 字段，其中包含要发送的数据。如果使用 EXTRA_TEXT，MIME 类型应为“text/plain”；否则它应该是 EXTRA_STREAM 中数据的 MIME 类型。如果 MIME 类型未知，请使用 *\/\*（这将仅允许能够处理通用数据流的发送者）。如果使用 EXTRA_TEXT，您还可以选择提供 EXTRA_HTML_TEXT 以便客户端查询具有 HTML 格式的文本。截至 Build。 VERSION_CODES。JELLY_BEAN，发送的数据可通过 setClipData(ClipData) 提供。这允许您在共享内容时使用 FLAG_GRANT_READ_URI_PERMISSION：URI 和 ClipData 的其他高级功能。如果使用此方法，您仍必须通过下面描述的 EXTRA_TEXT 或 EXTRA_STREAM 字段提供相同的数据，以便与旧应用程序兼容。如果您未设置 ClipData，则在调用 Context.startActivity(Intent) 时会将其复制到那里。从 Build 开始。VERSION_CODES。O，如果传递了 CATEGORY_TYPED_OPENABLE，则在 EXTRA_STREAM 中或通过 setClipData(ClipData) 传递的 Uris 可能只能使用 ContentResolver.openTypedAssetFileDescriptor(Uri, String, Bundle) 作为资产类型文件打开。可选的标准附加功能（某些收件人可能会对其进行适当解释）包括：EXTRA_EMAIL、EXTRA_CC、EXTRA_BCC、EXTRA_SUBJECT。
    输出：无。
    */
    pub const ACTION_SEND: &'static str = "android.intent.action.SEND";

    //noinspection SpellCheckingInspection
    /**
    活动操作：向其他人提供多个数据。与 ACTION_SEND 类似，但数据是多个。
    输入：getType 是发送的数据的 MIME 类型。get*ArrayListExtra 可以具有 EXTRA_TEXT 或 EXTRA_STREAM 字段，其中包含要发送的数据。如果使用 EXTRA_TEXT，您还可以选择提供 EXTRA_HTML_TEXT，以便客户端查询具有 HTML 格式的文本。支持多种类型，接收方应尽可能处理混合类型。接收方检查它们的正确方法是在每个 URI 上使用内容解析器。意图发送者应尝试将最具体的 MIME 类型放入意图类型中，但它可以根据需要回退到 /* 或 */*。例如，如果您发送 image/jpg 和 image/jpg，则意图的类型可以是 image/jpg，但如果您发送 image/jpg 和 image/png，则意图的类型应为 image/\*。截至 Build。VERSION_CODES。 JELLY_BEAN，发送的数据可通过 setClipData(ClipData) 提供。这样，您可以在共享内容时使用 FLAG_GRANT_READ_URI_PERMISSION：URI 和 ClipData 的其他高级功能。如果使用此方法，您仍必须通过下面描述的 EXTRA_TEXT 或 EXTRA_STREAM 字段提供相同的数据，以便与旧应用程序兼容。如果您未设置 ClipData，则在调用 Context.startActivity(Intent) 时，它将被复制到那里。从 Build 开始。VERSION_CODES。O，如果传递了 CATEGORY_TYPED_OPENABLE，则在 EXTRA_STREAM 中或通过 setClipData(ClipData) 传递的 Uris 可能只能使用 ContentResolver.openTypedAssetFileDescriptor(Uri, String, Bundle) 作为资产类型文件打开。可选的标准附加功能（某些收件人可能会对其进行适当解释）包括：EXTRA_EMAIL、EXTRA_CC、EXTRA_BCC、EXTRA_SUBJECT。
    输出：无。
    */
    pub const ACTION_SEND_MULTIPLE: &'static str = "android.intent.action.SEND_MULTIPLE";

    /**
    活动操作：处理来电。
    输入：无。
    输出：无。
    */
    pub const ACTION_ANSWER: &'static str = "android.intent.action.ANSWER";

    /**
    活动操作：将一个空项目插入给定的容器。
    输入：getData 是放置数据的目录 (vnd.android.cursor.dir/\*) 的 URI。
    输出：已创建的新数据的 URI。
    */
    pub const ACTION_INSERT: &'static str = "android.intent.action.INSERT";

    /**
    活动操作：在给定容器中创建新项目，并从剪贴板的当前内容对其进行初始化。
    输入：getData 是放置数据的目录 (vnd.android.cursor.dir/\*) 的 URI。
    输出：已创建的新数据的 URI。
    */
    pub const ACTION_PASTE: &'static str = "android.intent.action.PASTE";

    /**
    活动操作：从其容器中删除给定的数据。
    输入：getData 是要删除的数据的 URI。
    输出：无。
    */
    pub const ACTION_DELETE: &'static str = "android.intent.action.DELETE";

    /**
    活动操作：运行数据，无论其含义如何。
    输入：？（注意：这目前特定于测试工具。）
    输出：无。
    */
    pub const ACTION_RUN: &'static str = "android.intent.action.RUN";

    /**
    活动操作：执行数据同步。
    输入：？
    输出：？
    */
    pub const ACTION_SYNC: &'static str = "android.intent.action.SYNC";

    /**
    活动操作：根据意图选择一个活动，返回所选的类。
    输入：get*Extra 字段 EXTRA_INTENT 是与 PackageManager 一起使用的 Intent。queryIntentActivities 用于确定要从中选择的活动集。
    输出：所选活动的类名。
    */
    pub const ACTION_PICK_ACTIVITY: &'static str = "android.intent.action.PICK_ACTIVITY";
    /**
    活动操作：执行搜索。
    输入：getStringExtra(SearchManager.QUERY) 是要搜索的文本。如果为空，只需在激活搜索 UI 的情况下输入搜索结果活动即可。
    输出：无。
    */
    pub const ACTION_SEARCH: &'static str = "android.intent.action.SEARCH";

    /**
    活动操作：启动平台定义的教程
    输入：getStringExtra(SearchManager.QUERY) 是要搜索的文本。如果为空，只需在激活搜索 UI 的情况下输入搜索结果活动即可。
    输出：无。
    */
    pub const ACTION_SYSTEM_TUTORIAL: &'static str = "android.intent.action.SYSTEM_TUTORIAL";

    /**
    活动操作：执行网络搜索。
    输入：getStringExtra(SearchManager.QUERY) 是要搜索的文本。如果 URL 以 http 或 https 开头，则会打开该网站。如果是纯文本，则会应用 Google 搜索。
    输出：无。
    */
    pub const ACTION_WEB_SEARCH: &'static str = "android.intent.action.WEB_SEARCH";

    /**
    活动操作：执行协助操作。
    输入：EXTRA_ASSIST_PACKAGE、EXTRA_ASSIST_CONTEXT，可以提供有关用户请求协助时所在位置的其他可选上下文信息；EXTRA_REFERRER 可以设置其他引荐来源信息。
    输出：无。
    */
    pub const ACTION_ASSIST: &'static str = "android.intent.action.ASSIST";

    /**
    活动操作：执行语音辅助操作。
    输入：EXTRA_ASSIST_PACKAGE、EXTRA_ASSIST_CONTEXT，可以提供有关用户请求语音辅助时所在位置的其他可选上下文信息。
    输出：无。
    */
    pub const ACTION_VOICE_ASSIST: &'static str = "android.intent.action.VOICE_ASSIST";

    /// ACTION_ASSIST 上的可选字段包含调用辅助时当前前台应用程序包的名称。
    pub const EXTRA_ASSIST_PACKAGE: &'static str = "android.intent.extra.ASSIST_PACKAGE";

    /// ACTION_ASSIST 上的可选字段包含调用辅助时当前前台应用程序包的 uid。
    pub const EXTRA_ASSIST_UID: &'static str = "android.intent.extra.ASSIST_UID";

    /// ACTION_ASSIST 上的可选字段，包含当前前台应用在发出辅助请求时提供的其他上下文信息。这是附加数据的 Bundle。
    pub const EXTRA_ASSIST_CONTEXT: &'static str = "android.intent.extra.ASSIST_CONTEXT";

    /// ACTION_ASSIST 上的可选字段表明用户可能会使用键盘作为主要的输入设备来获取帮助。
    pub const EXTRA_ASSIST_INPUT_HINT_KEYBOARD: &'static str =
        "android.intent.extra.ASSIST_INPUT_HINT_KEYBOARD";

    /// ACTION_ASSIST 上的可选字段包含用于调用辅助的 InputDevice id。
    pub const EXTRA_ASSIST_INPUT_DEVICE_ID: &'static str =
        "android.intent.extra.ASSIST_INPUT_DEVICE_ID";

    /**
    活动行动：列出所有可用的应用程序。
    输入：没有。
    输出：没有。
    */
    pub const ACTION_ALL_APPS: &'static str = "android.intent.action.ALL_APPS";

    /**
    活动操作：显示启动器中所有工作应用列表的操作。例如，显示工作应用文件夹或选项卡。
    输入：无。
    输出：无。
    */
    pub const ACTION_SHOW_WORK_APPS: &'static str = "android.intent.action.SHOW_WORK_APPS";

    /**
    活动操作：显示选择壁纸的设置。
    输入：无。
    输出：无。
    */
    pub const ACTION_SET_WALLPAPER: &'static str = "android.intent.action.SET_WALLPAPER";

    /**
    活动操作：显示报告错误的活动。
    输入：无。
    输出：无。
    */
    pub const ACTION_BUG_REPORT: &'static str = "android.intent.action.BUG_REPORT";

    /**
    活动动作：出厂测试的主要切入点。仅当设备在工厂测试节点中启动时才使用。实现软件包必须安装在系统图像中。
    输入：没有
    输出：没有
    */
    pub const ACTION_FACTORY_TEST: &'static str = "android.intent.action.FACTORY_TEST";

    /**
    活动操作：用户按下“呼叫”按钮进入拨号器或其他适当的 UI 以拨打电话。
    输入：无。
    输出：无。
    */
    pub const ACTION_CALL_BUTTON: &'static str = "android.intent.action.CALL_BUTTON";

    /**
    活动操作：启动语音命令。
    输入：无。
    输出：无。在某些情况下，可能不存在匹配的活动，因此请确保防范这种情况。
    */
    pub const ACTION_VOICE_COMMAND: &'static str = "android.intent.action.VOICE_COMMAND";

    /**
    活动操作：启动与长按搜索键相关的操作。
    输入：无。
    输出：无。
    */
    pub const ACTION_SEARCH_LONG_PRESS: &'static str = "android.intent.action.SEARCH_LONG_PRESS";

    /**
    活动操作：用户按下了崩溃/ ANR 对话框中的“报告”按钮。此意图会传递到安装应用程序的软件包，通常是 Google Play。
    输入：未指定任何数据。错误报告使用 EXTRA_BUG_REPORT 字段传递。
    输出：无。
    */
    pub const ACTION_APP_ERROR: &'static str = "android.intent.action.APP_ERROR";

    /**
    已记录事件或错误报告，并且系统应用已请求共享该报告，因此触发确认屏幕。这将直接发送给具有 android. 权限的注册接收者。APPROVE_INCIDENT_REPORTS 权限。
    */
    pub const ACTION_PENDING_INCIDENT_REPORTS_CHANGED: &'static str =
        "android.intent.action.PENDING_INCIDENT_REPORTS_CHANGED";

    /**
    已记录事件报告，且用户已批准共享。这将直接发送给已注册的接收者，该接收者必须同时具有 DUMP 和 USAGE_STATS 权限。收到此信息后，应用程序应等待合适的时间（例如网络可用），使用 IncidentManager.getIncidentReportList(String) 获取可用报告列表，然后在报告成功上传后调用 IncidentManager.deleteIncidentReport(Uri)。
    */
    pub const ACTION_INCIDENT_REPORT_READY: &'static str =
        "android.intent.action.INCIDENT_REPORT_READY";

    /**
    活动操作：向用户显示用电量信息。
    输入：无。
    输出：无。
    */
    pub const ACTION_POWER_USAGE_SUMMARY: &'static str =
        "android.intent.action.POWER_USAGE_SUMMARY";

    /**
    活动操作：为 OTA 配置提供的设置向导操作，用于确定是否需要运行。
    输入：无。
    输出：无。
    */
    #[deprecated(
        note = "从 Build.VERSION_CODES.M 开始，可以使用 ACTION_MAIN 和 CATEGORY_SETUP_WIZARD 识别设置向导"
    )]
    pub const ACTION_DEVICE_INITIALIZATION_WIZARD: &'static str =
        "android.intent.action.DEVICE_INITIALIZATION_WIZARD";

    /**
    活动操作：平台更新后启动的安装向导。此活动应具有与其关联的字符串元数据字段 METADATA_SETUP_VERSION，该字段定义要安装的平台的当前版本。仅当 android.provider.Settings.Secure.LAST_SETUP_SHOWN 不是相同值时，才会启动此活动。
    输入：无。
    输出：无。
    */
    pub const ACTION_UPGRADE_SETUP: &'static str = "android.intent.action.UPGRADE_SETUP";

    /**
    活动操作：启动键盘快捷键助手屏幕。
    输入：无。
    输出：无。
    */
    pub const ACTION_SHOW_KEYBOARD_SHORTCUTS: &'static str =
        "com.android.intent.action.SHOW_KEYBOARD_SHORTCUTS";

    /**
    活动操作：关闭键盘快捷键助手屏幕。
    输入：无。
    输出：无。
    */
    pub const ACTION_DISMISS_KEYBOARD_SHORTCUTS: &'static str =
        "com.android.intent.action.DISMISS_KEYBOARD_SHORTCUTS";

    /**
    活动操作：显示用于管理特定应用程序的网络数据使用的设置。应用程序应定义一个活动，提供控制数据使用的选项。
    */
    pub const ACTION_MANAGE_NETWORK_USAGE: &'static str =
        "android.intent.action.MANAGE_NETWORK_USAGE";

    /**
    活动操作：启动应用程序安装程序。
    输入：数据必须是 content: URI，可从中查询应用程序。从 Build. VERSION_CODES. JELLY_BEAN_MR1 开始，您还可以使用“package: ”为当前用户安装已为其他用户安装的应用程序。您可以选择提供 EXTRA_INSTALLER_PACKAGE_NAME、EXTRA_NOT_UNKNOWN_SOURCE、EXTRA_ALLOW_REPLACE 和 EXTRA_RETURN_RESULT。
    输出：如果为 EXTRA_RETURN_RESULT，则返回安装是否成功。
    注意：如果您的应用针对的 API 级别高于 25，则需要持有 Manifest.permission.REQUEST_INSTALL_PACKAGES 才能启动应用程序安装程序。
    */
    #[deprecated(note = "改用 android. content. pm. PackageInstaller")]
    pub const ACTION_INSTALL_PACKAGE: &'static str = "android.intent.action.INSTALL_PACKAGE";

    /**
    活动动作：处理分裂安装失败的活动。拆分可以动态安装。当启动活动时，这会发生这种情况，但是包含应用程序的拆分未安装。当以这种方式安装拆分时，包含的软件包通常不知道发生这种情况。但是，如果在安装过程中发生错误，则包含软件包可以定义单个活动来处理此操作以处理此类故障。处理此操作的活动必须在基本包中。
    输入：Extra_intent开始拆分安装的原始意图。 extra_split_name未安装的拆分名称。
    */
    pub const ACTION_INSTALL_FAILURE: &'static str = "android.intent.action.INSTALL_FAILURE";

    /**
    活动操作：启动即时应用程序安装程序。这是一个受保护的意图，只能由系统发送。
    */
    pub const ACTION_INSTALL_INSTANT_APP_PACKAGE: &'static str =
        "android.intent.action.INSTALL_INSTANT_APP_PACKAGE";

    /**
    服务操作：解决即时应用。系统将与此服务建立持久连接。这是一个受保护的意图，只能由系统发送。
    */
    pub const ACTION_RESOLVE_INSTANT_APP_PACKAGE: &'static str =
        "android.intent.action.RESOLVE_INSTANT_APP_PACKAGE";

    /**
    活动操作：启动即时应用设置。这是一个受保护的意图，只能由系统发送。
    */
    pub const ACTION_INSTANT_APP_RESOLVER_SETTINGS: &'static str =
        "android.intent.action.INSTANT_APP_RESOLVER_SETTINGS";

    /// 用作 ACTION_INSTALL_PACKAGE 的字符串额外字段来安装软件包。指定安装程序包名称；此软件包将接收 ACTION_APP_ERROR 意图。
    pub const EXTRA_INSTALLER_PACKAGE_NAME: &'static str =
        "android.intent.extra.INSTALLER_PACKAGE_NAME";

    /// 用作 ACTION_INSTALL_PACKAGE 的布尔额外字段来安装软件包。指定正在安装的应用程序不应被视为来自未知来源，而是来自调用 Intent 的应用程序。要实现此功能，您必须使用 startActivityForResult() 启动安装程序。
    pub const EXTRA_NOT_UNKNOWN_SOURCE: &'static str = "android.intent.extra.NOT_UNKNOWN_SOURCE";

    /// 用作 ACTION_INSTALL_PACKAGE 和 ACTION_VIEW 的 URI 额外字段，以指示 Intent 数据字段中的本地 APK 源自的 URI。
    pub const EXTRA_ORIGINATING_URI: &'static str = "android.intent.extra.ORIGINATING_URI";

    /// 此 extra 可与用于启动活动的任何 Intent 一起使用，提供有关谁在启动该活动的信息。此字段包含一个 Uri 对象，通常是引荐来源网站的 http: 或 https: URI；它还可以使用 android-app: 方案来标识它来自的本机应用程序。要在客户端中查询此值，请使用 Activity.getReferrer，而不是直接查询 extra。对于应用程序来说，在只能创建字符串而不是 Uri 的情况下，提供 EXTRA_REFERRER_NAME 也是有效的；但是，如果提供了此处的字段，则该字段将始终优先。
    pub const EXTRA_REFERRER: &'static str = "android.intent.extra.REFERRER";

    /// EXTRA_REFERRER 的替代版本，以字符串而非 Uri 对象的形式提供 URI。仅适用于无法创建 Uri 对象的情况，尤其是当通过 intent: 或 android-app: 方案提供 Intent extra 时。
    pub const EXTRA_REFERRER_NAME: &'static str = "android.intent.extra.REFERRER_NAME";

    /// 用作 ACTION_INSTALL_PACKAGE 和 ACTION_VIEW 的 int 额外字段，以指示启动安装的包的 uid 当前只有托管提供程序权限“下载”或持有权限 android.Manifest.permission.MANAGE_DOCUMENTS 的系统应用可以使用此功能。
    pub const EXTRA_ORIGINATING_UID: &'static str = "android.intent.extra.ORIGINATING_UID";

    /// 用作 ACTION_INSTALL_PACKAGE 的布尔额外字段来安装软件包。如果 .apk 正在替换现有 .apk，则告诉安装程序 UI 跳过用户确认。
    #[deprecated(
        note = "从 Build.VERSION_CODES.JELLY_BEAN 开始，Android 将不再显示有关更新现有应用程序的插页消息，因此不再需要此消息。"
    )]
    pub const EXTRA_ALLOW_REPLACE: &'static str = "android.intent.extra.ALLOW_REPLACE";

    /// 用作 ACTION_INSTALL_PACKAGE 或 ACTION_UNINSTALL_PACKAGE 的布尔额外字段。指定安装程序 UI 应向应用程序返回安装/卸载的结果代码。返回的结果代码将是 Activity. RESULT_OK（成功）或 Activity. RESULT_FIRST_USER（失败）。
    pub const EXTRA_RETURN_RESULT: &'static str = "android.intent.extra.RETURN_RESULT";

    /// 包管理器安装结果代码。@hide 因为结果代码尚未准备好公开。
    pub const EXTRA_INSTALL_RESULT: &'static str = "android.intent.extra.INSTALL_RESULT";

    /**
    活动操作：启动应用程序卸载程序。
    输入：数据必须是 package: URI，其方案特定部分是要卸载的当前安装包的包名称。您可以选择提供 EXTRA_RETURN_RESULT。
    输出：如果是 EXTRA_RETURN_RESULT，则返回卸载是否成功。需要 Manifest. 权限。REQUEST_DELETE_PACKAGES 自 Build. VERSION_CODES. P.
    */
    #[deprecated(note = "改用 android.content.pm.PackageInstaller.uninstall(String, IntentSender)")]
    pub const ACTION_UNINSTALL_PACKAGE: &'static str = "android.intent.action.UNINSTALL_PACKAGE";

    /**
    指定是否应为所有用户卸载该包。因为这些不应该是正常应用程序流程的一部分。
    */
    pub const EXTRA_UNINSTALL_ALL_USERS: &'static str = "android.intent.extra.UNINSTALL_ALL_USERS";

    /// 与元数据条目关联的字符串，指示设置的平台的最后运行版本。
    pub const METADATA_SETUP_VERSION: &'static str = "android.SETUP_VERSION";

    /**
    活动操作：启动 UI 来管理应用的权限。
    输入：EXTRA_PACKAGE_NAME 指定将由启动的 UI 管理其权限的包。
    输出：无。
    */
    pub const ACTION_MANAGE_APP_PERMISSIONS: &'static str =
        "android.intent.action.MANAGE_APP_PERMISSIONS";

    /**
    活动操作：启动 UI 以管理应用的特定权限组。
    输入：EXTRA_PACKAGE_NAME 指定其权限将由启动的 UI 管理的包。
    输入：EXTRA_PERMISSION_NAME 指定其组应由启动的 UI 管理的（单个）权限。
    输入：EXTRA_PERMISSION_GROUP_NAME 指定应由启动的 UI 管理的权限组。请勿同时发送此信息和 EXTRA_PERMISSION_NAME。 EXTRA_USER 指定拥有该应用的用户的 UserHandle。
    输出：无。
    */
    pub const ACTION_MANAGE_APP_PERMISSION: &'static str =
        "android.intent.action.MANAGE_APP_PERMISSION";

    /**
    活动操作：启动 UI 来管理权限。
    输入：无。
    输出：无。
    */
    pub const ACTION_MANAGE_PERMISSIONS: &'static str = "android.intent.action.MANAGE_PERMISSIONS";

    /**
    活动操作：启动 UI 来管理自动撤销状态。这相当于 Intent#ACTION_APPLICATION_DETAILS_SETTINGS
    输入：数据应为具有包名称的包方案 Uri，其自动撤销状态将被审核（强制）。例如 Uri.fromParts("package", packageName, null)
    输出：无。
    */
    pub const ACTION_AUTO_REVOKE_PERMISSIONS: &'static str =
        "android.intent.action.AUTO_REVOKE_PERMISSIONS";

    /**
    活动操作：启动 UI 来管理未使用的应用程序（休眠应用程序）。
    输入：无。
    输出：无。
    */
    pub const ACTION_MANAGE_UNUSED_APPS: &'static str = "android.intent.action.MANAGE_UNUSED_APPS";

    /**
    活动操作：启动 UI 以审查应用的权限。如果启用了不支持新运行时权限模型的应用的权限审查，系统将使用此意图。在此模式下，任何应用组件运行之前都需要进行权限审查。
    输入：EXTRA_PACKAGE_NAME 指定将审查其权限的软件包（必需）。
    输入：EXTRA_INTENT 指定在权限审查后要触发的待处理意图（可选）。
    输入：EXTRA_REMOTE_CALLBACK 指定在权限审查后要调用的回调（可选）。
    输入：EXTRA_RESULT_NEEDED 指定通过 EXTRA_INTENT 传递的意图是否需要结果（可选）。
    输出：无。
    */
    pub const ACTION_REVIEW_PERMISSIONS: &'static str = "android.intent.action.REVIEW_PERMISSIONS";

    /**
    活动操作：启动 UI 以显示有关给定权限组的使用情况的信息。此操作将由想要显示有关如何使用以及为何使用给定权限组的详细信息的应用处理。重要提示：您必须使用 START_VIEW_PERMISSION_USAGE 权限保护处理此操作的活动，以确保只有系统可以启动此活动。系统不会启动未得到适当保护的活动。
    输入：EXTRA_PERMISSION_GROUP_NAME 指定启动的 UI 所针对的权限组。
    输出：无。
    */
    pub const ACTION_VIEW_PERMISSION_USAGE: &'static str =
        "android.intent.action.VIEW_PERMISSION_USAGE";

    /**
    活动操作：启动 UI 以显示有关给定权限组在给定时间段内的使用情况的信息。此操作将由想要显示有关如何使用以及为何使用给定权限组的详细信息的应用处理。重要提示：您必须使用 Manifest. 权限保护处理此操作的活动。START_VIEW_PERMISSION_USAGE 权限以确保只有系统可以启动此活动。系统不会启动未得到适当保护的活动。
    输入：EXTRA_PERMISSION_GROUP_NAME 指定启动 UI 所针对的权限组。
    输入：EXTRA_ATTRIBUTION_TAGS 指定使用条目的归因标签。
    输入：EXTRA_START_TIME 指定时间段的开始时间（以毫秒为单位的纪元时间）。开始时间和结束时间都是必需的，并且开始时间必须 <= 结束时间。
    输入：EXTRA_END_TIME 指定时间段的结束时间（以毫秒为单位的纪元时间）。开始时间和结束时间都是必需的，并且开始时间必须 <= 结束时间。
    输出：无。
    */
    pub const ACTION_VIEW_PERMISSION_USAGE_FOR_PERIOD: &'static str =
        "android.intent.action.VIEW_PERMISSION_USAGE_FOR_PERIOD";

    /**
    活动行动：启动安全中心快​​速设置UI。
    输入：没有。
    输出：没有。
    */
    pub const ACTION_VIEW_SAFETY_CENTER_QS: &'static str =
        "android.intent.action.VIEW_SAFETY_CENTER_QS";

    /**
    活动操作：启动 UI 来管理默认应用。
    输入：EXTRA_ROLE_NAME 指定将由启动的 UI 管理的默认应用的角色。
    输出：无。
    */
    pub const ACTION_MANAGE_DEFAULT_APP: &'static str = "android.intent.action.MANAGE_DEFAULT_APP";

    /// Intent extra：角色名称。类型：字符串
    pub const EXTRA_ROLE_NAME: &'static str = "android.intent.extra.ROLE_NAME";

    /**
    活动操作：启动 UI 来管理特殊应用访问。
    输入：无。
    输出：无。
    */
    pub const ACTION_MANAGE_SPECIAL_APP_ACCESSES: &'static str =
        "android.intent.action.MANAGE_SPECIAL_APP_ACCESSES";

    /// Intent extra：用于将远程结果作为捆绑包报告的回调。类型：IRemoteCallback
    pub const EXTRA_REMOTE_CALLBACK: &'static str = "android.intent.extra.REMOTE_CALLBACK";

    /// Intent extra：应用包名称。类型：字符串
    pub const EXTRA_PACKAGE_NAME: &'static str = "android.intent.extra.PACKAGE_NAME";

    /// Intent extra：android.os.LocaleList 类型：LocaleList
    pub const EXTRA_LOCALE_LIST: &'static str = "android.intent.extra.LOCALE_LIST";

    /// Intent extra：被暂停的软件包的 extra 包。将作为 ACTION_MY_PACKAGE_SUSPENDED 的 extra 发送。此 Bundle 的内容是被暂停的应用和正在暂停的应用（即具有权限 android.permission.SUSPEND_APPS 的任何应用）之间的契约。这是为了使被暂停的应用能够更好地处理被暂停的状态。
    pub const EXTRA_SUSPENDED_PACKAGE_EXTRAS: &'static str =
        "android.intent.extra.SUSPENDED_PACKAGE_EXTRAS";

    /// Intent extra：应用拆分名称。类型：字符串
    pub const EXTRA_SPLIT_NAME: &'static str = "android.intent.extra.SPLIT_NAME";

    /**
    Intent extra：ComponentName 值。类型：字符串
    */
    pub const EXTRA_COMPONENT_NAME: &'static str = "android.intent.extra.COMPONENT_NAME";

    /// Intent extra：用于指定是否需要结果的额外信息。类型：布尔值
    pub const EXTRA_RESULT_NEEDED: &'static str = "android.intent.extra.RESULT_NEEDED";

    /// Intent extra：用于发送分享意图的快捷方式 ID。将随 ACTION_SEND 一起发送。
    /// 类型：字符串
    pub const EXTRA_SHORTCUT_ID: &'static str = "android.intent.extra.shortcut.ID";

    /**
    活动操作：启动 UI 来管理哪些应用具有给定权限。
    输入：EXTRA_PERMISSION_NAME 或 EXTRA_PERMISSION_GROUP_NAME 指定将由启动的 UI 管理的权限组。
    输出：无。
    */
    pub const ACTION_MANAGE_PERMISSION_APPS: &'static str =
        "android.intent.action.MANAGE_PERMISSION_APPS";

    /// 意图额外：许可的名称。 类型：字符串
    pub const EXTRA_PERMISSION_NAME: &'static str = "android.intent.extra.PERMISSION_NAME";

    /// Intent extra：权限组的名称。类型：字符串
    pub const EXTRA_PERMISSION_GROUP_NAME: &'static str =
        "android.intent.extra.PERMISSION_GROUP_NAME";

    /// Intent extra：毫秒数。类型：long
    pub const EXTRA_DURATION_MILLIS: &'static str = "android.intent.extra.DURATION_MILLIS";

    /**
    活动操作：启动 UI 以查看应用对权限的使用情况。
    输入：EXTRA_PERMISSION_NAME 指定启动的 UI 将显示的权限名称。不要同时传递此值和 EXTRA_PERMISSION_GROUP_NAME。
    输入：EXTRA_PERMISSION_GROUP_NAME 指定启动的 UI 将显示的权限组名称。不要同时传递此值和 EXTRA_PERMISSION_NAME。
    输入：EXTRA_DURATION_MILLIS 指定要显示的最近活动的最小毫秒数（可选）。必须为非负数。
    输出：无。这需要 android.Manifest.permission#GRANT_RUNTIME_PERMISSIONS 权限。
    */
    pub const ACTION_REVIEW_PERMISSION_USAGE: &'static str =
        "android.intent.action.REVIEW_PERMISSION_USAGE";

    /**
    活动操作：启动 UI 以查看权限的时间线历史记录。
    输入：EXTRA_PERMISSION_GROUP_NAME 指定启动的 UI 将显示的权限组名称。
    输出：无。这需要 android.Manifest.permission#GRANT_RUNTIME_PERMISSIONS 权限。
    */
    pub const ACTION_REVIEW_PERMISSION_HISTORY: &'static str =
        "android.intent.action.REVIEW_PERMISSION_HISTORY";

    /**
    活动操作：启动 UI 以查看应用程序正在使用的权限。
    输入：EXTRA_DURATION_MILLIS 指定要显示的最近活动的最小毫秒数（可选）。必须为非负数。
    输出：无。这需要 android.Manifest.permission#GRANT_RUNTIME_PERMISSIONS 权限。
    */
    pub const ACTION_REVIEW_ONGOING_PERMISSION_USAGE: &'static str =
        "android.intent.action.REVIEW_ONGOING_PERMISSION_USAGE";

    /**
    活动动作：启动UI来审查运行可访问性服务。
    输入：没有。
    输出：没有。
    */
    pub const ACTION_REVIEW_ACCESSIBILITY_SERVICES: &'static str =
        "android.intent.action.REVIEW_ACCESSIBILITY_SERVICES";

    /**
    活动操作：启动 UI 来管理给定权限组的使用情况。此操作将由想要显示使用该权限组的功能的控件的应用处理。
    输入：EXTRA_PERMISSION_GROUP_NAME 指定启动的 UI 所针对的权限组。
    输入：EXTRA_ATTRIBUTION_TAGS 指定使用条目的归因标签。
    输入：EXTRA_START_TIME 指定时间段的开始时间（纪元时间，以毫秒为单位）。如果同时存在开始时间和结束时间，则开始时间必须是
    输出：无。您必须使用 android.Manifest.permission#START_VIEW_PERMISSION_USAGE 权限保护处理此操作的活动，以确保只有系统可以启动此活动。系统不会启动未得到适当保护的活动。
    */
    pub const ACTION_MANAGE_PERMISSION_USAGE: &'static str =
        "android.intent.action.MANAGE_PERMISSION_USAGE";

    /**
    活动操作：启动UI以查看应用程序的功能信息。
    输出：无。您必须使用 android.Manifest.permission#START_VIEW_APP_FEATURES 权限保护处理此操作的 Activity，以确保只有系统可以启动此 Activity。系统不会启动未得到适当保护的 Activity。
    活动清单中带有 android:name=app_features_preference_summary 和 android:resource=@string/ 的可选标签将用于为设置中的“所有服务”偏好设置添加摘要行。
    */
    pub const ACTION_VIEW_APP_FEATURES: &'static str = "android.intent.action.VIEW_APP_FEATURES";

    /**
    活动操作：启动 UI 以打开安全中心，突出显示用户的安全和隐私状态。
    */
    pub const ACTION_SAFETY_CENTER: &'static str = "android.intent.action.SAFETY_CENTER";

    /**
    活动操作：启动 UI 以查看已安装的应用程序对其安全标签中的数据共享策略所做的最新更新。
    输入：无。
    输出：无。
    此意图操作需要 android.Manifest.permission#GRANT_RUNTIME_PERMISSIONS 权限。
    */
    pub const ACTION_REVIEW_APP_DATA_SHARING_UPDATES: &'static str =
        "android.intent.action.REVIEW_APP_DATA_SHARING_UPDATES";

    /**
    广播操作：当设备进入睡眠状态并变为非交互状态时发送。由于历史原因，此广播操作的名称指的是屏幕的电源状态，但实际上它是响应设备整体交互状态的变化而发送的。当设备变为非交互状态时发送此广播，这可能与屏幕关闭无关。要确定屏幕的实际状态，请使用 android.view.Display#getState。有关详细信息，请参阅 android.os.PowerManager#isInteractive。您无法通过清单中声明的​​组件接收此信息，只能通过使用 Context#registerReceiver(BroadcastReceiver, IntentFilter) Context.registerReceiver() 明确注册。
    这是一个受保护的意图，只能由系统发送。
    */
    pub const ACTION_SCREEN_OFF: &'static str = "android.intent.action.SCREEN_OFF";

    /**
    广播操作：当设备唤醒并变为可交互时发送。由于历史原因，此广播操作的名称指的是屏幕的电源状态，但实际上它是响应设备整体交互状态的变化而发送的。当设备变为可交互时发送此广播，这可能与屏幕打开无关。要确定屏幕的实际状态，请使用 android.view.Display#getState。有关详细信息，请参阅 android.os.PowerManager#isInteractive。您无法通过清单中声明的​​组件接收此信息，只能通过使用 Context#registerReceiver(BroadcastReceiver, IntentFilter) Context.registerReceiver() 明确注册。
    这是一个受保护的意图，只能由系统发送。
    */
    pub const ACTION_SCREEN_ON: &'static str = "android.intent.action.SCREEN_ON";

    /**
    广播动作：系统停止做梦后发送。
    这是一个受保护的意图，只能由系统发送。它只发送给已注册的接收者。
    */
    pub const ACTION_DREAMING_STOPPED: &'static str = "android.intent.action.DREAMING_STOPPED";

    /**
    广播动作：系统开始做梦后发送。
    这是一个受保护的意图，只能由系统发送。它只发送给已注册的接收者。
    */
    pub const ACTION_DREAMING_STARTED: &'static str = "android.intent.action.DREAMING_STARTED";

    /**
    广播动作：当设备唤醒后用户在场时发送（例如，当键盘锁消失时）。
    这是一个受保护的意图，只能由系统发送。
    */
    pub const ACTION_USER_PRESENT: &'static str = "android.intent.action.USER_PRESENT";

    /**
    广播操作：当前时间已更改。每分钟发送一次。您无法通过清单中声明的​​组件接收此信息，只能通过使用 Context#registerReceiver(BroadcastReceiver, IntentFilter) Context.registerReceiver() 明确注册来接收。
    这是一个受保护的意图，只能由系统发送。
    */
    pub const ACTION_TIME_TICK: &'static str = "android.intent.action.TIME_TICK";

    /**
    广播动作：时间已设置。
    */
    pub const ACTION_TIME_CHANGED: &'static str = "android.intent.action.TIME_SET";

    /**
    广播动作：日期已更改。
    */
    pub const ACTION_DATE_CHANGED: &'static str = "android.intent.action.DATE_CHANGED";

    /**
    广播操作：时区已更改。此意图将具有以下额外值：EXTRA_TIMEZONE - 标识新时区的 java.util.TimeZone.getID() 值。
    这是一个受保护的意图，只能由系统发送。
    */
    pub const ACTION_TIMEZONE_CHANGED: &'static str = "android.intent.action.TIMEZONE_CHANGED";

    /**
    闹钟更改操作：当 AlarmClock 应用程序的闹钟设置或取消设置时，将广播此操作。它由 AlarmClock 应用程序和 StatusBar 服务使用。
    */
    pub const ACTION_ALARM_CHANGED: &'static str = "android.intent.action.ALARM_CHANGED";

    /**
    广播操作：在用户完成启动后但仍处于“锁定”状态时，广播一次。它可用于执行特定于应用程序的初始化，例如安装闹钟。您必须拥有 android.Manifest.permission#RECEIVE_BOOT_COMPLETED 权限才能接收此广播。所有运行 android.os.Build.VERSION_CODES#N 或更高版本的设备（无论是否支持直接启动）在启动时都会立即发送此广播。
    收到此广播后，用户仍处于锁定状态，只能安全访问受设备保护的存储。如果您想访问受凭据保护的存储，您需要等待用户解锁（通常是通过首次输入锁定图案或 PIN），然后发送 ACTION_USER_UNLOCKED 和 ACTION_BOOT_COMPLETED 广播。要接收此广播，您的接收器组件必须标记为 ComponentInfo#directBootAware。这是一个受保护的意图，只能由系统发送。
    */
    pub const ACTION_LOCKED_BOOT_COMPLETED: &'static str =
        "android.intent.action.LOCKED_BOOT_COMPLETED";

    /**
    广播操作：用户完成启动后，广播一次。它可用于执行特定于应用程序的初始化，例如安装闹钟。您必须拥有 android.Manifest.permission#RECEIVE_BOOT_COMPLETED 权限才能接收此广播。所有设备（无论是否支持直接启动）在启动时都会发送此广播。
    收到此广播后，用户将被解锁，设备保护和凭据保护的存储都可以安全访问。如果您需要在用户仍处于锁定状态时运行（在他们首次输入锁定图案或 PIN 之前），您可以监听 ACTION_LOCKED_BOOT_COMPLETED 广播。这是一个受保护的意图，只能由系统发送。
    */
    pub const ACTION_BOOT_COMPLETED: &'static str = "android.intent.action.BOOT_COMPLETED";

    /**
    广播操作：当用户操作应请求关闭临时系统对话框时，将广播此操作。临时系统对话框的一些示例包括通知窗口阴影和最近任务对话框。
    */
    #[deprecated(
        note = "出于安全原因，从 Android Build.VERSION_CODES#S 开始，第三方应用已弃用此 Intent。如果应用未经授权使用，则会导致针对低于 Build.VERSION_CODES#S 的 API 级别的应用放弃广播 Intent，而针对 Build.VERSION_CODES#S 或更高 SDK 级别的应用将引发 SecurityException。从 shell 启动的检测（例如测试）仍可使用该 Intent。平台将在适当的用例中自动折叠适当的系统对话框。对于所有其他情况，用户是控制对话框关闭的人。"
    )]
    pub const ACTION_CLOSE_SYSTEM_DIALOGS: &'static str =
        "android.intent.action.CLOSE_SYSTEM_DIALOGS";

    /**
    广播操作：触发包的下载和最终安装。
    输入：getData 是要下载的包文件的 URI。
    这是一个受保护的意图，只能由系统发送。
    */
    #[deprecated(note = "此常数从未被使用过。")]
    pub const ACTION_PACKAGE_INSTALL: &'static str = "android.intent.action.PACKAGE_INSTALL";

    /**
    广播操作：设备上已安装新的应用程序包。数据包含包的名称。请注意，新安装的包不会接收此广播。可能包含以下额外信息：EXTRA_UID，其中包含分配给新包的整数 uid。如果此操作紧随针对同一包的 ACTION_PACKAGE_REMOVED 广播，则 EXTRA_REPLACING 设置为 true。
    这是一个受保护的意图，只能由系统发送。
    */
    pub const ACTION_PACKAGE_ADDED: &'static str = "android.intent.action.PACKAGE_ADDED";

    /**
    广播操作：已安装应用程序包的新版本，替换之前安装的现有版本。数据包含包的名称。可能包括以下额外信息：EXTRA_UID，包含分配给新包的整数 uid。
    这是一个受保护的意图，只能由系统发送。
    */
    pub const ACTION_PACKAGE_REPLACED: &'static str = "android.intent.action.PACKAGE_REPLACED";

    /**
    广播操作：您的应用程序的新版本已安装在现有版本上。此操作仅发送给被替换的应用程序。它不包含任何其他数据；要接收它，只需使用此操作的意图过滤器即可。
    这是一个受保护的意图，只能由系统发送。
    */
    pub const ACTION_MY_PACKAGE_REPLACED: &'static str =
        "android.intent.action.MY_PACKAGE_REPLACED";

    /**
    广播操作：现有应用程序包已从设备中删除。数据包含包的名称。正在删除的包不会接收此 Intent。EXTRA_UID 包含先前分配给包的整数 uid。如果要删除整个应用程序（数据和代码），则将 EXTRA_DATA_REMOVED 设置为 true。如果随后将针对同一包进行 ACTION_PACKAGE_ADDED 广播，则将 EXTRA_REPLACING 设置为 true。EXTRA_USER_INITIATED 包含布尔字段，用于表示应用程序已通过用户启动的操作被删除。
    这是一个受保护的意图，只能由系统发送。
    */
    pub const ACTION_PACKAGE_REMOVED: &'static str = "android.intent.action.PACKAGE_REMOVED";

    /**
    广播操作：现有应用程序包已从设备中删除。数据包含包的名称和可见性允许列表。正在删除的包不会接收此 Intent。EXTRA_UID 包含先前分配给包的整数 uid。如果要删除整个应用程序（数据和代码），则将 EXTRA_DATA_REMOVED 设置为 true。如果随后将针对同一包进行 ACTION_PACKAGE_ADDED 广播，则将 EXTRA_REPLACING 设置为 true。EXTRA_USER_INITIATED 包含布尔字段，用于表示应用程序已通过用户启动的操作被删除。EXTRA_VISIBILITY_ALLOW_LIST 包含一个 int 数组，用于指示可见性允许列表。
    这是一个受保护的意图，只能由系统发送。此广播由系统内部使用。
    */
    pub const ACTION_PACKAGE_REMOVED_INTERNAL: &'static str =
        "android.intent.action.PACKAGE_REMOVED_INTERNAL";

    /**
    广播操作：现有应用程序包已从设备中完全移除。数据包含包的名称。这类似于 ACTION_PACKAGE_REMOVED，但仅在该广播的 EXTRA_DATA_REMOVED 为 true 且 EXTRA_REPLACING 为 false 时设置。
    EXTRA_UID 包含先前分配给包的整数 uid。
    这是一个受保护的意图，只能由系统发送。
    */
    pub const ACTION_PACKAGE_FULLY_REMOVED: &'static str =
        "android.intent.action.PACKAGE_FULLY_REMOVED";

    /**
    广播操作：现有应用程序包已更改（例如，已启用或禁用组件）。数据包含包的名称。EXTRA_UID 包含分配给包的整数 uid。EXTRA_CHANGED_COMPONENT_NAME_LIST 包含已更改组件的类名（或包名本身）。EXTRA_DONT_KILL_APP 包含布尔字段，用于覆盖重新启动应用程序的默认操作。
    这是一个受保护的意图，只能由系统发送。
    */
    pub const ACTION_PACKAGE_CHANGED: &'static str = "android.intent.action.PACKAGE_CHANGED";

    /**
    广播操作：当包需要启用回滚时，发送给系统回滚管理器。这是一个受保护的意图，只能由系统发送。此广播由系统内部使用。
    */
    pub const ACTION_PACKAGE_ENABLE_ROLLBACK: &'static str =
        "android.intent.action.PACKAGE_ENABLE_ROLLBACK";

    /**
    广播动作：当需要取消某个包的回滚时，发送给系统回滚管理器。
    此意图由 PackageManagerService 发送，用于通知 RollbackManager 启用特定回滚已超时。
    */
    pub const ACTION_CANCEL_ENABLE_ROLLBACK: &'static str =
        "android.intent.action.CANCEL_ENABLE_ROLLBACK";

    /**
    广播动作：已提交回滚。
    这是一个受保护的意图，只能由系统发送。接收者必须拥有 MANAGE_ROLLBACK 权限。
    */
    pub const ACTION_ROLLBACK_COMMITTED: &'static str = "android.intent.action.ROLLBACK_COMMITTED";

    /**
    广播操作：询问系统服务是否有理由重新启动给定的包。数据包含包的名称。EXTRA_UID 包含分配给包的整数 uid。EXTRA_PACKAGES 要检查的所有包的字符串数组。
    这是一个受保护的意图，只能由系统发送。
    */
    pub const ACTION_QUERY_PACKAGE_RESTART: &'static str =
        "android.intent.action.QUERY_PACKAGE_RESTART";

    /**
    广播动作：用户重新启动了一个包裹，其所有流程都被杀死。 所有与之关联的运行时状态（过程，警报，通知等）均应删除。 请注意，重新启动的软件包未收到此广播。数据包含软件包的名称。 Extra_UID包含分配给包装的整数UID。
    这是一个受保护的意图，只能由系统发送。
    */
    pub const ACTION_PACKAGE_RESTARTED: &'static str = "android.intent.action.PACKAGE_RESTARTED";

    /**
    广播操作：用户已清除软件包的数据。此操作应以 ACTION_PACKAGE_RESTARTED 为前提，之后将清除其所有持久数据并发送此广播。请注意，已清除的软件包不会收到此广播。数据包含软件包的名称。EXTRA_UID 包含分配给软件包的整数 uid。如果已清除数据的软件包是已卸载的免安装应用，则 UID 将为 -1。平台会在卸载免安装应用后保留一些与之关联的元数据。EXTRA_PACKAGE_NAME 仅当清除的数据用于免安装应用时才包含软件包名称。
    这是一个受保护的意图，只能由系统发送。
    */
    pub const ACTION_PACKAGE_DATA_CLEARED: &'static str =
        "android.intent.action.PACKAGE_DATA_CLEARED";

    /**
    广播操作：软件包已被暂停。包括以下附加信息：EXTRA_CHANGED_PACKAGE_LIST 是已暂停的软件包集合 EXTRA_CHANGED_UID_LIST 是已暂停的 uid 集合
    这是一个受保护的意图，只能由系统发送。它只发送给已注册的接收者。
    */
    pub const ACTION_PACKAGES_SUSPENDED: &'static str = "android.intent.action.PACKAGES_SUSPENDED";

    /**
    广播操作：软件包已取消暂停。包括以下附加信息：EXTRA_CHANGED_PACKAGE_LIST 是已取消暂停的软件包集合 EXTRA_CHANGED_UID_LIST 是已取消暂停的 uid 集合
    这是一个受保护的意图，只能由系统发送。它只发送给已注册的接收者。
    */
    pub const ACTION_PACKAGES_UNSUSPENDED: &'static str =
        "android.intent.action.PACKAGES_UNSUSPENDED";

    /**
    广播操作：软件包的暂停条件之一已被修改。包括以下附加信息：EXTRA_CHANGED_PACKAGE_LIST 是已修改的软件包集 EXTRA_CHANGED_UID_LIST 是已修改的 uid 集
    这是一个受保护的意图，只能由系统发送。它只发送给已注册的接收者。
    */
    pub const ACTION_PACKAGES_SUSPENSION_CHANGED: &'static str =
        "android.intent.action.PACKAGES_SUSPENSION_CHANGED";

    /**
    广播操作：分散注意力的软件包已发生更改。包括以下额外内容：EXTRA_CHANGED_PACKAGE_LIST 是已更改的软件包集。EXTRA_CHANGED_UID_LIST 是已更改的 uid 集。EXTRA_DISTRACTION_RESTRICTIONS 是针对这些软件包设置的新限制。
    这是一个受保护的意图，只能由系统发送。它只发送给已注册的接收者。
    */
    pub const ACTION_DISTRACTING_PACKAGES_CHANGED: &'static str =
        "android.intent.action.DISTRACTING_PACKAGES_CHANGED";

    /**
    广播操作：发送给已被系统暂停的软件包。每当软件包处于暂停状态或其任何应用附加功能在暂停状态下发生变化时，都会发送此操作。可选地包括以下附加功能：EXTRA_SUSPENDED_PACKAGE_EXTRAS，这是一个 Bundle，其中包含有关暂停应用的有用信息。
    这是一个受保护的意图，只能由系统发送。这将传递给清单中声明的​​ BroadcastReceiver 组件。
    */
    pub const ACTION_MY_PACKAGE_SUSPENDED: &'static str =
        "android.intent.action.MY_PACKAGE_SUSPENDED";

    /**
    活动操作：开始显示有关应用程序被暂停的原因的更多详细信息。
    每当系统检测到已暂停应用程序的活动启动时，可以使用此操作显示有关暂停原因的更多详细信息。
    持有 android.Manifest.permission#SUSPEND_APPS 的应用程序必须声明一个处理此意图的活动并使用 android.Manifest.permission#SEND_SHOW_SUSPENDED_APP_DETAILS 保护它。
    包含一个额外的 EXTRA_PACKAGE_NAME，它是暂停包的名称。
    这是一个受保护的意图，只能由系统发送。
    */
    pub const ACTION_SHOW_SUSPENDED_APP_DETAILS: &'static str =
        "android.intent.action.SHOW_SUSPENDED_APP_DETAILS";

    //noinspection SpellCheckingInspection
    /**
    广播动作：发送以表明用户取消暂停某个套餐。
    当用户点击使用 SuspendDialogInfo#BUTTON_ACTION_UNSUSPEND 创建的 SuspendDialogInfo 暂停对话框的中性按钮时，可能会发生这种情况。此广播仅发送给在调用 PackageManager#setPackagesSuspended(String[], boolean, PersistableBundle, PersistableBundle, SuspendDialogInfo) 时最初指定此对话框的暂停应用。
    包含一个额外的 EXTRA_PACKAGE_NAME，它是刚刚取消暂停的包的名称。
    这是一个受保护的意图，只能由系统发送。它将被传递给清单中声明的​​ BroadcastReceiver 组件。
    */
    pub const ACTION_PACKAGE_UNSUSPENDED_MANUALLY: &'static str =
        "android.intent.action.PACKAGE_UNSUSPENDED_MANUALLY";

    /**
    广播动作：发送给已经取消暂停的包裹。
    这是一个受保护的意图，只能由系统发送。它将被传递给清单中声明的​​ BroadcastReceiver 组件。
    */
    pub const ACTION_MY_PACKAGE_UNSUSPENDED: &'static str =
        "android.intent.action.MY_PACKAGE_UNSUSPENDED";

    /**
    广播动作：已从系统中删除一个 uid。uid 号存储在 EXTRA_UID 下的额外数据中。
    在某些情况下，如果 UID 未被完全删除，则 EXTRA_REPLACING 设置为 true。
    这是一个受保护的意图，只能由系统发送。
    */
    pub const ACTION_UID_REMOVED: &'static str = "android.intent.action.UID_REMOVED";

    /**
    广播操作：当应用程序首次启动时（即首次退出停止状态时），发送给应用程序的安装程序包。数据包含包的名称。
    当应用程序首次启动时，应用程序本身不会接收该广播。
    这是一个受保护的意图，只能由系统发送。
    */
    pub const ACTION_PACKAGE_FIRST_LAUNCH: &'static str =
        "android.intent.action.PACKAGE_FIRST_LAUNCH";

    /**
    广播操作：当需要验证包裹时，发送给系统包裹验证器。数据包含包裹 URI。这是一个受保护的意图，只能由系统发送。
    */
    pub const ACTION_PACKAGE_NEEDS_VERIFICATION: &'static str =
        "android.intent.action.PACKAGE_NEEDS_VERIFICATION";

    /**
    广播操作：当包裹被验证时发送给系统包裹验证器。数据包含包裹 URI。这是一个受保护的意图，只能由系统发送。
    */
    pub const ACTION_PACKAGE_VERIFIED: &'static str = "android.intent.action.PACKAGE_VERIFIED";

    /**
    广播操作：当需要验证意图过滤器时，发送给系统意图过滤器验证器。数据包含要验证的过滤器数据主机。这是一个受保护的意图，只能由系统发送。
    */
    #[deprecated(note = "已被域验证 API 取代。请参阅 DomainVerificationManager。")]
    pub const ACTION_INTENT_FILTER_NEEDS_VERIFICATION: &'static str =
        "android.intent.action.INTENT_FILTER_NEEDS_VERIFICATION";

    /**
    广播操作：当应用的域需要验证时，发送给系统域验证代理。数据包含要验证的域主机。这是一个受保护的意图，只能由系统发送。
    */
    pub const ACTION_DOMAINS_NEED_VERIFICATION: &'static str =
        "android.intent.action.DOMAINS_NEED_VERIFICATION";

    /**
    广播操作：一组软件包（以前不可用）的资源目前可用，因为它们所在的介质可用。额外数据 EXTRA_CHANGED_PACKAGE_LIST 包含可用性已发生改变的软件包列表。额外数据 EXTRA_CHANGED_UID_LIST 包含可用性已发生改变的软件包的 uid 列表。请注意，此列表中的软件包不会收到此广播。指定的软件包集现在在系统上可用。包括以下额外内容：EXTRA_CHANGED_PACKAGE_LIST 是资源（以前不可用）目前可用的软件包集。EXTRA_CHANGED_UID_LIST 是资源（以前不可用）目前可用的软件包的 uid 集。
    这是一个受保护的意图，只能由系统发送。
    */
    pub const ACTION_EXTERNAL_APPLICATIONS_AVAILABLE: &'static str =
        "android.intent.action.EXTERNAL_APPLICATIONS_AVAILABLE";

    /**
    广播动作：一组软件包的资源目前不可用，因为它们存在的媒体是不可用的。额外的数据EXTRA_CHANGED_PACKAGE_LIST包含一个可用性更改的软件包列表。额外的数据EXTRA_CHANGED_UID_LIST包含可用性更改的软件包的列表。指定的一组软件包不再可以启动，并且在系统上实际上不可用。包括以下附加内容：EXTRA_CHANGED_PACKAGE_LIST是一组不再可用的软件包。 EXTRA_CHANGED_UID_LIST是一组软件包，其资源不再可用。
    这是一个受保护的意图，只能由系统发送。
    */
    pub const ACTION_EXTERNAL_APPLICATIONS_UNAVAILABLE: &'static str =
        "android.intent.action.EXTERNAL_APPLICATIONS_UNAVAILABLE";

    /**
    广播行动：首选活动已*明确*改变。
    请注意，在某些情况下，首选活动会被*隐式*无效，例如当安装或卸载应用程序时，但在这种情况下，此广播将*不会*被发送。
    EXTRA_USER_HANDLE 包含相关的用户 ID。
    */
    pub const ACTION_PREFERRED_ACTIVITY_CHANGED: &'static str =
        "android.intent.action.ACTION_PREFERRED_ACTIVITY_CHANGED";

    /**
    广播动作：当前系统壁纸已更改。请参阅 android.app.WallpaperManager 以查询新壁纸。这只应用于确定壁纸何时更改以向用户显示新壁纸。您绝对不应该为了响应此消息而更改壁纸或其其他属性（例如建议尺寸）。那会出乎意料，对吧？您会导致各种循环，尤其是当其他应用程序也在做类似的事情时，对吧？当然。所以请不要这样做。
    */
    #[deprecated(
        note = "现代应用程序应该使用 android.view.WindowManager.LayoutParams#FLAG_SHOW_WALLPAPER WindowManager.LayoutParams.FLAG_SHOW_WALLPAPER 在其 UI 后面显示壁纸，而不是监视此广播并自行渲染壁纸。"
    )]
    pub const ACTION_WALLPAPER_CHANGED: &'static str = "android.intent.action.WALLPAPER_CHANGED";

    /**
    广播操作：当前设备的 android.content.res.Configuration（方向、语言环境等）已更改。发生此类更改时，需要根据此新信息重建 UI（视图层次结构）；大多数情况下，应用程序无需担心这一点，因为系统将负责停止并重新启动应用程序以确保其看到新地更改。某些无法重新启动的系统代码将需要监视此操作并进行适当处理。
    您无法通过清单中声明的​​组件来接收该信息，只能通过使用 Context#registerReceiver(BroadcastReceiver, IntentFilter) Context.registerReceiver() 明确注册来接收该信息。
    这是一个受保护的意图，只能由系统发送。
    */
    pub const ACTION_CONFIGURATION_CHANGED: &'static str =
        "android.intent.action.CONFIGURATION_CHANGED";

    /**
    广播动作：当前设备Android.content.Res.Configuration已更改，因此该设备可能有资格安装其他配置拆分。可以触发此广播的配置属性包括位置和显示密度。
    与 ACTION_CONFIGURATION_CHANGED 不同，您可以通过清单中声明的​​组件接收此消息。但是，接收者必须拥有 android.Manifest.permission#INSTALL_PACKAGES 权限。
    这是一个受保护的意图，只能由系统发送。
    */
    pub const ACTION_SPLIT_CONFIGURATION_CHANGED: &'static str =
        "android.intent.action.SPLIT_CONFIGURATION_CHANGED";

    /**
    广播动作：接收者的有效区域设置已经改变。
    当设备区域设置、接收应用程序的区域设置（通过 android.app.LocaleManager#setApplicationLocales 设置）或区域偏好设置的语言标签发生变化时，就会发生这种情况。
    可以被清单声明的接收器接收。
    如果仅应用程序语言环境发生变化，则包含以下附加信息：EXTRA_PACKAGE_NAME 是语言环境发生变化的软件包的名称。EXTRA_LOCALE_LIST 包含当前为指定应用程序设置的语言环境
    这是一个受保护的意图，只能由系统发送。
    */
    pub const ACTION_LOCALE_CHANGED: &'static str = "android.intent.action.LOCALE_CHANGED";

    /**
    广播动作：特定应用程序的区域设置已更改。
    此广播明确发送到语言环境已更改的应用的 android.content.pm.InstallSourceInfo#getInstallingPackageName 安装程序。此广播也可以由清单声明的接收器接收，其中的“ ”
    这是一个受保护的意图，只能由系统发送。
    包括以下附加内容：EXTRA_PACKAGE_NAME 是更改语言环境的软件包的名称。EXTRA_LOCALE_LIST 包含当前为指定应用设置的语言环境
    */
    pub const ACTION_APPLICATION_LOCALE_CHANGED: &'static str =
        "android.intent.action.APPLICATION_LOCALE_CHANGED";

    /**
    广播操作：这是一个粘性广播，包含充电状态、电量水平和有关电池的其他信息。有关 Intent 内容的文档，请参阅 android.os.BatteryManager。
    您无法通过清单中声明的​​组件接收此信息，只能通过使用 Context#registerReceiver(BroadcastReceiver, IntentFilter) Context.registerReceiver() 明确注册。请参阅 ACTION_BATTERY_LOW、ACTION_BATTERY_OKAY、ACTION_POWER_CONNECTED 和 ACTION_POWER_DISCONNECTED 了解通过清单接收器发送和接收的不同电池相关广播。
    这是一个受保护的意图，只能由系统发送。
    */
    pub const ACTION_BATTERY_CHANGED: &'static str = "android.intent.action.BATTERY_CHANGED";

    /**
    广播动作：当当前电池电量或插头类型发生变化时发送。
    它具有android.os.BatteryManager#EXTRA_EVENTS，它带有代表单个电池级别变化的捆绑列表，并随着ACTION_BATTERY_CHANGED的相关额外范围而变化。
    此广播需要 android.Manifest.permission#BATTERY_STATS 权限。
    */
    pub const ACTION_BATTERY_LEVEL_CHANGED: &'static str =
        "android.intent.action.BATTERY_LEVEL_CHANGED";

    /**
    广播动作：指示设备电池电量不足。此广播对应于“电池电量不足警告”系统对话框。
    这是一个受保护的意图，只能由系统发送。
    */
    pub const ACTION_BATTERY_LOW: &'static str = "android.intent.action.BATTERY_LOW";

    /**
    广播操作：表示电池电量低后现在恢复正常。一旦电池电量恢复正常，将在 ACTION_BATTERY_LOW 之后发送此操作。
    这是只能由系统发送的受保护意图。
    */
    pub const ACTION_BATTERY_OKAY: &'static str = "android.intent.action.BATTERY_OKAY";

    /**
    广播操作：外部电源已连接到设备。这适用于希望专门注册此通知的应用程序。与 ACTION_BATTERY_CHANGED 不同，应用程序将为此被唤醒，因此无需保持活动状态即可接收此通知。此操作可用于实现等待电源可用时触发的操作。
    这是一个受保护的意图，只能由系统发送。
    */
    pub const ACTION_POWER_CONNECTED: &'static str = "android.intent.action.ACTION_POWER_CONNECTED";

    /**
    广播操作：外部电源已从设备上移除。这适用于希望专门注册此通知的应用程序。与 ACTION_BATTERY_CHANGED 不同，应用程序将为此被唤醒，因此无需保持活动状态即可接收此通知。此操作可用于实现等待电源可用时触发的操作。
    这是一个受保护的意图，只能由系统发送。
    */
    pub const ACTION_POWER_DISCONNECTED: &'static str =
        "android.intent.action.ACTION_POWER_DISCONNECTED";

    /**
    广播操作：设备正在关闭。当设备正在关闭（完全关闭，而不是休眠）时，会广播此消息。广播完成后，将进行最终关闭，所有未保存的数据将丢失。应用通常不需要处理此问题，因为前台活动也将暂停。从 Build.VERSION_CODES#P 开始，此广播仅发送给通过 Context#registerReceiver(BroadcastReceiver, IntentFilter) Context.registerReceiver 注册的接收器。
    这是一个受保护的意图，只能由系统发送。可能包括以下额外内容：EXTRA_SHUTDOWN_USERSPACE_ONLY 布尔值，如果此关闭仅适用于用户空间进程，则设置为 true。如果未设置，则假定为 false。
    */
    pub const ACTION_SHUTDOWN: &'static str = "android.intent.action.ACTION_SHUTDOWN";

    /**
    活动操作：启动此活动以请求系统关闭。可选布尔额外字段 EXTRA_KEY_CONFIRM 可以设置为 true，以在关闭前请求用户确认。可选布尔额外字段 EXTRA_USER_REQUESTED_SHUTDOWN 可以设置为 true，以指示用户请求关闭。
    这是一个受保护的意图，只能由系统发送。
    */
    pub const ACTION_REQUEST_SHUTDOWN: &'static str =
        "com.android.internal.intent.action.REQUEST_SHUTDOWN";

    /**
    广播动作：指示设备存储空间不足的粘性广播。这是一个受保护的意图，只能由系统发送。
    */
    #[deprecated(
        note = "如果您的应用针对的是 android.os.Build.VERSION_CODES#O 或更高版本，此广播将不再传送到清单中定义的任何 BroadcastReceiver。相反，强烈建议应用使用改进的 Context#getCacheDir() 行为，以便系统可以在需要时自动释放存储空间。"
    )]
    pub const ACTION_DEVICE_STORAGE_LOW: &'static str = "android.intent.action.DEVICE_STORAGE_LOW";

    /**
    广播动作：表示设备上的低存储空间情况不再存在。这是一个受保护的意图，只能由系统发送。
    */
    #[deprecated(
        note = "如果您的应用针对的是 android.os.Build.VERSION_CODES#O 或更高版本，此广播将不再传送到清单中定义的任何 BroadcastReceiver。相反，强烈建议应用使用改进的 Context#getCacheDir() 行为，以便系统可以在需要时自动释放存储空间。"
    )]
    pub const ACTION_DEVICE_STORAGE_OK: &'static str = "android.intent.action.DEVICE_STORAGE_OK";

    /**
    广播操作：一种粘性广播，表示设备上的存储空间已满。这适用于希望能够完全填充数据分区的活动，只留下足够的可用空间以防止系统范围内的 SQLite 故障。这是一个受保护的意图，只能由系统发送。
    */
    #[deprecated(
        note = "如果您的应用针对的是 android.os.Build.VERSION_CODES#O 或更高版本，此广播将不再传送到清单中定义的任何 BroadcastReceiver。相反，强烈建议应用使用改进的 Context#getCacheDir() 行为，以便系统可以在需要时自动释放存储空间。"
    )]
    pub const ACTION_DEVICE_STORAGE_FULL: &'static str =
        "android.intent.action.DEVICE_STORAGE_FULL";

    /**
    广播动作：表示设备上的存储空间已满的情况不再存在。这是一个受保护的意图，只能由系统发送。
    */
    #[deprecated(
        note = "如果您的应用针对的是 android.os.Build.VERSION_CODES#O 或更高版本，此广播将不再传送到清单中定义的任何 BroadcastReceiver。相反，强烈建议应用使用改进的 Context#getCacheDir() 行为，以便系统可以在需要时自动释放存储空间。"
    )]
    pub const ACTION_DEVICE_STORAGE_NOT_FULL: &'static str =
        "android.intent.action.DEVICE_STORAGE_NOT_FULL";

    /**
    广播操作：表示用户确认内存不足情况通知，并应启动包管理。这是由用户通过 ACTION_DEVICE_STORAGE_LOW 通知触发的。
    */
    pub const ACTION_MANAGE_PACKAGE_STORAGE: &'static str =
        "android.intent.action.MANAGE_PACKAGE_STORAGE";

    /**
    广播操作：设备已进入 USB 大容量存储模式。这主要用于 USB 设置面板。应用应监听 ACTION_MEDIA_MOUNTED 和 ACTION_MEDIA_UNMOUNTED 广播，以便在 SD 卡文件系统挂载或卸载时收到通知
    */
    #[deprecated(note = "由 android.os.storage.StorageEventListener 替换")]
    pub const ACTION_UMS_CONNECTED: &'static str = "android.intent.action.UMS_CONNECTED";

    /**
    广播操作：设备已退出 USB 大容量存储模式。这主要用于 USB 设置面板。应用应监听 ACTION_MEDIA_MOUNTED 和 ACTION_MEDIA_UNMOUNTED 广播，以便在 SD 卡文件系统挂载或卸载时收到通知
    */
    #[deprecated(note = "由 android.os.storage.StorageEventListener 替换")]
    pub const ACTION_UMS_DISCONNECTED: &'static str = "android.intent.action.UMS_DISCONNECTED";

    /**
    广播操作：外部媒体已被移除。已移除媒体的安装点路径包含在 Intent.mData 字段中。
    */
    pub const ACTION_MEDIA_REMOVED: &'static str = "android.intent.action.MEDIA_REMOVED";

    /**
    广播操作：外部媒体存在，但未安装在其安装点。未安装媒体的安装点路径包含在 Intent.mData 字段中。
    */
    pub const ACTION_MEDIA_UNMOUNTED: &'static str = "android.intent.action.MEDIA_UNMOUNTED";

    /**
    广播动作：外部媒体存在，并且正在检查磁盘。检查媒体的挂载点的路径包含在 Intent.mData 字段中。
    */
    pub const ACTION_MEDIA_CHECKING: &'static str = "android.intent.action.MEDIA_CHECKING";

    //noinspection SpellCheckingInspection
    /**
    广播操作：外部媒体存在，但使用不兼容的 fs（或为空白）检查媒体的挂载点路径包含在 Intent.mData 字段中。
    */
    pub const ACTION_MEDIA_NOFS: &'static str = "android.intent.action.MEDIA_NOFS";

    /**
    广播操作：外部媒体存在并安装在其安装点。安装媒体的安装点路径包含在 Intent.mData 字段中。Intent 包含一个名为“read-only”的额外内容和布尔值，用于指示媒体是否以只读方式安装。
    */
    pub const ACTION_MEDIA_MOUNTED: &'static str = "android.intent.action.MEDIA_MOUNTED";

    /**
    广播操作：外部媒体已卸载，因为它正在通过 USB 大容量存储共享。共享媒体的挂载点路径包含在 Intent.mData 字段中。
    */
    pub const ACTION_MEDIA_SHARED: &'static str = "android.intent.action.MEDIA_SHARED";

    /**
    广播操作：外部媒体不再通过 USB 大容量存储共享。之前共享的媒体的挂载点路径包含在 Intent.mData 字段中。
    */
    pub const ACTION_MEDIA_UNSHARED: &'static str = "android.intent.action.MEDIA_UNSHARED";

    /**
    广播操作：外部媒体已从 SD 卡插槽中移除，但挂载点未卸载。已移除媒体的挂载点路径包含在 Intent.mData 字段中。
    */
    pub const ACTION_MEDIA_BAD_REMOVAL: &'static str = "android.intent.action.MEDIA_BAD_REMOVAL";

    /**
    广播操作：外部媒体存在但无法安装。无法安装的媒体的安装点路径包含在 Intent.mData 字段中。
    */
    pub const ACTION_MEDIA_UNMOUNTABLE: &'static str = "android.intent.action.MEDIA_UNMOUNTABLE";

    /**
    广播操作：用户表示希望移除 外部存储媒体。收到此意图时，应用应关闭挂载点内打开的所有文件。要弹出的媒体的挂载点路径包含在 Intent.mData 字段中。
    */
    pub const ACTION_MEDIA_EJECT: &'static str = "android.intent.action.MEDIA_EJECT";

    /**
    广播操作：媒体扫描器已开始扫描目录。正在扫描的目录的路径包含在 Intent.mData 字段中。
    */
    pub const ACTION_MEDIA_SCANNER_STARTED: &'static str =
        "android.intent.action.MEDIA_SCANNER_STARTED";

    /**
    广播动作：媒体扫描器已完成目录扫描。扫描目录的路径包含在 Intent.mData 字段中。
    */
    pub const ACTION_MEDIA_SCANNER_FINISHED: &'static str =
        "android.intent.action.MEDIA_SCANNER_FINISHED";

    /**
    广播操作：请求媒体扫描仪扫描文件并将其添加到媒体数据库。文件路径包含在 Intent#getData() 中。
    */
    #[deprecated(note = "调用者应该将项目直接插入到 MediaStore 中，每次变异后它们都会被自动扫描。")]
    pub const ACTION_MEDIA_SCANNER_SCAN_FILE: &'static str =
        "android.intent.action.MEDIA_SCANNER_SCAN_FILE";

    /**
    广播操作：按下了“媒体按钮”。包括一个额外字段 EXTRA_KEY_EVENT，其中包含引发广播的按键事件。
    */
    pub const ACTION_MEDIA_BUTTON: &'static str = "android.intent.action.MEDIA_BUTTON";

    /**
    广播操作：按下了“相机按钮”。包括一个额外字段 EXTRA_KEY_EVENT，其中包含引发广播的按键事件。
    */
    pub const ACTION_CAMERA_BUTTON: &'static str = "android.intent.action.CAMERA_BUTTON";

    //noinspection SpellCheckingInspection
    /**
    广播动作：GTalk 连接已建立。
    */
    pub const ACTION_GTALK_SERVICE_CONNECTED: &'static str =
        "android.intent.action.GTALK_CONNECTED";

    //noinspection SpellCheckingInspection
    /**
    广播动作：GTalk 连接已断开。
    */
    pub const ACTION_GTALK_SERVICE_DISCONNECTED: &'static str =
        "android.intent.action.GTALK_DISCONNECTED";

    /**
    广播动作：输入法已改变。
    */
    pub const ACTION_INPUT_METHOD_CHANGED: &'static str =
        "android.intent.action.INPUT_METHOD_CHANGED";

    /**
    广播动作：用户已将手机切换至或退出飞行模式。一个或多个无线电已关闭或打开。此意图将具有以下额外值：state - 一个布尔值，表示飞行模式是否已打开。如果为 true，则蜂窝无线电和可能的蓝牙或 WiFi 等其他无线电也可能已关闭
    这是一个受保护的意图，只能由系统发送。
    */
    pub const ACTION_AIRPLANE_MODE_CHANGED: &'static str = "android.intent.action.AIRPLANE_MODE";

    /**
    广播动作：一些内容提供商在其命名空间的部分区域发布用户可能特别感兴趣的新事件或项目。对于这些事情，他们可能会在感兴趣的项目集发生变化时广播此动作。
    例如，当收件箱中未读邮件的数量发生变化时，GmailProvider 会发送此通知。
    Intent 的数据标识了哪个提供程序的哪个部分发生了变化。通过内容解析器查询时，数据 URI 将返回相关的数据集。
    Intent 会有以下额外值：count - 数据集中的项目数。这与查询数据 URI 返回的游标中的项目数相同。
    此意图将在启动时（如果计数不为零）以及数据集发生变化时发送。数据集可能会发生变化而计数不会发生变化（例如，如果在存档消息的同一同步操作中收到一条新的未读消息）。在这种情况下，手机仍应正常响铃/振动/等。
    */
    pub const ACTION_PROVIDER_CHANGED: &'static str = "android.intent.action.PROVIDER_CHANGED";

    /**
    广播动作：有线耳机已插入或拔出。
    与 android.media.AudioManager#ACTION_HEADSET_PLUG 相同，请参阅值和文档。如果您的应用程序的最低 SDK 版本是 android.os.Build.VERSION_CODES#LOLLIPOP，建议改为引用接收器注册代码中的 AudioManager 常量。
    */
    pub const ACTION_HEADSET_PLUG: &'static str =
        crate::android::media::AudioManager::ACTION_HEADSET_PLUG;

    /**
    广播操作：用户已在设置应用中打开高级设置：状态 - 一个布尔值，指示设置是打开还是关闭。
    这是一个受保护的意图，只能由系统发送。
    */
    pub const ACTION_ADVANCED_SETTINGS_CHANGED: &'static str =
        "android.intent.action.ADVANCED_SETTINGS";

    /**
    广播动作：应用程序限制改变后发送。
    这是一个受保护的意图，只能由系统发送。
    */
    pub const ACTION_APPLICATION_RESTRICTIONS_CHANGED: &'static str =
        "android.intent.action.APPLICATION_RESTRICTIONS_CHANGED";

    /**
    广播动作：即将拨打外拨电话。
    Intent 将具有以下额外值：android.content.Intent#EXTRA_PHONE_NUMBER - 最初打算拨打的电话号码。广播结束后，resultData 将用作实际要拨打的号码。如果为 null，则不会拨打任何电话。多个接收器依次处理拨出的电话是完全可以接受的：例如，家长控制应用程序可能会验证用户当时是否有权拨打电话，然后号码重写应用程序可能会添加区号（如果未指定）。为了保持一致性，任何旨在禁止电话呼叫的接收器都应具有 0 的优先级，以确保它会看到最终要拨打的电话号码。任何旨在重写要拨打的电话号码的接收器都应具有正优先级。负优先级是为系统为此广播保留的；使用它们可能会导致问题。任何接收此 Intent 的 BroadcastReceiver 都不得中止广播。无法使用此机制拦截紧急呼叫，也无法使用此机制修改其他呼叫以呼叫紧急号码。某些应用（例如 VoIP 应用）可能希望重定向拨出电话以使用自己的服务。这些应用应首先通过将 resultData 设置为 null 来阻止拨打电话，然后启动自己的应用拨打电话。您必须拥有 android.Manifest.permission#PROCESS_OUTGOING_CALLS 权限才能接收此 Intent。
    这是一个受保护的意图，只能由系统发送。
    如果用户已选择 android.telecom.CallRedirectionService 来处理拨出电话的重定向，则此 Intent 将不会作为有序广播发送。这意味着其他应用使用此 Intent 重写拨出电话的尝试将被忽略。
    */
    #[deprecated(
        note = "重定向呼叫的应用程序应使用android.telecom.CallRedirectionService API。 执行呼叫筛选的应用程序应使用android.telecom.CallScreeningService API。 需要通知基本呼叫状态的应用程序应使用android.telephony.PhoneStateListener#onCallStatateChanged(int, String)来确定何时放置新的传出调用。"
    )]
    pub const ACTION_NEW_OUTGOING_CALL: &'static str = "android.intent.action.NEW_OUTGOING_CALL";

    /**
    广播动作：让设备重新启动。这仅供系统代码使用。
    这是一个受保护的意图，只能由系统发送。
    */
    pub const ACTION_REBOOT: &'static str = "android.intent.action.REBOOT";

    /**
    广播动作：针对设备物理对接状态变化的粘性广播。
    该意图将具有以下额外值：EXTRA_DOCK_STATE - 当前底座状态，指示设备物理上位于哪个底座中。这用于监控当前物理底座状态。请参阅 android.app.UiModeManager 了解处理底座模式更改的常规 API。
    */
    pub const ACTION_DOCK_EVENT: &'static str = "android.intent.action.DOCK_EVENT";

    /**
    广播动作：当空闲维护可以启动时发出的广播。这意味着用户没有与设备交互，并且预计不会很快与设备交互。空闲维护的典型用途是执行某种昂贵的任务，这些任务可以在不会降低用户体验的情况下推迟。为了在发生意外用户交互时保持设备响应，维护任务的实现应该是可中断的。在这种情况下，将发送带有动作 ACTION_IDLE_MAINTENANCE_END 的广播。换句话说，您不应该在 BroadcastReceiver#onReceive(Context, Intent) 中执行维护工作，而应该通过 Context#startService(Intent) 启动维护服务。此外，您还应该在维护服务运行时保持唤醒锁，以防止设备进入睡眠状态。
    这是一个受保护的意图，只能由系统发送。
    */
    pub const ACTION_IDLE_MAINTENANCE_START: &'static str =
        "android.intent.action.ACTION_IDLE_MAINTENANCE_START";

    /**
    广播操作：应停止空闲维护时的广播。这意味着用户未与设备交互，因此发送了带有操作 ACTION_IDLE_MAINTENANCE_START 的广播，现在用户开始与设备交互。空闲维护的典型用途是执行某种昂贵的任务，这些任务可以在不会降低用户体验的情况下推迟。为了在发生意外用户交互时保持设备响应，维护任务的实现应该是可中断的。因此，在收到带有此操作的广播时，应尽快中断维护任务。换句话说，您不应该在 BroadcastReceiver#onReceive(Context, Intent) 中执行维护工作，而应该停止在收到 ACTION_IDLE_MAINTENANCE_START 时启动的维护服务。您还应该释放在维护服务启动时获取的唤醒锁。
    这是一个受保护的意图，只能由系统发送。
    */
    pub const ACTION_IDLE_MAINTENANCE_END: &'static str =
        "android.intent.action.ACTION_IDLE_MAINTENANCE_END";

    /**
    广播动作：远程意图将被广播。
    远程意图用于设备之间的远程 RPC。远程意图被序列化并从一个设备发送到另一个设备。接收设备解析远程意图并进行广播。请注意，任何人都可以广播远程意图。但是，如果远程意图的意图接收者不信任来自任意意图发送者的意图广播，则应要求发送者持有某些权限，以便只有受信任的发送者的广播才会通过。
    */
    pub const ACTION_REMOTE_INTENT: &'static str = "com.google.android.c2dm.intent.RECEIVE";

    /**
    广播操作：系统更新后用户启动时会广播一次。可用于在系统更新后执行清理或升级。此广播在 ACTION_LOCKED_BOOT_COMPLETED 广播之后但在 ACTION_BOOT_COMPLETED 广播之前发送。仅当 Build#FINGERPRINT 发生更改时才会发送，并且仅发送给系统映像中的接收器。
    */
    pub const ACTION_PRE_BOOT_COMPLETED: &'static str = "android.intent.action.PRE_BOOT_COMPLETED";

    /**
    向特定应用广播，以查询任何受支持的限制，以对受限用户实施。广播意图包含一个额外的 EXTRA_RESTRICTIONS_BUNDLE，其中当前持久化的限制作为键/值对的 Bundle。值类型可以是布尔值、字符串或 String[]，具体取决于限制类型。响应应包含一个额外的 EXTRA_RESTRICTIONS_LIST，其类型为 `ArrayList<RestrictionEntry>`。它还可以包含一个额外的 EXTRA_RESTRICTIONS_INTENT，其类型为 Intent。将启动该意图指定的活动，以获得必须包含额外 EXTRA_RESTRICTIONS_LIST 或 EXTRA_RESTRICTIONS_BUNDLE 之一的结果。返回的限制的键和值将被持久化。
    */
    pub const ACTION_GET_RESTRICTION_ENTRIES: &'static str =
        "android.intent.action.GET_RESTRICTION_ENTRIES";

    /**
    在用户首次启动时发送，以允许系统应用执行一次性初始化。（第三方应用看不到此消息，因为新初始化的用户没有为其安装任何第三方应用。）此消息在用户启动初期发送，大约在主应用启动时，在发送 ACTION_BOOT_COMPLETED 之前。此消息作为前台广播发送，因为它是可见用户交互的一部分；处理时应尽可能快。
    */
    pub const ACTION_USER_INITIALIZE: &'static str = "android.intent.action.USER_INITIALIZE";

    /**
    如果切换导致进程的用户被带到前台，则在用户切换完成后发送。这仅发送给通过 Context#registerReceiver(BroadcastReceiver, IntentFilter) Context.registerReceiver 注册的接收器。它发送给将要转到前台的用户。这是作为前台广播发送的，因为它是可见用户交互的一部分；处理时要尽可能快。
    */
    pub const ACTION_USER_FOREGROUND: &'static str = "android.intent.action.USER_FOREGROUND";

    /**
    如果切换导致进程的用户被发送到后台，则在用户切换完成后发送。这仅发送给通过 Context#registerReceiver(BroadcastReceiver, IntentFilter) Context.registerReceiver 注册的接收器。它发送给将要转到后台的用户。这是作为前台广播发送的，因为它是可见用户交互的一部分；处理时要尽可能快。
    */
    pub const ACTION_USER_BACKGROUND: &'static str = "android.intent.action.USER_BACKGROUND";

    /**
    添加用户时向系统发送的广播。带有一个额外的 EXTRA_USER，用于指定新用户的 UserHandle（由于遗留原因，还带有一个 int 额外的 EXTRA_USER_HANDLE，用于指定该用户的用户 ID）。它会发送给所有正在运行的用户。您必须持有 android.Manifest.permission#MANAGE_USERS 才能接收此广播。
    */
    pub const ACTION_USER_ADDED: &'static str = "android.intent.action.USER_ADDED";

    /**
    用户启动时系统发送的广播。带有一个额外的 EXTRA_USER_HANDLE，其中包含用户的用户 ID。这只会发送给已注册的接收器，而不会发送给清单接收器。它会发送给已启动的用户。这是作为前台广播发送的，因为它是可见用户交互的一部分；处理时要尽可能快。
    注意：在收到广播时，用户的实际状态可能已经发生变化。例如，无论您收到哪个广播，用户可能已被删除、启动或停止。因此，接收者应始终检查用户的当前状态。
    */
    pub const ACTION_USER_STARTED: &'static str = "android.intent.action.USER_STARTED";

    /**
    用户启动时发送的广播。带有一个额外的 EXTRA_USER_HANDLE，其中包含用户的用户 ID。此广播仅发送给已注册的接收器，不发送给清单接收器。此广播发送给所有用户（包括正在启动的用户）。您必须持有 android.Manifest.permission#INTERACT_ACROSS_USERS 才能接收此广播。此广播作为后台广播发送，因为其结果不是主要 UX 流程的一部分；为了安全地跟踪用户的启动/停止状态，您可以将其与 ACTION_USER_STOPPING 结合使用。与其他用户状态广播一起使用通常不安全，因为那些是前台广播，因此可以按不同的顺序执行。
    注意：在收到广播时，用户的实际状态可能已经发生变化。例如，无论您收到哪个广播，用户可能已被删除、启动或停止。因此，接收者应始终检查用户的当前状态。
    */
    pub const ACTION_USER_STARTING: &'static str = "android.intent.action.USER_STARTING";

    /**
    当用户即将停止时发送的广播。带有一个额外的 EXTRA_USER_HANDLE，其中包含用户的用户 ID。这只发送给已注册的接收器，而不是清单接收器。它发送给所有用户（包括正在停止的用户）。您必须持有 android.Manifest.permission#INTERACT_ACROSS_USERS 才能接收此广播。直到所有接收器都处理完广播后，用户才会停止。这是作为后台广播发送的，因为其结果不是主要 UX 流程的一部分；为了安全地跟踪用户的启动/停止状态，您可以将它与 ACTION_USER_STARTING 结合使用。与其他用户状态广播一起使用通常不安全，因为那些是前台广播，因此可以按不同的顺序执行。
    注意：用户的实际状态可能在收到广播时已经发生变化。例如，无论您收到哪个广播，用户可能已被删除、启动或停止。因此，接收器应始终检查用户的当前状态。
    */
    pub const ACTION_USER_STOPPING: &'static str = "android.intent.action.USER_STOPPING";

    /**
    当用户停止时向系统发送的广播。携带一个额外的 EXTRA_USER_HANDLE，其中包含用户的用户 ID。这类似于 ACTION_PACKAGE_RESTARTED，但针对的是整个用户，而不是特定的软件包。这只发送给已注册的接收器，而不是清单接收器。它会发送给所有正在运行的用户，除了刚刚停止的用户（不再运行）。
    注意：在收到广播时，用户的实际状态可能已经发生变化。例如，无论您收到哪个广播，用户可能已被删除、启动或停止。因此，接收者应始终检查用户的当前状态。
    */
    pub const ACTION_USER_STOPPED: &'static str = "android.intent.action.USER_STOPPED";

    /**
    当用户被移除时向系统发送的广播。带有一个额外的 EXTRA_USER，用于指定被移除用户的 UserHandle（并且由于遗留原因，还带有一个 int 额外的 EXTRA_USER_HANDLE，用于指定该用户的用户 ID）。它会发送给除被移除用户之外的所有正在运行的用户。直到所有接收器都处理完广播后，用户才会被完全移除。您必须持有 android.Manifest.permission#MANAGE_USERS 才能接收此广播。
    */
    pub const ACTION_USER_REMOVED: &'static str = "android.intent.action.USER_REMOVED";

    /**
    当用户切换时向系统发送的广播。带有一个额外的 EXTRA_USER，用于指定要成为当前用户的 UserHandle（并且由于遗留原因，还带有一个 int 额外的 EXTRA_USER_HANDLE，用于指定该用户的用户 ID）。这只发送给已注册的接收器，而不是清单接收器。它会发送给所有正在运行的用户。您必须持有 android.Manifest.permission#MANAGE_USERS 才能接收此广播。
    注意：在收到广播时，用户的实际状态可能已经发生变化。例如，无论您收到哪个广播，用户可能已被删除、启动或停止。因此，接收者应始终检查用户的当前状态。
    此广播在用户切换完成后发送。如果在切换过程中（即在屏幕冻结以隐藏 UI 卡顿时）需要执行任务，请使用 ActivityManagerService.registerUserSwitchObserver 方法。
    */
    pub const ACTION_USER_SWITCHED: &'static str = "android.intent.action.USER_SWITCHED";

    /**
    广播操作：当凭证加密的私人存储已为目标用户解锁时发送。这仅发送给已注册的接收器，而不会发送给清单接收器。
    注意：在收到广播时，用户的实际状态可能已经发生变化。例如，无论您收到哪个广播，用户可能已被删除、启动或停止。因此，接收者应始终检查用户的当前状态。
    */
    pub const ACTION_USER_UNLOCKED: &'static str = "android.intent.action.USER_UNLOCKED";

    /**
    当用户信息发生变化时向系统发送的广播。带有额外的 EXTRA_USER_HANDLE 来指示哪个用户的信息发生了变化。这只发送给已注册的接收器，而不是清单接收器。它会发送给所有用户。
    */
    pub const ACTION_USER_INFO_CHANGED: &'static str = "android.intent.action.USER_INFO_CHANGED";

    /**
    当添加关联的托管配置文件时（配置文件已创建并准备使用）向主要用户发送的广播。携带一个额外的 EXTRA_USER，用于指定已添加配置文件的 UserHandle。只有需要显示主要配置文件和托管配置文件中合并内容的应用（例如启动器）才需要担心此广播。此广播仅发送给已注册的接收器，而不会发送给清单接收器。
    */
    pub const ACTION_MANAGED_PROFILE_ADDED: &'static str =
        "android.intent.action.MANAGED_PROFILE_ADDED";

    /**
    当关联的托管配置文件被移除时，向主要用户发送的广播。携带一个额外的 EXTRA_USER，用于指定被移除配置文件的 UserHandle。只有需要显示主要配置文件和托管配置文件中合并内容的应用（例如启动器）才需要担心此广播。此广播仅发送给已注册的接收器，而不会发送给清单接收器。
    */
    pub const ACTION_MANAGED_PROFILE_REMOVED: &'static str =
        "android.intent.action.MANAGED_PROFILE_REMOVED";

    /**
    当关联的托管配置文件的凭据加密私人存储解锁时，向主要用户发送广播。携带一个额外的 EXTRA_USER，用于指定已解锁配置文件的 UserHandle。只有需要显示主要配置文件和托管配置文件中合并内容的应用（例如启动器）才需要担心此广播。此广播仅发送给已注册的接收器，而不会发送给清单接收器。
    */
    pub const ACTION_MANAGED_PROFILE_UNLOCKED: &'static str =
        "android.intent.action.MANAGED_PROFILE_UNLOCKED";

    /**
    当关联的受管理配置文件可用时，向主要用户发送广播。目前，这包括用户停用配置文件的安静模式时。携带一个额外的 EXTRA_USER，用于指定配置文件的 UserHandle。当安静模式发生变化时，此广播将携带一个布尔额外的 EXTRA_QUIET_MODE，指示安静模式的新状态。这仅发送给已注册的接收器，而不是清单接收器。
    */
    pub const ACTION_MANAGED_PROFILE_AVAILABLE: &'static str =
        "android.intent.action.MANAGED_PROFILE_AVAILABLE";

    /**
    当关联的受管理配置文件不可用时，向主要用户发送广播。目前，这包括用户为配置文件启用安静模式的情况。携带一个额外的 EXTRA_USER，用于指定配置文件的 UserHandle。当安静模式发生变化时，此广播将携带一个布尔额外的 EXTRA_QUIET_MODE，指示安静模式的新状态。这仅发送给已注册的接收器，而不是清单接收器。
    */
    pub const ACTION_MANAGED_PROFILE_UNAVAILABLE: &'static str =
        "android.intent.action.MANAGED_PROFILE_UNAVAILABLE";

    /**
    当关联的配置文件已启动并解锁时，向父级用户发送的广播。带有一个额外的 EXTRA_USER，用于指定配置文件的 UserHandle。这仅发送给已注册的接收器，而不是清单接收器。
    */
    pub const ACTION_PROFILE_ACCESSIBLE: &'static str = "android.intent.action.PROFILE_ACCESSIBLE";

    /**
    当相关配置文件停止时，向父级用户发送广播。携带一个额外的 EXTRA_USER，用于指定配置文件的 UserHandle。这只会发送给已注册的接收器，而不会发送给清单接收器。
    */
    pub const ACTION_PROFILE_INACCESSIBLE: &'static str =
        "android.intent.action.PROFILE_INACCESSIBLE";

    /**
    当关联的配置文件被删除时，向父级用户发送的广播。携带一个额外的 EXTRA_USER，用于指定被删除的配置文件的 UserHandle。
    此广播与 ACTION_MANAGED_PROFILE_REMOVED 类似，但功能类似于 android.content.pm.UserInfo#isProfile() 类型的所有用户的通用广播。当管理用户被移除时，除了 ACTION_MANAGED_PROFILE_REMOVED 广播外，还会发送此广播。
    只有需要显示父用户及其关联个人资料的合并内容的应用程序（例如启动器）才需要担心此广播。此广播仅发送给使用 Context#registerReceiver 创建的注册接收器。它不会发送给清单接收器。
    */
    pub const ACTION_PROFILE_REMOVED: &'static str = "android.intent.action.PROFILE_REMOVED";

    /**
    当添加关联配置文件时（配置文件已创建并可供使用）向父用户发送的广播。带有一个额外的 EXTRA_USER，用于指定已添加配置文件的 UserHandle。
    此广播与 ACTION_MANAGED_PROFILE_ADDED 类似，但功能类似于 android.content.pm.UserInfo#isProfile() 类型的所有用户的通用广播。添加受管理用户时，除了 ACTION_MANAGED_PROFILE_ADDED 广播外，还会发送此广播。
    只有需要显示父用户及其关联个人资料的合并内容的应用程序（例如启动器）才需要担心此广播。此广播仅发送给使用 Context#registerReceiver 创建的注册接收器。它不会发送给清单接收器。
    */
    pub const ACTION_PROFILE_ADDED: &'static str = "android.intent.action.PROFILE_ADDED";

    /**
    当任何用户的“设备锁定”状态发生变化时，向系统用户发送广播。携带额外的 EXTRA_USER_HANDLE，用于指定设备锁定或解锁的用户 ID。
    这仅发送给已注册的接收者。
    */
    pub const ACTION_DEVICE_LOCKED_CHANGED: &'static str =
        "android.intent.action.DEVICE_LOCKED_CHANGED";

    /**
    当用户点击系统“快速设置”区域中的时钟小部件时发送。
    */
    pub const ACTION_QUICK_CLOCK: &'static str = "android.intent.action.QUICK_CLOCK";

    /**
    活动操作：显示亮度设置对话框。
    */
    pub const ACTION_SHOW_BRIGHTNESS_DIALOG: &'static str =
        "com.android.intent.action.SHOW_BRIGHTNESS_DIALOG";

    /**
    活动操作：显示对比度设置对话框。
    */
    pub const ACTION_SHOW_CONTRAST_DIALOG: &'static str =
        "com.android.intent.action.SHOW_CONTRAST_DIALOG";

    /**
    广播操作：按下了全局按钮。包括一个额外字段 EXTRA_KEY_EVENT，其中包含引发广播的按键事件。
    */
    pub const ACTION_GLOBAL_BUTTON: &'static str = "android.intent.action.GLOBAL_BUTTON";

    /**
    广播操作：授予媒体资源时发送。EXTRA_PACKAGES 指定持有授予的媒体资源的进程上的软件包。这是一个受保护的意图，只能由系统发送。这需要 android.Manifest.permission#RECEIVE_MEDIA_RESOURCE_USAGE 权限。
    */
    pub const ACTION_MEDIA_RESOURCE_GRANTED: &'static str =
        "android.intent.action.MEDIA_RESOURCE_GRANTED";

    /**
    广播操作：覆盖包已更改。数据包含已更改的覆盖包的名称。此操作将在 android.content.om.IOverlayManager#getOverlayInfo(String, int) 返回的 OverlayInfo 的所有更改上广播。最常见的更改是状态更改，无论覆盖是否启用，状态都会更改。
    */
    pub const ACTION_OVERLAY_CHANGED: &'static str = "android.intent.action.OVERLAY_CHANGED";

    /**
    活动操作：允许用户选择并返回一个或多个现有文档。调用时，系统将显示设备上安装的各种 DocumentsProvider 实例，让用户以交互方式浏览它们。这些文档包括本地媒体（例如照片和视频）以及已安装的云存储提供商提供的文档。每个文档都表示为 ` ` URI，以便任何接收者都可以访问它。如果选择了多个文档，则它们将在 getClipData() 中返回。
    */
    pub const ACTION_OPEN_DOCUMENT: &'static str = "android.intent.action.OPEN_DOCUMENT";

    /**
    活动操作：允许用户创建新文档。调用时，系统将显示设备上安装的各种 DocumentsProvider 实例，让用户浏览它们。返回的文档可能是没有内容的新创建的文档，也可能是具有请求的 MIME 类型的现有文档。每个文档都表示为 ` ` URI，以便任何接收者都可以访问它。
    */
    pub const ACTION_CREATE_DOCUMENT: &'static str = "android.intent.action.CREATE_DOCUMENT";

    /**
    活动操作：允许用户选择目录子树。调用时，系统将显示设备上安装的各种 DocumentsProvider 实例，让用户浏览它们。应用可以完全管理返回目录中的文档。要访问后代（子、孙等）文档，请使用 DocumentsContract#buildDocumentUriUsingTree(Uri, String) 和 DocumentsContract#buildChildDocumentsUriUsingTree(Uri, String) 以及返回的 URI。调用者可以通过 DocumentsContract#EXTRA_INITIAL_URI 设置文档 URI，以指示文档导航器的初始位置。如果是文件夹，系统将尽力在指定文档中启动导航器，如果不是，则在包含指定文档的文件夹中启动导航器。
    输出：表示所选目录树的 URI。
    */
    pub const ACTION_OPEN_DOCUMENT_TREE: &'static str = "android.intent.action.OPEN_DOCUMENT_TREE";

    /**
    活动操作：执行文本翻译。
    输入：EXTRA_TEXT getCharSequence(EXTRA_TEXT) 是要翻译的文本。
    输出：无。
    */
    pub const ACTION_TRANSLATE: &'static str = "android.intent.action.TRANSLATE";

    /**
    活动操作：定义所选单词的含义。
    输入：EXTRA_TEXT getCharSequence(EXTRA_TEXT) 是要定义的文本。
    输出：无。
    */
    pub const ACTION_DEFINE: &'static str = "android.intent.action.DEFINE";

    /**
    广播动作：由于新传感器的连接或现有传感器的断开连接，动态传感器列表发生变化。
    这是一个受保护的意图，只能由系统发送。
    */
    pub const ACTION_DYNAMIC_SENSOR_CHANGED: &'static str =
        "android.intent.action.DYNAMIC_SENSOR_CHANGED";

    #[doc(hidden)]
    #[deprecated(note = "已弃用 - 改用 ACTION_FACTORY_RESET。")]
    pub const ACTION_MASTER_CLEAR: &'static str = "android.intent.action.MASTER_CLEAR";

    /**
    RecoverySystem 发送的广播意图，通知监听器即将执行全局清除（擦除）。
    */
    pub const ACTION_MASTER_CLEAR_NOTIFICATION: &'static str =
        "android.intent.action.MASTER_CLEAR_NOTIFICATION";

    /**
    布尔意图额外与 ACTION_MASTER_CLEAR 一起使用，以便强​​制恢复出厂设置，即使设置了 android.os.UserManager#DISALLOW_FACTORY_RESET。
    */
    #[deprecated(note = "已弃用 - 改用 EXTRA_FORCE_FACTORY_RESET。")]
    pub const EXTRA_FORCE_MASTER_CLEAR: &'static str = "android.intent.extra.FORCE_MASTER_CLEAR";

    /**
    触发恢复出厂设置的广播操作。
    发送者必须拥有 android.Manifest.permission#MASTER_CLEAR 权限。恢复出厂设置的原因应指定为 EXTRA_REASON。
    不适用于第三方应用程序。
    */
    pub const ACTION_FACTORY_RESET: &'static str = "android.intent.action.FACTORY_RESET";

    /**
    布尔意图额外与 ACTION_MASTER_CLEAR 一起使用，以便强​​制恢复出厂设置，即使设置了 android.os.UserManager#DISALLOW_FACTORY_RESET。
    不适用于第三方应用程序。
    */
    pub const EXTRA_FORCE_FACTORY_RESET: &'static str = "android.intent.extra.FORCE_FACTORY_RESET";

    /**
    广播操作：报告正在从备份中恢复设置元素。此意图包含四个额外内容：EXTRA_SETTING_NAME 是命名已恢复设置的字符串，EXTRA_SETTING_NEW_VALUE 是正在恢复的值，EXTRA_SETTING_PREVIOUS_VALUE 是恢复操作之前该设置条目的值，EXTRA_SETTING_RESTORED_FROM_SDK_INT 是已从中恢复设置的 SDK 版本（对应于 android.os.Build.VERSION#SDK_INT）。前三个值表示为字符串，第四个值表示为 int。
    此广播仅针对已知需要在还原时间对特定接收方进行特殊处理的设置提供程序条目发送。这些条目位于提供程序的备份代理实现中的 BROADCAST_ON_RESTORE 表中。
    */
    pub const ACTION_SETTING_RESTORED: &'static str = "android.os.action.SETTING_RESTORED";

    /**
    字符串意图额外用于action_setting_restored。包含恢复设置的名称。
    */
    pub const EXTRA_SETTING_NAME: &'static str = "setting_name";

    /// 与 ACTION_SETTING_RESTORED 一起使用的字符串意图附加项。包含恢复操作之前的 EXTRA_SETTING_NAME 设置条目的值。
    pub const EXTRA_SETTING_PREVIOUS_VALUE: &'static str = "previous_value";

    /// 与 ACTION_SETTING_RESTORED 一起使用的字符串意图附加项。包含正在恢复的 EXTRA_SETTING_NAME 设置条目的值。
    pub const EXTRA_SETTING_NEW_VALUE: &'static str = "new_value";

    /// 与 ACTION_SETTING_RESTORED 一起使用的 Int Intent Extra。包含已从中恢复设置的 SDK 版本（对应于 android.os.Build.VERSION#SDK_INT）。
    pub const EXTRA_SETTING_RESTORED_FROM_SDK_INT: &'static str = "restored_from_sdk_int";

    /**
    活动动作：处理一段文本。
    输入：extra_process_text包含要处理的文本。 extra_process_text_readonly表示，如果结果文本仅读取。
    输出：extra_process_text包含已处理的文本。
    */
    pub const ACTION_PROCESS_TEXT: &'static str = "android.intent.action.PROCESS_TEXT";

    /**
    广播操作：SIM 卡状态已更改。有关更多详细信息，请参阅 TelephonyIntents.ACTION_SIM_STATE_CHANGED。之所以出现这种情况，是因为 TelephonyIntents 是一个内部类。该意图将具有以下附加功能。
    */
    #[deprecated(
        note = "使用 android.telephony.TelephonyManager#ACTION_SIM_CARD_STATE_CHANGED 或 android.telephony.TelephonyManager#ACTION_SIM_APPLICATION_STATE_CHANGED"
    )]
    pub const ACTION_SIM_STATE_CHANGED: &'static str = "android.intent.action.SIM_STATE_CHANGED";

    /**
    用Action_SIM_STATE_CHANGED用于广播SIM状态的额外使用。这将具有以下意图值之一。
    */
    #[deprecated(note = "使用 android.telephony.TelephonyManager#ACTION_SIM_CARD_STATE_CHANGED")]
    pub const EXTRA_SIM_STATE: &'static str = "ss";

    /**
    意图值 UNKNOWN 表示 SIM 状态未知
    */
    #[deprecated(note = "使用 android.telephony.TelephonyManager#ACTION_SIM_CARD_STATE_CHANGED")]
    pub const SIM_STATE_UNKNOWN: &'static str = "UNKNOWN";

    /**
    意图值 NOT_READY 表示 SIM 卡尚未准备好，例如，无线电已关闭或正在打开
    */
    #[deprecated(note = "Use android.telephony.TelephonyManager#ACTION_SIM_CARD_STATE_CHANGED")]
    pub const SIM_STATE_NOT_READY: &'static str = "NOT_READY";

    /**
    Intent 值 ABSENT 表示 SIM 卡缺失
    */
    #[deprecated(note = "使用 android.telephony.TelephonyManager#ACTION_SIM_CARD_STATE_CHANGED")]
    pub const SIM_STATE_ABSENT: &'static str = "ABSENT";

    /**
    Intent 值 PRESENT 表示设备已插入 SIM 卡
    */
    #[deprecated(note = "使用 android.telephony.TelephonyManager#ACTION_SIM_CARD_STATE_CHANGED")]
    pub const SIM_STATE_PRESENT: &'static str = "PRESENT";

    /**
    Intent 值 CARD_IO_ERROR 表示连续三次出现 SIM IO 错误
    */
    #[deprecated(note = "使用 android.telephony.TelephonyManager#ACTION_SIM_CARD_STATE_CHANGED")]
    pub const SIM_STATE_CARD_IO_ERROR: &'static str = "CARD_IO_ERROR";

    /**
    Intent 值 CARD_RESTRICTED 表示卡存在，但由于运营商限制而无法使用
    */
    #[deprecated(note = "使用 android.telephony.TelephonyManager#ACTION_SIM_CARD_STATE_CHANGED")]
    pub const SIM_STATE_CARD_RESTRICTED: &'static str = "CARD_RESTRICTED";

    /**
    意图值 LOCKED 表示 SIM 卡已被 PIN 或网络锁定
    */
    #[deprecated(note = "使用 android.telephony.TelephonyManager#ACTION_SIM_CARD_STATE_CHANGED")]
    pub const SIM_STATE_LOCKED: &'static str = "LOCKED";

    /**
    Intent 值 READY 表示 SIM 卡已准备好被访问
    */
    #[deprecated(note = "使用 android.telephony.TelephonyManager#ACTION_SIM_CARD_STATE_CHANGED")]
    pub const SIM_STATE_READY: &'static str = "READY";

    //noinspection SpellCheckingInspection
    /**
    意图值 IMSI 表示 SIM IMSI 在属性中已准备就绪
    */
    #[deprecated(note = "使用 android.telephony.TelephonyManager#ACTION_SIM_CARD_STATE_CHANGED")]
    pub const SIM_STATE_IMSI: &'static str = "IMSI";

    //noinspection SpellCheckingInspection
    /**
    意图值 LOADED 表示所有 SIM 记录（包括 IMSI）都已加载
    */
    #[deprecated(note = "使用 android.telephony.TelephonyManager#ACTION_SIM_CARD_STATE_CHANGED")]
    pub const SIM_STATE_LOADED: &'static str = "LOADED";

    /**
    与 ACTION_SIM_STATE_CHANGED 一起使用的 extra，用于广播 SIM 状态。此 extra 将具有以下 Intent 值之一。
    */
    #[deprecated(
        note = "使用 android.telephony.TelephonyManager#ACTION_SIM_APPLICATION_STATE_CHANGED"
    )]
    pub const EXTRA_SIM_LOCKED_REASON: &'static str = "reason";

    /**
    意图值 PIN 表示 SIM 卡已锁定在 PIN1
    */
    #[deprecated(
        note = "使用 android.telephony.TelephonyManager#ACTION_SIM_APPLICATION_STATE_CHANGED"
    )]
    pub const SIM_LOCKED_ON_PIN: &'static str = "PIN";

    /**
    Intent 值 PUK 表示 SIM 卡已锁定在 PUK1
    PUK 表示 ICC 已锁定在 PUK1 上
    */
    #[deprecated(
        note = "使用 android.telephony.TelephonyManager#ACTION_SIM_APPLICATION_STATE_CHANGED"
    )]
    pub const SIM_LOCKED_ON_PUK: &'static str = "PUK";

    /**
    Intent 值 NETWORK 表示 SIM 卡已锁定在 NETWORK PERSONALIZATION
    */
    #[deprecated(
        note = "使用 android.telephony.TelephonyManager#ACTION_SIM_APPLICATION_STATE_CHANGED"
    )]
    pub const SIM_LOCKED_NETWORK: &'static str = "NETWORK";

    /**
    Intent 值 PERM_DISABLED 表示由于 puk 失败，SIM 卡被永久禁用
    */
    #[deprecated(
        note = "使用 android.telephony.TelephonyManager#ACTION_SIM_APPLICATION_STATE_CHANGED"
    )]
    pub const SIM_ABSENT_ON_PERM_DISABLED: &'static str = "PERM_DISABLED";

    /**
    与 ACTION_SIM_STATE_CHANGED 一起使用的额外信息，用于指示此广播是否是解锁时的重新广播。如果未指定，则默认为 ` `。
    */
    #[deprecated(
        note = "使用 android.telephony.TelephonyManager#ACTION_SIM_CARD_STATE_CHANGED 或 android.telephony.TelephonyManager#ACTION_SIM_APPLICATION_STATE_CHANGED"
    )]
    pub const EXTRA_REBROADCAST_ON_UNLOCK: &'static str = "rebroadcastOnUnlock";

    /**
    广播动作：表示电话服务状态已更改。此意图将具有以下额外值：
    需要READ_PHONE_STATE权限。
    这是一个受保护的意图，只能由系统发送。
    */
    #[deprecated(
        note = "使用 android.provider.Telephony.ServiceStateTable 和辅助函数 `` 通过 ContentObserver 或使用 JobScheduler 订阅给定订阅 ID 和字段的 ServiceState 更改。"
    )]
    pub const ACTION_SERVICE_STATE: &'static str = "android.intent.action.SERVICE_STATE";

    /**
    由 services.core.java.com.android.server.pm.DataLoaderManagerService 用于查询数据加载器服务提供商。数据加载器服务提供商在其清单中注册此意图过滤器，以便可以通过“ ”查找和绑定它们。
    这是一个受保护的意图，只能由系统发送。
    数据加载器服务提供商必须是特权应用。请参阅 com.android.server.pm.PackageManagerShellCommandDataLoader 作为此类数据加载器服务提供商的示例。
    */
    pub const ACTION_LOAD_DATA: &'static str = "android.intent.action.LOAD_DATA";

    /**
    与 ACTION_SERVICE_STATE 一起使用的 int extra，指示语音注册状态。
    */
    #[deprecated(note = "使用 android.provider.Telephony.ServiceStateTable#VOICE_REG_STATE。")]
    pub const EXTRA_VOICE_REG_STATE: &'static str = "voiceRegState";

    /**
    与 ACTION_SERVICE_STATE 一起使用的 int extra，用于指示数据注册状态。
    */
    #[deprecated(note = "使用 android.provider.Telephony.ServiceStateTable#DATA_REG_STATE。")]
    pub const EXTRA_DATA_REG_STATE: &'static str = "dataRegState";

    /**
    与 ACTION_SERVICE_STATE 一起使用的整数附加值，指示语音漫游类型。
    */
    #[deprecated(note = "使用 android.provider.Telephony.ServiceStateTable#VOICE_ROAMING_TYPE。")]
    pub const EXTRA_VOICE_ROAMING_TYPE: &'static str = "voiceRoamingType";

    /**
    与 ACTION_SERVICE_STATE 一起使用的整数附加项，指示数据漫游类型。
    */
    #[deprecated(note = "使用 android.provider.Telephony.ServiceStateTable#DATA_ROAMING_TYPE。")]
    pub const EXTRA_DATA_ROAMING_TYPE: &'static str = "dataRoamingType";

    /**
    与 ACTION_SERVICE_STATE 一起使用的字符串附加项，以长字母数字格式表示当前注册的语音操作员名称。` ` 如果操作员名称未知或未注册。
    */
    #[deprecated(
        note = "使用 android.provider.Telephony.ServiceStateTable#VOICE_OPERATOR_ALPHA_LONG。"
    )]
    pub const EXTRA_OPERATOR_ALPHA_LONG: &'static str = "operator-alpha-long";

    /**
    与 ACTION_SERVICE_STATE 一起使用的字符串附加项，以短字母数字格式表示当前注册的语音操作员名称。` ` 如果操作员名称未知或未注册。
    */
    #[deprecated(
        note = "使用 android.provider.Telephony.ServiceStateTable#VOICE_OPERATOR_ALPHA_SHORT。"
    )]
    pub const EXTRA_OPERATOR_ALPHA_SHORT: &'static str = "operator-alpha-short";

    /**
    与 ACTION_SERVICE_STATE 一起使用的字符串附加项，包含移动网络的 MCC（移动国家代码，3 位数字）和 MNC（移动网络代码，2-3 位数字）。
    */
    #[deprecated(
        note = "使用 android.provider.Telephony.ServiceStateTable#VOICE_OPERATOR_NUMERIC。"
    )]
    pub const EXTRA_OPERATOR_NUMERIC: &'static str = "operator-numeric";

    /**
    与 ACTION_SERVICE_STATE 一起使用的字符串附加项，以长字母数字格式表示当前注册的数据操作员名称。` ` 如果操作员名称未知或未注册。
    */
    #[deprecated(
        note = "使用 android.provider.Telephony.ServiceStateTable#DATA_OPERATOR_ALPHA_LONG。"
    )]
    pub const EXTRA_DATA_OPERATOR_ALPHA_LONG: &'static str = "data-operator-alpha-long";

    /**
    与 ACTION_SERVICE_STATE 一起使用的字符串附加项，以简短的字母数字格式表示当前注册的数据操作员名称。` ` 如果操作员名称未知或未注册。
    */
    #[deprecated(
        note = "使用 android.provider.Telephony.ServiceStateTable#DATA_OPERATOR_ALPHA_SHORT。"
    )]
    pub const EXTRA_DATA_OPERATOR_ALPHA_SHORT: &'static str = "data-operator-alpha-short";

    /**
    与 ACTION_SERVICE_STATE 一起使用的字符串附加项，包含数据运营商的 MCC（移动国家代码，3 位数字）和 MNC（移动网络代码，2-3 位数字）。
    */
    #[deprecated(
        note = "使用 android.provider.Telephony.ServiceStateTable#DATA_OPERATOR_NUMERIC。"
    )]
    pub const EXTRA_DATA_OPERATOR_NUMERIC: &'static str = "data-operator-numeric";

    /**
    与 ACTION_SERVICE_STATE 一起使用的布尔值附加值，指示当前网络选择模式是否为手动。如果是自动模式，则为 ` `。
    */
    #[deprecated(
        note = "使用 android.provider.Telephony.ServiceStateTable#IS_MANUAL_NETWORK_SELECTION。"
    )]
    pub const EXTRA_MANUAL: &'static str = "manual";

    /**
    与 ACTION_SERVICE_STATE 一起使用的整数附加项，表示当前的语音无线电技术。
    */
    #[deprecated(
        note = "使用 android.provider.Telephony.ServiceStateTable#RIL_VOICE_RADIO_TECHNOLOGY。"
    )]
    pub const EXTRA_VOICE_RADIO_TECH: &'static str = "radioTechnology";

    /**
    与 ACTION_SERVICE_STATE 一起使用的整数附加项，表示当前的数据无线电技术。
    */
    #[deprecated(
        note = "使用 android.provider.Telephony.ServiceStateTable#RIL_DATA_RADIO_TECHNOLOGY。"
    )]
    pub const EXTRA_DATA_RADIO_TECH: &'static str = "dataRadioTechnology";

    /**
    与 ACTION_SERVICE_STATE 一起使用的布尔额外值，表示 CDMA 网络上的并发服务支持。否则将为 ` `。
    */
    #[deprecated(note = "使用 android.provider.Telephony.ServiceStateTable#CSS_INDICATOR。")]
    pub const EXTRA_CSS_INDICATOR: &'static str = "cssIndicator";

    /**
    与 ACTION_SERVICE_STATE 一起使用的整数附加项，表示 CDMA 网络 ID。如果未知，则为 ` `。
    */
    #[deprecated(note = "使用 android.provider.Telephony.ServiceStateTable#NETWORK_ID。")]
    pub const EXTRA_NETWORK_ID: &'static str = "networkId";

    /**
    与 ACTION_SERVICE_STATE 一起使用的整数附加项，表示 CDMA 系统 ID。如果未知，则为 ` `。
    */
    #[deprecated(note = "使用 android.provider.Telephony.ServiceStateTable#SYSTEM_ID。")]
    pub const EXTRA_SYSTEM_ID: &'static str = "systemId";

    //noinspection SpellCheckingInspection
    /**
    如果在 CDMA 或 EVDO 系统上注册，则与 ACTION_SERVICE_STATE 一起使用的整数额外值表示 TSB-58 漫游指示符，否则表示 ` `。
    */
    #[deprecated(
        note = "使用 android.provider.Telephony.ServiceStateTable#CDMA_ROAMING_INDICATOR。"
    )]
    pub const EXTRA_CDMA_ROAMING_INDICATOR: &'static str = "cdmaRoamingIndicator";

    //noinspection SpellCheckingInspection
    /**
    如果在 CDMA 或 EVDO 系统上注册，则与 ACTION_SERVICE_STATE 一起使用的整数额外值表示来自 PRL 的默认漫游指示符 ` ` 如果没有。
    */
    #[deprecated(
        note = "使用 android.provider.Telephony.ServiceStateTable#CDMA_DEFAULT_ROAMING_INDICATOR。"
    )]
    pub const EXTRA_CDMA_DEFAULT_ROAMING_INDICATOR: &'static str = "cdmaDefaultRoamingIndicator";

    /**
    与 ACTION_SERVICE_STATE 一起使用的布尔值，表示是否处于紧急模式。否则为 ` `。
    */
    #[deprecated(note = "使用 android.provider.Telephony.ServiceStateTable#IS_EMERGENCY_ONLY。")]
    pub const EXTRA_EMERGENCY_ONLY: &'static str = "emergencyOnly";

    /**
    与 ACTION_SERVICE_STATE 一起使用的布尔值，指示数据网络注册状态是否为漫游。` ` 否则
    */
    #[deprecated(
        note = "使用 android.provider.Telephony.ServiceStateTable#IS_DATA_ROAMING_FROM_REGISTRATION。"
    )]
    pub const EXTRA_IS_DATA_ROAMING_FROM_REGISTRATION: &'static str =
        "isDataRoamingFromRegistration";

    /**
    与 ACTION_SERVICE_STATE 一起使用的布尔值，指示是否正在使用载波聚合。否则为 ` `。
    */
    #[deprecated(
        note = "使用 android.provider.Telephony.ServiceStateTable#IS_USING_CARRIER_AGGREGATION。"
    )]
    pub const EXTRA_IS_USING_CARRIER_AGGREGATION: &'static str = "isUsingCarrierAggregation";

    //noinspection SpellCheckingInspection
    /**
    与 ACTION_SERVICE_STATE 一起使用的额外整数，表示在计算信号强度级别时从 rsrp 阈值减少的偏移量。
    */
    #[deprecated]
    pub const EXTRA_LTE_EARFCN_RSRP_BOOST: &'static str = "LteEarfcnRsrpBoost";

    /// 用于定义要处理的文本的额外名称，为 CharSequence。请注意，这可能是样式化的 CharSequence，因此您必须使用 Bundle#getCharSequence(String) Bundle.getCharSequence() 来查询它。
    pub const EXTRA_PROCESS_TEXT: &'static str = "android.intent.extra.PROCESS_TEXT";

    /// 用于定义已处理文本是否将用作只读的布尔附加名称。
    pub const EXTRA_PROCESS_TEXT_READONLY: &'static str =
        "android.intent.extra.PROCESS_TEXT_READONLY";

    /// 广播动作：报告何时发生新的热事件。当设备达到其最高温度时，报告的热级别
    pub const ACTION_THERMAL_EVENT: &'static str = "android.intent.action.THERMAL_EVENT";

    #[doc(hidden)]
    pub const EXTRA_THERMAL_STATE: &'static str = "android.intent.extra.THERMAL_STATE";

    /// 设备正常时的热状态。此状态在 ACTION_THERMAL_EVENT 广播中作为 EXTRA_THERMAL_STATE 发送。
    pub const EXTRA_THERMAL_STATE_NORMAL: i32 = 0;

    /// 设备接近其最大阈值的热状态。此状态在 ACTION_THERMAL_EVENT 广播中作为 EXTRA_THERMAL_STATE 发送。
    pub const EXTRA_THERMAL_STATE_WARNING: i32 = 1;

    /// 设备已达到其最大阈值的热状态。此状态在 ACTION_THERMAL_EVENT 广播中作为 EXTRA_THERMAL_STATE 发送。
    pub const EXTRA_THERMAL_STATE_EXCEEDED: i32 = 2;

    /**
    广播动作：指示设备对接时基座处于空闲状态。
    这是一个受保护的意图，只能由系统发送。
    */
    pub const ACTION_DOCK_IDLE: &'static str = "android.intent.action.DOCK_IDLE";

    /**
    广播动作：指示设备对接时基座处于活动状态。
    这是一个受保护的意图，只能由系统发送。
    */
    pub const ACTION_DOCK_ACTIVE: &'static str = "android.intent.action.DOCK_ACTIVE";

    /**
    广播操作：表示已下载并应用新的设备定制（安装的软件包、启用的运行时资源覆盖、复制的 xml 文件等），并且现在是时候让需要清除缓存的组件执行这些操作了。
    */
    pub const ACTION_DEVICE_CUSTOMIZATION_READY: &'static str =
        "android.intent.action.DEVICE_CUSTOMIZATION_READY";

    /**
    活动操作：显示与唯一 LocusId 关联的活动状态。
    例如，聊天应用程序可以使用上下文来恢复两个用户之间的对话。
    输入：EXTRA_LOCUS_ID 指定应用程序域中轨迹的唯一标识符。应在重启和备份/恢复后保持稳定。
    输出：无。
    */
    pub const ACTION_VIEW_LOCUS: &'static str = "android.intent.action.VIEW_LOCUS";

    /**
    Activity 操作：启动可用于创建笔记的笔记记录 Activity。此操作可用于在锁定屏幕上启动 Activity。Activity 应确保在锁定屏幕上启动时适当处理隐私敏感数据和功能。请参阅 android.app.KeyguardManager 了解锁定屏幕检查。
    */
    pub const ACTION_CREATE_NOTE: &'static str = "android.intent.action.CREATE_NOTE";

    /**
    与 ACTION_CREATE_NOTE 一起使用的布尔值，指示启动的笔记记录活动是否应显示适合使用手写笔输入的 UI。
    */
    pub const EXTRA_USE_STYLUS_MODE: &'static str = "android.intent.extra.USE_STYLUS_MODE";

    /**
    活动操作：与 startActivityForResult 一起使用，启动系统活动以捕获屏幕上的内容，从而截取屏幕截图并将其呈现给用户进行编辑。编辑后的屏幕截图将保存在设备上，并通过 getData() 作为 Uri 返回给调用活动。需要用户交互才能将编辑后的屏幕截图返回给调用活动。
    此意图操作需要权限android.Manifest.permission#LAUNCH_CAPTURE_CONTENT_ACTIVITY_FOR_NOTE。
    调用者应该在显示允许用户触发此流程的 UI 元素之前查询 StatusBarManager#canLaunchCaptureContentActivityForNote(Activity)。
    */
    pub const ACTION_LAUNCH_CAPTURE_CONTENT_ACTIVITY_FOR_NOTE: &'static str =
        "android.intent.action.LAUNCH_CAPTURE_CONTENT_ACTIVITY_FOR_NOTE";

    /**
    由以 ACTION_LAUNCH_CAPTURE_CONTENT_ACTIVITY_FOR_NOTE 启动的活动使用的 int extra 来指示响应状态。此 extra 与设置为 android.app.Activity#RESULT_OK 的结果代码一起使用。
    此额外值可以是以下之一：CAPTURE_CONTENT_FOR_NOTE_SUCCESS CAPTURE_CONTENT_FOR_NOTE_FAILED CAPTURE_CONTENT_FOR_NOTE_USER_CANCELED CAPTURE_CONTENT_FOR_NOTE_WINDOW_MODE_UNSUPPORTED CAPTURE_CONTENT_FOR_NOTE_BLOCKED_BY_ADMIN
    */
    pub const EXTRA_CAPTURE_CONTENT_FOR_NOTE_STATUS_CODE: &'static str =
        "android.intent.extra.CAPTURE_CONTENT_FOR_NOTE_STATUS_CODE";

    /**
    与 EXTRA_CAPTURE_CONTENT_FOR_NOTE_STATUS_CODE 一起使用的响应代码表明请求成功。
    仅当用户与系统截图活动交互同意与笔记共享数据后，才会返回此代码。
    捕获的截图通过 getData() 以 Uri 形式返回。
    */
    pub const CAPTURE_CONTENT_FOR_NOTE_SUCCESS: i32 = 0;

    /// 与 EXTRA_CAPTURE_CONTENT_FOR_NOTE_STATUS_CODE 一起使用的响应代码表示出现了问题。
    pub const CAPTURE_CONTENT_FOR_NOTE_FAILED: i32 = 1;

    /// 与 EXTRA_CAPTURE_CONTENT_FOR_NOTE_STATUS_CODE 一起使用的响应代码表明用户取消了内容捕获流程。
    pub const CAPTURE_CONTENT_FOR_NOTE_USER_CANCELED: i32 = 2;

    /// 与 EXTRA_CAPTURE_CONTENT_FOR_NOTE_STATUS_CODE 一起使用的响应代码，指示意图操作 ACTION_LAUNCH_CAPTURE_CONTENT_ACTIVITY_FOR_NOTE 是由在非支持的窗口模式下运行的活动启动的。
    pub const CAPTURE_CONTENT_FOR_NOTE_WINDOW_MODE_UNSUPPORTED: i32 = 3;

    /// 与 EXTRA_CAPTURE_CONTENT_FOR_NOTE_STATUS_CODE 一起使用的响应代码，表示屏幕截图已被 IT 管理员阻止。
    pub const CAPTURE_CONTENT_FOR_NOTE_BLOCKED_BY_ADMIN: i32 = 4;

    /**
    广播动作：当需要验证包时发送给完整性组件。数据包含包 URI 以及其他相关信息。
    这是一个受保护的意图，只能由系统发送。
    */
    pub const ACTION_PACKAGE_NEEDS_INTEGRITY_VERIFICATION: &'static str =
        "android.intent.action.PACKAGE_NEEDS_INTEGRITY_VERIFICATION";

    /**
    广播动作：启动前台服务管理器。
    这是一个受保护的意图，只能由系统发送。
    */
    pub const ACTION_SHOW_FOREGROUND_SERVICE_MANAGER: &'static str =
        "android.intent.action.SHOW_FOREGROUND_SERVICE_MANAGER";

    /// 设置活动是否应该是默认操作（中心按下）在数据片段上执行的选项。 在对某些数据执行操作时，设置此操作将对用户隐藏任何没有设置的活动。 请注意，在启动操作时，通常是在意图中设置的 - 用于在软件包中指定的意图过滤器中使用。
    pub const CATEGORY_DEFAULT: &'static str = "android.intent.category.DEFAULT";

    /// 可以从浏览器安全调用的活动必须支持此类别。例如，如果用户正在查看网页或电子邮件并点击文本中的链接，则生成的执行该链接的 Intent 将需要 BROWSABLE 类别，因此只有支持此类别的活动才会视为可能的操作。通过支持此类别，您可以保证调用任何匹配的 Intent 都不会发生任何破坏性事件（无需用户干预）。
    pub const CATEGORY_BROWSABLE: &'static str = "android.intent.category.BROWSABLE";

    //noinspection SpellCheckingInspection
    /// 可参与语音交互的活动类别。支持此类别的活动必须准备好在完全不显示 UI 的情况下运行（尽管在某些情况下可能会显示 UI），并依靠 android.app.VoiceInteractor 与用户交互。
    pub const CATEGORY_VOICE: &'static str = "android.intent.category.VOICE";

    /**
    如果活动应被视为用户当前正在查看的数据的替代操作，则设置此设置。另请参阅 CATEGORY_SELECTED_ALTERNATIVE，了解适用于项目列表中的选择的替代操作。
    支持此类别意味着您希望您的活动显示在用户可以执行的备选操作集合中，通常作为当前活动的选项菜单的一部分。您通常希望在此操作的 &lt;intent-filter&gt; 中包含一个特定标签，向用户描述它的作用。
    此类别的 IntentFilter 操作非常重要，因为它描述了目标将执行的特定操作。这通常不应是通用操作（例如 ACTION_VIEW），而应是特定名称，例如“com.android.camera.action.CROP”。任何特定操作都只会向用户显示一种替代方案，因此使用这样的特定操作可确保您的替代方案能够显示，同时还允许其他应用程序提供对该特定操作的自己的覆盖。
    */
    pub const CATEGORY_ALTERNATIVE: &'static str = "android.intent.category.ALTERNATIVE";

    /// 如果活动应被视为用户当前所选数据的替代选择操作，则设置此操作。这类似于 CATEGORY_ALTERNATIVE，但用于显示用户可以从中选择的项目列表的活动，为他们提供将对其执行的默认操作的替代方案。
    pub const CATEGORY_SELECTED_ALTERNATIVE: &'static str =
        "android.intent.category.SELECTED_ALTERNATIVE";

    /// 旨在用作包含 TabActivity 内的选项卡。
    pub const CATEGORY_TAB: &'static str = "android.intent.category.TAB";

    /// 应显示在顶层启动器中。
    pub const CATEGORY_LAUNCHER: &'static str = "android.intent.category.LAUNCHER";

    /// 表示针对 Leanback 模式优化的活动，并且应显示在 Leanback 启动器中。
    pub const CATEGORY_LEANBACK_LAUNCHER: &'static str =
        "android.intent.category.LEANBACK_LAUNCHER";

    /// 指示从车载启动器启动应用时的首选入口点活动。如果不存在，车载启动器可以选择使用 CATEGORY_LAUNCHER 作为后备，或完全排除该应用。
    pub const CATEGORY_CAR_LAUNCHER: &'static str = "android.intent.category.CAR_LAUNCHER";

    /// 用于表明该活动可以以公共模式使用。
    pub const CATEGORY_COMMUNAL_MODE: &'static str = "android.intent.category.COMMUNAL_MODE";

    /// 表示在 Leanback 启动器中显示的 Leanback 设置活动。
    pub const CATEGORY_LEANBACK_SETTINGS: &'static str =
        "android.intent.category.LEANBACK_SETTINGS";

    /// 提供有关其所在包的信息；通常用于当包不包含 CATEGORY_LAUNCHER 时为用户提供前门，而不必显示在所有应用程序列表中。
    pub const CATEGORY_INFO: &'static str = "android.intent.category.INFO";

    /// 这是主活动，即设备启动时显示的第一个活动。
    pub const CATEGORY_HOME: &'static str = "android.intent.category.HOME";

    /// 这是设备完成设置并准备使用时显示的主活动。
    pub const CATEGORY_HOME_MAIN: &'static str = "android.intent.category.HOME_MAIN";

    /// 主活动显示在支持显示主活动的辅助显示屏上。
    pub const CATEGORY_SECONDARY_HOME: &'static str = "android.intent.category.SECONDARY_HOME";

    /// 这是设置向导活动，即用户首次设置设备时显示的第一个活动。
    pub const CATEGORY_SETUP_WIZARD: &'static str = "android.intent.category.SETUP_WIZARD";

    /// 这是主活动，即充当启动器应用的活动，用户可以从那里启动其他应用。通常，具有较低/较高优先级意图过滤器的组件会处理主意图（例如 SetupWizard）来设置设备，我们需要能够将主应用与这些设置助手区分开来。
    pub const CATEGORY_LAUNCHER_APP: &'static str = "android.intent.category.LAUNCHER_APP";

    /// 此活动是一个偏好面板。
    pub const CATEGORY_PREFERENCE: &'static str = "android.intent.category.PREFERENCE";

    /// 此活动是开发偏好面板。
    pub const CATEGORY_DEVELOPMENT_PREFERENCE: &'static str =
        "android.intent.category.DEVELOPMENT_PREFERENCE";

    /// 能够在父活动容器内运行。
    pub const CATEGORY_EMBED: &'static str = "android.intent.category.EMBED";

    /// 此活动允许用户浏览和下载新的应用程序。
    pub const CATEGORY_APP_MARKET: &'static str = "android.intent.category.APP_MARKET";

    /// 这个活动可以由猴子或者其他自动化测试工具来执行。
    pub const CATEGORY_MONKEY: &'static str = "android.intent.category.MONKEY";

    /// 用作测试（不是正常用户体验的一部分）。
    pub const CATEGORY_TEST: &'static str = "android.intent.category.TEST";

    /// 用作单元测试（通过测试工具运行）。
    pub const CATEGORY_UNIT_TEST: &'static str = "android.intent.category.UNIT_TEST";

    /// 用作示例代码示例（不是正常用户体验的一部分）。
    pub const CATEGORY_SAMPLE_CODE: &'static str = "android.intent.category.SAMPLE_CODE";

    //noinspection SpellCheckingInspection
    /// 用于指示意图仅需要可以使用 ContentResolver#openFileDescriptor(Uri, String) 打开的 URI。可打开的 URI 在查询时必须至少支持 OpenableColumns 中定义的列。
    pub const CATEGORY_OPENABLE: &'static str = "android.intent.category.OPENABLE";

    //noinspection SpellCheckingInspection
    /// 用于指示意图过滤器可以接受不一定可通过 ContentResolver#openFileDescriptor(Uri, String) 打开的文件，但至少可通过 ContentResolver#openTypedAssetFileDescriptor(Uri, String, Bundle) 使用通过 ContentResolver#getStreamTypes(Uri, String) 公开的流类型之一进行流式传输。
    pub const CATEGORY_TYPED_OPENABLE: &'static str = "android.intent.category.TYPED_OPENABLE";

    /// 用作框架检测测试的被测代码。
    pub const CATEGORY_FRAMEWORK_INSTRUMENTATION_TEST: &'static str =
        "android.intent.category.FRAMEWORK_INSTRUMENTATION_TEST";

    /// 当设备插入车载基座时运行的活动。与 ACTION_MAIN 一起使用以启动活动。有关更多信息，请参阅 android.app.UiModeManager。
    pub const CATEGORY_CAR_DOCK: &'static str = "android.intent.category.CAR_DOCK";

    /// 当设备插入桌面基座时运行的活动。与 ACTION_MAIN 一起使用以启动活动。有关更多信息，请参阅 android.app.UiModeManager。
    pub const CATEGORY_DESK_DOCK: &'static str = "android.intent.category.DESK_DOCK";

    /// 当设备插入模拟（低端）基座时运行的活动。与 ACTION_MAIN 一起使用以启动活动。有关更多信息，请参阅 android.app.UiModeManager。
    pub const CATEGORY_LE_DESK_DOCK: &'static str = "android.intent.category.LE_DESK_DOCK";

    /// 当设备插入数字（高端）基座时运行的活动。与 ACTION_MAIN 一起使用以启动活动。有关更多信息，请参阅 android.app.UiModeManager。
    pub const CATEGORY_HE_DESK_DOCK: &'static str = "android.intent.category.HE_DESK_DOCK";

    /// 用于表明该活动可以在车载环境中使用。
    pub const CATEGORY_CAR_MODE: &'static str = "android.intent.category.CAR_MODE";

    /// 当设备放置在 VR 头戴式设备查看器中时，用于启动器的活动。与 ACTION_MAIN 一起使用以启动活动。有关更多信息，请参阅 android.app.UiModeManager。
    pub const CATEGORY_VR_HOME: &'static str = "android.intent.category.VR_HOME";

    //noinspection SpellCheckingInspection
    /**
    辅助功能快捷方式是残障用户触发对他们来说很重要的辅助功​​能的一种全局手势，可帮助开发人员确定是否要将其活动设为快捷方式目标。有辅助功能需求的用户感兴趣的活动可能会请求成为辅助功能快捷方式的目标。它处理此类别的意图 ACTION_MAIN，当用户激活配置为指向此目标的快捷方式时，系统将分派该意图。活动在 AndroidManifest.xml 中声明自己是快捷方式的目标。它还必须做两件事：指定它处理类别为 android.intent.category.ACCESSIBILITY_SHORTCUT_TARGET 的 android.intent.action.MAIN android.content.Intent。在声明活动时，在清单中提供元数据条目 android.accessibilityshortcut.target。如果缺少其中任何一项，系统将忽略辅助功能快捷方式目标。以下是示例声明：
    ```xml
    <activity android:name=".MainActivity" ...
    <intent-filter>
    <action android:name="android.intent.action.MAIN" />
    <category android:name="android.intent.category.ACCESSIBILITY_SHORTCUT_TARGET" />
    </intent-filter>
    <meta-data android:name="android.accessibilityshortcut.target" android:resource="@xml/accessibilityshortcut" />
    </activity>
    ```
    这是配置辅助功能快捷方式目标的示例 XML 文件：
    ```xml
    <accessibility-shortcut-target android:description="@string/shortcut_target_description" android:summary="@string/shortcut_target_summary" android:animatedImageDrawable="@drawable/shortcut_target_animated_image" android:htmlDescription="@string/shortcut_target_html_description" android:settingsActivity="com.example.android.shortcut.target.SettingsActivity" />
    ```
    描述和摘要都是必需的。如果缺少它们，系统将忽略辅助功能快捷方式目标。支持动画图像和 html 描述，以帮助用户了解如何使用快捷方式目标。设置活动是一个组件名称，允许用户修改此辅助功能快捷方式目标的设置。
    */
    pub const CATEGORY_ACCESSIBILITY_SHORTCUT_TARGET: &'static str =
        "android.intent.category.ACCESSIBILITY_SHORTCUT_TARGET";

    /// 与 ACTION_MAIN 一起使用以启动浏览器应用程序。该活动应该能够浏览互联网。注意：不应将其用作 Intent 的主键，因为它不会导致应用程序以正确的操作和类别启动。相反，应将其与 makeMainSelectorActivity(String, String) 一起使用以在选择器中生成具有此类别的主 Intent。
    pub const CATEGORY_APP_BROWSER: &'static str = "android.intent.category.APP_BROWSER";

    /// 与 ACTION_MAIN 一起使用以启动计算器应用程序。该活动应能够执行标准算术运算。注意：不应将其用作 Intent 的主键，因为它不会导致应用程序以正确的操作和类别启动。相反，应将其与 makeMainSelectorActivity(String, String) 一起使用以在选择器中生成具有此类别的主 Intent。
    pub const CATEGORY_APP_CALCULATOR: &'static str = "android.intent.category.APP_CALCULATOR";

    /// 与 ACTION_MAIN 一起使用以启动日历应用程序。该活动应该能够查看和操作日历条目。注意：不应将其用作 Intent 的主键，因为它不会导致应用程序以正确的操作和类别启动。相反，应将其与 makeMainSelectorActivity(String, String) 一起使用以在选择器中生成具有此类别的主 Intent。
    pub const CATEGORY_APP_CALENDAR: &'static str = "android.intent.category.APP_CALENDAR";

    /// 与 ACTION_MAIN 一起使用以启动联系人应用程序。此活动应能够查看和操作地址簿条目。注意：不应将其用作 Intent 的主键，因为它不会导致应用程序以正确的操作和类别启动。相反，应将其与 makeMainSelectorActivity(String, String) 一起使用以在选择器中生成具有此类别的主 Intent。
    pub const CATEGORY_APP_CONTACTS: &'static str = "android.intent.category.APP_CONTACTS";

    /// 与 ACTION_MAIN 一起使用以启动电子邮件应用程序。该活动应该能够发送和接收电子邮件。注意：不应将其用作 Intent 的主键，因为它不会导致应用程序以正确的操作和类别启动。相反，应将其与 makeMainSelectorActivity(String, String) 一起使用以在选择器中生成具有此类别的主 Intent。
    pub const CATEGORY_APP_EMAIL: &'static str = "android.intent.category.APP_EMAIL";

    /// 与 ACTION_MAIN 一起使用以启动图库应用程序。此活动应能够查看和操作存储在设备上的图像和视频文件。注意：不应将其用作 Intent 的主键，因为它不会导致应用程序以正确的操作和类别启动。相反，应将其与 makeMainSelectorActivity(String, String) 一起使用以在选择器中生成带有此类别的主 Intent。
    pub const CATEGORY_APP_GALLERY: &'static str = "android.intent.category.APP_GALLERY";

    /// 与 ACTION_MAIN 一起使用以启动地图应用程序。活动应能够显示用户的当前位置和周围环境。注意：不应将其用作 Intent 的主键，因为它不会导致应用程序以正确的操作和类别启动。相反，应将其与 makeMainSelectorActivity(String, String) 一起使用以在选择器中生成具有此类别的主 Intent。
    pub const CATEGORY_APP_MAPS: &'static str = "android.intent.category.APP_MAPS";

    /// 与 ACTION_MAIN 一起使用以启动消息传递应用程序。该活动应该能够发送和接收短信。注意：不应将其用作 Intent 的主键，因为它不会导致应用程序以正确的操作和类别启动。相反，应将其与 makeMainSelectorActivity(String, String) 一起使用以在选择器中生成具有此类别的主 Intent。
    pub const CATEGORY_APP_MESSAGING: &'static str = "android.intent.category.APP_MESSAGING";

    /// 与 ACTION_MAIN 一起使用以启动音乐应用程序。该活动应能够播放、浏览或操作设备上存储的音乐文件。注意：不应将其用作 Intent 的主键，因为它不会导致应用程序以正确的操作和类别启动。相反，应将其与 makeMainSelectorActivity(String, String) 一起使用以在选择器中生成具有此类别的主 Intent。
    pub const CATEGORY_APP_MUSIC: &'static str = "android.intent.category.APP_MUSIC";

    /// 与 ACTION_MAIN 一起使用以启动文件应用程序。该活动应该能够浏览和管理设备上存储的文件。注意：不应将其用作 Intent 的主键，因为它不会导致应用程序以正确的操作和类别启动。相反，应将其与 makeMainSelectorActivity(String, String) 一起使用以在选择器中生成具有此类别的主 Intent。
    pub const CATEGORY_APP_FILES: &'static str = "android.intent.category.APP_FILES";

    /// 与 ACTION_MAIN 一起使用以启动天气应用程序。该活动应该能够向用户提供有关天气的信息。注意：这不应用作 Intent 的主键，因为它不会导致应用程序以正确的操作和类别启动。相反，将其与 makeMainSelectorActivity(String, String) 一起使用以在选择器中生成具有此类别的主 Intent。
    pub const CATEGORY_APP_WEATHER: &'static str = "android.intent.category.APP_WEATHER";

    /// 与 ACTION_MAIN 一起使用以启动健身应用程序。该活动应能够向用户提供健身信息并管理锻炼。注意：不应将其用作 Intent 的主键，因为它不会导致应用程序以正确的操作和类别启动。相反，应将其与 makeMainSelectorActivity(String, String) 一起使用以在选择器中生成具有此类别的主 Intent。
    pub const CATEGORY_APP_FITNESS: &'static str = "android.intent.category.APP_FITNESS";

    /// 放置在新创建记录中的初始数据。与 ACTION_INSERT 一起使用。此处的数据是一个 Map，其中包含与提供给底层 ContentProvider.insert() 调用的相同字段。
    pub const EXTRA_TEMPLATE: &'static str = "android.intent.extra.TEMPLATE";

    /// 与 Intent 关联的常量 CharSequence，与 ACTION_SEND 一起使用以提供要发送的文字数据。请注意，这可能是样式化的 CharSequence，因此您必须使用 Bundle#getCharSequence(String) Bundle.getCharSequence() 来查询它。
    pub const EXTRA_TEXT: &'static str = "android.intent.extra.TEXT";

    /// 与 Intent 关联的常量字符串，与 ACTION_SEND 一起使用，以 HTML 格式的文本提供 EXTRA_TEXT 的替代方案。请注意，您还必须提供 EXTRA_TEXT。
    pub const EXTRA_HTML_TEXT: &'static str = "android.intent.extra.HTML_TEXT";

    /// 内容：URI 保存与 Intent 关联的数据流，与 ACTION_SEND 一起使用来提供正在发送的数据。
    pub const EXTRA_STREAM: &'static str = "android.intent.extra.STREAM";

    /// 一个 String[]，包含需要发送到的电子邮件地址。
    pub const EXTRA_EMAIL: &'static str = "android.intent.extra.EMAIL";

    /// 一个 String[]，其中包含需要抄送的电子邮件地址。
    pub const EXTRA_CC: &'static str = "android.intent.extra.CC";

    /// 一个 String[]，其中包含需要密送的电子邮件地址。
    pub const EXTRA_BCC: &'static str = "android.intent.extra.BCC";

    /// 保存所需消息主题行的常量字符串。
    pub const EXTRA_SUBJECT: &'static str = "android.intent.extra.SUBJECT";

    /// 一个 Intent，描述您希望通过 ACTION_PICK_ACTIVITY 或 ACTION_CHOOSER 显示的选项。
    pub const EXTRA_INTENT: &'static str = "android.intent.extra.INTENT";

    /// 一个表示要使用的用户 ID 的 int。
    pub const EXTRA_USER_ID: &'static str = "android.intent.extra.USER_ID";

    /// 表示要查询的任务 ID 的 int。当最近启动被另一个操作（例如凭证确认）拦截时使用此 ID，以记住完成后应恢复哪个任务。
    pub const EXTRA_TASK_ID: &'static str = "android.intent.extra.TASK_ID";

    /**
    与 ACTION_VIEW_PERMISSION_USAGE_FOR_PERIOD 和 ACTION_MANAGE_PERMISSION_USAGE 一起使用时，包含归因标签的 String[]
    例如，归因标签可以是location_provider、com.google.android.gms.*等。
    */
    pub const EXTRA_ATTRIBUTION_TAGS: &'static str = "android.intent.extra.ATTRIBUTION_TAGS";

    /// 与 ACTION_VIEW_PERMISSION_USAGE_FOR_PERIOD 和 ACTION_MANAGE_PERMISSION_USAGE 一起使用时，表示权限使用开始时间戳（以毫秒为单位的纪元时间）的长整型值
    pub const EXTRA_START_TIME: &'static str = "android.intent.extra.START_TIME";

    /// 与 ACTION_VIEW_PERMISSION_USAGE_FOR_PERIOD 和 ACTION_MANAGE_PERMISSION_USAGE 一起使用时，表示权限使用结束时间戳（以毫秒为单位的纪元时间）的长整型值
    pub const EXTRA_END_TIME: &'static str = "android.intent.extra.END_TIME";

    /**
    当与 ACTION_VIEW_PERMISSION_USAGE_FOR_PERIOD 和 ACTION_MANAGE_PERMISSION_USAGE 一起使用时，布尔值额外指定权限使用系统 UI 是否显示所选条目的归因信息。
    仅当应用程序在其清单中指定了 attributionsAreUserVisible 时，额外内容才为真。
    应用程序可以使用此额外功能来改善其权限使用解释体验。
    */
    pub const EXTRA_SHOWING_ATTRIBUTION: &'static str = "android.intent.extra.SHOWING_ATTRIBUTION";

    /**
    Intent[] 描述了您希望使用 ACTION_CHOOSER 显示的附加、替代选择。
    应用可能能够提供多种不同的有效负载类型来完成用户的预期操作。例如，调用 ACTION_SEND 与另一个应用共享照片的应用可以使用 EXTRA_ALTERNATE_INTENTS 让选择器透明地提供多种不同的受支持的共享发送机制，例如实际的“image/\*”照片数据或可以查看照片的托管链接。
    EXTRA_INTENT 中存在的意图将被视为集合中的第一个/主要/首选意图。此额外内容中指定的其他意图是有序的；默认情况下，数组中较早出现的意图将优先于数组中较晚出现的意图，作为同一目标组件的匹配项。要更改此偏好，调用应用还可以提供 EXTRA_CHOOSER_REFINEMENT_INTENT_SENDER。
    */
    pub const EXTRA_ALTERNATE_INTENTS: &'static str = "android.intent.extra.ALTERNATE_INTENTS";

    /**
    ComponentName ComponentName[] 描述应该从呈现给用户的组件列表中过滤掉和省略的组件。
    与 ACTION_CHOOSER 一起使用时，选择器将忽略此数组中的任何组件（如果本来会显示这些组件）。如果发送到这些目标的想法与其他应用功能重复，则可用于忽略您自己的软件包或组织中的其他应用中的特定目标。已筛选的组件将无法显示来自关联 ChooserTargetService 的目标。
    */
    pub const EXTRA_EXCLUDE_COMPONENTS: &'static str = "android.intent.extra.EXCLUDE_COMPONENTS";

    /**
    ACTION_CHOOSER 的 android.service.chooser.ChooserTarget ChooserTarget[] 描述了选择器向用户呈现得额外高优先级深层链接目标。
    以此方式提供的目标将与其他应用的服务提供的所有其他目标一起显示。它们的优先级将高于其他服务目标，但低于用户手动固定在前面的来源提供的目标。您最多可以在此额外内容中提供两个目标（从 Android 10 开始，两个目标的限制开始生效）。
    */
    pub const EXTRA_CHOOSER_TARGETS: &'static str = "android.intent.extra.CHOOSER_TARGETS";

    /**
    当用户从 ACTION_CHOOSER 呈现的选择器活动中做出选择时，将调用 Activity 的 IntentSender。
    一个应用程序正在为另一个应用程序准备一个操作以完成它，它可能希望允许用户根据所选目标在几个完成该操作的选项之间进行消除歧义，或者在调用该操作之前以其他方式优化该操作。
    发送时，此 IntentSender 可能会填充以下额外内容：EXTRA_INTENT 与用户选择的目标匹配的第一个意图 EXTRA_ALTERNATE_INTENTS 除了第一个意图之外，还与用户选择的目标匹配的任何其他意图 EXTRA_RESULT_RECEIVER 细化活动应在消除歧义后填写并发送的 ResultReceiver
    */
    pub const EXTRA_CHOOSER_REFINEMENT_INTENT_SENDER: &'static str =
        "android.intent.extra.CHOOSER_REFINEMENT_INTENT_SENDER";

    /// ChooserAction对象的一个​​包裹[]，以在调用Action_Chooser时向用户提供特定于App的操作。您可以提供多达五个自定义操作。
    pub const EXTRA_CHOOSER_CUSTOM_ACTIONS: &'static str =
        "android.intent.extra.CHOOSER_CUSTOM_ACTIONS";

    /// 与 ACTION_CHOOSER 一起使用的可选参数。ChooserAction 允许用户以某种方式修改正在共享的内容。这可以集成到具有预览 UI 的共享表上的内容预览中。
    pub const EXTRA_CHOOSER_MODIFY_SHARE_ACTION: &'static str =
        "android.intent.extra.CHOOSER_MODIFY_SHARE_ACTION";

    /**
    一个描述 ACTION_CHOOSER 内容的“”注释。
    如果用于启动 ACTION_CHOOSER 活动的意图中存在 EXTRA_CONTENT_ANNOTATIONS，则前三个注释将用于对应用程序进行排名。
    注释应描述内容的主要组成部分或主题。启动 ACTION_CHOOSER 的应用需要了解并添加注释。应提前了解注释，例如在创建或保存内容时，以避免增加启动 ACTION_CHOOSER 的延迟。自定义注释的名称不应包含冒号字符。如果自定义注释在过去 14 天内很少用于 ACTION_CHOOSER，则其性能可能会受到影响。因此，建议在适用时使用以下注释。
    “product”表示内容的主题主要与产品有关，例如健康与美容和办公用品。“emotion”表示内容的主题主要与情绪有关，例如快乐和悲伤。“person”表示内容的主题主要与人有关，例如脸、手指、站立和行走。“child”表示内容的主题主要与儿童有关，例如儿童和婴儿。 “自拍”表示内容主题主要为自拍。“人群”表示内容主题主要为人群。“聚会”表示内容主题主要为聚会。“动物”表示内容主题主要为动物。“植物”表示内容主题主要为植物，例如鲜花。“度假”表示内容主题主要为度假。“时尚”表示内容主题主要为时尚，例如太阳镜、珠宝、手袋和服装。“材料”表示内容主题主要为材料，例如纸张和丝绸。“交通工具”表示内容主题主要为交通工具，例如汽车和船只。“文档”表示内容主题主要为文档，例如海报。“设计”表示内容主题主要为设计，例如艺术和房屋设计。 “holiday”代表内容主题主要与节日有关，例如圣诞节、感恩节。
    */
    pub const EXTRA_CONTENT_ANNOTATIONS: &'static str = "android.intent.extra.CONTENT_ANNOTATIONS";

    /**
    ResultReceiver 用于将数据返回给发送者。
    用于完成针对 ACTION_CHOOSER 的应用特定的 EXTRA_CHOOSER_REFINEMENT_INTENT_SENDER 细化。
    如果用于启动 ACTION_CHOOSER 活动的 Intent 中存在 EXTRA_CHOOSER_REFINEMENT_INTENT_SENDER，则此 extra 将以 fillIn(Intent, int) 形式填充到该 IntentSender 中，并在用户从选择器中选择目标组件时发送。接收者负责将结果发送到此 ResultReceiver，以表示歧义消除已完成，并且选择器应调用用户的选择。
    歧义消除器应向 ResultReceiver 提供一个 Bundle，其中将一个意图分配给键 EXTRA_INTENT。选择器将在启动之前使用此提供的意图匹配并填写最终的 Intent 或 ChooserTarget。提供的意图必须 filterEquals(Intent) 与传递给 EXTRA_CHOOSER_REFINEMENT_INTENT_SENDER 的 EXTRA_INTENT 或 EXTRA_ALTERNATE_INTENTS 中的一个意图匹配，才能被接受。
    如果细化成功并且应该启动选择器中提供的意图的目标，则传递给 ResultReceiver 的结果代码应该是 android.app.Activity#RESULT_OK，或者如果选择器应该在不启动目标的情况下完成，则传递给 ResultReceiver 的结果代码应该是 android.app.Activity#RESULT_CANCELED。
    */
    pub const EXTRA_RESULT_RECEIVER: &'static str = "android.intent.extra.RESULT_RECEIVER";

    /// 与 ACTION_CHOOSER 一起使用时向用户提供的 CharSequence 对话框标题。
    pub const EXTRA_TITLE: &'static str = "android.intent.extra.TITLE";

    /// 使用 putExtra(String, Parcelable[]) 设置的 Intent 或 android.content.pm.LabeledIntent 对象的 Parcelable[]，当使用 ACTION_CHOOSER 显示给用户时，放置在选择列表的最前面。您最多可以选择在应用建议之前显示两个附加活动（从 Android 10 开始，两个附加活动的限制开始生效）。
    pub const EXTRA_INITIAL_INTENTS: &'static str = "android.intent.extra.INITIAL_INTENTS";

    /// 即时应用安装成功后启动的 IntentSender。
    pub const EXTRA_INSTANT_APP_SUCCESS: &'static str = "android.intent.extra.INSTANT_APP_SUCCESS";

    /// 即时应用安装失败后启动的 IntentSender。
    pub const EXTRA_INSTANT_APP_FAILURE: &'static str = "android.intent.extra.INSTANT_APP_FAILURE";

    /// 触发即时应用解析的主机名。
    pub const EXTRA_INSTANT_APP_HOSTNAME: &'static str =
        "android.intent.extra.INSTANT_APP_HOSTNAME";

    /// 用于跟踪即时应用程序分辨率的不透明令牌。
    pub const EXTRA_INSTANT_APP_TOKEN: &'static str = "android.intent.extra.INSTANT_APP_TOKEN";

    /// 触发即时应用程序解析的操作。
    pub const EXTRA_INSTANT_APP_ACTION: &'static str = "android.intent.extra.INSTANT_APP_ACTION";

    /// 包含有关已解析的即时应用程序的详细信息的 Bundles 数组。
    pub const EXTRA_INSTANT_APP_BUNDLES: &'static str = "android.intent.extra.INSTANT_APP_BUNDLES";

    /// 一堆元数据，描述了需要安装的即时应用程序。此数据是根据注册即时应用程序解析器提供的对Android.content.pm.InstantAppResolveInfo#getExtras()的响应填充的。
    pub const EXTRA_INSTANT_APP_EXTRAS: &'static str = "android.intent.extra.INSTANT_APP_EXTRAS";

    /// 一个布尔值，表示即时应用解析器无法确定它是否有在 EXTRA_INTENT 中定义的已清理 Intent 的应用。
    pub const EXTRA_UNKNOWN_INSTANT_APP: &'static str = "android.intent.extra.UNKNOWN_INSTANT_APP";

    /// 要安装组件的应用程序的版本代码。
    #[deprecated(note = "使用 EXTRA_LONG_VERSION_CODE。")]
    pub const EXTRA_VERSION_CODE: &'static str = "android.intent.extra.VERSION_CODE";

    /// 要安装组件的应用程序的版本代码。
    pub const EXTRA_LONG_VERSION_CODE: &'static str = "android.intent.extra.LONG_VERSION_CODE";

    /// 触发免安装应用的应用程序。
    pub const EXTRA_CALLING_PACKAGE: &'static str = "android.intent.extra.CALLING_PACKAGE";

    /// 可选的调用应用程序提供的捆绑包，其中包含安装程序可能使用的附加启动信息。
    pub const EXTRA_VERIFICATION_BUNDLE: &'static str = "android.intent.extra.VERIFICATION_BUNDLE";

    /**
    一个 Bundle，形成潜在目标软件包名称与不同额外 Bundle 之间的映射，用于在 与 ACTION_CHOOSER 一起使用时添加到 EXTRA_INTENT 中的默认 Intent 额外内容中。每个键都应为软件包名称。该软件包不需要当前安装在设备上。
    当用户从一组预定的目标包中选择活动时，应用可以选择提供替代的额外内容。如果用户从选择器中选择的活动属于一个包，并且其包名称是此包中的键，则该包的相应额外内容将与 EXTRA_INTENT 中已存在于 Intent 中的额外内容合并。如果替换额外内容与 Intent 中已存在的额外内容具有相同的键，它将覆盖 Intent 中的额外内容。
    示例：当通过 ACTION_SEND 与应用程序共享时，应用程序可能会向其提供不同的 EXTRA_TEXT，从而为该目标添加额外的查询参数。应用程序可能会为给定意图的已知目标提供额外的元数据，以传递仅与该目标相关的信息，例如该应用程序已知的账户或内容标识符。
    */
    pub const EXTRA_REPLACEMENT_EXTRAS: &'static str = "android.intent.extra.REPLACEMENT_EXTRAS";

    /**
    如果用户成功选择目标组件来处理 ACTION_CHOOSER 活动中的操作，则 IntentSender 将收到通知。IntentSender 将附加额外的 EXTRA_CHOSEN_COMPONENT，其中包含所选组件的 ComponentName。
    在某些情况下，此回调可能永远不会发生，例如，如果用户放弃选择器、切换到其他任务或任何其他原因。应用程序不应假设此回调将始终发生。
    */
    pub const EXTRA_CHOSEN_COMPONENT_INTENT_SENDER: &'static str =
        "android.intent.extra.CHOSEN_COMPONENT_INTENT_SENDER";

    /// 用户选择的用于完成操作的 ComponentName。
    pub const EXTRA_CHOSEN_COMPONENT: &'static str = "android.intent.extra.CHOSEN_COMPONENT";

    /// android.view.KeyEvent 对象包含触发其所在 Intent 创建的事件。
    pub const EXTRA_KEY_EVENT: &'static str = "android.intent.extra.KEY_EVENT";

    /// 在 ACTION_REQUEST_SHUTDOWN 中设置为 true，以便在关闭前请求用户确认。
    pub const EXTRA_KEY_CONFIRM: &'static str = "android.intent.extra.KEY_CONFIRM";

    /// 在ACTION_REQUEST_SHUTDOWN中设置为true，表示关机是用户请求的。
    pub const EXTRA_USER_REQUESTED_SHUTDOWN: &'static str =
        "android.intent.extra.USER_REQUESTED_SHUTDOWN";

    /// 用作 android.content.Intent#ACTION_PACKAGE_REMOVED 或 android.content.Intent#ACTION_PACKAGE_CHANGED 意图中的布尔额外字段，以覆盖重新启动应用程序的默认操作。
    pub const EXTRA_DONT_KILL_APP: &'static str = "android.intent.extra.DONT_KILL_APP";

    /// 用作 android.content.Intent#ACTION_PACKAGE_REMOVED 意图中的布尔额外字段，以表示应用程序已通过用户发起的操作被删除。
    pub const EXTRA_USER_INITIATED: &'static str = "android.intent.extra.USER_INITIATED";

    /// 包含最初输入的电话号码的字符串。
    pub const EXTRA_PHONE_NUMBER: &'static str = "android.intent.extra.PHONE_NUMBER";

    /// 用作 android.content.Intent#ACTION_UID_REMOVED 意图中的 int 额外字段，以提供已分配包的 uid。也可用作 android.content.Intent#ACTION_PACKAGE_REMOVED 或 android.content.Intent#ACTION_PACKAGE_CHANGED 中的可选额外字段，用于相同目的。
    pub const EXTRA_UID: &'static str = "android.intent.extra.UID";

    /// 包名称的字符串数组。
    pub const EXTRA_PACKAGES: &'static str = "android.intent.extra.PACKAGES";

    /// 用作 android.content.Intent#ACTION_PACKAGE_REMOVED 意图中的布尔额外字段，以指示这是否代表完全卸载（删除代码及其数据）或部分卸载（保留其数据，意味着这是一个更新）。
    pub const EXTRA_DATA_REMOVED: &'static str = "android.intent.extra.DATA_REMOVED";

    /// 用作 android.content.Intent#ACTION_PACKAGE_REMOVED 意图中的布尔额外字段，以指示此时已为设备上的所有用户删除该软件包。
    pub const EXTRA_REMOVED_FOR_ALL_USERS: &'static str =
        "android.intent.extra.REMOVED_FOR_ALL_USERS";

    /// 用作 android.content.Intent#ACTION_PACKAGE_REMOVED 意图中的布尔额外字段，以指示这是软件包的替换，因此此广播将立即跟着针对同一软件包的不同版本的添加广播。
    pub const EXTRA_REPLACING: &'static str = "android.intent.extra.REPLACING";

    /// 用作 android.content.Intent#ACTION_PACKAGE_REMOVED 意图中的布尔额外字段，以指示这是系统更新卸载。
    pub const EXTRA_SYSTEM_UPDATE_UNINSTALL: &'static str =
        "android.intent.extra.SYSTEM_UPDATE_UNINSTALL";

    /**
    用作 android.app.AlarmManager 待处理意图中的 int 额外字段，用于告知被调用的应用程序有多少待处理警报与意图一起传递。对于一次性警报，该值始终为 1。对于重复性警报，如果设备在传递先前警报时处于睡眠状态或关闭状态，该值可能大于 1。
    注意：您必须在设置闹钟时向 ` ` 提供可变的 android.app.PendingIntent，以便在收到闹钟时读取此值。待定意图的可变性必须由针对 Build.VERSION_CODES#S 或更高版本的应用明确指定。
    */
    pub const EXTRA_ALARM_COUNT: &'static str = "android.intent.extra.ALARM_COUNT";

    /// 用作 android.content.Intent#ACTION_DOCK_EVENT 意图中的 int 额外字段，以请求停靠状态。可能的值是 android.content.Intent#EXTRA_DOCK_STATE_UNDOCKED、android.content.Intent#EXTRA_DOCK_STATE_DESK、android.content.Intent#EXTRA_DOCK_STATE_CAR、android.content.Intent#EXTRA_DOCK_STATE_LE_DESK 或 android.content.Intent#EXTRA_DOCK_STATE_HE_DESK。
    pub const EXTRA_DOCK_STATE: &'static str = "android.intent.extra.DOCK_STATE";

    /// 用作 android.content.Intent#EXTRA_DOCK_STATE 的 int 值，表示手机不在任何基座中。
    pub const EXTRA_DOCK_STATE_UNDOCKED: i32 = 0;

    /// 用作 android.content.Intent#EXTRA_DOCK_STATE 的 int 值，表示手机位于桌面基座中。
    pub const EXTRA_DOCK_STATE_DESK: i32 = 1;

    /// 用作 android.content.Intent#EXTRA_DOCK_STATE 的 int 值，表示手机位于车载基座中。
    pub const EXTRA_DOCK_STATE_CAR: i32 = 2;

    /// 用作 android.content.Intent#EXTRA_DOCK_STATE 的 int 值，表示手机处于模拟（低端）基座中。
    pub const EXTRA_DOCK_STATE_LE_DESK: i32 = 3;

    /// 用作 android.content.Intent#EXTRA_DOCK_STATE 的 int 值，表示手机位于数字（高端）基座中。
    pub const EXTRA_DOCK_STATE_HE_DESK: i32 = 4;

    /// 可以作为元数据与 dock 活动一起提供的布尔值，以指示 dock 在活动时应该接管主页键。
    pub const METADATA_DOCK_HOME: &'static str = "android.dock_home";

    /// 用作 ACTION_APP_ERROR 中的可打包额外字段，包含错误报告。
    pub const EXTRA_BUG_REPORT: &'static str = "android.intent.extra.BUG_REPORT";

    /// 用于远程意图中的额外字段。它是随远程意图传递的字符串标记。
    pub const EXTRA_REMOTE_INTENT_TOKEN: &'static str = "android.intent.extra.remote_intent_token";

    #[doc(hidden)]
    #[deprecated(note = "请参阅EXTRA_CHANGED_COMPONENT_NAME_LIST;该字段将仅包含列表中的名字。")]
    pub const EXTRA_CHANGED_COMPONENT_NAME: &'static str =
        "android.intent.extra.changed_component_name";

    /// 此字段是 android.content.Intent#ACTION_PACKAGE_CHANGED 的一部分，包含已更改的所有组件的字符串数组。如果整个软件包的状态已更改，则它将包含一个带有软件包名称本身的条目。
    pub const EXTRA_CHANGED_COMPONENT_NAME_LIST: &'static str =
        "android.intent.extra.changed_component_name_list";

    /// 该字段是 android.content.Intent#ACTION_EXTERNAL_APPLICATIONS_AVAILABLE、android.content.Intent#ACTION_EXTERNAL_APPLICATIONS_UNAVAILABLE、android.content.Intent#ACTION_PACKAGES_SUSPENDED、android.content.Intent#ACTION_PACKAGES_UNSUSPENDED 的一部分，包含所有已更改组件的字符串数组。
    pub const EXTRA_CHANGED_PACKAGE_LIST: &'static str =
        "android.intent.extra.changed_package_list";

    /// 该字段是 android.content.Intent#ACTION_EXTERNAL_APPLICATIONS_AVAILABLE、android.content.Intent#ACTION_EXTERNAL_APPLICATIONS_UNAVAILABLE 的一部分，包含所有已更改组件的 uid 整数数组。
    pub const EXTRA_CHANGED_UID_LIST: &'static str = "android.intent.extra.changed_uid_list";

    /// 一个整数，表示通过 PackageManager#setDistractingPackageRestrictions(String[], int) 对分散包设置的限制的按位组合
    pub const EXTRA_DISTRACTION_RESTRICTIONS: &'static str =
        "android.intent.extra.distraction_restrictions";

    /// 绑定时可以使用的魔法额外系统代码，为绑定到服务的人提供标签。这是一个整数，提供可显示给用户的框架字符串资源。
    pub const EXTRA_CLIENT_LABEL: &'static str = "android.intent.extra.client_label";

    /// 绑定时可以使用神奇的额外系统代码，为用户提供一个可启动的 PendingIntent 对象，以禁用系统对此服务的使用。
    pub const EXTRA_CLIENT_INTENT: &'static str = "android.intent.extra.client_intent";

    /// Extra 用于指示 Intent 应仅返回本地设备上的数据。这是一个布尔值 Extra；默认值为 false。如果为 true，则实现应仅允许用户选择设备上已有的数据，而无需在打开时从远程服务下载数据。
    pub const EXTRA_LOCAL_ONLY: &'static str = "android.intent.extra.LOCAL_ONLY";

    /// Extra 用于指示 Intent 是否允许用户选择并返回多个项目。这是一个布尔值 extra；默认值为 false。如果为 true，则允许实现向用户呈现一个 UI，用户可在其中选择多个项目，这些项目将全部返回给调用者。发生这种情况时，应将它们作为结果 Intent 的 getClipData() 部分返回。
    pub const EXTRA_ALLOW_MULTIPLE: &'static str = "android.intent.extra.ALLOW_MULTIPLE";

    /// 用户ID整数带有与用户和托管配置文件的添加，删除和切换相关的广播意见-ACTION_USER_ADDED，ACTION_USER_REMOVED和ACTION_USER_SWITCHED。
    pub const EXTRA_USER_HANDLE: &'static str = "android.intent.extra.user_handle";

    /// UserHandle 携带了意图。
    pub const EXTRA_USER: &'static str = "android.intent.extra.USER";

    /// 处理 ACTION_GET_RESTRICTION_ENTRIES 的 BroadcastReceiver 的响应中使用的额外内容。额外内容的类型为 ArrayList&lt;RestrictionEntry&gt;。
    pub const EXTRA_RESTRICTIONS_LIST: &'static str = "android.intent.extra.restrictions_list";

    /// Extra 以 Intent 的形式发送给处理 ACTION_GET_RESTRICTION_ENTRIES 的 BroadcastReceiver。Extra 的类型是 Bundle，其中包含以键/值对形式呈现的限制。
    pub const EXTRA_RESTRICTIONS_BUNDLE: &'static str = "android.intent.extra.restrictions_bundle";

    /// 在处理 ACTION_GET_RESTRICTION_ENTRIES 的 BroadcastReceiver 的响应中额外使用。
    pub const EXTRA_RESTRICTIONS_INTENT: &'static str = "android.intent.extra.restrictions_intent";

    /// 用于传达一组可接受的 MIME 类型的额外信息。额外信息的类型为 ` `。值可以是具体的 MIME 类型（例如“image/png”）和/或部分 MIME 类型（例如“audio/\*”）的组合。
    pub const EXTRA_MIME_TYPES: &'static str = "android.intent.extra.MIME_TYPES";

    /// ACTION_SHUTDOWN 的可选附加信息，允许发送者限定此关闭仅适用于系统的用户空间，而不是完全关闭。当此信息为真时，硬件设备可以使用此信息来确定它们不应完全关闭其设备，因为这不是完全关闭到内核，而只是重新启动用户空间。如果不提供，则默认为 false。
    pub const EXTRA_SHUTDOWN_USERSPACE_ONLY: &'static str =
        "android.intent.extra.SHUTDOWN_USERSPACE_ONLY";

    /// 可选附加项，指定自纪元以来的毫秒数。该值必须是非负数。类型：长整型
    pub const EXTRA_TIME: &'static str = "android.intent.extra.TIME";

    /**
    通过 ACTION_TIMEZONE_CHANGED 发送的额外信息指定了设备的新时区。
    类型：字符串，与 TimeZone#getID() 返回的相同，用于标识时区。
    */
    pub const EXTRA_TIMEZONE: &'static str = "time-zone";

    /// ACTION_TIME_CHANGED 的可选 int extra 表示用户已设置其时间格式偏好。请参阅 EXTRA_TIME_PREF_VALUE_USE_12_HOUR、EXTRA_TIME_PREF_VALUE_USE_24_HOUR 和 EXTRA_TIME_PREF_VALUE_USE_LOCALE_DEFAULT。该值不得为负数。
    /// * 仅供内部使用。
    pub const EXTRA_TIME_PREF_24_HOUR_FORMAT: &'static str =
        "android.intent.extra.TIME_PREF_24_HOUR_FORMAT";

    #[doc(hidden)]
    pub const EXTRA_TIME_PREF_VALUE_USE_12_HOUR: i32 = 0;

    #[doc(hidden)]
    pub const EXTRA_TIME_PREF_VALUE_USE_24_HOUR: i32 = 1;

    #[doc(hidden)]
    pub const EXTRA_TIME_PREF_VALUE_USE_LOCALE_DEFAULT: i32 = 2;

    /**
    意图额外：执行与此意图相关的操作的原因。
    类型：字符串
    */
    pub const EXTRA_REASON: &'static str = "android.intent.extra.REASON";

    /// 此 extra 将与 ACTION_FACTORY_RESET 一起发送
    pub const EXTRA_WIPE_EXTERNAL_STORAGE: &'static str =
        "android.intent.extra.WIPE_EXTERNAL_STORAGE";

    //noinspection SpellCheckingInspection
    /// 当用户选择在带有 eSIM 的设备恢复出厂设置期间擦除 eSIM 上的数据时，此 extra 将设置为 true。此 extra 将与 ACTION_FACTORY_RESET 一起发送
    pub const EXTRA_WIPE_ESIMS: &'static str = "com.android.internal.intent.extra.WIPE_ESIMS";

    /// 可选的 android.app.PendingIntent extra 用于传递 SIM 激活请求的结果。TODO：添加有关待处理意图所使用的结构和响应数据的信息。
    pub const EXTRA_SIM_ACTIVATION_RESPONSE: &'static str =
        "android.intent.extra.SIM_ACTIVATION_RESPONSE";

    /**
    可选索引具有语义的可选索引，具体取决于意图动作。
    该值必须是大于或等于0的整数。
    */
    pub const EXTRA_INDEX: &'static str = "android.intent.extra.INDEX";

    /**
    告诉快速查看器显示适合传递的 Uris 的其他 UI 操作，例如在其他应用程序中打开、共享、打开、编辑、打印、删除、投射等。
    该值为布尔值。默认为 false。
    */
    #[deprecated]
    pub const EXTRA_QUICK_VIEW_ADVANCED: &'static str = "android.intent.extra.QUICK_VIEW_ADVANCED";

    /// 一个可选的额外功能 ` `，指示在传递 Intent#ACTION_QUICK_VIEW 意图时，应在快速查看 UI 中向用户提供哪些快速查看功能。此处列举的功能并不意味着限制快速查看器的功能。快速查看器可以实现下面未列出的功能。
    /// 此时包含的功能有：QuickViewConstants#FEATURE_VIEW、QuickViewConstants#FEATURE_EDIT、QuickViewConstants#FEATURE_DELETE、QuickViewConstants#FEATURE_DOWNLOAD、QuickViewConstants#FEATURE_SEND、QuickViewConstants#FEATURE_PRINT。要求：如果 EXTRA_QUICK_VIEW_FEATURES 中不存在该功能，则快速查看器不应显示该功能。当 EXTRA_QUICK_VIEW_FEATURES 不存在时，快速查看器应遵循内部政策。EXTRA_QUICK_VIEW_FEATURES 中存在某项功能并不构成必须显示该功能的要求。快速查看器可能会根据其自己的政策禁用或隐藏功能。
    pub const EXTRA_QUICK_VIEW_FEATURES: &'static str = "android.intent.extra.QUICK_VIEW_FEATURES";

    /// 可选布尔额外值，表示静音模式是打开还是关闭。当配置文件进入静音模式时，配置文件中的所有应用都将被终止，配置文件用户也将停止使用。来自配置文件的小部件将被屏蔽，应用启动器图标将变灰。
    pub const EXTRA_QUIET_MODE: &'static str = "android.intent.extra.QUIET_MODE";

    /**
    可选的CharSequence额外提供搜索查询。此查询的格式取决于接收应用程序。
    适用于具有以下操作的 Intent：Intent#ACTION_GET_CONTENT Intent#ACTION_OPEN_DOCUMENT
    */
    pub const EXTRA_CONTENT_QUERY: &'static str = "android.intent.extra.CONTENT_QUERY";

    /// 用作 ACTION_MEDIA_RESOURCE_GRANTED 意图中的 int 额外字段，以指定授予的资源类型。可能的值是 EXTRA_MEDIA_RESOURCE_TYPE_VIDEO_CODEC 或 EXTRA_MEDIA_RESOURCE_TYPE_AUDIO_CODEC。
    pub const EXTRA_MEDIA_RESOURCE_TYPE: &'static str = "android.intent.extra.MEDIA_RESOURCE_TYPE";

    /// 在Action_Chooser中用作布尔值额外字段，目的是指定是否只有一个应用程序可供选择时显示chooser。
    pub const EXTRA_AUTO_LAUNCH_SINGLE_CHOICE: &'static str =
        "android.intent.extra.AUTO_LAUNCH_SINGLE_CHOICE";

    /// 用作 EXTRA_MEDIA_RESOURCE_TYPE 的 int 值，表示允许使用视频编解码器。
    pub const EXTRA_MEDIA_RESOURCE_TYPE_VIDEO_CODEC: i32 = 0;

    /// 用作 EXTRA_MEDIA_RESOURCE_TYPE 的 int 值，表示允许使用音频编解码器。
    pub const EXTRA_MEDIA_RESOURCE_TYPE_AUDIO_CODEC: i32 = 1;

    /**
    意图额外：ACTION_VIEW_LOCUS 上使用的上下文的 ID。
    类型：LocusId
    */
    pub const EXTRA_LOCUS_ID: &'static str = "android.intent.extra.LOCUS_ID";

    /// 用作 android.content.Intent#ACTION_PACKAGE_REMOVED_INTERNAL 意图中的 int 数组额外字段，以指示此已删除包的可见性允许列表。
    pub const EXTRA_VISIBILITY_ALLOW_LIST: &'static str =
        "android.intent.extra.VISIBILITY_ALLOW_LIST";

    /// 如果设置，则此 Intent 的接收者将被授予对 Intent 数据中的 URI 以及其 ClipData 中指定的任何 URI 执行读取操作的权限。当应用于 Intent 的 ClipData 时，将授予所有 URI 以及对 Intent 项目中的数据或其他 ClipData 的递归遍历；仅使用顶级 Intent 的授权标志。
    pub const FLAG_GRANT_READ_URI_PERMISSION: u32 = 0x00000001;

    /// 如果设置，则此 Intent 的接收者将被授予对 Intent 数据中的 URI 以及其 ClipData 中指定的任何 URI 执行写入操作的权限。当应用于 Intent 的 ClipData 时，将授予所有 URI 以及对 Intent 项目中的数据或其他 ClipData 的递归遍历；仅使用顶级 Intent 的授予标志。
    pub const FLAG_GRANT_WRITE_URI_PERMISSION: u32 = 0x00000002;

    /// 可以由调用者设置，以表明此 Intent 来自后台操作，而不是来自直接用户交互。
    pub const FLAG_FROM_BACKGROUND: u32 = 0x00000004;

    /// 您可以启用一个标志来进行调试：设置后，在解决此意图期间将打印日志消息，以向您显示已找到的内容以创建最终地解析列表。
    pub const FLAG_DEBUG_LOG_RESOLUTION: u32 = 0x00000008;

    /// 如果设置，此意图将不会匹配当前已停止的软件包中的任何组件。如果未设置，则默认行为是将此类应用程序包含在结果中。
    pub const FLAG_EXCLUDE_STOPPED_PACKAGES: u32 = 0x00000010;

    /// 如果设置了该 Intent，则该 Intent 将始终匹配当前已停止的软件包中的任何组件。这是未设置 FLAG_EXCLUDE_STOPPED_PACKAGES 时的默认行为。如果同时设置了这两个标志，则该标志优先（它允许在框架可能自动设置排除标志的地方覆盖排除）。
    pub const FLAG_INCLUDE_STOPPED_PACKAGES: u32 = 0x00000020;

    //noinspection SpellCheckingInspection
    /// 与 FLAG_GRANT_READ_URI_PERMISSION 和/或 FLAG_GRANT_WRITE_URI_PERMISSION 结合使用时，URI 权限授予可在设备重启后继续保留，直到使用 Context#revokeUriPermission(Uri, int) 明确撤销。此标志仅提供可能保留的授予；接收应用程序必须调用 ContentResolver#takePersistableUriPermission(Uri, int) 才能真正保留。
    pub const FLAG_GRANT_PERSISTABLE_URI_PERMISSION: u32 = 0x00000040;

    /// 当与 FLAG_GRANT_READ_URI_PERMISSION 和/或 FLAG_GRANT_WRITE_URI_PERMISSION 结合使用时，URI 权限授予适用于任何与原始授予 URI 前缀匹配的 URI。（如果没有此标记，URI 必须完全匹配才能授予访问权限。）仅当方案、权限和前缀定义的所有路径段完全匹配时，另一个 URI 才被视为前缀匹配。
    pub const FLAG_GRANT_PREFIX_URI_PERMISSION: u32 = 0x00000080;

    /// 用于根据直接启动感知和当前用户状态自动匹配意图的标志。由于默认行为是自动应用当前用户状态，因此这实际上是一个哨兵值，不会根据其存在与否更改任何查询的输出。相反，此值可以与 android.os.StrictMode.VmPolicy.Builder#detectImplicitDirectBoot() 结合使用，以检测调用者何时依赖隐式自动匹配，而不是确认他们想要的显式行为。
    pub const FLAG_DIRECT_BOOT_AUTO: u32 = 0x00000100;

    #[doc(hidden)]
    #[deprecated]
    pub const FLAG_DEBUG_TRIAGED_MISSING: u32 = Self::FLAG_DIRECT_BOOT_AUTO;

    /// 在解析意图时，不应考虑用于指示短暂应用程序的内部标志。
    pub const FLAG_IGNORE_EPHEMERAL: u32 = 0x80000000;

    //noinspection SpellCheckingInspection
    /**
    如果设置，新活动将不会保留在历史堆栈中。一旦用户离开，活动就会结束。也可以使用 android.R.styleable#AndroidManifestActivity_noHistory noHistory 属性进行设置。
    如果设置，则当当前活动启动一个设置结果并完成的新活动时，永远不会调用 android.app.Activity#onActivityResult onActivityResult()。
    */
    pub const FLAG_ACTIVITY_NO_HISTORY: u32 = 0x40000000;

    /// 如果设置，则如果活动已在历史堆栈顶部运行，则不会启动该活动。有关更多信息，请参阅任务和返回堆栈。
    pub const FLAG_ACTIVITY_SINGLE_TOP: u32 = 0x20000000;

    /**
    如果设置，此活动将成为此历史堆栈上新任务的开始。任务（从启动它的活动到下一个任务活动）定义了用户可以移动到的原子活动组。任务可以移动到前台和后台；特定任务内的所有活动始终保持相同的顺序。有关任务的更多信息，请参阅任务和返回堆栈。
    此标志通常由想要呈现“启动器”样式行为的活动使用：它们为用户提供可以执行的单独操作的列表，这些操作否则将完全独立于启动它们的活动运行。
    使用此标志时，如果一个任务已经为您现在启动的活动运行，则不会启动新的活动。取而代之的是，当前的任务将简单地带到屏幕的前面。
    当调用者正在请求正在启动的活动的结果时不能使用此标志。
    */
    pub const FLAG_ACTIVITY_NEW_TASK: u32 = 0x10000000;

    /**
    此标志用于创建新任务并在其中启动活动。此标志始终与 FLAG_ACTIVITY_NEW_DOCUMENT 或 FLAG_ACTIVITY_NEW_TASK 配对使用。在这两种情况下，仅使用这些标志即可在现有任务中搜索与此 Intent 匹配的任务。只有未找到此类任务时才会创建新任务。当与 FLAG_ACTIVITY_MULTIPLE_TASK 配对使用时，这两种行为都会被修改为跳过搜索匹配任务并无条件启动新任务。
    与 FLAG_ACTIVITY_NEW_TASK 一起使用时，除非您正在实现自己的顶级应用程序启动器，否则请勿使用此标志。与 FLAG_ACTIVITY_NEW_TASK 一起使用可禁用将现有任务带到前台的行为。设置后，无论是否已有现有任务正在运行相同的任务，都会启动一个新任务来托管 Intent 的 Activity。
    由于默认系统不包含图形任务管理，因此您不应使用此标志，除非您为用户提供某种方式返回到您已启动的任务。
    有关此标志用于创建新文档任务的详细信息，请参阅 FLAG_ACTIVITY_NEW_DOCUMENT。
    如果 FLAG_ACTIVITY_NEW_TASK 或 FLAG_ACTIVITY_NEW_DOCUMENT 之一未设置，则忽略此标志。
    有关任务的更多信息，请参阅任务和返回堆栈。
    */
    pub const FLAG_ACTIVITY_MULTIPLE_TASK: u32 = 0x08000000;

    /**
    如果设置了，并且正在启动的活动已经在当前任务中运行，那么将不会启动该活动的新实例，而是关闭其上的所有其他活动，并且此 Intent 将作为新 Intent 传递给（现在位于顶部的）旧活动。
    例如，考虑一个由活动 A、B、C、D 组成的任务。如果 D 调用 startActivity() 并把 Intent 解析为活动 B 的组件，那么 C 和 D 将完成，并且 B 会收到给定的 Intent，导致堆栈现在为：A、B。
    上例中，当前正在运行的 Activity B 实例要么在其 onNewIntent() 方法中接收您在此处启动的新 Intent，要么自行完成并重新启动新 Intent。如果它已将其启动模式声明为“多个”（默认），并且您未在同一 Intent 中设置 FLAG_ACTIVITY_SINGLE_TOP，则它将完成并重新创建；对于所有其他启动模式，或者如果设置了 FLAG_ACTIVITY_SINGLE_TOP，则此 Intent 将传递到当前实例的 onNewIntent()。
    此启动模式还可与 FLAG_ACTIVITY_NEW_TASK 配合使用，效果良好：如果用于启动任务的根 Activity，它会将该任务的任何当前正在运行的实例带到前台，然后将其清除为其根状态。这尤其有用，例如，当从通知管理器启动 Activity 时。
    有关任务的更多信息，请参阅任务和返回堆栈。
    */
    pub const FLAG_ACTIVITY_CLEAR_TOP: u32 = 0x04000000;

    /// 如果已设置，并且此意图用于从现有活动启动新活动，则现有活动的回复目标将转移到新活动。这样，新活动可以调用 android.app.Activity#setResult 并将该结果发送回原始活动的回复目标。
    pub const FLAG_ACTIVITY_FORWARD_RESULT: u32 = 0x02000000;

    /// 如果已设置，并且此意图用于从现有活动启动新活动，则当前活动将不被视为顶部活动，用于决定是否应将新意图传递到顶部活动而不是启动新活动。前一个活动将用作顶部活动，假设当前活动将立即完成。
    pub const FLAG_ACTIVITY_PREVIOUS_IS_TOP: u32 = 0x01000000;

    //noinspection SpellCheckingInspection
    /// 如果设置，新活动将不会保存在最近启动的活动列表中。
    pub const FLAG_ACTIVITY_EXCLUDE_FROM_RECENTS: u32 = 0x00800000;

    //noinspection SpellCheckingInspection
    /// 此标志通常不是按应用程序代码设置的，而是按系统为您设置的，如android.R.styleable#AndroidManifestActivity_launchMode启动模式文档。
    pub const FLAG_ACTIVITY_BROUGHT_TO_FRONT: u32 = 0x00400000;

    /// 如果已设置，并且此活动要么在新任务中启动，要么置于现有任务的顶部，则它将作为任务的前门启动。这将导致应用使该任务处于正确状态所需的任何亲和性（将活动移入或移出），或者在需要时简单地将该任务重置为其初始状态。
    pub const FLAG_ACTIVITY_RESET_TASK_IF_NEEDED: u32 = 0x00200000;

    /// 此标志通常不由应用程序代码设置，但如果此活动是从历史记录中启动的，则由系统为您设置。
    pub const FLAG_ACTIVITY_LAUNCHED_FROM_HISTORY: u32 = 0x00100000;

    #[doc(hidden)]
    #[deprecated(
        note = "从 API 21 开始，它的执行方式与 FLAG_ACTIVITY_NEW_DOCUMENT 相同，应使用 FLAG_ACTIVITY_NEW_DOCUMENT 来代替它。"
    )]
    pub const FLAG_ACTIVITY_CLEAR_WHEN_TASK_RESET: u32 = 0x00080000;

    //noinspection SpellCheckingInspection
    /**
    此标志用于将文档打开到以此 Intent 启动的活动为基础的新任务中。通过使用此标志或其等效属性，android.R.attr#documentLaunchMode 包含不同文档的同一活动的多个实例将出现在最近任务列表中。
    与此处描述的 Intent 标志相比，最好使用活动属性形式 android.R.attr#documentLaunchMode。属性形式允许 Activity 为 Activity 的所有启动器指定多个文档行为，而使用此标志则要求启动 Activity 的每个 Intent 都指定它。
    请注意，此标志的默认语义（即活动结束后是否保留其最近条目）与 FLAG_ACTIVITY_NEW_TASK 和 android.R.attr#documentLaunchMode 的使用不同 - 如果使用此标志创建新的最近条目，则默认情况下，活动结束后将删除该条目。您可以使用 FLAG_ACTIVITY_RETAIN_IN_RECENTS 修改此行为。
    FLAG_ACTIVITY_NEW_DOCUMENT 可与 FLAG_ACTIVITY_MULTIPLE_TASK 结合使用。单独使用时，它相当于 Activity 清单指定 android.R.attr#documentLaunchMode="intoExisting"。与 FLAG_ACTIVITY_MULTIPLE_TASK 结合使用时，它相当于 Activity 清单指定 android.R.attr#documentLaunchMode="always"。当 Activity 清单指定 android.R.attr#documentLaunchMode="never" 时，即使与 FLAG_ACTIVITY_MULTIPLE_TASK 结合使用，也会忽略该标志。
    有关更多信息，请参阅 android.R.attr#documentLaunchMode。
    */
    #[allow(deprecated)]
    pub const FLAG_ACTIVITY_NEW_DOCUMENT: u32 = Self::FLAG_ACTIVITY_CLEAR_WHEN_TASK_RESET;

    /**
    如果设置了此标志，则在新启动的活动被置于最前面时，将阻止正常的 android.app.Activity#onUserLeaveHint 回调在当前最前面的活动暂停之前发生在该活动上。
    通常，Activity 可以依赖该回调来指示明确的用户操作已导致其 Activity 移出前台。回调标记了 Activity 生命周期中的适当点，以便其关闭打算显示的任何通知“直到用户看到它们”，例如闪烁的 LED。
    如果活动是通过任何非用户驱动的事件（例如电话接听或警报处理程序）启动的，则应将此标志传递给 Context#startActivity Context.startActivity，确保暂停活动不会认为用户已确认其通知。
    */
    pub const FLAG_ACTIVITY_NO_USER_ACTION: u32 = 0x00040000;

    /**
    如果在传递给 Context#startActivity Context.startActivity() 的 Intent 中设置，此标志将导致启动的活动被带到其任务历史堆栈的前面（如果它已经在运行）。
    例如，考虑一个由四个活动组成的任务：A、B、C、D。如果 D 使用解析为活动 B 的组件的 Intent 调用 startActivity()，则 B 将被带到历史堆栈的前面，结果顺序为：A、C、D、B。
    如果还指定了 FLAG_ACTIVITY_CLEAR_TOP，则此标志将被忽略。
    */
    pub const FLAG_ACTIVITY_REORDER_TO_FRONT: u32 = 0x00020000;

    /// 如果在传递给 Context#startActivity Context.startActivity() 的 Intent 中设置了此标志，则系统不会应用活动 过渡动画 来进入下一个活动状态。这并不意味着动画永远不会运行 - 如果在显示此处启动的活动之前发生了另一个未指定此标志的活动更改，则将使用该过渡。当您要执行一系列活动操作但用户看到的动画不应由第一个活动更改驱动而应由后续活动更改驱动时，可以很好地使用此标志。
    pub const FLAG_ACTIVITY_NO_ANIMATION: u32 = 0x00010000;

    /// 如果在传递给 Context#startActivity Context.startActivity() 的 Intent 中设置此标志，则此标志将导致在启动活动之前清除与活动相关联的任何现有任务。也就是说，活动将成为原本为空的任务的新根，并且所有旧活动都将完成。这只能与 FLAG_ACTIVITY_NEW_TASK 结合使用。
    pub const FLAG_ACTIVITY_CLEAR_TASK: u32 = 0x00008000;

    /// 如果在传递给 Context#startActivity Context.startActivity() 的 Intent 中设置此标志，则会导致将新启动的任务置于当前主页活动任务（如果有）之上。也就是说，从任务中按“返回”按钮将始终让用户返回主页，即使这不是他们看到的最后一个活动。这只能与 FLAG_ACTIVITY_NEW_TASK 结合使用。
    pub const FLAG_ACTIVITY_TASK_ON_HOME: u32 = 0x00004000;

    //noinspection SpellCheckingInspection
    /// 默认情况下，当用户关闭由 FLAG_ACTIVITY_NEW_DOCUMENT 创建的文档时（使用 back 或其他任何方法 finish()），其在最近任务中的条目将被删除。如果您希望允许文档保留在最近任务中以便重新启动，则可以使用此标志。设置后，任务的活动已完成，最近任务条目将保留在界面中，以便用户重新启动它，就像顶级应用程序的最近任务条目一样。接收活动可以使用 android.R.attr#autoRemoveFromRecents 或通过明确调用 android.app.Activity#finishAndRemoveTask() Activity.finishAndRemoveTask() 来覆盖此请求。
    pub const FLAG_ACTIVITY_RETAIN_IN_RECENTS: u32 = 0x00002000;

    /// 此标志仅用于分屏多窗口模式。新活动将显示在启动它的活动旁边。这只能与 FLAG_ACTIVITY_NEW_TASK 结合使用。此外，如果您想要创建现有活动的新实例，则需要设置 FLAG_ACTIVITY_MULTIPLE_TASK。
    pub const FLAG_ACTIVITY_LAUNCH_ADJACENT: u32 = 0x00001000;

    /// 如果在传递给 Context#startActivity Context.startActivity() 的 Intent 中设置了此标志，则如果设备上没有完整的应用可以处理此意图，则此标志将尝试启动免安装应用。尝试从外部解析免安装应用时，支持以下 Intent 属性：Intent#setAction(String) Intent#addCategory(String) Intent#setData(Uri) Intent#setType(String) Intent#setPackage(String) Intent#addFlags(int) 如果找不到免安装应用，则将启动安装程序以通知用户无法解析该意图。在不支持免安装应用的设备上，将忽略此标志。
    pub const FLAG_ACTIVITY_MATCH_EXTERNAL: u32 = 0x00000800;

    /// 如果在传递给 Context#startActivity Context.startActivity() 的 Intent 中设置此标志，则仅当该 Intent 解析为非浏览器结果时，才会启动该 Intent。如果不存在这样的结果，则会抛出 ActivityNotFoundException。
    pub const FLAG_ACTIVITY_REQUIRE_NON_BROWSER: u32 = 0x00000400;

    /// 如果在传递给 Context#startActivity Context.startActivity() 的 Intent 中设置此标志，则仅当该 Intent 解析为单个结果时才会启动该 Intent。如果不存在这样的结果或系统选择器会以其他方式显示，则会抛出 ActivityNotFoundException。
    pub const FLAG_ACTIVITY_REQUIRE_DEFAULT: u32 = 0x00000200;

    /// 如果设置，则发送广播时只会调用已注册的接收器——不会启动任何 BroadcastReceiver 组件。
    pub const FLAG_RECEIVER_REGISTERED_ONLY: u32 = 0x40000000;

    /**
    如果设置，则在发送广播时，新广播将替换与其匹配的任何现有待处理广播。匹配由 Intent#filterEquals(Intent) 定义，Intent.filterEquals 为两个广播的意图返回 true。找到匹配项后，新广播（及其关联的接收器）将替换待处理广播列表中的现有广播，并保持在列表中的相同位置。
    此标志通常用于粘性广播，粘性广播仅关心将广播的最新值传递给接收者。
    */
    pub const FLAG_RECEIVER_REPLACE_PENDING: u32 = 0x20000000;

    /// 如果设置，则在发送广播时，允许接收者以前台优先级运行，超时间隔更短。在正常广播期间，接收者不会自动脱离后台优先级类别。
    pub const FLAG_RECEIVER_FOREGROUND: u32 = 0x10000000;

    /// 如果设置，则在发送广播时，接收者将在卸载队列上运行。
    pub const FLAG_RECEIVER_OFFLOAD: u32 = 0x80000000;

    /// 如果设置，当发送广播时，接收者将在系统专用队列上运行。
    pub const FLAG_RECEIVER_OFFLOAD_FOREGROUND: u32 = 0x00000800;

    /// 如果这是有序广播，则不允许接收者中止广播。它们仍然可以将结果传播给后面的接收者，但不能阻止后面的接收者看到广播。
    pub const FLAG_RECEIVER_NO_ABORT: u32 = 0x08000000;

    /**
    如果设置，则在系统完全启动之前发送广播时（甚至在发送 ACTION_LOCKED_BOOT_COMPLETED 之前）只会调用已注册的接收器 - 不会启动任何 BroadcastReceiver 组件。即使没有接收器被调用，粘性意图状态也会被正确记录。如果在广播意图中指定了 FLAG_RECEIVER_REGISTERED_ONLY，则此标志是不必要的。
    此标志仅供系统服务（甚至主线模块的服务）使用，以避免必须在检测启动完成方面实现更复杂的机制。
    这对系统服务器主线模块很有用
    */
    pub const FLAG_RECEIVER_REGISTERED_ONLY_BEFORE_BOOT: u32 = 0x04000000;

    /// 当此广播用于启动升级时设置，这是一种特殊模式，允许在系统准备就绪之前发送广播并启动没有运行任何提供程序的应用程序进程。
    pub const FLAG_RECEIVER_BOOT_UPGRADE: u32 = 0x02000000;

    //noinspection SpellCheckingInspection
    /**
    如果设置，广播将始终发送到后台（缓存或未运行）应用中的清单接收器，无论默认情况下是否这样做。默认情况下，它们仅在广播指定了显式组件或软件包名称时才会接收广播。
    注意：dumpstate 以数字方式使用此标志，因此当其值改变时，那里的广播代码也必须改变以匹配。
    */
    pub const FLAG_RECEIVER_INCLUDE_BACKGROUND: u32 = 0x01000000;

    /// 如果设置，则无论是否默认执行此操作，广播都不会发送到后台（缓存或未运行）应用中的清单接收器。默认情况下，如果广播指定了明确的组件或软件包名称，它们将接收广播。
    pub const FLAG_RECEIVER_EXCLUDE_BACKGROUND: u32 = 0x00800000;

    /// 如果设置，则此广播将从 shell 发送。
    pub const FLAG_RECEIVER_FROM_SHELL: u32 = 0x00400000;

    /**
    如果设置，广播将对 Instant Apps 中的接收者可见。默认情况下，Instant Apps 不会接收广播。
    当 Instant App 使用时，此标志无效。
    */
    pub const FLAG_RECEIVER_VISIBLE_TO_INSTANT_APPS: u32 = 0x00200000;

    /**
    无法使用 PendingIntent 更改的标志。
    */
    pub const IMMUTABLE_FLAGS: u32 = Self::FLAG_GRANT_READ_URI_PERMISSION
        | Self::FLAG_GRANT_WRITE_URI_PERMISSION
        | Self::FLAG_GRANT_PERSISTABLE_URI_PERMISSION
        | Self::FLAG_GRANT_PREFIX_URI_PERMISSION;

    /// 本地标志表明该实例是由系统创建的。
    pub const LOCAL_FLAG_FROM_SYSTEM: u32 = 1 << 5;

    /// 与 toUri 和 parseUri 一起使用的标志：URI 字符串始终具有“intent:”方案。当您希望稍后在用于描述 Intent 的 URI 与所有其他应被视为原始 URI 的 URI 之间进行区分时，可以使用此语法。与 parseUri 一起使用时，任何其他方案都会导致该原始 URI 的通用 VIEW 操作。
    pub const URI_INTENT_SCHEME: u32 = 1 << 0;

    /**
    与 toUri 和 parseUri 一起使用的标志：URI 字符串始终具有“android-app:”方案。这是 URI_INTENT_SCHEME 的变体，其格式对于将 http/https URI 传递到特定软件包名称的情况更简单。格式为：
    android-app://{package_id}[/{scheme}[/{host}[/{path}]]][#Intent;{...}]
    在此方案中，仅需要package_id。如果您包括主机，则还必须包括一个方案；包括路径还需要主机和方案。最后的#intent;片段无需方案，主机或路径即可使用。请注意，这不能与具有SetSelector的意图一起使用，因为基本意图始终具有明确的软件包名称。 该方案如何映射到意图对象的一些示例：
    URI | Intent
    android-app://com.example.app |
    Action:
    ACTION_MAIN
    Package:
    com.example.app
    android-app://com.example.app/http/example.com |
    Action:
    ACTION_VIEW
    Data:
    <http://example.com/>
    Package:
    com.example.app
    android-app://com.example.app/http/example.com/foo?1234 |
    Action:
    ACTION_VIEW
    Data:
    <http://example.com/foo?1234>
    Package:
    com.example.app
    android-app://com.example.app/#Intent;action=com.example.MY_ACTION;end |
    Action:
    com.example.MY_ACTION
    Package:
    com.example.app
    android-app://com.example.app/http/example.com/foo?1234#Intent;action=com.example.MY_ACTION;end |
    Action:
    com.example.MY_ACTION
    Data:
    <http://example.com/foo?1234>
    Package:
    com.example.app
    android-app://com.example.app/#Intent;action=com.example.MY_ACTION;i.some_int=100;S.some_str=hello;end |
    Action:
    com.example.MY_ACTION
    Package:
    com.example.app
    Extras:
    some_int=(int) 100 some_str=(String) hello
    */
    pub const URI_ANDROID_APP_SCHEME: u32 = 1 << 1;

    //noinspection SpellCheckingInspection
    /**
    与 toUri 和 parseUri 一起使用的标志：允许解析不安全的信息。特别是，不能设置标志 FLAG_GRANT_READ_URI_PERMISSION、FLAG_GRANT_WRITE_URI_PERMISSION、FLAG_GRANT_PERSISTABLE_URI_PERMISSION 和 FLAG_GRANT_PREFIX_URI_PERMISSION，以便生成的 Intent 不会导致意外的数据访问。
    如果您不信任正在解析的 URI 的来源，您仍应进行进一步处理以保护自己免受其害。特别是，当使用它来启动活动时，您通常应该添加 CATEGORY_BROWSABLE 来限制可以处理它的活动。
    */
    pub const URI_ALLOW_UNSAFE: u32 = 1 << 2;

    /**
    创建一个空的意图。
    */
    #[java_constructor]
    pub fn new() -> Self {}

    /**
    创建具有给定操作的 Intent。所有其他字段（数据、类型、类）均为空。请注意，操作必须位于命名空间中，因为 Intent 在系统中全局使用 - 例如系统 VIEW 操作是 android.intent.action.VIEW；应用程序的自定义操作将类似于 com.google.app.myapp.CUSTOM_ACTION。
    `action` Intent 操作，例如 ACTION_VIEW。
    */
    #[java_constructor]
    pub fn from_action(action: String) -> Self {}

    /**
    向意图添加新类别。类别提供有关意图执行的操作的更多详细信息。解析意图时，仅使用提供所有请求类别的活动。
    返回：返回相同的意图对象，用于将多个调用链接到单个语句中。
    `category` 所需类别。这可以是预定义的意图类别之一，也可以是您自己的命名空间中的自定义类别。
    */
    #[java_method]
    pub fn add_category(&self, category: String) -> Self {}

    /**
    从意图中删除类别。
    `category` 要删除的类别。
    */
    #[java_method]
    pub fn remove_category(&self, category: String) {}

    /**
    检查意图中是否存在类别。
    返回：布尔值 如果意图包含类别，则返回 True，否则返回 false。
    `category` 要检查的类别。
    */
    #[java_method]
    pub fn has_category(&self, category: String) -> bool {}

    /**
    查询要执行的一般操作，例如 ACTION_VIEW。操作描述了意图中其余信息的一般解释方式——最重要的是，如何处理 getData 返回的数据。
    返回：此意图的操作，如果未指定，则返回 null。
    */
    #[java_method]
    pub fn get_action(&self) -> String {}

    /**
    设置要执行的一般操作。
    返回：返回相同的 Intent 对象，用于将多个调用链接到单个语句中。
    `action` 操作名称，例如 ACTION_VIEW。特定于应用程序的操作应以供应商的软件包名称作为前缀。
    */
    #[java_method]
    pub fn set_action(&self, action: String) -> Self {}

    /**
    向意图添加其他标志（或使用现有标志值）。
    返回：返回相同的意图对象，用于将多个调用链接到单个语句中。
    `flags` 要设置的新标志。
    */
    #[java_method]
    pub fn add_flags(&self, flags: u32) -> Self {}

    /**
    查询与此意图相关的任何特殊标志。通常只需使用 setFlags 设置它们，然后让系统对它们采取适当的操作。
    返回：当前设置的标志。
    */
    #[java_method]
    pub fn get_flags(&self) -> u32 {}

    /**
    从意图中删除这些标志。
    `flags` 要删除的标志。
    */
    #[java_method]
    pub fn remove_flags(&self, flags: u32) {}

    /**
    与 getData() 相同，但以编码字符串的形式返回 URI。
    */
    #[java_method]
    pub fn get_data_string(&self) -> Option<String> {}

    /**
    向 Intent 添加扩展数据。名称必须包含包前缀，例如，应用程序 com.android.contacts 将使用“com.android.contacts.ShowAll”之类的名称。
    返回:返回相同的 Intent 对象，用于将多个调用链接到单个语句中。
    `name` 额外数据的名称，带有包前缀。
    `value` 可序列化的数据值。
    */
    #[java_method]
    pub fn put_extra<S: Serializable>(&self, name: String, value: Option<S>) -> Self {}

    /**
    向 Intent 添加扩展数据。名称必须包含包前缀，例如，应用程序 com.android.contacts 将使用“com.android.contacts.ShowAll”之类的名称。
    返回:返回相同的 Intent 对象，用于将多个调用链接到单个语句中。
    `name` 额外数据的名称，带有包前缀。
    `value` CharSequence 数据值。
    */
    #[java_method(overload = putExtra)]
    pub fn put_extra_char_sequence<CS: CharSequence>(
        &self,
        name: String,
        value: Option<CS>,
    ) -> Self {
    }

    /**
    向 Intent 添加扩展数据。名称必须包含包前缀，例如，应用程序 com.android.contacts 将使用“com.android.contacts.ShowAll”之类的名称。
    返回:返回相同的 Intent 对象，用于将多个调用链接到单个语句中。
    `name` 额外数据的名称，带有包前缀。
    `value` 字符串数据值。
    */
    #[java_method(overload = putExtra)]
    pub fn put_extra_string(&self, name: String, value: Option<String>) -> Self {}

    /**
    向 Intent 添加扩展数据。名称必须包含包前缀，例如，应用程序 com.android.contacts 将使用“com.android.contacts.ShowAll”之类的名称。
    返回:返回相同的 Intent 对象，用于将多个调用链接到单个语句中。
    `name` 额外数据的名称，带有包前缀。
    `value` 双精度数据值。
    */
    #[java_method(overload = putExtra)]
    pub fn put_extra_double(&self, name: String, value: f64) -> Self {}

    /**
    向 Intent 添加扩展数据。名称必须包含包前缀，例如，应用程序 com.android.contacts 将使用“com.android.contacts.ShowAll”之类的名称。
    返回:返回相同的 Intent 对象，用于将多个调用链接到单个语句中。
    `name` 额外数据的名称，带有包前缀。
    `value` 浮点数据值。
    */
    #[java_method(overload = putExtra)]
    pub fn put_extra_float(&self, name: String, value: f32) -> Self {}

    /**
    向 Intent 添加扩展数据。名称必须包含包前缀，例如，应用程序 com.android.contacts 将使用“com.android.contacts.ShowAll”之类的名称。
    返回:返回相同的 Intent 对象，用于将多个调用链接到单个语句中。
    `name` 额外数据的名称，带有包前缀。
    `value` 长数据值。
    */
    #[java_method(overload = putExtra)]
    pub fn put_extra_long(&self, name: String, value: i64) -> Self {}

    /**
    向 Intent 添加扩展数据。名称必须包含包前缀，例如，应用程序 com.android.contacts 将使用“com.android.contacts.ShowAll”之类的名称。
    返回:返回相同的 Intent 对象，用于将多个调用链接到单个语句中。
    `name` 额外数据的名称，带有包前缀。
    `value` 整数数据值。
    */
    #[java_method(overload = putExtra)]
    pub fn put_extra_int(&self, name: String, value: i32) -> Self {}

    /**
    向 Intent 添加扩展数据。名称必须包含包前缀，例如，应用程序 com.android.contacts 将使用“com.android.contacts.ShowAll”之类的名称。
    返回:返回相同的 Intent 对象，用于将多个调用链接到单个语句中。
    `name` 额外数据的名称，带有包前缀。
    `value` 短数据值。
    */
    #[java_method(overload = putExtra)]
    pub fn put_extra_short(&self, name: String, value: i16) -> Self {}

    /**
    向 Intent 添加扩展数据。名称必须包含包前缀，例如，应用程序 com.android.contacts 将使用“com.android.contacts.ShowAll”之类的名称。
    返回:返回相同的 Intent 对象，用于将多个调用链接到单个语句中。
    `name` 额外数据的名称，带有包前缀。
    `value` char 数据值。
    */
    #[java_method(overload = putExtra)]
    pub fn put_extra_char(&self, name: String, value: char) -> Self {}

    /**
    向 Intent 添加扩展数据。名称必须包含包前缀，例如，应用程序 com.android.contacts 将使用“com.android.contacts.ShowAll”之类的名称。
    返回:返回相同的 Intent 对象，用于将多个调用链接到单个语句中。
    `name` 额外数据的名称，带有包前缀。
    `value` 字节数据值。
    */
    #[java_method(overload = putExtra)]
    pub fn put_extra_byte(&self, name: String, value: u8) -> Self {}

    /**
    向 Intent 添加扩展数据。名称必须包含包前缀，例如，应用程序 com.android.contacts 将使用“com.android.contacts.ShowAll”之类的名称。
    返回：返回相同的意图对象，以将多个调用链接到一个语句中。
    `name` 带有包前缀的额外数据的名称。
    `value` 布尔数据值。
    */
    #[java_method(overload = putExtra)]
    pub fn put_extra_boolean(&self, name: String, value: bool) -> Self {}

    /**
    向 Intent 添加扩展数据。名称必须包含包前缀，例如，应用程序 com.android.contacts 将使用“com.android.contacts.ShowAll”之类的名称。
    返回:返回相同的 Intent 对象，用于将多个调用链接到单个语句中。
    `name` 额外数据的名称，带有包前缀。
    `value` Bundle 数据值。
    */
    #[java_method(overload = putExtra)]
    pub fn put_extra_bundle(&self, name: String, value: Option<Bundle>) -> Self {}

    /**
    将“src”中的所有额外内容复制到此意图中。
    `src` 包含要复制的额外内容。
    */
    #[java_method]
    pub fn put_extras(&self, src: &Self) -> Self {}

    /**
    向 Intent 添加一组扩展数据。键必须包含包前缀，例如，应用 com.android.contacts 将使用“com.android.contacts.ShowAll”之类的名称。
    `extras` 要添加到此 Intent 的 extras 包。
    */
    #[java_method(overload = putExtras)]
    pub fn put_extras_bundle(&self, extras: &Bundle) -> Self {}

    /**
    使用给定 Intent 中的额外内容完全替换 Intent 中的额外内容。
    `src` 此 Intent 中包含的确切额外内容将被复制到目标 Intent 中，替换之前存在的任何额外内容。
    */
    #[java_method]
    pub fn replace_extras(&self, src: &Self) -> Self {}

    /**
    使用给定的额外内容包完全替换 Intent 中的额外内容。
    `extras` Intent 中的新额外内容集，或为 null 以删除所有额外内容。
    */
    #[java_method(overload = replaceExtras)]
    pub fn replace_extras_bundle(&self, extras: Option<Bundle>) -> Self {}

    /**
    从意图中删除扩展数据。
    */
    #[java_method]
    pub fn remove_extra(&self, name: String) {}

    /**
    （通常为可选）设置一个明确的应用程序包名称，以限制此 Intent 将解析到的组件。如果保留默认值 null，则将考虑所有应用程序中的所有组件。
    如果非空，则 Intent 只能匹配给定应用程序包中的组件。
    返回：返回相同的 Intent 对象，用于将多个调用链接到单个语句中。
    `package_name` 用于处理 Intent 的应用程序包的名称，或 null 以允许任何应用程序包。
    */
    #[java_method]
    pub fn set_package(&self, package_name: Option<String>) -> Self {}

    /**
    （通常可选）明确设置处理意图的组件。如果保留默认值 null，系统将根据意图中的其他字段（操作、数据、类型、类别）确定要使用的适当类。
    如果定义了此类，则无论其他字段如何，都将始终使用指定的类。只有当您知道绝对需要使用特定类时，才应设置此值；否则，最好让系统找到适当的类，以便您尊重已安装的应用程序和用户偏好。
    返回：返回相同的 Intent 对象，用于将多个调用链接到单个语句中。
    `component` 处理意图的应用程序组件的名称，或 null 让系统为您找到一个。
    */
    #[java_method]
    pub fn set_component(&self, component: Option<ComponentName>) -> Self {}

    /**
    使用显式类名调用 setComponent 非常方便。
    返回：返回相同的 Intent 对象，用于将多个调用链接到单个语句中。
    `package_context` 实现此类的应用程序包的上下文。
    `class_name` 应用程序包内将用作此 Intent 组件的类的名称。
    */
    #[java_method(overload = setClassName)]
    pub fn set_class_name_from_context(
        &self,
        package_context: &Context,
        class_name: String,
    ) -> Self {
    }

    /**
    使用显式应用程序包名称和类名称调用 setComponent 非常方便。
    返回：返回相同的 Intent 对象，用于将多个调用链接到单个语句中。
    `package_name` 实现所需组件的包的名称。
    `class_name` 应用程序包内将用作此 Intent 组件的类的名称。
    */
    #[java_method]
    pub fn set_class_name(&self, package_name: String, class_name: String) -> Self {}

    /**
    查询此 Intent 所限的应用程序包名称。解析 Intent 时，如果非空，则将解析限制为仅给定应用程序包中的组件。
    返回：Intent 的应用程序包名称。
    */
    #[java_method]
    pub fn get_package(&self) -> Option<String> {}

    /**
    检索与意图相关的具体组件。当收到意图时，这是被发现最适合处理该意图的组件（即您自己），并且始终为非空；在所有其他情况下，除非明确设置，否则它将为空。
    返回：处理意图的应用程序组件的名称。
    */
    #[java_method]
    pub fn get_component(&self) -> Option<ComponentName> {}
}

/**
可用的特定应用组件（android.app.Activity、android.app.Service、BroadcastReceiver 或 ContentProvider）的标识符。
需要在此处封装两条信息来识别组件：组件所在的包（字符串）以及该包内的类名（字符串）。
*/
#[java_class(name = "android/content/ComponentName")]
pub struct ComponentName;

impl ComponentName {
    /**
    创建一个新的组件标识符，其中类名可以指定为绝对名称或相对于包含包的名称。相对包名以“.”字符开头。
    对于包“com.example”和类名“.app.MyActivity”，此方法将返回包“com.example”和类名“com.example.app.MyActivity”的 ComponentName。
    还允许使用完全限定的类名。
    返回：新的 ComponentName
    `pkg` 组件所在的包的名称
    `cls` pkg 中实现组件的类的名称
    */
    #[java_method]
    pub fn create_relative(pkg: String, cls: String) -> Self {}

    /**
    创建一个新的组件标识符，其中类名可以指定为绝对名称或相对于包含包的名称。相对包名以“.”字符开头。
    对于包“com.example”和类名“.app.MyActivity”，此方法将返回一个包含包“com.example”和类名“com.example.app.MyActivity”的 ComponentName。
    还允许使用完全限定的类名。
    返回：新的 ComponentName
    `pkg` 实现组件的包的上下文
    `cls` 实现组件的 pkg 内类的名称
    */
    #[java_method(overload = createRelative)]
    pub fn create_relative_context(pkg: &Context, cls: String) -> Self {}

    /**
    创建一个新的组件标识符。
    `pkg` 组件所在包的名称。不能为空。
    `cls` pkg 中实现组件的类的名称。不能为空。
    */
    #[java_constructor]
    pub fn new(pkg: String, cls: String) -> Self {}

    /**
    根据上下文和类名创建新的组件标识符。
    `pkg` 实现组件的包的上下文，将从中检索实际的包名称。
    `cls` pkg 中实现组件的类的名称。
    */
    #[java_constructor]
    pub fn new_context(pkg: &Context, cls: String) -> Self {}

    #[doc(hidden)]
    #[java_method]
    pub fn clone(&self) -> Self {}

    /// 返回该组件的包名称。
    #[java_method]
    pub fn get_package_name(&self) -> String {}

    /// 返回此组件的类名。
    #[java_method]
    pub fn get_class_name(&self) -> String {}

    /// 返回类名，如果它是包的后缀，则返回完全限定或缩写形式（以“。”为前导）。
    #[java_method]
    pub fn get_short_class_name(&self) -> String {}

    /**
    助手在可以为空的 ComponentName 引用中获取 flattenToShortString()。
    */
    #[java_method(overload = flattenToShortString)]
    pub fn flatten_to_short_string_static(component_name: Option<Self>) -> String {}

    //noinspection SpellCheckingInspection
    /**
    返回一个明确描述 ComponentName 中包含的包和类名称的字符串。稍后您可以通过 unflattenFromString(String) 从此字符串中恢复 ComponentName。
    返回：返回一个包含包和类名称的新字符串。这表示为包名称，用“/”连接，然后是类名称。
    */
    #[java_method]
    pub fn flatten_to_string(&self) -> String {}

    //noinspection SpellCheckingInspection
    /**
    与 flattenToString() 相同，但如果类名是包的后缀，则缩写类名。结果仍可与 unflattenFromString(String) 一起使用。
    返回：返回一个包含包和类名的新字符串。这表示为包名称，用“/”连接，然后是类名。
    */
    #[java_method]
    pub fn flatten_to_short_string(&self) -> String {}

    //noinspection SpellCheckingInspection
    /**
    从之前使用 flattenToString() 创建的字符串中恢复 ComponentName。它在第一个“/”处拆分字符串，将前面的部分作为包名称，将后面的部分作为类名称。
    为了特别方便（例如，在命令行上解析组件名称时使用），如果“/”后面紧跟着“.”，则最终的类名将是包名称与“/”后面的字符串的连接。因此，“com.foo/.Blah”变为 package="com.foo" class="com.foo.Blah"。
    返回：返回一个新的 ComponentName，其中包含在 str 中编码的包和类名称
    `str` flattenToString() 返回的字符串。
    */
    #[java_method]
    pub fn unflatten_from_string(r#str: String) -> Option<Self> {}

    /**
    返回此类的字符串表示形式，不带类名作为前缀。
    */
    #[java_method]
    pub fn to_short_string(&self) -> String {}

    #[doc(hidden)]
    #[java_method]
    pub fn describe_contents(&self) -> i32 {}
}

impl Comparable<ComponentName> for ComponentName {
    #[java_method]
    fn compare_to(&self, o: &ComponentName) -> Result<i32, <Self as JType>::Error> {}
}

#[cfg(feature = "android_app")]
impl From<&crate::android::app::Activity> for Context {
    fn from(value: &crate::android::app::Activity) -> Self {
        Self::_new(&value.java_ref(), ())
    }
}

/// 与组件名称关联的类的接口。
#[allow(non_camel_case_types)]
#[java_interface(name = "android/content/ComponentName$WithComponentName")]
pub trait ComponentName_WithComponentName {
    /// 返回关联的组件名称。
    fn get_component_name(&self) -> ComponentName;
}

/// 测试android.content
#[cfg(feature = "test_android_content")]
pub fn test() {
    let act = crate::android::app::Activity::fetch();
    assert!(act.to_string().starts_with("android.app.NativeActivity"));
    let context: Context = (&act).into();
    assert!(context
        .to_string()
        .starts_with("android.app.NativeActivity"));
    assert_ne!(context.get_class_loader(), ClassLoader::null());
    assert_eq!("rust.droid_wrap_test", context.get_package_name());
    assert_eq!(
        context.get_base_package_name(),
        context.get_op_package_name()
    );
    assert_eq!(None, context.get_attribution_tag());
    assert!(context
        .get_package_resource_path()
        .contains("rust.droid_wrap_test"));
    assert!(context
        .get_package_code_path()
        .contains("rust.droid_wrap_test"));

    let intent = Intent::new();
    assert!(intent.to_string().starts_with("Intent"));
    assert_eq!(
        intent,
        intent.add_category(Intent::CATEGORY_ACCESSIBILITY_SHORTCUT_TARGET.to_string())
    );
    intent.remove_category(Intent::CATEGORY_ACCESSIBILITY_SHORTCUT_TARGET.to_string());
    assert_eq!(
        false,
        intent.has_category(Intent::CATEGORY_ACCESSIBILITY_SHORTCUT_TARGET.to_string())
    );
    assert_eq!(intent, intent.set_action(Intent::ACTION_VIEW.to_string()));
    assert_eq!(Intent::ACTION_VIEW, intent.get_action());
    assert_eq!(intent, intent.add_flags(Intent::FLAG_ACTIVITY_NEW_TASK));
    assert_eq!(Intent::FLAG_ACTIVITY_NEW_TASK, intent.get_flags());
    intent.remove_flags(Intent::FLAG_ACTIVITY_NEW_TASK);
    assert_eq!(0, intent.get_flags());
    assert_eq!(None, intent.get_data_string());

    context.send_broadcast(&intent);
    context.start_activity(&intent);
    let am = context.get_system_service(Context::AUDIO_SERVICE.to_string());
    assert!(am.is_some());
}
