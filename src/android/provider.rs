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

use crate::{android::content::Context, JObjNew, JObjRef, JType};
use droid_wrap_derive::{java_class, java_method};

/**
设置提供者包含全局系统级设备首选项。
*/
#[java_class(name = "android/provider/Settings")]
pub struct Settings;

impl Settings {
    #[doc(hidden)]
    pub const DEFAULT_OVERRIDEABLE_BY_RESTORE: bool = false;

    // Intent actions for Settings

    /**
    活动操作：显示系统设置。
    输入：无。输出：无。
    */
    pub const ACTION_SETTINGS: &'static str = "android.settings.SETTINGS";

    /**
    活动操作：显示提供运营商卫星消息指南的设置。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    输入：无。输出：无。
    */
    pub const ACTION_SATELLITE_SETTING: &'static str = "android.settings.SATELLITE_SETTING";

    /**
    活动操作：显示允许配置APN的设置。
    输入：无。输出：无。
    在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    */
    pub const ACTION_APN_SETTINGS: &'static str = "android.settings.APN_SETTINGS";

    /**
    活动操作：显示允许配置当前位置源的设置。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    输入：无。输出：无。
    */
    pub const ACTION_LOCATION_SOURCE_SETTINGS: &'static str =
        "android.settings.LOCATION_SOURCE_SETTINGS";

    /**
    活动操作：显示允许配置位置控制器额外包的设置。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    输入：无。输出：无。
    */
    pub const ACTION_LOCATION_CONTROLLER_EXTRA_PACKAGE_SETTINGS: &'static str =
        "android.settings.LOCATION_CONTROLLER_EXTRA_PACKAGE_SETTINGS";

    /**
    活动操作：显示扫描设置，允许配置Wi-Fi和蓝牙扫描设置。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    输入：无。输出：无。
    */
    pub const ACTION_LOCATION_SCANNING_SETTINGS: &'static str =
        "android.settings.LOCATION_SCANNING_SETTINGS";

    /**
    活动操作：显示管理克隆应用创建/删除的设置。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    输入：无。输出：无。
    */
    pub const ACTION_MANAGE_CLONED_APPS_SETTINGS: &'static str =
        "android.settings.MANAGE_CLONED_APPS_SETTINGS";

    /**
    活动操作：显示允许配置用户的设置。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    输入：无。输出：无。
    */
    pub const ACTION_USER_SETTINGS: &'static str = "android.settings.USER_SETTINGS";

    /**
    活动操作：显示允许配置无线控制设置，如Wi-Fi、蓝牙和移动网络。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    输入：无。输出：无。
    */
    pub const ACTION_WIRELESS_SETTINGS: &'static str = "android.settings.WIRELESS_SETTINGS";

    /**
    活动操作：显示连接提供活动。
    在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    输入：ConnectivityManager#EXTRA_TETHER_TYPE应包含以指定应检查哪种类型的连接共享。ConnectivityManager#EXTRA_PROVISION_CALLBACK应包含一个ResultReceiver，该ResultReceiver将在连接共享结果代码被调用时返回。
    输出：连接共享检查的结果。如果成功，则为ConnectivityManager#TETHER_ERROR_NO_ERROR；如果失败，则为ConnectivityManager#TETHER_ERROR_PROVISION_FAILED。
    */
    pub const ACTION_TETHER_PROVISIONING_UI: &'static str =
        "android.settings.TETHER_PROVISIONING_UI";

    /**
    活动操作：显示一个对话框活动，通知运营商不支持连接共享。
    当android.telephony.CarrierConfigManager#KEY_CARRIER_SUPPORTS_TETHERING_BOOL为false，并且连接共享由设置启动时，将启动此对话框活动，通知用户运营商不支持连接共享。
    */
    pub const ACTION_TETHER_UNSUPPORTED_CARRIER_UI: &'static str =
        "android.settings.TETHER_UNSUPPORTED_CARRIER_UI";

    /**
    活动操作：显示允许进入/退出飞行模式的设置。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    输入：无。输出：无。
    */
    pub const ACTION_AIRPLANE_MODE_SETTINGS: &'static str =
        "android.settings.AIRPLANE_MODE_SETTINGS";

    //noinspection SpellCheckingInspection
    /**
    活动操作：显示当前启用eSIM配置文件的设置页面。
    输入：无。输出：无。
    */
    pub const ACTION_SHOW_ENABLED_ESIM_PROFILE: &'static str =
        "android.settings.SHOW_ENABLED_ESIM_PROFILE";

    /**
    活动操作：显示移动数据使用列表。
    输入：应包含EXTRA_NETWORK_TEMPLATE和EXTRA_SUB_ID以指定如何和收集哪些移动数据统计信息。
    输出：无
    */
    pub const ACTION_MOBILE_DATA_USAGE: &'static str = "android.settings.MOBILE_DATA_USAGE";

    #[doc(hidden)]
    pub const EXTRA_NETWORK_TEMPLATE: &'static str = "network_template";

    /**
    活动操作：显示单手模式设置页面。
    输入：无。输出：无。
    */
    pub const ACTION_ONE_HANDED_SETTINGS: &'static str =
        "android.settings.action.ONE_HANDED_SETTINGS";

    /**
    KEY_CONFIG_SET_ALL_RETURN的返回值，表示操作失败。
    */
    pub const SET_ALL_RESULT_FAILURE: i32 = 0;

    /**
    KEY_CONFIG_SET_ALL_RETURN的返回值，表示操作成功。
    */
    pub const SET_ALL_RESULT_SUCCESS: i32 = 1;

    /**
    KEY_CONFIG_SET_ALL_RETURN的返回值，表示全部设置功能已被禁用。
    */
    pub const SET_ALL_RESULT_DISABLED: i32 = 2;

    #[doc(hidden)]
    pub const KEY_CONFIG_SET_ALL_RETURN: &'static str = "config_set_all_return";

    #[doc(hidden)]
    pub const KEY_CONFIG_GET_SYNC_DISABLED_MODE_RETURN: &'static str =
        "config_get_sync_disabled_mode_return";

    /**
    一个整型额外数据，用于指定订阅ID。
    */
    pub const EXTRA_SUB_ID: &'static str = "android.provider.extra.SUB_ID";

    /**
    活动动作：使用语音命令修改飞行模式设置。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。此意图必须使用android.service.voice.VoiceInteractionSession#startVoiceActivity方法启动。
    注意：实现此意图的活动在修改设置之前，必须验证android.app.Activity#isVoiceInteraction方法返回true。
    输入：要指定飞行模式应设置为哪个状态，请将此Intent的EXTRA_AIRPLANE_MODE_ENABLED额外数据添加为指定状态。如果不包含此额外数据，则不会进行任何更改。
    输出：无。
    */
    pub const ACTION_VOICE_CONTROL_AIRPLANE_MODE: &'static str =
        "android.settings.VOICE_CONTROL_AIRPLANE_MODE";

    /**
    活动动作：显示无障碍模块的设置。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    输入：无。输出：无。
    */
    pub const ACTION_ACCESSIBILITY_SETTINGS: &'static str =
        "android.settings.ACCESSIBILITY_SETTINGS";

    /**
    活动动作：显示特定无障碍服务的详细设置。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    输入：Intent#EXTRA_COMPONENT_NAME必须指定要显示的无障碍服务组件名称。
    输出：无。
    */
    pub const ACTION_ACCESSIBILITY_DETAILS_SETTINGS: &'static str =
        "android.settings.ACCESSIBILITY_DETAILS_SETTINGS";

    /**
    活动动作：显示设置以允许配置属于无障碍功能的一个或多个无障碍快捷操作。输入：“:settings:show_fragment_args”必须包含“targets”，表示要编辑的服务。输出：无。
    */
    pub const ACTION_ACCESSIBILITY_SHORTCUT_SETTINGS: &'static str =
        "android.settings.ACCESSIBILITY_SHORTCUT_SETTINGS";

    /**
    活动动作：显示设置以允许配置无障碍功能的颜色和动态效果。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    输入：无。输出：无。
    */
    pub const ACTION_ACCESSIBILITY_COLOR_MOTION_SETTINGS: &'static str =
        "android.settings.ACCESSIBILITY_COLOR_MOTION_SETTINGS";

    /**
    活动动作：显示设置以允许配置无障碍功能的颜色对比度。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    输入：无。输出：无。
    */
    pub const ACTION_ACCESSIBILITY_COLOR_CONTRAST_SETTINGS: &'static str =
        "android.settings.ACCESSIBILITY_COLOR_CONTRAST_SETTINGS";

    /**
    活动动作：显示设置以允许配置减弱鲜艳颜色。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    输入：无。输出：无。
    */
    pub const ACTION_REDUCE_BRIGHT_COLORS_SETTINGS: &'static str =
        "android.settings.REDUCE_BRIGHT_COLORS_SETTINGS";

    /**
    活动动作：显示设置以允许配置颜色校正。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    输入：无。输出：无。
    */
    pub const ACTION_COLOR_CORRECTION_SETTINGS: &'static str =
        "com.android.settings.ACCESSIBILITY_COLOR_SPACE_SETTINGS";

    /**
    活动动作：显示允许颜色反转配置的设置。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    输入：无。输出：无。
    */
    pub const ACTION_COLOR_INVERSION_SETTINGS: &'static str =
        "android.settings.COLOR_INVERSION_SETTINGS";

    /**
    活动动作：显示允许文本阅读配置的设置。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    输入：无。输出：无。
    */
    pub const ACTION_TEXT_READING_SETTINGS: &'static str = "android.settings.TEXT_READING_SETTINGS";

    /**
    活动动作：显示控制访问使用信息的设置。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    输入：无。输出：无。
    */
    pub const ACTION_USAGE_ACCESS_SETTINGS: &'static str = "android.settings.USAGE_ACCESS_SETTINGS";

    /**
    活动类别：显示与应用使用访问相关的设置。一个为用户提供界面以调整其包含应用程序的使用访问相关偏好的活动。对于使用android.Manifest.permission#PACKAGE_USAGE_STATS权限的应用程序来说，这是可选但建议的。该活动可以使用METADATA_USAGE_ACCESS_REASON定义元数据来描述其应用内使用访问的用途，该描述将在设置中显示。
    输入：无。输出：无。
    */
    pub const INTENT_CATEGORY_USAGE_ACCESS_CONFIG: &'static str =
        "android.intent.category.USAGE_ACCESS_CONFIG";

    /**
    元数据键：需要使用访问的原因。这是附加到接收动作INTENT_CATEGORY_USAGE_ACCESS_CONFIG的活动的元数据键，向用户显示为描述应用如何使用使用访问的说明。
    */
    pub const METADATA_USAGE_ACCESS_REASON: &'static str =
        "android.settings.metadata.USAGE_ACCESS_REASON";

    /**
    活动操作：显示设置以允许配置安全和位置隐私。在某些情况下，可能不存在匹配的活动，因此请确保防范这种情况。
    输入：无。输出：无。
    */
    pub const ACTION_SECURITY_SETTINGS: &'static str = "android.settings.SECURITY_SETTINGS";

    /**
    活动操作：显示设置以允许配置受信任的外部源
    输入：可选，意图的数据URI可以指定应用程序包名称，以直接调用特定于软件包名称的管理GUI。例如“软件包：com.my.app”。
    输出：没有。
    */
    pub const ACTION_MANAGE_UNKNOWN_APP_SOURCES: &'static str =
        "android.settings.MANAGE_UNKNOWN_APP_SOURCES";

    /**
    活动操作：显示设置以允许列出。
    输入：Intent 的数据 URI 可以指定应用程序包名称，以直接调用特定于包名称的管理 GUI（可选）。例如“package:com.my.app”。
    输出：当将包数据 URI 作为输入传递时，如果已向应用程序授予权限，则活动结果将设置为 android.app.Activity#RESULT_OK。否则，结果将设置为 android.app.Activity#RESULT_CANCELED。
    */
    pub const ACTION_REQUEST_SCHEDULE_EXACT_ALARM: &'static str =
        "android.settings.REQUEST_SCHEDULE_EXACT_ALARM";

    /**
    活动操作：显示设置以允许列出的配置。
    输入：可选，意图的数据URI可以指定应用程序包名称，以直接调用特定于软件包名称的管理GUI。例如“软件包：com.my.app”。
    输出：没有。
    */
    pub const ACTION_REQUEST_MANAGE_MEDIA: &'static str = "android.settings.REQUEST_MANAGE_MEDIA";

    /**
    活动操作：显示设置以允许Manifest.permission#MEDIA_ROUTING_CONTROL权限的配置。
    输入：可选，意图的数据URI可以指定应用程序包名称，以直接调用特定于软件包名称的管理GUI。例如“软件包：com.my.app”。但是，仅当该软件包保存适当的伴随设备配置文件（例如android.companion.associationRequest＃device_profile_watch）时，才允许修改任何软件包的此权限设置。
    输出：没有。
    */
    pub const ACTION_REQUEST_MEDIA_ROUTING_CONTROL: &'static str =
        "android.settings.REQUEST_MEDIA_ROUTING_CONTROL";

    /**
    活动操作：显示设置以允许Manifest.permission#RUN_USER_INITIATED_JOBS权限配置
    输入：Intent 的数据 URI 可以指定应用程序包名称，以直接调用特定于包名称的管理 GUI（可选）。例如“package:com.my.app”。
    输出：当将包数据 URI 作为输入传递时，如果已向应用程序授予权限，则活动结果将设置为 android.app.Activity#RESULT_OK。否则，结果将设置为 android.app.Activity#RESULT_CANCELED。
    */
    pub const ACTION_MANAGE_APP_LONG_RUNNING_JOBS: &'static str =
        "android.settings.MANAGE_APP_LONG_RUNNING_JOBS";

    /**
    活动操作：显示设置以允许配置应用程序的跨profile访问
    输入：可选，意图的数据URI可以指定应用程序包名称，以直接调用特定于软件包名称的管理GUI。例如“软件包：com.my.app”。
    输出：没有。
    */
    pub const ACTION_MANAGE_CROSS_PROFILE_ACCESS: &'static str =
        "android.settings.MANAGE_CROSS_PROFILE_ACCESS";

    /**
    活动操作：在特定应用程序的详细信息页面中显示“默认打开”页面。在某些情况下，可能不存在匹配活动，因此请确保您保护这一点。
    输入：意图的数据URI指定了“包”方案的应用程序包名称。那是“软件包：com.my.app”。
    输出：没有。
    */
    pub const ACTION_APP_OPEN_BY_DEFAULT_SETTINGS: &'static str =
        "android.settings.APP_OPEN_BY_DEFAULT_SETTINGS";

    /**
    活动操作：显示可信赖的凭据设置，打开“用户”选项卡，以允许管理已安装的凭据。 在某些情况下，可能不存在匹配活动，因此请确保您保护这一点。
    输入：没有。 输出：没有。
    */
    pub const ACTION_TRUSTED_CREDENTIALS_USER: &'static str =
        "com.android.settings.TRUSTED_CREDENTIALS_USER";

    /**
    活动动作：显示对话框，解释了安装的CA证书可以启用对加密网络流量的监视。 在某些情况下，可能不存在匹配活动，因此请确保您保护这一点。添加extra_number_of_certificate额外指示证书数量。
    输入：没有。 输出：没有。
    */
    pub const ACTION_MONITORING_CERT_INFO: &'static str =
        "com.android.settings.MONITORING_CERT_INFO";

    /**
    活动操作：显示设置以允许配置隐私选项，即权限经理，隐私仪表板，隐私控制等。 在某些情况下，可能不存在匹配活动，因此请确保您保护这一点。
    输入：没有。 输出：没有。
    */
    pub const ACTION_PRIVACY_SETTINGS: &'static str = "android.settings.PRIVACY_SETTINGS";

    /**
    活动操作：显示隐私控制子页面，即隐私（摄像头/麦克风）切换等。在某些情况下，可能不存在匹配的活动，因此请确保防范这种情况。
    输入：无。输出：无。
    */
    pub const ACTION_PRIVACY_CONTROLS: &'static str = "android.settings.PRIVACY_CONTROLS";

    /**
    活动操作：显示设置以允许VPN配置。 在某些情况下，可能不存在匹配活动，因此请确保您保护这一点。
    输入：没有。 输出：没有。
    */
    pub const ACTION_VPN_SETTINGS: &'static str = "android.settings.VPN_SETTINGS";

    /**
    活动动作：显示设置以允许对Wi-Fi进行配置。 在某些情况下，可能不存在匹配活动，因此请确保您保护这一点。
    输入：没有。 输出：没有。
    */
    pub const ACTION_WIFI_SETTINGS: &'static str = "android.settings.WIFI_SETTINGS";

    /**
    活动操作：显示设置以允许配置高级内存保护。 内存标记扩展名（MTE）是CPU扩展程序，可在较小的运行时性能成本开销中保护某些类别的安全问题。 在某些情况下，可能不存在匹配活动，因此请确保您保护这一点。
    输入：没有。 输出：没有。
    */
    pub const ACTION_ADVANCED_MEMORY_PROTECTION_SETTINGS: &'static str =
        "android.settings.ADVANCED_MEMORY_PROTECTION_SETTINGS";

    /**
    活动操作：显示设置以允许配置 Wi-Fi 的静态 IP 地址。在某些情况下，可能不存在匹配的活动，因此请确保防范这种情况。
    输入：无。输出：无。
    */
    pub const ACTION_WIFI_IP_SETTINGS: &'static str = "android.settings.WIFI_IP_SETTINGS";

    /**
    活动操作：显示设置页面以处理Wi-Fi Easy Connect（又称DPP）URI并开始配置。当您要使用此设备来扮演物联网/其他设备的配置角色时，应使用此意图。
    在提供有效的DPP URI字符串时，设置将打开Wi-Fi选择屏幕，供用户指示他们想配置DPP URI字符串中指定的设备，并将其携带通过流量的其余部分，以配置设备。在某些情况下，可能不存在匹配活动，因此请确保通过检查WifiManager＃isEasyConnectSupported()来保护此问题。
    输入：意图的数据URI指定了自举信息，以对同伴进行身份验证和配置，并使用“ DPP”方案。 URI应使用意图＃setData（uri）附加到意图上。调用应用程序可以以任何方式获得DPP URI，例如通过扫描QR码或其他带外方法。调用应用程序还可以连接Extra_easy_connect_band_list额外的额外，以提供有关注册设备支持的频段的信息。
    输出：调用Android.app.Activity＃startActivityForResult后，回调``将作为额外的 EXTRA_EASY_CONNECT_ERROR_CODE 返回。 Easy Connect R2 报告了有关其遇到的错误的更多详细信息，这些详细信息将在Extra_easy_easy_connect_attempted_ssid，extra_easy_connect_channel_list和extra_easy_easy_connect_band_list中提供。
    */
    pub const ACTION_PROCESS_WIFI_EASY_CONNECT_URI: &'static str =
        "android.settings.PROCESS_WIFI_EASY_CONNECT_URI";

    /**
    活动额外：轻松连接操作错误代码使用 ACTION_PROCESS_WIFI_EASY_CONNECT_URI 意图启动Easy Connect操作时，收到的结果意图上返回了额外的返回。此额外包含操作的整数错误代码-Android.net.wifi.easyConnectStatusCallback之一。
    如果没有错误，即如果操作返回android.app.Activity#RESULT_OK，则此额外不会附加到结果意图。 使用Intent#hasExtra(String)来确定是否附加了额外的意图，并且Intent#getIntExtra(String, int)以获取错误代码数据。
    */
    pub const EXTRA_EASY_CONNECT_ERROR_CODE: &'static str =
        "android.provider.extra.EASY_CONNECT_ERROR_CODE";

    //noinspection GrazieInspection
    /**
    Activity Extra：尝试连接的 SSID。使用 ACTION_PROCESS_WIFI_EASY_CONNECT_URI Intent 启动 Easy Connect Operation 时收到的结果 Intent 上返回的 extra。此 extra 包含远程尝试连接的接入点的 SSID。
    此值仅由远程 R2 设备填充，并且仅用于以下错误代码：android.net.wifi.EasyConnectStatusCallback#EASY_CONNECT_EVENT_FAILURE_CANNOT_FIND_NETWORK android.net.wifi.EasyConnectStatusCallback#EASY_CONNECT_EVENT_FAILURE_ENROLLEE_AUTHENTICATION。因此，请始终使用 Intent#hasExtra(String) 检查此 extra 是否可用。
    如果没有错误，即如果操作返回 android.app.Activity#RESULT_OK，则此 extra 未附加到结果 Intent。使用 Intent#getStringExtra(String) 获取 SSID。
    */
    pub const EXTRA_EASY_CONNECT_ATTEMPTED_SSID: &'static str =
        "android.provider.extra.EASY_CONNECT_ATTEMPTED_SSID";

    /**
    活动额外：参与者用来扫描网络的通道列表。 使用 ACTION_PROCESS_WIFI_EASY_CONNECT_URI 意图启动易于连接操作时，收到的结果意图又有额外的返回。此额外包含参与者扫描网络的频道列表。
    此值仅由远程R2设备填充，仅用于以下错误代码：android.net.wifi.easyConnectStatusCallback#EASY_CONNECT_EVENT_FAILURE_CANNOT_FIND_NETWORK。因此，始终使用意图＃hasExtra(String)检查此额外是否可用。
    如果没有错误，即如果操作返回android.app.Activity#RESULT_OK，则此额外不会附加到结果意图。该列表是JSON格式的，作为数组（Wi-Fi频道）的数组（Wi-Fi全局操作类）。 使用意图＃getStringExtra(String)获取列表。
    */
    pub const EXTRA_EASY_CONNECT_CHANNEL_LIST: &'static str =
        "android.provider.extra.EASY_CONNECT_CHANNEL_LIST";

    /**
    活动额外：参与者支持的乐队列表。 这额外包含了注册人支持的乐队，以全球运营类表示，请参见IEEE STD 802.11-2016全球运营类中的表E-4。它既将其用作输入，以配置简易连接操作和操作的输出。
    作为输入：可选的要附加到 ACTION_PROCESS_WIFI_EASY_CONNECT_URI 。如果附加，则指示远程设备（注册设备，到达设备配置）支持的频段。在向用户呈现要使用的网络配置列表时，设置操作可能会考虑到这一点。
    调用应用程序可以以任何带外方法获取此信息。该信息应作为原始整数数组附加 - 使用Intent#putExtra(String, int[]) 作为输出：使用 ACTION_PROCESS_WIFI_EASY_CONNECT_URI 意图启动Easy Connect操作时收到的结果意图的额外返回。此值仅由远程R2设备填充，仅用于以下错误代码：
    android.net.wifi.easyConnectStatusCallbackEASY_CONNECT_EVENT_FAILURE_CANNOT_FIND_NETWORK，android.net.wifi.EasyConnectStatusCallback#EASY_CONNECT_EVENT_FAILURE_ENROLLED_REJECTED_CONFIGURATION。
    因此，始终使用Intent#hasExtra(String)检查此额外是否可用。如果没有错误，即如果操作返回android.app.Activity#RESULT_OK，则此额外不会附加到结果意图。 使用Intent#getIntarrayExtra(String)获取列表。
    */
    pub const EXTRA_EASY_CONNECT_BAND_LIST: &'static str =
        "android.provider.extra.EASY_CONNECT_BAND_LIST";

    /**
    活动操作：显示设置以允许配置数据并查看数据使用情况。 在某些情况下，可能不存在匹配活动，因此请确保您保护这一点。
    输入：没有。 输出：没有。
    */
    pub const ACTION_DATA_USAGE_SETTINGS: &'static str = "android.settings.DATA_USAGE_SETTINGS";

    /**
    活动动作：显示设置以允许蓝牙配置。 在某些情况下，可能不存在匹配活动，因此请确保您保护这一点。
    输入：没有。 输出：没有。
    */
    pub const ACTION_BLUETOOTH_SETTINGS: &'static str = "android.settings.BLUETOOTH_SETTINGS";

    /**
    活动操作：显示设置以允许配置听力设备。在某些情况下，可能不存在匹配的活动，因此请确保采取预防措施。
    输入：无。输出：无。
    */
    pub const ACTION_HEARING_DEVICES_SETTINGS: &'static str =
        "android.settings.HEARING_DEVICES_SETTINGS";

    /**
    活动操作：当此操作可用于设备时，显示“设置”应用搜索 UI。
    输入：无。输出：无。
    */
    pub const ACTION_APP_SEARCH_SETTINGS: &'static str = "android.settings.APP_SEARCH_SETTINGS";

    /**
    活动动作：显示设置以允许配置辅助手势。 在某些情况下，可能不存在匹配活动，因此请确保您保护这一点。
    输入：没有。 输出：没有。
    */
    pub const ACTION_ASSIST_GESTURE_SETTINGS: &'static str =
        "android.settings.ASSIST_GESTURE_SETTINGS";

    /**
    活动动作：显示设置以注册指纹，并在必要时设置PIN/模式/通过。
    输入：没有。 输出：没有
    */
    #[deprecated(note = "参阅 ACTION_BIOMETRIC_ENROLL")]
    pub const ACTION_FINGERPRINT_ENROLL: &'static str = "android.settings.FINGERPRINT_ENROLL";

    /**
    活动操作：显示注册生物特征的设置，并根据需要设置 PIN/图案/密码。默认情况下，这会提示用户注册强度为“弱”或以上的生物特征（如 CDD 所定义）。只有达到或超过“强”的生物特征（如 CDD 所定义）才允许参与密钥库操作。
    输入：extras EXTRA_BIOMETRIC_AUTHENTICATORS_ALLOWED 为整数，常量在 android.hardware.biometrics.BiometricManager.Authenticators 中定义，例如 android.hardware.biometrics.BiometricManager.Authenticators#BIOMETRIC_STRONG。如果未指定，则默认行为是 android.hardware.biometrics.BiometricManager.Authenticators#BIOMETRIC_WEAK。
    输出：无。请注意，调用者之后仍应检查 android.hardware.biometrics.BiometricManager#canAuthenticate(int)，以确保用户确实完成了注册。
    */
    pub const ACTION_BIOMETRIC_ENROLL: &'static str = "android.settings.BIOMETRIC_ENROLL";

    /**
    Activity Extra：请求注册的最低强度。这可以作为额外字段传递给 ACTION_BIOMETRIC_ENROLL 意图，以指示仅应显示符合这些要求的传感器的注册。该值应为 android.hardware.biometrics.BiometricManager.Authenticators 中定义的常量的组合。
    */
    pub const EXTRA_BIOMETRIC_AUTHENTICATORS_ALLOWED: &'static str =
        "android.provider.extra.BIOMETRIC_AUTHENTICATORS_ALLOWED";

    /**
    活动动作：显示设置以允许铸造端点配置。 在某些情况下，可能不存在匹配活动，因此请确保您保护这一点。
    输入：没有。 输出：没有。
    */
    pub const ACTION_CAST_SETTINGS: &'static str = "android.settings.CAST_SETTINGS";

    /**
    活动操作：显示设置以允许配置日期和时间。在某些情况下，可能不存在匹配的活动，因此请确保防范这种情况。
    输入：无。输出：无。
    */
    pub const ACTION_DATE_SETTINGS: &'static str = "android.settings.DATE_SETTINGS";

    /**
    活动操作：显示设置以允许配置声音和音量。在某些情况下，可能不存在匹配的活动，因此请确保防范这种情况。
    输入：无。输出：无。
    */
    pub const ACTION_SOUND_SETTINGS: &'static str = "android.settings.SOUND_SETTINGS";

    /**
    活动操作：显示设置以允许显示显示。 在某些情况下，可能不存在匹配活动，因此请确保您保护这一点。
    输入：没有。 输出：没有。
    */
    pub const ACTION_DISPLAY_SETTINGS: &'static str = "android.settings.DISPLAY_SETTINGS";

    /**
    活动动作：显示自动旋转配置设置。
    */
    pub const ACTION_AUTO_ROTATE_SETTINGS: &'static str = "android.settings.AUTO_ROTATE_SETTINGS";

    /**
    活动操作：显示设置以允许配置夜间显示。在某些情况下，可能不存在匹配的活动，因此请确保防范这种情况。
    输入：无。输出：无。
    */
    pub const ACTION_NIGHT_DISPLAY_SETTINGS: &'static str =
        "android.settings.NIGHT_DISPLAY_SETTINGS";

    /**
    活动动作：显示设置以允许对黑暗主题进行配置。 在某些情况下，可能不存在匹配活动，因此请确保您保护这一点。
    输入：没有。 输出：没有。
    */
    pub const ACTION_DARK_THEME_SETTINGS: &'static str = "android.settings.DARK_THEME_SETTINGS";

    /**
    活动操作：显示设置以允许配置语言环境。在某些情况下，可能不存在匹配的活动，因此请确保防范这种情况。
    输入：可选的 ` ` 包含不受支持的语言环境，它仍会在列表中显示此语言环境，但设备可能不支持。
    输出：无。
    */
    pub const ACTION_LOCALE_SETTINGS: &'static str = "android.settings.LOCALE_SETTINGS";

    /**
    活动额外：在启动的Locale Picker活动中显示明确的地区。这可以作为一个或多个语言标签作为本地主义者的活动意图中的额外字段传递。必须将其作为额外字段传递给ACTION_LOCALE_SETTINGS。
    */
    pub const EXTRA_EXPLICIT_LOCALES: &'static str = "android.provider.extra.EXPLICIT_LOCALES";

    /**
    活动操作：显示设置以允许每个应用程序区域的配置。
    输入：意图的数据URI可以指定应用程序包名称，以直接调用App Locale详细信息，GUI特定于软件包名称。例如“软件包：com.my.app”。
    输出：没有。
    */
    pub const ACTION_APP_LOCALE_SETTINGS: &'static str = "android.settings.APP_LOCALE_SETTINGS";

    /**
    活动操作：显示设置以允许对区域首选项的配置
    输入：无；输出：无。
    */
    pub const ACTION_REGIONAL_PREFERENCES_SETTINGS: &'static str =
        "android.settings.REGIONAL_PREFERENCES_SETTINGS";

    /**
    活动操作：显示设置以允许锁定屏幕配置。 在某些情况下，可能不存在匹配活动，因此请确保您保护这一点。
    输入：没有。 输出：没有。
    */
    pub const ACTION_LOCKSCREEN_SETTINGS: &'static str = "android.settings.LOCK_SCREEN_SETTINGS";

    /**
    活动操作：显示允许配对蓝牙设备的设置。在某些情况下，可能不存在匹配的活动，因此请确保采取预防措施。
    输入：无。输出：无。
    */
    pub const ACTION_BLUETOOTH_PAIRING_SETTINGS: &'static str =
        "android.settings.BLUETOOTH_PAIRING_SETTINGS";

    /**
    活动动作：显示设置以允许配对听证设备。 在某些情况下，可能不存在匹配活动，因此请确保您保护这一点。
    输入：没有。 输出：没有。
    */
    pub const ACTION_HEARING_DEVICE_PAIRING_SETTINGS: &'static str =
        "android.settings.HEARING_DEVICES_PAIRING_SETTINGS";

    /**
    活动操作：显示以配置输入方法的设置，特别是允许用户启用输入方法。 在某些情况下，可能不存在匹配活动，因此请确保您保护这一点。
    输入：没有。 输出：没有。
    */
    pub const ACTION_VOICE_INPUT_SETTINGS: &'static str = "android.settings.VOICE_INPUT_SETTINGS";

    /**
    活动操作：显示以配置输入方法的设置，特别是允许用户启用输入方法。 在某些情况下，可能不存在匹配活动，因此请确保您保护这一点。
    输入：没有。 输出：没有。
    */
    pub const ACTION_INPUT_METHOD_SETTINGS: &'static str = "android.settings.INPUT_METHOD_SETTINGS";

    /**
    活动动作：显示设置以启用/禁用输入方法子类型。 在某些情况下，可能不存在匹配活动，因此请确保您保护这一点。
    要确定在设置中显示哪种输入方法的子类型，请使用输入方法ID添加Extra_input_method_id extra。如果此意图中没有额外的内容，则将在设置中显示所有已安装的输入方法的子类型。
    输入：没有。 输出：没有。
    */
    pub const ACTION_INPUT_METHOD_SUBTYPE_SETTINGS: &'static str =
        "android.settings.INPUT_METHOD_SUBTYPE_SETTINGS";

    /**
    活动动作：显示设置以管理用户输入字典。 从android.os.build.version_codes＃kitkat开始，可以保证，始终将有适当地实现此意图动作。在平台的先前版本中，这是可选的，因此请确保您保护它。
    输入：没有。 输出：没有。
    */
    pub const ACTION_USER_DICTIONARY_SETTINGS: &'static str =
        "android.settings.USER_DICTIONARY_SETTINGS";
    /**
    活动动作：显示用于配置硬件键盘的设置。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。输入：无。输出：无。
    */
    pub const ACTION_HARD_KEYBOARD_SETTINGS: &'static str =
        "android.settings.HARD_KEYBOARD_SETTINGS";

    /**
    活动动作：向用户词典中添加一个单词。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    输入：一个包含应添加到词典中的单词的额外关键字。
    输出：无。
    */
    pub const ACTION_USER_DICTIONARY_INSERT: &'static str =
        "com.android.settings.USER_DICTIONARY_INSERT";

    /**
    活动动作：显示允许配置与应用相关的设置的界面。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    输入：无。输出：无。
    */
    pub const ACTION_APPLICATION_SETTINGS: &'static str = "android.settings.APPLICATION_SETTINGS";

    /**
    活动动作：显示允许配置与应用开发相关的设置的界面。从 android.os.Build.VERSION_CODES#JELLY_BEAN_MR1（即Android 4.2.2）开始，此动作是平台的一个必要部分。
    输入：无。输出：无。
    */
    pub const ACTION_APPLICATION_DEVELOPMENT_SETTINGS: &'static str =
        "android.settings.APPLICATION_DEVELOPMENT_SETTINGS";

    /**
    活动动作：显示允许配置快速启动快捷方式的设置。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    输入：无。输出：无。
    */
    pub const ACTION_QUICK_LAUNCH_SETTINGS: &'static str = "android.settings.QUICK_LAUNCH_SETTINGS";

    /**
    活动动作：显示用于管理已安装应用程序的设置。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    输入：无。输出：无。
    */
    pub const ACTION_MANAGE_APPLICATIONS_SETTINGS: &'static str =
        "android.settings.MANAGE_APPLICATIONS_SETTINGS";

    /**
    活动动作：显示用于管理所有应用程序的设置。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    输入：无。输出：无。
    */
    pub const ACTION_MANAGE_ALL_APPLICATIONS_SETTINGS: &'static str =
        "android.settings.MANAGE_ALL_APPLICATIONS_SETTINGS";

    /**
    活动动作：显示用于管理所有SIM卡的设置的界面。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    输入：无。输出：无。
    */
    pub const ACTION_MANAGE_ALL_SIM_PROFILES_SETTINGS: &'static str =
        "android.settings.MANAGE_ALL_SIM_PROFILES_SETTINGS";

    /**
    活动动作：显示用于控制哪些应用可以在其他应用上方绘制的屏幕。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    输入（可选）：在Android android.os.Build.VERSION_CODES#R之前的版本中，Intent的数据URI可以指定应用程序包名，以直接调用与该包名特定的管理GUI。例如"package:com.my.app"。
    输出：无。
    */
    pub const ACTION_MANAGE_OVERLAY_PERMISSION: &'static str =
        "android.settings.action.MANAGE_OVERLAY_PERMISSION";

    /**
    活动动作：显示用于控制意图的数据URI中指定的应用程序是否可以在其他应用程序之上绘制的屏幕。与在Android android.os.Build.VERSION_CODES#R中无法用于为特定软件包显示图形用户界面(GUI)的ACTION_MANAGE_OVERLAY_PERMISSION不同，启动具有此意图的活动需要 `权限名称` 权限。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    输入：Intent的数据URI必须指定要控制其在其他应用程序之上绘制能力的应用程序包名。例如"package:com.my.app"。
    输出：无。
    */
    pub const ACTION_MANAGE_APP_OVERLAY_PERMISSION: &'static str =
        "android.settings.MANAGE_APP_OVERLAY_PERMISSION";

    /**
    活动动作：显示用于控制哪些应用程序被允许写入/修改系统设置的屏幕。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    输入：可选地，Intent的数据URI可以指定应用程序包名，以直接调用与该包名特定的管理GUI。例如"package:com.my.app"。
    输出：无。
    */
    pub const ACTION_MANAGE_WRITE_SETTINGS: &'static str =
        "android.settings.action.MANAGE_WRITE_SETTINGS";

    /**
    活动动作：显示用于控制应用程序使用属性的屏幕。
    输入：Intent的额外信息android.content.Intent#EXTRA_PACKAGE_NAME必须指定应用程序包名。
    输出：无。
    */
    pub const ACTION_APP_USAGE_SETTINGS: &'static str =
        "android.settings.action.APP_USAGE_SETTINGS";

    /**
    活动动作：显示关于特定应用程序的详细信息的屏幕。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    输入：Intent的数据URI使用"package"方案指定要显示的应用程序包名。即"package:com.my.app"。
    输出：无。
    */
    pub const ACTION_APPLICATION_DETAILS_SETTINGS: &'static str =
        "android.settings.APPLICATION_DETAILS_SETTINGS";

    /**
    活动动作：显示一直在运行前景服务的应用程序列表（对用户“在后台运行”）。
    输入：Extras“ packages”是软件包名称的字符串数组。
    输出：没有。
    */
    pub const ACTION_FOREGROUND_SERVICES_SETTINGS: &'static str =
        "android.settings.FOREGROUND_SERVICES_SETTINGS";

    /**
    活动动作：显示控制哪些应用程序可以忽略电池优化的屏幕。
    输入：没有。 输出：没有。
    您可以使用android.os.PowerManager#isIgnoringBatteryOptimizations PowerManager.isIgnoringBatteryOptimization()来确定应用程序是否已经忽略了优化。 您可以使用ACTION_REQUEST_IGNORE_BATTERY_OPTIMIZATIONS要求用户将您列入此列表。
    */
    pub const ACTION_IGNORE_BATTERY_OPTIMIZATION_SETTINGS: &'static str =
        "android.settings.IGNORE_BATTERY_OPTIMIZATION_SETTINGS";

    /**
    活动动作：请求用户允许应用忽略电池优化（即，将其添加到ACTION_IGNORE_BATTERY_OPTIMIZATION_SETTINGS显示的允许列表中的应用中）。要使用此功能，应用还必须持有android.Manifest.permission#REQUEST_IGNORE_BATTERY_OPTIMIZATIONS权限。
    注意：大多数应用不应使用此功能；平台为应用在各种省电模式下正确运行提供了许多功能。这仅适用于需要深度控制自身执行（可能以牺牲用户电池寿命为代价）的非常规应用。请注意，这些应用极大地增加了在用户设备上显示为高耗电应用的风险。
    输入：Intent的数据URI必须使用“package”模式指定要显示的应用包名。即“package:com.my.app”。
    输出：无。
    您可以使用android.os.PowerManager#isIgnoringBatteryOptimizations（PowerManager.isIgnoringBatteryOptimizations()）来确定一个应用是否已忽略优化。
    */
    pub const ACTION_REQUEST_IGNORE_BATTERY_OPTIMIZATIONS: &'static str =
        "android.settings.REQUEST_IGNORE_BATTERY_OPTIMIZATIONS";

    /**
    活动动作：打开关联应用的高级电量使用详情页面。
    输入：Intent的数据URI使用“package”模式设置应用名称（如“package:com.my.app”）。
    输出：无。
    */
    pub const ACTION_VIEW_ADVANCED_POWER_USAGE_DETAIL: &'static str =
        "android.settings.VIEW_ADVANCED_POWER_USAGE_DETAIL";

    /**
    活动动作：显示用于控制特定应用程序后台数据限制的屏幕。
    输入：使用“package”模式（如“package:com.my.app”）设置包含应用程序名称的Intent的数据URI。
    输出：无。
    应用程序还可以使用android.net.ConnectivityManager#getRestrictBackgroundStatus()方法来确定它们自己的后台数据限制状态。在某些情况下，可能不存在匹配的活动，因此请确保您对此进行了防护。
    */
    pub const ACTION_IGNORE_BACKGROUND_DATA_RESTRICTIONS_SETTINGS: &'static str =
        "android.settings.IGNORE_BACKGROUND_DATA_RESTRICTIONS_SETTINGS";

    /**
    活动动作：显示“应用程序操作”设置屏幕。
    输入：无。输出：无。
    */
    pub const ACTION_APP_OPS_SETTINGS: &'static str = "android.settings.APP_OPS_SETTINGS";

    /**
    活动动作：显示系统更新功能的设置屏幕。在某些情况下，可能不存在匹配的活动，因此请确保您对此进行了防护。
    输入：无。输出：无。
    */
    pub const ACTION_SYSTEM_UPDATE_SETTINGS: &'static str =
        "android.settings.SYSTEM_UPDATE_SETTINGS";

    /**
    活动动作：显示受管理用户配置的设置。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    输入：无。输出：无。
    */
    pub const ACTION_MANAGED_PROFILE_SETTINGS: &'static str =
        "android.settings.MANAGED_PROFILE_SETTINGS";

    /**
    活动动作：显示允许配置同步设置的设置。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    通过添加账户按钮可添加的账户类型可能会受到限制，方法是在此Intent中添加一个EXTRA_AUTHORITIES附加数据，包含一个或多个可同步的内容提供者的权限。仅向用户提供能够与该内容提供者同步的账户类型。
    输入：无。输出：无。
    */
    pub const ACTION_SYNC_SETTINGS: &'static str = "android.settings.SYNC_SETTINGS";

    /**
    活动动作：显示添加账户屏幕以创建新账户。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    可添加的账户类型可能会受到限制，方法是在Intent中添加一个EXTRA_AUTHORITIES附加数据，包含一个或多个可同步的内容提供者的权限。仅向用户提供能够与该内容提供者同步的账户类型。还可以通过在Intent中添加一个EXTRA_ACCOUNT_TYPES附加数据，包含一个或多个账户类型来过滤账户类型。
    输入：无。输出：无。
    */
    pub const ACTION_ADD_ACCOUNT: &'static str = "android.settings.ADD_ACCOUNT_SETTINGS";

    /**
    Activity Action：显示用于启用或禁用数据节省程序的设置。在某些情况下，可能不存在匹配的活动，因此请确保您对此进行了防护。
    输入：无。输出：无。
    */
    pub const ACTION_DATA_SAVER_SETTINGS: &'static str = "android.settings.DATA_SAVER_SETTINGS";

    /**
    Activity Action：显示用于选择网络运营商的设置。在某些情况下，可能不存在匹配的活动，因此请确保您对此进行了防护。
    可以通过EXTRA_SUB_ID（订阅ID）可选地指定应显示可用网络运营商的订阅，该订阅ID用于指定哪个订阅的可用网络运营商应被显示。
    输入：无。输出：无。
    */
    pub const ACTION_NETWORK_OPERATOR_SETTINGS: &'static str =
        "android.settings.NETWORK_OPERATOR_SETTINGS";

    /**
    Activity Action：显示用于选择网络提供商的设置。在某些情况下，可能不提供匹配的活动，因此请确保您对此进行了防护。
    可以通过“设置”应用程序自定义对此偏好的访问。
    输入：无。输出：无。
    */
    pub const ACTION_NETWORK_PROVIDER_SETTINGS: &'static str =
        "android.settings.NETWORK_PROVIDER_SETTINGS";

    /**
    Activity Action：显示用于选择2G/3G的设置。在某些情况下，可能不存在匹配的活动，因此请确保您对此进行了防护。
    输入：无。输出：无。
    */
    pub const ACTION_DATA_ROAMING_SETTINGS: &'static str = "android.settings.DATA_ROAMING_SETTINGS";

    /**
    活动动作：显示内部存储的设置。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    输入：无。输出：无。
    */
    pub const ACTION_INTERNAL_STORAGE_SETTINGS: &'static str =
        "android.settings.INTERNAL_STORAGE_SETTINGS";

    /**
    活动动作：显示存储卡存储的设置。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    输入：无。输出：无。
    */
    pub const ACTION_MEMORY_CARD_SETTINGS: &'static str = "android.settings.MEMORY_CARD_SETTINGS";

    /**
    活动动作：显示全局搜索的设置。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    输入：无。输出：无。
    */
    pub const ACTION_SEARCH_SETTINGS: &'static str = "android.search.action.SEARCH_SETTINGS";

    /**
    活动动作：显示设备的一般信息设置（序列号、软件版本、电话号码等）。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    输入：无。输出：无。
    */
    pub const ACTION_DEVICE_INFO_SETTINGS: &'static str = "android.settings.DEVICE_INFO_SETTINGS";

    /**
    活动动作：显示NFC设置。这会显示一个用户界面，允许用户开启或关闭NFC。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    输入：无。输出：无。
    */
    pub const ACTION_NFC_SETTINGS: &'static str = "android.settings.NFC_SETTINGS";

    //noinspection SpellCheckingInspection
    /**
    活动动作：显示NFC共享设置。这会显示一个用户界面，允许用户开启或关闭NDEF推送（Android Beam）。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    输入：无。输出：无。
    */
    pub const ACTION_NFCSHARING_SETTINGS: &'static str = "android.settings.NFCSHARING_SETTINGS";

    /**
    活动动作：显示NFC轻触支付设置。这会显示一个用户界面，允许用户配置轻触支付（Tap&Pay）设置。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    输入：无。输出：无。
    */
    pub const ACTION_NFC_PAYMENT_SETTINGS: &'static str = "android.settings.NFC_PAYMENT_SETTINGS";

    /**
    活动动作：显示Daydream设置。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    输入：无。输出：无。
    */
    pub const ACTION_DREAM_SETTINGS: &'static str = "android.settings.DREAM_SETTINGS";

    /**
    活动动作：显示公共设置。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    输入：无。输出：无。
    */
    pub const ACTION_COMMUNAL_SETTING: &'static str = "android.settings.COMMUNAL_SETTINGS";

    /**
    活动动作：显示通知助手设置。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    输入：无。输出：无。
    */
    pub const ACTION_NOTIFICATION_ASSISTANT_SETTINGS: &'static str =
        "android.settings.NOTIFICATION_ASSISTANT_SETTINGS";

    /**
    活动动作：显示通知监听器设置。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    输入：无。输出：无。
    */
    pub const ACTION_NOTIFICATION_LISTENER_SETTINGS: &'static str =
        "android.settings.ACTION_NOTIFICATION_LISTENER_SETTINGS";

    /**
    活动动作：显示应用程序的通知监听器权限设置页面。用户可以在此处授予或拒绝组件名称访问通知的权限。有关更多详细信息，请参阅android.app.NotificationManager#isNotificationListenerAccessGranted(ComponentName)。
    输入：包含要授予或撤销通知监听器访问权限的组件名称的EXTRA_NOTIFICATION_LISTENER_COMPONENT_NAME额外字段。
    输出：无。
    */
    pub const ACTION_NOTIFICATION_LISTENER_DETAIL_SETTINGS: &'static str =
        "android.settings.NOTIFICATION_LISTENER_DETAIL_SETTINGS";

    /**
    活动额外字段：显示哪个组件名称的通知监听器权限页面。一个包含ComponentName的字符串额外字段。这必须作为ACTION_NOTIFICATION_LISTENER_DETAIL_SETTINGS的额外字段传递。
    */
    pub const EXTRA_NOTIFICATION_LISTENER_COMPONENT_NAME: &'static str =
        "android.provider.extra.NOTIFICATION_LISTENER_COMPONENT_NAME";

    /**
    活动动作：显示不要干扰访问设置。 用户可以从这里授予并拒绝访问不要打扰配置。托管配置文件不能授予不要干扰访问。有关更多详细信息，请参见android.app.NotificationManager＃isNotificationPolicyAccessGranted()。
    输入：没有。 输出：没有。
    在某些情况下，可能不存在匹配活动，因此请确保您保护这一点。
    */
    pub const ACTION_NOTIFICATION_POLICY_ACCESS_SETTINGS: &'static str =
        "android.settings.NOTIFICATION_POLICY_ACCESS_SETTINGS";

    /**
    活动动作：显示应用的勿扰设置页面。用户可以在此授予或拒绝应用访问勿扰配置。更多详情，请参阅android.app.NotificationManager#isNotificationPolicyAccessGranted()。
    输入：使用“package”架构（如“package:com.my.app”）设置Intent的数据URI，并附带应用名称。
    输出：无。
    */
    pub const ACTION_NOTIFICATION_POLICY_ACCESS_DETAIL_SETTINGS: &'static str =
        "android.settings.NOTIFICATION_POLICY_ACCESS_DETAIL_SETTINGS";

    /**
    活动动作：显示自动勿扰规则列表页面。用户可以在此屏幕添加、启用、禁用和删除自动勿扰规则。更多详情，请参阅`NotificationManager#addAutomaticZenRule(AutomaticZenRule)`。
    输入：无。输出：无。
    */
    pub const ACTION_CONDITION_PROVIDER_SETTINGS: &'static str =
        "android.settings.ACTION_CONDITION_PROVIDER_SETTINGS";

    /**
    活动动作：显示AutomaticZenRule模式的设置页面。用户可以在此更改模式激活时的行为，并访问拥有该模式的应用的附加配置屏幕，在该屏幕中可以修改触发条件（请参阅`AutomaticZenRule#setConfigurationActivity(ComponentName)`）。如果`NotificationManager#areAutomaticZenRulesUserManaged()`返回true，才会找到匹配的活动。
    输入：规则的ID，通过`EXTRA_AUTOMATIC_ZEN_RULE_ID`附加数据提供。
    输出：无。
    */
    pub const ACTION_AUTOMATIC_ZEN_RULE_SETTINGS: &'static str =
        "android.settings.AUTOMATIC_ZEN_RULE_SETTINGS";

    /**
    活动额外信息：要显示的自动禅定模式（AutomaticZenRule）设置的字符串ID。这必须作为额外字段传递给ACTION_AUTOMATIC_ZEN_RULE_SETTINGS动作。
    */
    pub const EXTRA_AUTOMATIC_ZEN_RULE_ID: &'static str =
        "android.provider.extra.AUTOMATIC_ZEN_RULE_ID";

    /**
    活动动作：显示视频字幕设置。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    输入：无。输出：无。
    */
    pub const ACTION_CAPTIONING_SETTINGS: &'static str = "android.settings.CAPTIONING_SETTINGS";

    /**
    活动动作：显示顶级打印设置。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    输入：无。输出：无。
    */
    pub const ACTION_PRINT_SETTINGS: &'static str = "android.settings.ACTION_PRINT_SETTINGS";

    /**
    活动动作：显示禅定模式配置设置。
    */

    pub const ACTION_ZEN_MODE_SETTINGS: &'static str = "android.settings.ZEN_MODE_SETTINGS";
    /**
    活动动作：显示禅模式视觉效果配置设置。
    */
    pub const ZEN_MODE_BLOCKED_EFFECTS_SETTINGS: &'static str =
        "android.settings.ZEN_MODE_BLOCKED_EFFECTS_SETTINGS";

    /**
    活动动作：显示禅模式引导活动。
    */
    pub const ZEN_MODE_ONBOARDING: &'static str = "android.settings.ZEN_MODE_ONBOARDING";

    /**
    活动动作：显示禅模式（又名勿扰模式）优先级配置设置。
    */
    pub const ACTION_ZEN_MODE_PRIORITY_SETTINGS: &'static str =
        "android.settings.ZEN_MODE_PRIORITY_SETTINGS";

    /**
    活动动作：显示禅模式自动化配置设置。
    */
    pub const ACTION_ZEN_MODE_AUTOMATION_SETTINGS: &'static str =
        "android.settings.ZEN_MODE_AUTOMATION_SETTINGS";

    /**
    活动动作：修改勿扰模式设置。在某些情况下，可能不存在匹配的活动，因此请确保您对此进行了防护。
    这个意图必须通过android.service.voice.VoiceInteractionSession#startVoiceActivity startVoiceActivity启动。注意：实现此意图的活动在修改设置之前，必须验证android.app.Activity#isVoiceInteraction isVoiceInteraction返回true。
    输入：可选的EXTRA_DO_NOT_DISTURB_MODE_MINUTES附加数据可用于指示用户希望避免打扰的时长。可选的EXTRA_DO_NOT_DISTURB_MODE_ENABLED附加数据可用于指示用户是启用还是禁用勿扰模式。如果未包含这两个附加数据中的任何一个，系统可能会提示用户提供值。
    输出：无。
    */
    pub const ACTION_VOICE_CONTROL_DO_NOT_DISTURB_MODE: &'static str =
        "android.settings.VOICE_CONTROL_DO_NOT_DISTURB_MODE";

    /**
    活动动作：显示禅模式（勿扰模式）计划规则配置设置。
    */
    pub const ACTION_ZEN_MODE_SCHEDULE_RULE_SETTINGS: &'static str =
        "android.settings.ZEN_MODE_SCHEDULE_RULE_SETTINGS";

    /**
    活动动作：显示禅模式（勿扰模式）事件规则配置设置。
    */
    pub const ACTION_ZEN_MODE_EVENT_RULE_SETTINGS: &'static str =
        "android.settings.ZEN_MODE_EVENT_RULE_SETTINGS";

    /**
    活动动作：显示禅模式外部规则配置设置。
    */
    pub const ACTION_ZEN_MODE_EXTERNAL_RULE_SETTINGS: &'static str =
        "android.settings.ZEN_MODE_EXTERNAL_RULE_SETTINGS";

    /**
    活动动作：显示设备的监管信息屏幕。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    输入：无。输出：无。
    */
    pub const ACTION_SHOW_REGULATORY_INFO: &'static str = "android.settings.SHOW_REGULATORY_INFO";

    /**
    活动动作：显示设备名称设置。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    */
    pub const DEVICE_NAME_SETTINGS: &'static str = "android.settings.DEVICE_NAME";

    /**
    活动动作：显示配对设置。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    */
    pub const ACTION_PAIRING_SETTINGS: &'static str = "android.settings.PAIRING_SETTINGS";

    /**
    活动动作：显示电池节能设置。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    */
    pub const ACTION_BATTERY_SAVER_SETTINGS: &'static str =
        "android.settings.BATTERY_SAVER_SETTINGS";

    /**
    活动动作：使用语音命令修改省电模式设置。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    必须使用android.service.voice.VoiceInteractionSession#startVoiceActivity启动此意图。注意：实现此意图的活动在修改设置之前，必须验证android.app.Activity#isVoiceInteraction返回true。
    输入：为了指定省电模式应设置为哪个状态，请将此意图添加EXTRA_BATTERY_SAVER_MODE_ENABLED附加数据，并指定状态。如果不包含此附加数据，则不会进行任何更改。
    输出：无。
    */
    pub const ACTION_VOICE_CONTROL_BATTERY_SAVER_MODE: &'static str =
        "android.settings.VOICE_CONTROL_BATTERY_SAVER_MODE";

    /**
    活动动作：显示主页选择设置。如果存在多个可以满足`Intent#CATEGORY_HOME`意图的活动，此屏幕允许您选择首选活动。
    */
    pub const ACTION_HOME_SETTINGS: &'static str = "android.settings.HOME_SETTINGS";

    /**
    活动动作：显示默认应用设置。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    输入：无。输出：无。
    */
    pub const ACTION_MANAGE_DEFAULT_APPS_SETTINGS: &'static str =
        "android.settings.MANAGE_DEFAULT_APPS_SETTINGS";

    /**
    活动动作：显示更多默认应用设置。如果某个设置活动处理此意图动作，将在“默认应用”设置中显示一个“更多默认项”条目，点击它将启动该活动。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    输入：无。输出：无。
    */
    pub const ACTION_MANAGE_MORE_DEFAULT_APPS_SETTINGS: &'static str =
        "android.settings.MANAGE_MORE_DEFAULT_APPS_SETTINGS";

    /**
    活动动作：显示应用屏幕尺寸列表设置，以便用户覆盖应用的宽高比。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    可以包含以下额外的参数`android.content.Intent#EXTRA_PACKAGE_NAME`，指定要滚动到的页面上的包名。
    */
    pub const ACTION_MANAGE_USER_ASPECT_RATIO_SETTINGS: &'static str =
        "android.settings.MANAGE_USER_ASPECT_RATIO_SETTINGS";

    /**
    活动动作：显示通知设置。
    */
    pub const ACTION_NOTIFICATION_SETTINGS: &'static str = "android.settings.NOTIFICATION_SETTINGS";

    /**
    活动动作：显示对话设置。
    */
    pub const ACTION_CONVERSATION_SETTINGS: &'static str = "android.settings.CONVERSATION_SETTINGS";

    /**
    活动动作：显示通知历史记录屏幕。
    */
    pub const ACTION_NOTIFICATION_HISTORY: &'static str = "android.settings.NOTIFICATION_HISTORY";

    /**
    活动动作：显示应用列表设置，按发送通知的应用进行筛选。
    */
    pub const ACTION_ALL_APPS_NOTIFICATION_SETTINGS: &'static str =
        "android.settings.ALL_APPS_NOTIFICATION_SETTINGS";

    /**
    活动动作：专门用于显示应用的通知设置，特别是用于审核通知的应用。与ALL_APPS_NOTIFICATION_SETTINGS相同，但旨在内部使用。
    */
    pub const ACTION_ALL_APPS_NOTIFICATION_SETTINGS_FOR_REVIEW: &'static str =
        "android.settings.ALL_APPS_NOTIFICATION_SETTINGS_FOR_REVIEW";

    /**
    活动动作：显示单个应用的通知设置。
    输入参数：EXTRA_APP_PACKAGE，表示要显示的应用包名。无输出。
    */
    pub const ACTION_APP_NOTIFICATION_SETTINGS: &'static str =
        "android.settings.APP_NOTIFICATION_SETTINGS";
    /**
    活动动作：显示单个通知频道的通知设置。输入参数：EXTRA_APP_PACKAGE，包含要显示频道的软件包；EXTRA_CHANNEL_ID，要显示频道的ID。无输出。
    */
    pub const ACTION_CHANNEL_NOTIFICATION_SETTINGS: &'static str =
        "android.settings.CHANNEL_NOTIFICATION_SETTINGS";

    /**
    活动动作：显示单个应用的通知气泡设置。参见NotificationManager#getBubblePreference()。
    输入参数：EXTRA_APP_PACKAGE，要显示的应用包。无输出。
    */
    pub const ACTION_APP_NOTIFICATION_BUBBLE_SETTINGS: &'static str =
        "android.settings.APP_NOTIFICATION_BUBBLE_SETTINGS";

    //noinspection SpellCheckingInspection
    /**
    Intent额外参数：对于记录物理键盘设置入口点的设置指标，该值是android.app.settings.SettingsEnums#EntryPointType的值。此参数必须作为额外字段传递给ACTION_HARD_KEYBOARD_SETTINGS动作。
    */
    pub const EXTRA_ENTRYPOINT: &'static str = "com.android.settings.inputmethod.EXTRA_ENTRYPOINT";

    /**
    活动额外参数：要显示的通知频道设置的软件包所有者。此参数必须作为额外字段传递给ACTION_CHANNEL_NOTIFICATION_SETTINGS动作。
    */
    pub const EXTRA_APP_PACKAGE: &'static str = "android.provider.extra.APP_PACKAGE";

    /**
    活动额外参数：要显示的通知频道设置的NotificationChannel#getId()值。此参数必须作为额外字段传递给ACTION_CHANNEL_NOTIFICATION_SETTINGS动作。
    */
    pub const EXTRA_CHANNEL_ID: &'static str = "android.provider.extra.CHANNEL_ID";

    /**
    活动附加信息：用于显示通知对话设置的NotificationChannel#getConversationId()。这是ACTION_CHANNEL_NOTIFICATION_SETTINGS的一个可选附加字段。如果包含此字段，系统将首先根据通道和对话ID查找通知设置，如果此对话没有专用的通道，则回退到通道ID，类似于NotificationManager#getNotificationChannel(String, String)。
    */
    pub const EXTRA_CONVERSATION_ID: &'static str = "android.provider.extra.CONVERSATION_ID";

    /**
    活动附加信息：要在设置UI上显示的`NotificationChannel`字段名称的列表。
    这是`ACTION_CHANNEL_NOTIFICATION_SETTINGS`的一个可选附加字段。如果包含此字段，系统将过滤掉任何不在此列表中但本应显示的设置。
    */
    pub const EXTRA_CHANNEL_FILTER_LIST: &'static str =
        "android.provider.extra.CHANNEL_FILTER_LIST";

    /**
    活动动作：显示通知编辑设置。
    */
    pub const ACTION_APP_NOTIFICATION_REDACTION: &'static str =
        "android.settings.ACTION_APP_NOTIFICATION_REDACTION";

    #[doc(hidden)]
    pub const EXTRA_APP_UID: &'static str = "app_uid";

    /**
    活动动作：显示电源菜单设置。
    */
    pub const ACTION_POWER_MENU_SETTINGS: &'static str =
        "android.settings.ACTION_POWER_MENU_SETTINGS";

    /**
    活动动作：显示控件设置。
    */
    pub const ACTION_DEVICE_CONTROLS_SETTINGS: &'static str =
        "android.settings.ACTION_DEVICE_CONTROLS_SETTINGS";

    /**
    活动动作：显示媒体控制设置
    */
    pub const ACTION_MEDIA_CONTROLS_SETTINGS: &'static str =
        "android.settings.ACTION_MEDIA_CONTROLS_SETTINGS";

    /**
    活动动作：显示一个包含策略禁用消息的对话框。如果用户的某个操作被策略禁用，可以触发此对话框通知用户。
    输入：Intent#EXTRA_USER：管理员的用户。
    输出：无。
    */
    // Intent#EXTRA_USER_ID 也可以使用
    pub const ACTION_SHOW_ADMIN_SUPPORT_DETAILS: &'static str =
        "android.settings.SHOW_ADMIN_SUPPORT_DETAILS";

    /**
    Intent 附加数据：被监管者限制的设置项的ID。类型：整数，其值来自下面的 SUPERVISOR_VERIFICATION_* 常量之一。
    SUPERVISOR_VERIFICATION_SETTING_UNKNOWN SUPERVISOR_VERIFICATION_SETTING_BIOMETRICS
    */
    pub const EXTRA_SUPERVISOR_RESTRICTED_SETTING_KEY: &'static str =
        "android.provider.extra.SUPERVISOR_RESTRICTED_SETTING_KEY";

    /**
    未知设置通常可以忽略，用于与未来的监管者设置保持兼容。
    */
    pub const SUPERVISOR_VERIFICATION_SETTING_UNKNOWN: i32 = 0;

    /**
    监管可以在设备上使用什么样的生物识别传感器，例如面部和指纹扫描仪。
    */
    pub const SUPERVISOR_VERIFICATION_SETTING_BIOMETRICS: i32 = 1;

    /**
    活动动作：启动用于管理受监管者限制的设置项的UI界面。
    输入：EXTRA_SUPERVISOR_RESTRICTED_SETTING_KEY指定要打开的设置项。
    输出：无。
    */
    pub const ACTION_MANAGE_SUPERVISOR_RESTRICTED_SETTING: &'static str =
        "android.settings.MANAGE_SUPERVISOR_RESTRICTED_SETTING";

    /**
    活动动作：显示远程错误报告流程的对话框。
    输入：无。输出：无。
    */
    pub const ACTION_SHOW_REMOTE_BUGREPORT_DIALOG: &'static str =
        "android.settings.SHOW_REMOTE_BUGREPORT_DIALOG";

    /**
    活动动作：显示VR监听器设置。
    输入：无。输出：无。
    */
    pub const ACTION_VR_LISTENER_SETTINGS: &'static str = "android.settings.VR_LISTENER_SETTINGS";

    /**
    活动动作：显示画中画设置。
    输入：无。输出：无。
    */
    pub const ACTION_PICTURE_IN_PICTURE_SETTINGS: &'static str =
        "android.settings.PICTURE_IN_PICTURE_SETTINGS";

    /**
    活动动作：显示存储管理器设置。
    输入：无。输出：无。
    */
    pub const ACTION_STORAGE_MANAGER_SETTINGS: &'static str =
        "android.settings.STORAGE_MANAGER_SETTINGS";

    /**
    活动操作：允许用户选择当前的 webview 实现。
    输入：无。输出：无。
    在某些情况下，匹配的活动可能不存在，因此请确保您采取防范措施。
    */
    pub const ACTION_WEBVIEW_SETTINGS: &'static str = "android.settings.WEBVIEW_SETTINGS";

    /**
    活动操作：显示企业隐私部分。
    输入：无。输出：无。
    */
    pub const ACTION_ENTERPRISE_PRIVACY_SETTINGS: &'static str =
        "android.settings.ENTERPRISE_PRIVACY_SETTINGS";

    /**
    Activity Action: 显示工作策略信息。DPC（设备管理控制器）应用可以实现一个处理此意图的活动，以显示与工作配置文件或托管设备相关联的设备策略。
    输入：无。输出：无。
    */
    pub const ACTION_SHOW_WORK_POLICY_INFO: &'static str = "android.settings.SHOW_WORK_POLICY_INFO";

    /**
    Activity Action: 显示允许用户选择其自动填充服务的屏幕。
    输入：使用“package”模式（如“package:com.my.app”）设置Intent的数据URI，其中包含应用程序名称。
    输出：如果用户选择了属于调用者包的自动填充服务，则返回android.app.Activity#RESULT_OK。
    注意：应用应调用android.view.autofill.AutofillManager#hasEnabledAutofillServices()和android.view.autofill.AutofillManager#isAutofillSupported()，并且只有当它们分别返回true时，才应使用此操作启动活动。
    */
    pub const ACTION_REQUEST_SET_AUTOFILL_SERVICE: &'static str =
        "android.settings.REQUEST_SET_AUTOFILL_SERVICE";

    /**
    Activity Action: 显示允许用户启用凭据管理器提供程序的屏幕。
    输入：使用“package”模式（如“package:com.my.app”）设置Intent的数据URI，其中包含应用程序名称。
    输出：如果用户选择了属于调用者包的提供程序，则返回android.app.Activity#RESULT_OK。
    注意：应用应调用android.credentials.CredentialManager#isEnabledCredentialProviderService(ComponentName)，并且只有当它返回true时，才应使用此操作启动活动。
    */
    pub const ACTION_CREDENTIAL_PROVIDER: &'static str = "android.settings.CREDENTIAL_PROVIDER";

    /**
    Activity Action: 显示用于控制快速访问钱包的屏幕。在某些情况下，可能不存在匹配的活动，因此请确保对此进行保护。
    输入：无。输出：无。
    */
    pub const ACTION_QUICK_ACCESS_WALLET_SETTINGS: &'static str =
        "android.settings.QUICK_ACCESS_WALLET_SETTINGS";

    /**
    活动操作：显示用于控制哪些应用有权访问卷目录的屏幕。
    输入：无。输出：无。
    应用通常使用此操作要求用户恢复由 android.os.storage.StorageVolume#createAccessIntent(String) 发出的目录访问请求的“不再询问”状态。
    */
    #[deprecated(note = "使用 ACTION_APPLICATION_DETAILS_SETTINGS 管理特定应用程序的存储权限")]
    pub const ACTION_STORAGE_VOLUME_ACCESS_SETTINGS: &'static str =
        "android.settings.STORAGE_VOLUME_ACCESS_SETTINGS";

    /**
    活动动作：显示允许用户选择启用（或禁用）内容捕获的屏幕。
    输入：无。输出：无。
    */
    pub const ACTION_REQUEST_ENABLE_CONTENT_CAPTURE: &'static str =
        "android.settings.REQUEST_ENABLE_CONTENT_CAPTURE";

    /**
    活动动作：显示允许用户管理Android如何处理URL解析的屏幕。
    输入：无。输出：无。
    */
    pub const ACTION_MANAGE_DOMAIN_URLS: &'static str = "android.settings.MANAGE_DOMAIN_URLS";

    /**
    活动动作：显示允许用户选择启用（或禁用）网络共享的屏幕。
    输入：无。输出：无。
    */
    pub const ACTION_TETHER_SETTINGS: &'static str = "android.settings.TETHER_SETTINGS";

    /**
    活动动作：显示允许用户配置Wi-Fi网络共享的屏幕。在某些情况下，可能不存在匹配的活动，因此请确保对此进行防护。
    输入：无。输出：无。
    */
    pub const ACTION_WIFI_TETHER_SETTING: &'static str =
        "com.android.settings.WIFI_TETHER_SETTINGS";

    /**
    广播，用于触发通知用户启用MMS的请求。需要指定EXTRA_ENABLE_MMS_DATA_REQUEST_REASON和EXTRA_SUB_ID。
    */
    pub const ACTION_ENABLE_MMS_DATA_REQUEST: &'static str =
        "android.settings.ENABLE_MMS_DATA_REQUEST";

    /**
    当设置被阻止时，显示受限设置对话框。
    */
    pub const ACTION_SHOW_RESTRICTED_SETTING_DIALOG: &'static str =
        "android.settings.SHOW_RESTRICTED_SETTING_DIALOG";

    /**
    指定触发启用MMS数据通知的原因的整数值。这必须作为ACTION_ENABLE_MMS_DATA_REQUEST的额外字段传递。该额外字段的值来自EnableMmsDataReason接口。
    */
    pub const EXTRA_ENABLE_MMS_DATA_REQUEST_REASON: &'static str =
        "android.settings.extra.ENABLE_MMS_DATA_REQUEST_REASON";

    /**
    请求启用MMS数据，因为有传入的MMS。
    */
    pub const ENABLE_MMS_DATA_REQUEST_REASON_INCOMING_MMS: i32 = 0;

    /**
    请求启用MMS数据，因为用户正在发送MMS。
    */
    pub const ENABLE_MMS_DATA_REQUEST_REASON_OUTGOING_MMS: i32 = 1;

    /**
    活动动作：显示蜂窝订阅的屏幕，并高亮显示“启用MMS”开关。
    输入：EXTRA_SUB_ID：订阅的Sub ID。
    输出：无。
    */
    pub const ACTION_MMS_MESSAGE_SETTING: &'static str = "android.settings.MMS_MESSAGE_SETTING";

    /**
    活动动作：显示由Wellbeing应用程序提供的就寝时间设置的屏幕。这种意图行动的处理程序可能不存在。
    为了以这种意图开始活动，应用程序应与此操作一起在意图中明确设置福利软件包。福利包在``中定义。
    输出：没有
    */
    pub const ACTION_BEDTIME_SETTINGS: &'static str = "android.settings.BEDTIME_SETTINGS";

    /**
    活动动作：启动用于管理应用权限的用户界面（UI）。
    输入：android.content.Intent#EXTRA_PACKAGE_NAME 指定了将要由启动的UI管理的权限所属的应用包。
    输出：无。
    */
    pub const ACTION_APP_PERMISSIONS_SETTINGS: &'static str =
        "android.settings.APP_PERMISSIONS_SETTINGS";

    // 设置（Settings）的Intent动作结束

    /**
    - SettingsProvider上的私有call()方法，用于从`system`表中读取数据。
    */
    pub const CALL_METHOD_GET_SYSTEM: &'static str = "GET_system";

    /**
    - SettingsProvider上的私有call()方法，用于从`secure`表中读取数据。
    */
    pub const CALL_METHOD_GET_SECURE: &'static str = "GET_secure";

    /**
    - SettingsProvider上的私有call()方法，用于从`global`表中读取数据。
    */
    pub const CALL_METHOD_GET_GLOBAL: &'static str = "GET_global";

    /**
    - 在SettingsProvider上的私有call()方法，用于从`config`表中读取数据。
    */
    pub const CALL_METHOD_GET_CONFIG: &'static str = "GET_config";

    /**
    - 指定基于快速路径call()方法的调用者跟踪设置生成，以便在本地缓存值。如果此键在请求包中被映射为一个空字符串的额外参数，那么响应包将包含相同的键，该键被映射为一个可打包的额外参数，该参数将是android.util.MemoryIntArray类型。
    响应还将包含一个整数，该整数映射到CALL_METHOD_GENERATION_INDEX_KEY，这是客户端在数组中查找生成信息时应使用的索引。为了提高效率，如果调用者尚未拥有生成跟踪内存数组，则应请求它。
    */
    pub const CALL_METHOD_TRACK_GENERATION_KEY: &'static str = "_track_generation";

    /**
    - 键，表示在android.util.MemoryIntArray中的位置，用于查找支持表的生成ID。该值是一个整数。
    */
    pub const CALL_METHOD_GENERATION_INDEX_KEY: &'static str = "_generation_index";

    /**
    与设置表生成相关的键。其值为整数。
    */
    pub const CALL_METHOD_GENERATION_KEY: &'static str = "_generation";

    /**
    - 基于快速路径call()方法的请求中，用户句柄的额外参数。
    */
    pub const CALL_METHOD_USER_KEY: &'static str = "_user";

    /**
    - 基于快速路径call()方法的请求中，布尔值的额外参数，用于设置为默认。
    */
    pub const CALL_METHOD_MAKE_DEFAULT_KEY: &'static str = "_make_default";

    /**
    - 基于快速路径call()方法的请求中，用户句柄的额外参数，用于重置模式。
    */
    pub const CALL_METHOD_RESET_MODE_KEY: &'static str = "_reset_mode";

    /**
    - 基于快速路径call()方法的请求中，字符串类型的额外参数，用作标签。
    */
    pub const CALL_METHOD_TAG_KEY: &'static str = "_tag";

    /**
    - 基于快速路径call()方法的请求中，字符串类型的额外参数，用作前缀。
    */
    pub const CALL_METHOD_PREFIX_KEY: &'static str = "_prefix";
    /**
    - 为基于快速路径call()方法的请求添加额外的字符串参数
    */
    pub const CALL_METHOD_SYNC_DISABLED_MODE_KEY: &'static str = "_disabled_mode";

    /**
    - 为基于快速路径call()方法的请求添加额外的RemoteCallback监控回调参数
    */
    pub const CALL_METHOD_MONITOR_CALLBACK_KEY: &'static str = "_monitor_callback_key";

    /**
    - 为基于快速路径call()方法的请求添加额外的字符串参数
    */
    pub const CALL_METHOD_FLAGS_KEY: &'static str = "_flags";

    /**
    - 为基于快速路径call()方法的请求添加额外的字符串参数，该参数可通过恢复操作被覆盖
    */
    pub const CALL_METHOD_OVERRIDEABLE_BY_RESTORE_KEY: &'static str = "_overrideable_by_restore";

    /**
    - 写入`system`表的私有call()方法
    */
    pub const CALL_METHOD_PUT_SYSTEM: &'static str = "PUT_system";

    /**
    - 写入`secure`表的私有call()方法
    */
    pub const CALL_METHOD_PUT_SECURE: &'static str = "PUT_secure";

    /**
    - 写入`global`表的私有call()方法
    */
    pub const CALL_METHOD_PUT_GLOBAL: &'static str = "PUT_global";

    /**
    • 私有 call() 方法，用于向 'configuration' 表写入数据
    */
    pub const CALL_METHOD_PUT_CONFIG: &'static str = "PUT_config";

    /**
    • 私有 call() 方法，用于向 'configuration' 表写入数据和从中删除数据
    */
    pub const CALL_METHOD_SET_ALL_CONFIG: &'static str = "SET_ALL_config";

    /**
    • 私有 call() 方法，用于从 'system' 表中删除数据
    */
    pub const CALL_METHOD_DELETE_SYSTEM: &'static str = "DELETE_system";

    /**
    • 私有 call() 方法，用于从 'secure' 表中删除数据
    */
    pub const CALL_METHOD_DELETE_SECURE: &'static str = "DELETE_secure";

    /**
    • 私有 call() 方法，用于从 'global' 表中删除数据
    */
    pub const CALL_METHOD_DELETE_GLOBAL: &'static str = "DELETE_global";

    /**
    • 私有 call() 方法，用于将 'configuration' 表重置为默认值
    */
    pub const CALL_METHOD_DELETE_CONFIG: &'static str = "DELETE_config";

    /**
    • 私有 call() 方法，用于将 'system' 表重置为默认值
    */
    pub const CALL_METHOD_RESET_SYSTEM: &'static str = "RESET_system";

    /**
    • 私有 call() 方法，用于将 'secure' 表重置为默认值
    */
    pub const CALL_METHOD_RESET_SECURE: &'static str = "RESET_secure";

    /**
    - 私有call()方法，用于将`global`表重置为默认值
    */
    pub const CALL_METHOD_RESET_GLOBAL: &'static str = "RESET_global";

    /**
    - 私有call()方法，用于将`configuration`表重置为默认值
    */
    pub const CALL_METHOD_RESET_CONFIG: &'static str = "RESET_config";

    /**
    - 私有call()方法，用于查询`system`表
    */
    pub const CALL_METHOD_LIST_SYSTEM: &'static str = "LIST_system";

    /**
    - 私有call()方法，用于查询`secure`表
    */
    pub const CALL_METHOD_LIST_SECURE: &'static str = "LIST_secure";

    /**
    - 私有call()方法，用于查询`global`表
    */
    pub const CALL_METHOD_LIST_GLOBAL: &'static str = "LIST_global";

    /**
    - 私有call()方法，用于查询`configuration`表
    */
    pub const CALL_METHOD_LIST_CONFIG: &'static str = "LIST_config";

    /**
    - 私有call()方法，用于禁用/重新启用对`configuration`表的同步
    */
    pub const CALL_METHOD_SET_SYNC_DISABLED_MODE_CONFIG: &'static str =
        "SET_SYNC_DISABLED_MODE_config";

    /**
    - 私有call()方法，用于返回`configuration`表当前的同步禁用模式
    */
    pub const CALL_METHOD_GET_SYNC_DISABLED_MODE_CONFIG: &'static str =
        "GET_SYNC_DISABLED_MODE_config";

    /**
    - 私有 call() 方法注册“配置”表的监控回调
    */
    pub const CALL_METHOD_REGISTER_MONITOR_CALLBACK_CONFIG: &'static str =
        "REGISTER_MONITOR_CALLBACK_config";

    /**
    - 私有 call() 方法，以解开“配置”表的监视回调
    */
    pub const CALL_METHOD_UNREGISTER_MONITOR_CALLBACK_CONFIG: &'static str =
        "UNREGISTER_MONITOR_CALLBACK_config";

    /**
    - 字符串参数额外到配置监视回调
    */
    pub const EXTRA_MONITOR_CALLBACK_TYPE: &'static str = "monitor_callback_type";

    /**
    • 传递给配置监控回调的额外字符串参数
    */
    pub const EXTRA_ACCESS_CALLBACK: &'static str = "access_callback";

    /**
    • 传递给配置监控回调的额外字符串参数，表示命名空间更新回调
    */
    pub const EXTRA_NAMESPACE_UPDATED_CALLBACK: &'static str = "namespace_updated_callback";

    /**
    • 传递给配置监控回调的额外字符串参数，表示命名空间
    */
    pub const EXTRA_NAMESPACE: &'static str = "namespace";

    /**
    • 传递给配置监控回调的额外字符串参数，表示调用包名
    */
    pub const EXTRA_CALLING_PACKAGE: &'static str = "calling_package";

    /**
    Activity 额外参数：基于给定的权限限制启动活动中可用的选项。这可以作为一个额外字段传递在活动意图（Activity Intent）中，包含一个或多个可同步内容提供者的权限作为字符串数组（String[]）。
    该字段被某些意图用来改变被调用活动的行为。例如：ACTION_ADD_ACCOUNT 意图根据给定的权限限制可用的账户类型。
    */
    pub const EXTRA_AUTHORITIES: &'static str = "authorities";

    /**
    Activity 额外参数：基于给定的账户类型限制启动活动中可用的选项。这可以作为一个额外字段传递在活动意图（Activity Intent）中，包含一个或多个账户类型作为字符串数组（String[]）。
    该字段被某些意图用来改变被调用活动的行为。例如：ACTION_ADD_ACCOUNT 意图将账户类型限制为指定的列表。
    */
    pub const EXTRA_ACCOUNT_TYPES: &'static str = "account_types";

    #[doc(hidden)]
    pub const EXTRA_INPUT_METHOD_ID: &'static str = "input_method_id";

    /**
    Activity 额外参数：要操作的设备标识符。这可以作为一个额外字段传递在活动意图（Activity Intent）中，包含一个单独的输入设备标识符（InputDeviceIdentifier）。
    该字段被某些活动用来直接跳转到给定设备的设置。例如：ACTION_INPUT_METHOD_SETTINGS 意图为给定设备打开键盘布局对话框。
    */
    pub const EXTRA_INPUT_DEVICE_IDENTIFIER: &'static str = "input_device_identifier";

    /**
    活动附加项：启用或禁用飞行模式。这可以作为布尔值作为额外字段传递给ACTION_VOICE_CONTROL_AIRPLANE_MODE意图，以指示是否应启用飞行模式。
    */
    pub const EXTRA_AIRPLANE_MODE_ENABLED: &'static str = "airplane_mode_enabled";

    /**
    活动附加项：启用或禁用省电模式。这可以作为布尔值作为额外字段传递给ACTION_VOICE_CONTROL_BATTERY_SAVER_MODE意图，以指示是否应启用省电模式。
    */
    pub const EXTRA_BATTERY_SAVER_MODE_ENABLED: &'static str =
        "android.settings.extra.battery_saver_mode_enabled";

    /**
    活动附加项：启用或禁用勿扰模式。这可以作为布尔值作为额外字段传递给ACTION_VOICE_CONTROL_DO_NOT_DISTURB_MODE意图，以指示是否应启用勿扰模式。
    */
    pub const EXTRA_DO_NOT_DISTURB_MODE_ENABLED: &'static str =
        "android.settings.extra.do_not_disturb_mode_enabled";

    /**
    活动附加项：勿扰模式应启用多少分钟。这可以作为额外字段传递给ACTION_VOICE_CONTROL_DO_NOT_DISTURB_MODE意图，以指示勿扰模式应启用多长时间。
    */
    pub const EXTRA_DO_NOT_DISTURB_MODE_MINUTES: &'static str =
        "android.settings.extra.do_not_disturb_mode_minutes";

    /**
    重置模式：仅重置由调用包更改的设置为默认值。如果存在默认值，则将该设置设置为默认值，否则将删除该设置。这是非系统客户端可用的唯一重置类型。
    */
    pub const RESET_MODE_PACKAGE_DEFAULTS: i32 = 1;

    /**
    重置模式：将不受信任的包（即不是系统一部分的包）设置的所有设置重置为当前默认值。如果存在默认值，则将该设置设置为默认值，否则将删除该设置。此模式仅供系统使用。
    */
    pub const RESET_MODE_UNTRUSTED_DEFAULTS: i32 = 2;

    /**
    重置模式：删除所有由不受信任的软件包（不属于系统的软件包）设置的设置。如果某个设置是由不受信任的软件包设置的，则如果系统未提供其默认值，则该设置将被删除，否则该设置将被设置为默认值。此模式仅适用于系统。
    */
    pub const RESET_MODE_UNTRUSTED_CHANGES: i32 = 3;

    /**
    重置模式：将所有设置重置为由受信任软件包指定的默认设置，该软件包是系统的一部分，并删除由不受信任的软件包设置的所有设置。如果设置具有由系统软件包设置的默认设置，则将设置为默认设置，否则将删除设置。此模式仅适用于系统。
    */
    pub const RESET_MODE_TRUSTED_DEFAULTS: i32 = 4;

    #[doc(hidden)]
    #[java_method]
    pub fn set_in_system_server() {}

    #[doc(hidden)]
    #[java_method]
    pub fn is_in_system_server() -> bool {}

    /**
    检查指定的上下文是否可以在其他应用之上绘制。从 API 级别 23 开始，除非应用在其清单中声明 android.Manifest.permission#SYSTEM_ALERT_WINDOW 权限，并且用户明确授予应用此功能，否则应用无法在其他应用之上绘制。
    要提示用户授予此批准，应用必须发送带有操作 android.provider.Settings#ACTION_MANAGE_OVERLAY_PERMISSION 的 Intent，这会导致系统显示权限管理屏幕。
    返回：如果指定的上下文可以在其他应用之上绘制，则返回 true，否则返回 false
    `context` App context.
    */
    #[java_method]
    pub fn can_draw_overlays(context: &Context) -> bool {}

    /**
    对是否允许呼叫包进行编写/修改系统设置，对PRE-M，M+和特权/预安装的应用程序的条件有所不同，对是否允许呼叫软件包进行严格的全面检查。如果所提供的UID与callingPackage不匹配，则将返回负面结果。
    */
    #[java_method]
    pub fn is_calling_package_allowed_to_write_settings(
        context: &Context,
        uid: i32,
        calling_package: String,
        throw_exception: bool,
    ) -> bool {
    }

    #[doc(hidden)]
    #[deprecated(
        note = "改用 checkAndNoteWriteSettingsOperation(Context, int, String, String, boolean)。"
    )]
    #[java_method(overload = checkAndNoteWriteSettingsOperation)]
    pub fn check_and_note_write_settings_operation_convenience(
        context: &Context,
        uid: i32,
        calling_package: String,
        throw_exception: bool,
    ) -> bool {
    }

    /**
    执行严格而全面的检查，以确定是否允许调用包写入/修改系统设置，因为 pre-M、M+ 和特权/预安装应用的条件不同。
    如果提供的 uid 与 callingPackage 不匹配，则会返回否定结果。调用者应声明 WRITE_SETTINGS 权限。
    注意：如果检查成功，此应用的操作将更新为当前时间。
    */
    #[java_method]
    pub fn check_and_note_write_settings_operation(
        context: &Context,
        uid: i32,
        calling_package: String,
        calling_attribution_tag: Option<String>,
        throw_exception: bool,
    ) -> bool {
    }

    /**
    对是否允许呼叫软件包在其他应用程序的顶部进行严格检查，因为PRE-M，M+和特权/预装应用程序的条件有所不同。如果所提供的UID与callingPackage不匹配，则将返回负面结果。
    */
    #[java_method]
    pub fn is_calling_package_allowed_to_draw_overlays(
        context: &Context,
        uid: i32,
        calling_package: String,
        throw_exception: bool,
    ) -> bool {
    }

    /**
    对是否允许呼叫软件包在其他应用程序的顶部进行严格检查，因为PRE-M，M+和特权/预装应用程序的条件有所不同。
    如果所提供的UID与callingPackage不匹配，则将返回负面结果。
    注意：如果检查成功，此应用程序的操作将更新到当前时间。
    */
    #[java_method]
    pub fn check_and_note_draw_overlays_operation(
        context: &Context,
        uid: i32,
        calling_package: String,
        calling_attribution_tag: String,
        throw_exception: bool,
    ) -> bool {
    }

    #[doc(hidden)]
    #[deprecated(
        note = "使用isCallingPackageAllowedToPerformAppOpsProtectedOperation(context, int, int, String, String, boolean, int, int, String[], boolean)。"
    )]
    #[java_method(overload = isCallingPackageAllowedToPerformAppOpsProtectedOperation)]
    pub fn is_calling_package_allowed_to_perform_app_ops_protected_operation_convenience(
        context: &Context,
        uid: i32,
        calling_package: String,
        throw_exception: bool,
        app_ops_op_code: i32,
        permissions: &[String],
        make_note: bool,
    ) -> bool {
    }

    //noinspection SpellCheckingInspection
    /**
    辅助方法用于执行一般且全面的检查，以确定受 appops 保护的操作是否可以由调用者执行。例如 OP_SYSTEM_ALERT_WINDOW 和 OP_WRITE_SETTINGS
    */
    #[java_method]
    pub fn is_calling_package_allowed_to_perform_app_ops_protected_operation(
        context: &Context,
        uid: i32,
        calling_package: String,
        calling_attribution_tag: String,
        throw_exception: bool,
        app_ops_op_code: i32,
        permissions: &[String],
        make_note: bool,
    ) -> bool {
    }

    /**
    查询给定 uid 的相应包名称。它将查询与给定 uid 关联的所有包，但只返回第零个结果。
    * 注意：如果找不到包，则返回 null。
    */
    #[java_method]
    pub fn get_package_name_for_uid(context: &Context, uid: i32) -> Option<String> {}
}

/// 测试android.provider
#[cfg(feature = "test_android_provider")]
pub fn test() {
    dbg!(Settings::is_in_system_server());
}
