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
    android::hardware::vibrator::{Effect, EffectImpl, EffectStrength, EffectStrengthImpl},
    java::lang::Runnable,
    JObjNew, JObjRef, JType,
};
use droid_wrap_derive::{java_class, java_constructor, java_method};
use std::{collections::HashSet, convert::Into, iter::Iterator, str::FromStr, string::ToString};

type ConstFn<R = String> = fn() -> R;

/**
从字符串键到各种 Parcelable 值的映射。
警告：请注意，Bundle 是一个惰性容器，因此它不实现 equals(Object) 或 hashCode()。
*/
#[java_class(name = "android/os/Bundle")]
pub struct Bundle;

impl Bundle {
    /**
    构造一个新的空 Bundle。
    */
    #[java_constructor]
    pub fn new() -> Self {}
}

/**
从设备提供对所有振动器的访问，以及以同步方式运行它们的能力。如果您的过程退出，则您开始的任何振动都将停止。
*/
#[java_class(name = "android/os/VibratorManager")]
pub struct VibratorManager;

impl VibratorManager {
    /**
    防止从框架外部进行子类化
    */
    #[java_constructor]
    pub fn new() -> Self {}

    /**
    通过 ID 检索单个振动器。
    返回：具有给定 vibratorId 的振动器，永不为空。
    `vibrator_id` 要检索的振动器的 ID。
    */
    #[java_method]
    pub fn get_vibrator(&self, vibrator_id: i32) -> Vibrator {}

    /// 返回设备的默认振动器。
    #[java_method]
    pub fn get_default_vibrator(&self) -> Vibrator {}

    /// 关掉所有振动器。
    #[java_method]
    pub fn cancel(&self) {}

    /**
    取消特定类型的正在进行的振动。
    `usage_filter` 要取消的振动类型，表示为 VibrationAttributes 的按位组合。使用值。
    */
    #[java_method(overload=cancel)]
    pub fn cancel_usage_filter(&self, usage_filter: i32) {}
}

/**
操作设备上的振动器的类。如果您的进程退出，您启动的任何振动都将停止。
*/
#[java_class(name = "android/os/Vibrator")]
pub struct Vibrator;

impl Vibrator {
    /// 振动强度：无振动。
    pub const VIBRATION_INTENSITY_OFF: i32 = 0;

    /// 振动强度：低。
    pub const VIBRATION_INTENSITY_LOW: i32 = 1;

    /// 振动强度：中。
    pub const VIBRATION_INTENSITY_MEDIUM: i32 = 2;

    /// 振动强度：高。
    pub const VIBRATION_INTENSITY_HIGH: i32 = 3;

    /// 振动效果支持：未知。
    /// 硬件未报告其支持的效果，因此我们无法确定是否支持该效果。
    pub const VIBRATION_EFFECT_SUPPORT_UNKNOWN: i32 = 0;

    /// 振动效果支持：支持。
    /// 该效果由底层硬件支持。
    pub const VIBRATION_EFFECT_SUPPORT_YES: i32 = 1;

    /// 振动效果支持：不支持。
    /// 此效果并非底层硬件原生支持，但系统仍可播放后备振动。
    pub const VIBRATION_EFFECT_SUPPORT_NO: i32 = 2;

    /// 防止从框架外部进行子类化
    #[java_constructor]
    pub fn new() -> Self {}

    /// 获取给定用途的默认振动强度。
    #[java_method]
    pub fn get_default_vibration_intensity(&self) -> i32 {}

    /**
    返回此振动器的ID。
    返回：一个非负整数，表示此服务控制的振动器的id，或者-1表示此服务未附加任何物理振动器。
    */
    #[java_method]
    pub fn get_id(&self) -> i32 {}

    /**
    检查硬件是否有振动器。
    返回：如果硬件有振动器则返回 True，否则返回 false。
    */
    #[java_method]
    pub fn has_vibrator(&self) -> bool {}

    /**
    检查振动器是否具有振幅控制。
    返回：如果硬件可以控制振动的振幅，则返回 True，否则返回 false。
    */
    #[java_method]
    pub fn has_amplitude_control(&self) -> bool {}

    /**
    检查振动器是否具有独立的频率控制。
    返回：如果硬件可以独立于振动幅度控制振动频率，则返回 True，否则返回 false。
    */
    #[java_method]
    pub fn has_frequency_control(&self) -> bool {}

    /**
    检查振动器是否支持给定 VibrationEffect 的所有组件（即振动器是否能按预期播放给定效果）。如果此方法返回 true，则 VibrationEffect 应按预期播放。 如果为 false，播放 VibrationEffect 仍可能产生振动，但振动效果可能与预期大不相同。此方法汇总功能检查方法的结果，例如 hasAmplitudeControl、areAllPrimitivesSupported(int...) 等，具体取决于 VibrationEffect 实际使用的功能。
    返回：如果振动器可以按预期播放给定效果，则返回 true，否则返回 false。
    `effect` 要检查是否支持的 VibrationEffect
    */
    #[java_method]
    pub fn are_vibration_features_supported(&self, effect: &VibrationEffect) -> bool {}

    /**
    使用 IExternalVibratorService 检查振动器是否可由外部服务控制。
    返回：如果硬件可由外部服务控制，则返回 True，否则返回 false。
    */
    #[java_method]
    pub fn has_external_control(&self) -> bool {}
    /**
    获取振动器的共振频率（如果适用）。
    返回：振动器的共振频率，如果未知、不适用或该振动器是具有不同频率的多个物理设备的组合，则返回 NaN。
    */
    #[java_method]
    pub fn get_resonant_frequency(&self) -> f32 {}

    /**
    获取振动器的 Q 因子。
    返回：振动器的 Q 因子，如果未知、不适用或该振动器是具有不同 Q 因子的多个物理设备的组合，则返回 NaN。
    */
    #[java_method]
    pub fn get_q_factor(&self) -> f32 {}

    /**
    返回振动器使用音频触觉通道可以播放的最大振幅。这是一个正值，如果未知则为 NaN。如果返回正值 maxAmplitude，则来自音轨触觉通道的信号应在 [-maxAmplitude, maxAmplitude] 范围内。
    返回：一个正值，表示设备可以从音频触觉通道播放信号的最大绝对值，如果未知则为 NaN。
    */
    #[java_method]
    pub fn get_haptic_channel_maximum_amplitude(&self) -> f32 {}

    /**
    在指定的时间段内持续振动。应用应处于前台才能发生振动。
    `milliseconds` 振动的毫秒数。
    */
    #[deprecated(note = "改用 vibrate(VibrationEffect)。")]
    #[java_method]
    pub fn vibrate(&self, milliseconds: i64) {}

    /**
    使用指定效果进行振动。应用程序应处于前台才能进行振动。
    `vibe` 描述要执行的振动的振动效果。
    */
    #[java_method(overload=vibrate)]
    pub fn vibrate_effect(&self, vibe: &VibrationEffect) {}

    /// 关掉振动器。
    #[java_method]
    pub fn cancel(&self) {}

    /**
    取消特定类型的正在进行的振动。
    `usage_filter` 要取消的振动类型，表示为 VibrationAttributes 的按位组合。使用值。
    */
    #[java_method(overload=cancel)]
    pub fn cancel_usage_filter(&self, usage_filter: i32) {}

    /**
    检查振动器是否正在振动。
    返回：如果硬件正在振动，则返回 True，否则返回 false。
    */
    #[java_method]
    pub fn is_vibrating(&self) -> bool {}
}

/**
VibrationEffect 描述了振动器执行的触觉效果。这些效果可以是任意数量，从单次振动到复杂波形。
*/
#[java_class(name = "android/os/VibrationEffect")]
pub struct VibrationEffect;

impl VibrationEffect {
    /// 设备默认的振动强度。
    pub const DEFAULT_AMPLITUDE: i32 = -1;

    /// 最大振幅值
    pub const MAX_AMPLITUDE: i32 = 255;

    /// 点击效果。使用此效果作为基准，因为它是最常见的点击效果类型。
    pub const EFFECT_CLICK: i32 = <EffectImpl as Effect>::CLICK;

    /// 双击效果。
    pub const EFFECT_DOUBLE_CLICK: i32 = <EffectImpl as Effect>::DOUBLE_CLICK;

    /// 勾选效果。与 EFFECT_CLICK 相比，此效果强度较低。
    pub const EFFECT_TICK: i32 = <EffectImpl as Effect>::TICK;

    /// 一个轰动的效果。
    pub const EFFECT_THUD: i32 = <EffectImpl as Effect>::THUD;

    /// 具有流行效果。
    pub const EFFECT_POP: i32 = <EffectImpl as Effect>::POP;

    /// 重击效果。此效果比 EFFECT_CLICK 更强。
    pub const EFFECT_HEAVY_CLICK: i32 = <EffectImpl as Effect>::HEAVY_CLICK;

    /// 一种纹理效果，用于复制柔软的滴答声。
    /// 与普通效果不同，纹理效果需要反复调用，通常是为了响应某些动作，以复制用户手指下方的某些纹理的感觉。
    pub const EFFECT_TEXTURE_TICK: i32 = <EffectImpl as Effect>::TEXTURE_TICK;

    #[doc(hidden)]
    pub const EFFECT_STRENGTH_LIGHT: i32 = <EffectStrengthImpl as EffectStrength>::LIGHT as i32;

    #[doc(hidden)]
    pub const EFFECT_STRENGTH_MEDIUM: i32 = <EffectStrengthImpl as EffectStrength>::MEDIUM as i32;

    #[doc(hidden)]
    pub const EFFECT_STRENGTH_STRONG: i32 = <EffectStrengthImpl as EffectStrength>::STRONG as i32;

    /**
    防止从框架外部进行子类化
    */
    #[java_constructor]
    pub fn new() -> Self {}

    /**
    创建一次性振动。一次性振动将以指定的振幅持续振动指定的时间段，然后停止。
    返回：所需的效果。
    `milliseconds` 振动的毫秒数。这必须是正数。
    `amplitude` 振动的强度。这必须是 1 到 255 之间的值，或 DEFAULT_AMPLITUDE。
    */
    #[java_method]
    pub fn create_one_shot(milliseconds: i64, amplitude: i32) -> Self {}

    /**
    创建预定义振动效果。预定义效果是一组常见的振动效果，无论它们来自哪个应用，它们都应该相同，以便为整个设备上的用户提供一致的体验。它们还可以根据设备硬件进行定制，以提供比使用通用构建块构建得更好的体验。如果存在通用模式，并且不存在特定于硬件的效果实现，则将回退到通用模式。
    返回：所需的效果。
    `effect_id` 要执行的效果的 ID：EFFECT_CLICK、EFFECT_DOUBLE_CLICK、EFFECT_TICK
    */
    #[java_method]
    pub fn create_predefined(effect_id: i32) -> Self {}

    /**
    获取预定义的振动效果。预定义效果是一组常见的振动效果，无论它们来自哪个应用，它们都应该相同，以便为整个设备上的用户提供一致的体验。它们还可以根据设备硬件进行定制，以提供比使用通用构建块构建得更好的体验。如果存在通用模式，并且不存在特定于硬件的效果实现，则将回退到通用模式。
    返回：所需的效果。
    `effect_id` 要执行的效果的 ID：EFFECT_CLICK、EFFECT_DOUBLE_CLICK、EFFECT_TICK
    */
    #[java_method]
    pub fn get(effect_id: i32) -> Self {}

    /**
    获取预定义的振动效果。预定义效果是一组常见的振动效果，无论它们来自哪个应用，它们都应该相同，以便为整个设备上的用户提供一致的体验。它们还可以根据设备硬件进行定制，以提供比使用通用构建块构建得更好的体验。您可能只想在有特定于硬件的实现时播放某些效果，因为它们可能会在不进行调整的情况下对用户造成太大的干扰。fallback 参数允许您决定是要回退到通用实现，还是仅在有经过调整的特定于硬件的实现时才播放。
    返回：所需的效果。
    `effect_id` 要执行的效果的 ID：EFFECT_CLICK、EFFECT_DOUBLE_CLICK、EFFECT_TICK
    `fallback` 如果不存在特定于硬件的实现，是否回退到通用模式。
    */
    #[java_method(overload=get)]
    pub fn get_with_fallback(effect_id: i32, fallback: bool) -> Self {}

    #[doc(hidden)]
    #[java_method]
    pub fn describe_contents(&self) -> i32 {}

    #[doc(hidden)]
    #[java_method]
    pub fn validate(&self) {}

    /**
    获取振动的预计持续时间（以毫秒为单位）。对于没有定义结束的效果（例如具有非负重复索引的波形），这将返回 Long.MAX_VALUE。对于持续时间未知的效果（例如长度取决于设备和可能取决于运行时间的预烘焙效果），这将返回 -1。
    */
    #[java_method]
    pub fn get_duration(&self) -> i64 {}

    /**
    检查给定的振动器是否能按预期播放此效果。有关振动器支持哪些功能以及不支持哪些功能的更多信息，请参阅Vibrator#areVibrationFeaturesSupported(VibrationEffect)。
    */
    #[java_method]
    pub fn are_vibration_features_supported(&self, vibrator: &Vibrator) -> bool {}

    /**
    将默认值解析为整数振幅数字。
    如果振幅值已设置，则返回此值；否则，返回具有给定默认振幅的此效果的副本隐藏
    `default_amplitude` 要应用的默认振幅，必须介于 0 和 MAX_AMPLITUDE 之间
    */
    #[java_method]
    pub fn resolve(&self, default_amplitude: i32) -> Self {}

    /**
    根据给定的约束缩放振动效果强度。
    返回：如果没有缩放，则返回此值；否则，返回此效果的副本，并缩放振动强度
    `scale_factor` 应用于强度的缩放因子。[0,1) 内的值将缩小强度，大于 1 的值将放大
    */
    #[java_method]
    pub fn scale(&self, scale_factor: f32) -> Self {}

    /**
    将给定的效果强度应用于由 VibrationEffect.EFFECT_* 之一表示的预烘焙效果。
    返回：如果此效果没有变化，则返回此效果；否则，返回应用了效果强度的此效果的副本。
    `effect_strength` 要应用的新效果强度，VibrationEffect.EFFECT_STRENGTH_* 之一。
    */
    #[java_method]
    pub fn apply_effect_strength(&self, effect_strength: i32) -> Self {}

    /**
    根据给定的因子缩放给定的振动强度。
    `intensity` 效果的相对强度，必须介于 0 和 1 之间
    `scale_factor` 应用于强度的比例因子。[0,1) 内的值将降低强度，大于 1 的值将增加强度
    */
    #[java_method(overload=scale)]
    pub fn scale_intensity(intensity: f32, scale_factor: f32) -> f32 {}

    #[doc(hidden)]
    #[java_method]
    pub fn effect_id_to_string(effect_id: i32) -> String {}

    #[doc(hidden)]
    #[java_method]
    pub fn effect_strength_to_string(effect_strength: i32) -> String {}
}

/// 从系统属性中提取的有关当前构建的信息。
#[java_class(name = "android/os/Build")]
pub struct Build;

impl Build {
    /// 构建属性未知时使用的值。
    pub const UNKNOWN: &'static str = "unknown";

    fn get_string(property: &str) -> String {
        SystemProperties::get_with_default(property.to_string(), Self::UNKNOWN.to_string())
    }

    /// 可以是变更列表编号，也可以是像“M4-rc20”这样的标签。
    pub const ID: ConstFn = || Self::get_string("ro.build.id");

    /// 用于向用户显示的构建 ID 字符串
    pub const DISPLAY: ConstFn = || Self::get_string("ro.build.display.id");

    /// 整体产品的名称。
    pub const PRODUCT: ConstFn = || Self::get_string("ro.product.name");
    /**
    返回证明特定属性。
    返回属性值或 UNKNOWN
    `property` 型号、名称、品牌、设备或制造商。
    */
    fn get_vendor_device_id_property(property: &str) -> String {
        let attest_prop =
            Self::get_string(format!("ro.product.{}_for_attestation", property).as_str());
        if attest_prop == Self::UNKNOWN {
            Self::get_string(format!("ro.product.vendor.{}", property).as_str())
        } else {
            Self::UNKNOWN.to_string()
        }
    }

    //noinspection SpellCheckingInspection
    /// 用于认证的产品名称。在非默认版本（如 AOSP 版本）中，“PRODUCT”系统属性的值可能与提供给 KeyMint 的值不同，而 Keymint 认证仍会认证提供给 KeyMint 的产品名称。
    pub const PRODUCT_FOR_ATTESTATION: ConstFn = || Self::get_vendor_device_id_property("name");

    //noinspection SpellCheckingInspection
    /// 用于认证的设备名称。在非默认版本（如 AOSP 版本）中，“DEVICE”系统属性的值可能与配置给 KeyMint 的值不同，而 Keymint 认证仍会认证配置的设备名称。
    pub const DEVICE_FOR_ATTESTATION: ConstFn = || Self::get_vendor_device_id_property("device");

    /// 底层板的名称，例如“金鱼”。
    pub const BOARD: ConstFn = || Self::get_string("ro.product.board");

    //noinspection SpellCheckingInspection
    /// 本机代码的指令集名称（CPU 类型 + ABI 约定）。
    #[deprecated(note = "改用 SUPPORTED_ABIS。")]
    pub const CPU_ABI: ConstFn = String::new;

    //noinspection SpellCheckingInspection
    /// 本机代码的第二个指令集 (CPU 类型 + ABI 约定) 的名称。
    #[deprecated(note = "改用 SUPPORTED_ABIS。")]
    pub const CPU_ABI2: ConstFn = String::new;

    /// 产品/硬件的制造商。
    pub const MANUFACTURER: ConstFn = || Self::get_string("ro.product.manufacturer");

    //noinspection SpellCheckingInspection
    /// 用于认证的制造商名称。在非默认版本（如 AOSP 版本）中，“MANUFACTURER”系统属性的值可能与提供给 KeyMint 的值不同，而 Keymint 认证仍将认证提供给所配置的制造商。
    pub const MANUFACTURER_FOR_ATTESTATION: ConstFn =
        || Self::get_vendor_device_id_property("manufacturer");

    /// 与产品/硬件相关的消费者可见品牌（如果有）。
    pub const BRAND: ConstFn = || Self::get_string("ro.product.brand");

    //noinspection SpellCheckingInspection
    /// 用于认证的产品品牌。在非默认版本（如 AOSP 版本）中，“BRAND”系统属性的值可能与提供给 KeyMint 的值不同，而 Keymint 认证仍将认证提供给的产品品牌。
    pub const BRAND_FOR_ATTESTATION: ConstFn = || Self::get_vendor_device_id_property("brand");

    /// 最终产品的最终用户可见的名称。
    pub const MODEL: ConstFn = || Self::get_string("ro.product.model");

    //noinspection SpellCheckingInspection
    /// 用于认证的产品型号。在非默认版本（如 AOSP 版本）中，“MODEL”系统属性的值可能与配置给 KeyMint 的值不同，而 Keymint 认证仍会认证配置的产品型号。
    pub const MODEL_FOR_ATTESTATION: ConstFn = || Self::get_vendor_device_id_property("model");

    /// 系统引导加载程序版本号。
    pub const BOOTLOADER: ConstFn = || Self::get_string("ro.bootloader");

    /// 硬件的名称（来自内核命令行或 /proc）。
    pub const HARDWARE: ConstFn = || Self::get_string("ro.hardware");

    /**
    硬件的 SKU（来自内核命令行）。
    引导加载程序会报告 SKU 以配置系统软件功能。如果引导加载程序未提供任何值，则报告为“未知”。
    */
    pub const SKU: ConstFn = || Self::get_string("ro.boot.hardware.sku");

    /**
    原始设计制造商 (ODM) 设置的设备 SKU。
    这是启动期间设置的运行时初始化属性，用于配置设备服务。如果未设置任何值，则报告为“未知”。
    如果制造商生产相同设计的变体，ODM SKU 可能会针对同一系统 SKU 提供多种变体。例如，同一版本可能会发布具有不同物理键盘和/或显示硬件的变体，每个变体都有不同的 ODM SKU。
    */
    pub const ODM_SKU: ConstFn = || Self::get_string("ro.boot.product.hardware.sku");

    /// 此版本是否适用于模拟器设备。
    pub const IS_EMULATOR: ConstFn<bool> = || Self::get_string("ro.boot.qemu") == "1";

    /// 硬件序列号（如果有）。仅限字母数字，不区分大小写。此字段始终设置为 Build#UNKNOWN。
    #[deprecated(note = "改用 getSerial()。")]
    // 重要提示：此字段应通过函数调用进行初始化，以防止其值在编译期间在应用程序中内联，因为我们稍后会根据应用程序的目标 SDK 将其设置为该值。
    pub const SERIAL: ConstFn = || Self::get_string("no.such.thing");

    /**
    获取硬件序列号（如果可用）。
    注意：root 访问权限可能允许您修改设备标识符，例如硬件序列号。如果您更改这些标识符，则无法使用密钥认证来获取设备原始标识符的证明。如果框架提供的标识符与其配置的标识符不匹配，KeyMint 将拒绝 ID 认证请求。
    从 API 级别 29 开始，持久设备标识符受到额外限制的保护，建议应用使用可重置标识符（请参阅唯一标识符的最佳实践）。如果满足以下要求之一，则可以调用此方法：
    如果调用应用已被授予 READ_PRIVILEGED_PHONE_STATE 权限；这是一项特权权限，只能授予设备上预加载的应用。如果调用应用在任何有效订阅上具有运营商权限（请参阅 android.telephony.TelephonyManager.hasCarrierPrivileges）。
    如果调用应用程序是默认的 SMS 角色持有者（请参阅 android.app.role.RoleManager.isRoleHeld(String)）。如果调用应用程序是完全托管设备的设备所有者、组织拥有的设备的配置文件所有者或其代表（请参阅 android.app.admin.DevicePolicyManager.getEnrollmentSpecificId()）。
    如果调用应用程序不满足这些要求之一，则此方法将按以下方式运行：
    如果调用应用程序的目标 SDK 为 API 级别 28 或更低，并且应用程序具有 READ_PHONE_STATE 权限，则返回 UNKNOWN。如果调用应用程序的目标 SDK 为 API 级别 28 或更低，并且应用程序没有 READ_PHONE_STATE 权限，或者如果调用应用程序的目标是 API 级别 29 或更高，则抛出 SecurityException。
    返回：序列号（如果指定）。
    */
    #[java_method]
    pub fn get_serial() -> Result<String, <Self as JType>::Error> {}

    fn get_string_list(property: &str, separator: &str) -> Vec<String> {
        let value = SystemProperties::get(property.to_string());
        if value.is_empty() {
            return vec![];
        }
        value.split(separator).map(|i| i.to_string()).collect()
    }

    //noinspection SpellCheckingInspection
    /**
    此设备支持的 ABI 的有序列表。最优先的 ABI 是列表中的第一个元素。
    参见SUPPORTED_32_BIT_ABIS和SUPPORTED_64_BIT_ABIS。
    */
    pub const SUPPORTED_ABIS: ConstFn<Vec<String>> =
        || Self::get_string_list("ro.product.cpu.abilist", ",");

    //noinspection SpellCheckingInspection
    /**
    此设备支持的 32 位 ABI 的有序列表。最优先的 ABI 是列表中的第一个元素。
    参见SUPPORTED_ABIS和SUPPORTED_64_BIT_ABIS。
    */
    pub const SUPPORTED_32_BIT_ABIS: ConstFn<Vec<String>> =
        || Self::get_string_list("ro.product.cpu.abilist32", ",");

    //noinspection SpellCheckingInspection
    /**
    此设备支持的 64 位 ABI 的有序列表。最优先的 ABI 是列表中的第一个元素。
    参见SUPPORTED_ABIS和SUPPORTED_32_BIT_ABIS。
    */
    pub const SUPPORTED_64_BIT_ABIS: ConstFn<Vec<String>> =
        || Self::get_string_list("ro.product.cpu.abilist64", ",");

    #[doc(hidden)]
    #[java_method]
    pub fn is64bit_abi(abi: String) {}

    /// 构建类型，如“user”或“eng”。
    pub const TYPE: ConstFn = || Self::get_string("ro.build.type");

    /// 用逗号分隔的标签描述构建，如“unsigned,debug”。
    pub const TAGS: ConstFn = || Self::get_string("ro.build.tags");

    /// 唯一标识此构建的字符串。请勿尝试解析此值。
    pub const FINGERPRINT: ConstFn = || Self::derive_fingerprint();

    /// 有些设备将指纹组件分成多个分区，因此我们可能会在运行时获取指纹。
    fn derive_fingerprint() -> String {
        let mut finger = SystemProperties::get("ro.build.fingerprint".to_string());
        if finger.is_empty() {
            finger = format!(
                "{}/{}/{}:{}/{}/{}:{}/{}",
                Self::get_string("ro.product.brand"),
                Self::get_string("ro.product.name"),
                Self::get_string("ro.product.device"),
                Self::get_string("ro.build.version.release"),
                Self::get_string("ro.build.id"),
                Self::get_string("ro.build.version.incremental"),
                Self::get_string("ro.build.type"),
                Self::get_string("ro.build.tags")
            );
        }
        finger
    }

    /// 确保定义了原始指纹系统属性。如果它是由 deriveFingerprint() 动态派生的，那么我们将派生值推送到属性服务中。
    #[java_method]
    pub fn ensure_fingerprint_property() {}

    /**
    系统上各种超时的乘数。
    目的是针对比真实硬件慢几个数量级的软件模拟器的产品可以将其设置为较大的数字。在真实设备和硬件加速的虚拟化设备上不应设置此数字。
    */
    pub const HW_TIMEOUT_MULTIPLIER: ConstFn<i32> =
        || SystemProperties::get_int("ro.hw_timeout_multiplier".to_string(), 1);

    /// 如果此设备启用了 Treble 并且需要该功能，则为 True。
    pub const IS_TREBLE_ENABLED: ConstFn<bool> =
        || SystemProperties::get_boolean("ro.treble.enabled".to_string(), false);

    //noinspection SpellCheckingInspection
    /**
    验证设备的当前闪存是否与构建时预期的一致。Treble 设备将验证供应商接口 (VINTF)。
    未使用 Treble 启动的设备：1) 检查设备指纹是否已定义以及它是否与各个分区匹配。2) 验证无线电和引导加载程序分区是否是构建中预期的分区。
    */
    #[java_method]
    pub fn is_build_consistent() -> bool {}

    fn get_long(property: &str) -> i64 {
        if let Ok(num) = i64::from_str(&SystemProperties::get(property.to_string())) {
            return num;
        }
        -1
    }

    /// 自Unix时期以来，生产构建的时间以毫秒为单位。
    pub const TIME: ConstFn<i64> = || Self::get_long("ro.build.date.utc") * 1000;

    #[doc(hidden)]
    pub const USER: ConstFn = || Self::get_string("ro.build.user");

    #[doc(hidden)]
    pub const HOST: ConstFn = || Self::get_string("ro.build.host");

    //noinspection SpellCheckingInspection
    /**
    如果设备正在运行可调试版本（例如“userdebug”或“eng”），则返回 true。
    可调试版本允许用户通过本地 shell 获得 root 访问权限、将调试器附加到任何应用程序（无论它们是否设置了“可调试”属性），或者将 selinux 降级为“宽容”模式。
    */
    pub const IS_DEBUGGABLE: ConstFn<bool> =
        || SystemProperties::get_int("ro.debuggable".to_string(), 0) == 1;

    //noinspection SpellCheckingInspection
    /**
    如果设备正在运行可调试版本（例如“userdebug”或“eng”），则返回 true。
    可调试版本允许用户通过本地 shell 获得 root 访问权限、将调试器附加到任何应用程序（无论它们是否设置了“可调试”属性），或者将 selinux 降级为“宽容”模式。
    */
    pub fn is_debuggable() -> bool {
        Self::IS_DEBUGGABLE()
    }

    #[doc(hidden)]
    pub const IS_ENG: ConstFn<bool> = || "eng" == Self::TYPE();

    //noinspection SpellCheckingInspection
    #[doc(hidden)]
    pub const IS_USERDEBUG: ConstFn<bool> = || "userdebug" == Self::TYPE();

    #[doc(hidden)]
    pub const IS_USER: ConstFn<bool> = || "user" == Self::TYPE();

    //noinspection SpellCheckingInspection
    /**
    此版本是否在 ARC（适用于 Chrome 的 Android 运行时）上运行<https://chromium.googlesource.com/chromiumos/docs/+/master/containers_and_vms.md>。在 R 之前，它被实现为容器，但从 R 开始，它将是 VM。属性的名称仍为 ro.boot.conntainer，因为它在其他项目中被引用。
    如果可能的话，我们应尽量避免检查此标记，以尽量减少 与 非容器 Android 行为的不必要差异。当低级资源不同时，检查此标记是可以接受的，例如某些功能的可用性、对系统资源的访问受到限制以及主机操作系统可能会为我们处理某些功能。对于更高级别的行为差异，应优先进行其他检查。
    */
    pub const IS_ARC: ConstFn<bool> =
        || SystemProperties::get_boolean("ro.boot.container".to_string(), false);

    /**
    指定是否应在旧版应用的任何组件运行之前审核其所需的权限。旧版应用是指 targetSdkVersion < 23 的应用，即使用旧权限模型的应用。如果无需审核，则会在安装应用之前审核权限。
    */
    pub const PERMISSIONS_REVIEW_REQUIRED: bool = true;
}

/// 各种版本字符串。
#[allow(non_camel_case_types)]
#[java_class(name = "android/os/Build$VERSION")]
pub struct Build_VERSION;
impl Build_VERSION {
    /// 底层源代码控制用来表示此构建的内部值。例如，perforce 变更列表编号或 git 哈希。
    pub const INCREMENTAL: ConstFn = || Build::get_string("ro.build.version.incremental");

    /**
    用户可见版本字符串。 例如，“ 1.0”或“ 3.4B5”或“香蕉”。
    此字段是一个不透明的字符串。不要假设其值具有任何特定结构，也不要假设不同版本的 RELEASE 值可以以某种方式排序。
    */
    pub const RELEASE: ConstFn = || Build::get_string("ro.build.version.release");

    /// 版本字符串。如果不是最终发布版本，则可能是 RELEASE 或 CODENAME。
    pub const RELEASE_OR_CODENAME: ConstFn =
        || Build::get_string("ro.build.version.release_or_codename");

    /// 我们向用户展示的版本字符串；如果不是最终发布版本，则可能是 RELEASE 或描述性字符串。
    pub const RELEASE_OR_PREVIEW_DISPLAY: ConstFn =
        || Build::get_string("ro.build.version.release_or_preview_display");

    /// 产品所基于的基本操作系统构建。
    pub const BASE_OS: ConstFn = || {
        SystemProperties::get_with_default("ro.build.version.base_os".to_string(), String::new())
    };

    /// 用户可见的安全补丁级别。此值表示设备最近应用安全补丁的日期。
    pub const SECURITY_PATCH: ConstFn = || {
        SystemProperties::get_with_default(
            "ro.build.version.security_patch".to_string(),
            String::new(),
        )
    };

    /// 框架的原始字符串表示形式中用户可见的 SDK 版本；请改用 SDK_INT。
    #[deprecated(note = "使用 SDK_INT 可轻松将其作为整数获取。")]
    pub const SDK: ConstFn = || Build::get_string("ro.build.version.sdk");

    /// 此硬件设备上当前运行的软件的 SDK 版本。设备启动时此值不会改变，但硬件制造商提供 OTA 更新时此值可能会增加。可能的值在 Build.VERSION_CODES 中定义。
    pub const SDK_INT: ConstFn<i32> =
        || SystemProperties::get_int("ro.build.version.sdk".to_string(), 0);

    /// 此硬件设备上最初搭载的软件的 SDK 版本。该版本在设备生命周期内永远不会改变，即使 SDK_INT 因 OTA 更新而增加也是如此。可能的值在 Build.VERSION_CODES 中定义。
    pub const DEVICE_INITIAL_SDK_INT: ConstFn<i32> =
        || SystemProperties::get_int("ro.product.first_api_level".to_string(), 0);

    /**
    预发布 SDK 的开发者预览修订版本。在生产平台版本/设备上，此值始终为 0。
    当此值非零时，自上次正式发布 SDK_INT API 级别以来添加的任何新 API 都只能保证在该特定预览修订版本中存在。例如，API Activity.fooBar() 可能存在于预览修订版本 1 中，但在预览修订版本 2 中被重命名或完全移除，这可能会导致尝试调用它的应用在运行时崩溃。
    针对预览 API 的实验性应用在使用任何预发布平台 API 之前，应检查此值是否与它们所针对的预览 SDK 修订版本相等 (==)。如果应用检测到的预览 SDK 修订版本不是它们所期望的特定修订版本，则应回退到仅使用之前发布的 API 级别的 API，以避免不必要的运行时异常。
    */
    pub const PREVIEW_SDK_INT: ConstFn<i32> =
        || SystemProperties::get_int("ro.build.version.preview_sdk".to_string(), 0);

    /**
    给定预发布 SDK 的 SDK 指纹。此值在生产平台版本/设备上始终为“ ”。
    当该值不是''时。
    此属性旨在供安装程序用于更细粒度地定位软件包。针对预览 API 的应用程序不应使用此字段，而应使用 ` ` 或使用反射或其他运行时检查来检测 API 的存在或防止意外的运行时行为。
    */
    pub const PREVIEW_SDK_FINGERPRINT: ConstFn = || {
        SystemProperties::get_with_default(
            "ro.build.version.preview_sdk_fingerprint".to_string(),
            "REL".to_string(),
        )
    };

    /// 当前开发代号，如果这是发布版本，则为字符串“REL”。
    pub const CODENAME: ConstFn = || Build::get_string("ro.build.version.codename");

    /**
    VERSION_CODES 中存在的所有已知代号。
    这也包括开发代号，即如果 CODENAME 不是“REL”，那么该集合中就存在该值。
    如果此集合中不存在特定字符串，则它不是代号，或者是未来版本的代号。例如，在 Android R 开发期间，“Tiramisu”并不是一个已知的代号。
    */
    pub const KNOWN_CODENAMES: ConstFn<HashSet<String>> = || {
        HashSet::from_iter(
            Build::get_string_list("ro.build.version.known_codenames", ",").into_iter(),
        )
    };

    /// 应用目标 SDK 当前支持的最低值。以较低值为目标的应用可能无法在运行此 SDK 版本的设备上运行。其可能的值在 Build.VERSION_CODES 中定义。
    pub const MIN_SUPPORTED_TARGET_SDK_INT: ConstFn<i32> =
        || SystemProperties::get_int("ro.build.version.min_supported_target_sdk".to_string(), 0);
}

/// 当前已知的 SDK 版本代码的枚举。这些值可以在 VERSION#SDK 中找到。版本号会随着每个官方平台版本的发布而单调递增。
#[allow(non_camel_case_types)]
#[java_class(name = "android/os/Build$VERSION_CODES")]
pub struct Build_VERSION_CODES;

impl Build_VERSION_CODES {
    /// 当前开发版本的魔术版本号，尚未发布正式版本。
    /// 这必须与 VMRuntime.SDK_VERSION_CUR_DEVELOPMENT 匹配。
    pub const CUR_DEVELOPMENT: i32 = 10000;

    /**
    Android 的原始、第一个版本。耶！
    2008 年 9 月作为 Android 1.0 公开发布。
    */
    pub const BASE: i32 = 1;

    /**
    第一个Android更新。
    2009 年 2 月作为 Android 1.1 公开发布。
    */
    pub const BASE_1_1: i32 = 2;

    /**
    C.
    2009年4月公开发行为Android 1.5。
    */
    pub const CUPCAKE: i32 = 3;

    /**
    D.
    于 2009 年 9 月作为 Android 1.6 公开发布。针对此版本或更高版本的应用程序将获得以下新的行为变化：
    它们必须明确请求 android.Manifest.permission#WRITE_EXTERNAL_STORAGE 权限才能修改 SD 卡的内容。（针对早期版本的应用程序将始终请求该权限。）
    它们必须明确请求 android.Manifest.permission#READ_PHONE_STATE 权限才能检索手机状态信息。（针对早期版本的应用程序将始终请求该权限。）
    它们被认为支持不同的屏幕密度和尺寸。（除非另有说明，否则针对早期版本的应用程序被认为仅支持中等密度正常尺寸的屏幕）。它们仍然可以通过 supports-screens 清单标记明确指定屏幕支持。
    android.widget.TabHost 将使用新的深色标签背景设计。
    */
    pub const DONUT: i32 = 4;

    /**
    E.
    2009 年 10 月公开发布 Android 2.0。针对此版本或更高版本的应用程序将获得以下新的行为变化：
    android.app.Service#onStartCommand Service.onStartCommand 函数将返回新的 android.app.Service#START_STICKY 行为，而不是旧的兼容性 android.app.Service#START_STICKY_COMPATIBILITY。
    android.app.Activity 类现在将在按下按键而不是按下按键时执行后退按键，以便能够检测到虚拟按键的取消按键。
    android.widget.TabWidget 类将使用新的选项卡配色方案。在新方案中，前景选项卡具有中等灰色背景，背景选项卡具有深灰色背景。
    */
    pub const ECLAIR: i32 = 5;

    /**
    E 增量更新。
    2009 年 12 月作为 Android 2.0.1 公开发布。
    */
    pub const ECLAIR_0_1: i32 = 6;

    /**
    E MR1.
    2010 年 1 月作为 Android 2.1 公开发布。
    */
    pub const ECLAIR_MR1: i32 = 7;

    /**
    F.
    2010 年 5 月作为 Android 2.2 公开发布。
    */
    pub const FROYO: i32 = 8;

    /**
    G.
    2010 年 12 月公开发布为 Android 2.3。针对此版本或更高版本的应用程序将获得以下新的行为变化：应用程序的通知图标将显示在新的深色状态栏背景上，因此在这种情况下必须可见。
    */
    pub const GINGERBREAD: i32 = 9;

    /**
    G MR1.
    2011 年 2 月作为 Android 2.3.3 公开发布。
    */
    pub const GINGERBREAD_MR1: i32 = 10;

    //noinspection SpellCheckingInspection
    /**
    H.
    2011 年 2 月公开发布为 Android 3.0。针对此版本或更高版本的应用程序将获得以下新的行为变化：
    应用程序的默认主题现在为深色全息：android.R.style#Theme_Holo。
    在没有物理菜单按钮的大屏幕设备上，软（兼容性）菜单被禁用。
    活动生命周期已根据 android.app.Activity 略有变化。如果应用程序不调用其 android.app.Activity#onPause Activity.onPause() 方法的超级实现，它将崩溃。
    当应用程序需要访问其某个组件（活动、接收器、服务、提供程序）的权限时，当应用程序想要访问自己的组件时，此权限不再强制执行。这意味着它可以要求获得它本身并不拥有的组件的权限，但仍然可以访问该组件。
    android.content.Context#getSharedPreferences Context.getSharedPreferences() 将不会自动重新加载存储中的首选项（除非使用 android.content.Context#MODE_MULTI_PROCESS）。
    android.view.ViewGroup#setMotionEventSplittingEnabled 默认为 true。
    android.view.WindowManager.LayoutParams#FLAG_SPLIT_TOUCH 在 Windows 上默认启用。
    android.widget.PopupWindow#isSplitTouchEnabled() PopupWindow.isSplitTouchEnabled() 默认返回 true。 如果 android.widget.GridView 和 android.widget.ListView 没有实现 android.widget.Checkable，它们将对选定项目使用 android.view.View#setActivated View.setActivated。 android.widget.Scroller 将默认构建为启用“飞轮”行为。
    */
    pub const HONEYCOMB: i32 = 11;

    /**
    H MR1.
    2011 年 5 月作为 Android 3.1 公开发布。
    */
    pub const HONEYCOMB_MR1: i32 = 12;

    //noinspection SpellCheckingInspection
    /**
    H MR2.
    2011 年 7 月作为 Android 3.2 公开发布。更新至 Honeycomb MR1 以支持 7 英寸平板电脑、改进屏幕兼容模式等。
    从此版本开始，未说明是否支持 XLARGE 屏幕的应用程序只有在针对 HONEYCOMB 或更高版本时才会被假定支持；之前是 GINGERBREAD 或更高版本。不支持至少与当前屏幕一样大屏幕尺寸的应用程序将为用户提供一个 UI，以将其切换到屏幕尺寸兼容模式。
    此版本引入了基于屏幕尺寸（单位为 dp）的新屏幕尺寸资源限定符：请参阅 android.content.res.Configuration#screenWidthDp、android.content.res.Configuration#screenHeightDp 和 android.content.res.Configuration#smallestScreenWidthDp。根据 android.content.pm.ApplicationInfo#requiresSmallestWidthDp、android.content.pm.ApplicationInfo#compatibleWidthLimitDp 和 android.content.pm.ApplicationInfo#largestWidthLimitDp 在 &lt;supports-screens&gt; 中提供这些资源限定符，优于旧屏幕尺寸存储桶，对于旧设备，将从中推断出适当的存储桶。
    以此版本或更高版本为目标的应用将获得以下新的行为变化：此版本引入了新的 android.content.pm.PackageManager#FEATURE_SCREEN_PORTRAIT 和 android.content.pm.PackageManager#FEATURE_SCREEN_LANDSCAPE 功能。以以前的平台版本为目标的应用被认为需要设备同时支持纵向和横向；当以 Honeycomb MR1 或更高版本为目标时，应用负责指定其所需的任何特定方向。android.os.AsyncTask 在调用 android.os.AsyncTask#execute 时将默认使用串行执行器。android.content.pm.ActivityInfo#configChanges ActivityInfo.configChanges 将设置 android.content.pm.ActivityInfo#CONFIG_SCREEN_SIZE 和 android.content.pm.ActivityInfo#CONFIG_SMALLEST_SCREEN_SIZE 位；对于较旧的应用程序，需要清除这些，因为一些开发人员对该值进行了绝对比较，而不是正确屏蔽他们感兴趣的位。
    */
    pub const HONEYCOMB_MR2: i32 = 13;

    /**
    I.
    2011 年 10 月公开发布为 Android 4.0。针对此版本或更高版本的应用程序将获得以下新的行为变化：
    对于没有专用菜单键的设备，即使在手机上也不会显示软件兼容性菜单键。通过针对 Ice Cream Sandwich 或更高版本，您的 UI 必须始终在平板电脑和手机上具有自己的菜单 UI 可供性（如果需要）。
    ActionBar 将为您处理此问题。2D 绘图硬件加速现在默认打开。您可以使用 android.R.attr#hardwareAccelerated android:hardwareAccelerated 将其关闭（如果需要），但强烈建议不要这样做，因为它会导致大屏幕设备的性能不佳。
    应用程序的默认主题现在是“设备默认”主题：android.R.style#Theme_DeviceDefault。这可能是全息深色主题或特定设备定义的其他深色主题。android.R.style#Theme_Holo 系列不得修改，设备才被视为兼容。
    明确请求 Holo 系列主题的应用程序将保证这些主题不会在同一平台版本内改变特性。希望与设备融合的应用程序应使用 android.R.style#Theme_DeviceDefault 系列的主题。
    如果您直接关闭光标而不停止对它的管理，托管光标现在可能会引发异常；以前失败会被默默忽略。
    视图上的 fadingEdge 属性将被忽略（淡入淡出边缘不再是 UI 的标准部分）。新的 requireFadingEdge 属性允许应用程序在特殊情况下强制淡入淡出边缘。
    android.content.Context#bindService Context.bindService() 不会自动添加 android.content.Context#BIND_WAIVE_PRIORITY。
    应用程序小部件将自动在其周围添加标准填充，而不是依赖于嵌入到小部件本身的填充。如果您在将窗口添加到窗口管理器后尝试更改窗口类型，则会引发异常。以前这会导致随机的错误行为。
    android.view.animation.AnimationSet 会解析出定义的 duration、fillBefore、fillAfter、repeatMode 和 startOffset XML 属性。
    android.app.ActionBar#setHomeButtonEnabled ActionBar.setHomeButtonEnabled() 默认为 false。
    */
    pub const ICE_CREAM_SANDWICH: i32 = 14;

    /**
    I MR1.
    2011年12月以Android 4.03公开发布。
    */
    pub const ICE_CREAM_SANDWICH_MR1: i32 = 15;

    //noinspection SpellCheckingInspection
    /**
    J.
    2012 年 7 月公开发布，版本号为 Android 4.1。针对此版本或更高版本的应用程序将获得以下新的行为变化：
    您必须明确请求 android.Manifest.permission#READ_CALL_LOG 和/或 android.Manifest.permission#WRITE_CALL_LOG 权限；不再通过 android.Manifest.permission#READ_CONTACTS 和 android.Manifest.permission#WRITE_CONTACTS 隐式提供对通话日志的访问权限。
    如果为集合容器的 android.widget.RemoteViewsService 生成的视图设置 onClick 处理程序，android.widget.RemoteViews 将引发异常；以前这只会导致警告日志消息。
    嵌入式选项卡的新 android.app.ActionBar 策略：现在，无论屏幕大小如何，嵌入式选项卡在纵向模式下始终堆叠在操作栏中。
    android.webkit.WebSettings#setAllowFileAccessFromFileURLs(boolean) WebSettings.setAllowFileAccessFromFileURLs 和 android.webkit.WebSettings#setAllowUniversalAccessFromFileURLs(boolean) WebSettings.setAllowUniversalAccessFromFileURLs 默认为 false。
    如果应用程序的清单中不存在给定的组件类名，则对 android.content.pm.PackageManager#setComponentEnabledSetting PackageManager.setComponentEnabledSetting 的调用现在将抛出 IllegalArgumentException。如果在 Activity 被销毁后调用，` ` 将抛出 IllegalStateException。
    辅助功能服务必须需要新的 android.Manifest.permission#BIND_ACCESSIBILITY_SERVICE 权限，否则它们将无法使用。 android.accessibilityservice.AccessibilityServiceInfo#FLAG_INCLUDE_NOT_IMPORTANT_VIEWS 必须设置 AccessibilityServiceInfo.FLAG_INCLUDE_NOT_IMPORTANT_VIEWS 才能将不重要的视图包含在查询中。
    */
    pub const JELLY_BEAN: i32 = 16;

    /**
    J MR1.
    于 2012 年 11 月公开发布为 Android 4.2。针对此版本或更高版本的应用程序将获得以下行为新变化：内容提供程序：
    有关详细信息，请参阅提供程序文档中的 android:exported 部分。android.view.View#getLayoutDirection() View.getLayoutDirection() 可以根据语言环境等返回不同于 android.view.View#LAYOUT_DIRECTION_LTR 的值。
    android.webkit.WebView#addJavascriptInterface(Object, String) WebView.addJavascriptInterface 要求在方法上进行明确注释，以便可以从 Javascript 访问它们。
    */
    pub const JELLY_BEAN_MR1: i32 = 17;

    /**
    J MR2.
    2013 年 7 月作为 Android 4.3 公开发布。
    */
    pub const JELLY_BEAN_MR2: i32 = 18;

    /**
    K.
    2013 年 10 月公开发布为 Android 4.4。针对此版本或更高版本的应用将获得这些新的行为变化。有关此版本的更多信息，请参阅 Android KitKat 概述。
    android.preference.PreferenceActivity#isValidFragment(String) PreferenceActivity.isValueFragment 的默认结果变为 false 而不是 true。
    在 android.webkit.WebView 中，针对早期版本的应用将直接评估 JS URL，并且评估的任何结果都不会替换当前页面内容。针对 KITKAT 或更高版本的加载 JS URL 的应用将让该 URL 的结果替换当前页面的内容
    android.app.AlarmManager#set AlarmManager.set 被解释为不精确的值，以便系统在安排闹钟方面更灵活。
    android.content.Context#getSharedPreferences(String, int) Context.getSharedPreferences 不再允许使用空名称。
    android.widget.RelativeLayout 更改为正确计算包装的内容边距。允许绘制 android.app.ActionBar 的窗口内容叠加层。
    android.Manifest.permission#READ_EXTERNAL_STORAGE 权限现在始终强制执行。访问属于调用应用的特定于软件包的外部存储目录不再需要 android.Manifest.permission#READ_EXTERNAL_STORAGE 或 android.Manifest.permission#WRITE_EXTERNAL_STORAGE 权限。
    */
    pub const KITKAT: i32 = 19;

    /**
    K for watches.
    2014年6月以Android 4.4W公开发布。针对此或以后的版本的应用程序将获得这些新的行为更改：
    android.app.AlertDialog如果主题未指定一个，则可能没有默认背景。
    */
    pub const KITKAT_WATCH: i32 = 20;

    /// 临时直到我们完全切换到Lollipop。
    pub const L: i32 = 21;

    /**
    L.
    于 2014 年 11 月作为 Android 5.0 公开发布。针对此版本或更高版本的应用程序将获得这些新的行为变化。有关此版本的更多信息，请参阅 Android Lollipop 概述。
    android.content.Context#bindService Context.bindService 现在需要显式 Intent，如果给定隐式 Intent，将会引发异常。
    android.app.Notification.Builder Notification.Builder 不会调整其各种通知元素的颜色以更好地匹配新的材料设计外观。
    android.os.Message 将在回收消息时验证消息当前未在使用中。大多数地方将自动启用窗口中的硬件加速绘制。
    如果连接具有多种项目类型的适配器，android.widget.Spinner 将会引发异常。
    如果应用程序是启动器，即使用户使用公司配置文件（这要求应用程序使用 android.content.pm.LauncherApps 来正确填充其应用程序 UI），启动器也将可供用户使用。
    调用 android.app.Service#stopForeground Service.stopForeground 并将 removeNotification 设置为 false 将修改仍在发布的通知，使其不再强制持续。
    android.service.dreams.DreamService 必须要求 android.Manifest.permission#BIND_DREAM_SERVICE 权限才可使用。
    */
    pub const LOLLIPOP: i32 = 21;

    /**
    L MR1.
    2015 年 3 月公开发布为 Android 5.1。有关此版本的更多信息，请参阅 Android 5.1 API。
    */
    pub const LOLLIPOP_MR1: i32 = 22;

    /**
    M.
    2015 年 10 月公开发布为 Android 6.0。针对此版本或更高版本的应用程序将获得这些新的行为变化。有关此版本的更多信息，请参阅 Android 6.0 Marshmallow 概述。
    运行时权限。危险权限不再在安装时授予，但必须由应用程序在运行时通过 android.app.Activity#requestPermissions 请求。蓝牙和 Wi-Fi 扫描现在需要持有位置权限。
    android.app.AlarmManager#setTimeZone 如果给定的时区不是 Olson，AlarmManager.setTimeZone 将失败。
    活动转换只会将返回视图层次结构中映射的共享元素返回给调用活动。 android.view.View 允许多种可能破坏现有应用的行为：如果 restore() 调用次数过多，则 Canvas 会抛出异常；当返回 UNSPECIFIED 测量规格时，小部件可能会返回提示大小；它会尊重属性 android.R.attr#foreground、android.R.attr#foregroundGravity、android.R.attr#foregroundTint 和 android.R.attr#foregroundTintMode。
    android.view.MotionEvent#getButtonState MotionEvent.getButtonState 将不再将 android.view.MotionEvent#BUTTON_PRIMARY 和 android.view.MotionEvent#BUTTON_SECONDARY 报告为 android.view.MotionEvent#BUTTON_STYLUS_PRIMARY 和 android.view.MotionEvent#BUTTON_STYLUS_SECONDARY 的同义词。 android.widget.ScrollView 现在会在测量时尊重布局参数边距。
    */
    pub const M: i32 = 23;

    /**
    N.
    2016 年 8 月公开发布，版本号为 Android 7.0。针对此版本或更高版本的应用将获得这些新的行为变化。有关此版本的更多信息，请参阅 Android Nougat 概述。
    android.app.DownloadManager.Request#setAllowedNetworkTypes DownloadManager.Request.setAllowedNetworkTypes 在仅指定 android.app.DownloadManager.Request#NETWORK_WIFI 时将禁用“允许超过计量”。android.app.DownloadManager 不再允许访问原始文件路径。
    android.app.Notification.Builder#setShowWhen 必须明确调用 Notification.Builder.setShowWhen 才能显示时间，android.app.Notification.Builder Notification.Builder 中还对通知的显示方式进行了各种其他更改。
    android.content.Context#MODE_WORLD_READABLE 和 android.content.Context#MODE_WORLD_WRITEABLE 不再受支持。
    android.os.FileUriExposedException 将抛出到应用程序。应用程序将根据 android.view.View#DRAG_FLAG_GLOBAL 看到全局拖放。
    android.webkit.WebView#evaluateJavascript WebView.evaluateJavascript 不会从空的 WebView 中保留状态。
    android.animation.AnimatorSet 不会忽略在 start() 之前对 end() 的调用。
    android.app.AlarmManager#cancel(android.app.PendingIntent) 如果给定一个空操作，AlarmManager.cancel 将抛出一个 NullPointerException。
    android.app.FragmentManager 将确保在将片段放入后退堆栈之前已创建片段。android.app.FragmentManager 在 android.app.Fragment#onCreate Fragment.onCreate 中恢复片段，而不是在方法返回后。
    android.R.attr#resizeableActivity 默认为 true。android.graphics.drawable.AnimatedVectorDrawable 在打开无效的 VectorDrawable 动画时抛出异常。
    在某些类型的布局参数之间进行转换时（例如从 android.widget.LinearLayout.LayoutParams LinearLayout.LayoutParams 到 android.widget.RelativeLayout.LayoutParams RelativeLayout.LayoutParams），android.view.ViewGroup.MarginLayoutParams 将不再被丢弃。
    设备密度发生变化时，您的应用进程不会被终止。拖放。视图收到 android.view.DragEvent#ACTION_DRAG_ENTERED 事件后，当拖动阴影移动到可以接受数据的后代视图中时，视图会收到 android.view.DragEvent#ACTION_DRAG_EXITED 事件，并且当拖动阴影位于该后代视图内时，不会收到 android.view.DragEvent#ACTION_DRAG_LOCATION 和 android.view.DragEvent#ACTION_DROP 事件，即使后代视图从这些事件的处理程序中返回 false。
    */
    pub const N: i32 = 24;

    /**
    *N MR1.
    于 2016 年 10 月作为 Android 7.1 公开发布。有关此版本的更多信息，请参阅面向开发者的 Android 7.1。
    */
    pub const N_MR1: i32 = 25;

    /**
    O.
    2017 年 8 月公开发布为 Android 8.0。针对此版本或更高版本的应用程序将获得这些新的行为变化。有关此版本的更多信息，请参阅 Android Oreo 概述。
    后台执行限制已应用于应用程序。AccountManager 的 android.accounts.AccountManager#getAccountsByType、android.accounts.AccountManager#getAccountsByTypeAndFeatures 和 android.accounts.AccountManager#hasFeatures 的行为已更改，如那里所述。
    android.app.ActivityManager.RunningAppProcessInfo#IMPORTANCE_PERCEPTIBLE_PRE_26 现在返回为 android.app.ActivityManager.RunningAppProcessInfo#IMPORTANCE_PERCEPTIBLE。
    android.app.NotificationManager 现在需要使用通知通道。
    在 Application#onCreate Application.onCreate 中设置的严格模式更改将在该函数返回后不再被破坏。
    带有本机代码的共享库 apk 将把该本机代码包含在其客户端的库路径中。
    android.content.Context#getSharedPreferences 凭据加密存储中的 Context.getSharedPreferences 将在用户解锁之前引发异常。
    尝试在不支持该功能的设备上检索 Context#FINGERPRINT_SERVICE 现在将引发运行时异常。
    当 fragment 停止时，android.app.Fragment 将停止任何活动视图动画。
    资源中尝试使用应用可能正在使用的默认主题的某些兼容性代码将被关闭，要求应用明确请求具有正确主题的资源。
    android.content.ContentResolver#notifyChange ContentResolver.notifyChange 和 android.content.ContentResolver#registerContentObserver 如果调用者无权访问提供程序（或提供程序不存在），ContentResolver.registerContentObserver 将引发 SecurityException；否则该调用将被默默忽略。
    android.hardware.camera2.CameraDevice#createCaptureRequest CameraDevice.createCaptureRequest 将默认启用 android.hardware.camera2.CaptureRequest#CONTROL_ENABLE_ZSL 进行静态图像捕获。
    如果您无法访问壁纸，WallpaperManager 的 android.app.WallpaperManager#getWallpaperFile、android.app.WallpaperManager#getDrawable、android.app.WallpaperManager#getFastDrawable、android.app.WallpaperManager#peekDrawable 和 android.app.WallpaperManager#peekFastDrawable 将引发异常。
    android.hardware.usb.UsbDeviceConnection#requestWait UsbDeviceConnection.requestWait 的行为已根据那里的文档进行了修改。
    StrictMode.VmPolicy.Builder#detectAll StrictMode.VmPolicy.Builder.detectAll 还将启用 StrictMode.VmPolicy.Builder#detectContentUriWithoutPermission 和 StrictMode.VmPolicy.Builder#detectUntaggedSockets。StrictMode.ThreadPolicy.Builder#detectAll StrictMode.ThreadPolicy.Builder.detectAll 还将启用 StrictMode.ThreadPolicy.Builder#detectUnbufferedIo。
    android.provider.DocumentsContract 的各种方法将向调用者抛出失败异常，而不是返回 null。
    View#hasFocusable() View.hasFocusable 现在包括可自动对焦的视图。
    android.view.SurfaceView 将不再总是在底层 Surface 对象发生变化时更改它；应用程序需要查看对象的当前状态以确定他们感兴趣的哪些内容发生了变化。
    android.view.WindowManager.LayoutParams#TYPE_APPLICATION_OVERLAY 必须用于覆盖窗口，不允许使用其他系统覆盖窗口类型。
    android.view.ViewTreeObserver#addOnDrawListener 如果从 onDraw 内部调用 ViewTreeObserver.addOnDrawListener，将会引发异常。
    android.graphics.Canvas#setBitmap Canvas.setBitmap 将不再保留画布的当前矩阵和剪辑堆栈。
    android.widget.ListPopupWindow#setHeight 如果提供负高度，ListPopupWindow.setHeight 将会引发异常。
    android.widget.TextView 将使用国际化输入数字、日期和时间。
    必须使用 android.widget.Toast 来显示 Toast 窗口；不能直接使用 Toast 窗口类型。
    android.net.wifi.WifiManager#getConnectionInfo WifiManager.getConnectionInfo 要求调用者持有位置权限才能返回 BSSID/SSID android.net.wifi.p2p.WifiP2pManager#requestPeers WifiP2pManager.requestPeers 要求调用者持有位置权限。
    android.R.attr#maxAspectRatio 默认为 0，表示对应用的最大宽高比没有限制（因此可以拉伸以填充更大的屏幕）。
    android.R.attr#focusable 默认为新状态（` `），除非明确覆盖，否则它将继承 android.R.attr#clickable 的值。
    所有不提供焦点状态可绘制对象的视图都将提供默认的主题适当的焦点状态突出显示。可以通过将 android.R.attr#defaultFocusHighlightEnabled 设置为 false 来禁用此功能。
    */
    pub const O: i32 = 26;

    /**
    O MR1.
    2017 年 12 月公开发布为 Android 8.1。针对此版本或更高版本的应用将获得这些新的行为变化。有关此版本的更多信息，请参阅 Android 8.1 功能和 API。
    导出和链接到 apk 共享库的应用必须以一致的顺序明确枚举所有签名证书。如果关联的活动不是全屏且不透明的，则不能使用 android.R.attr#screenOrientation 来请求固定方向。
    */
    pub const O_MR1: i32 = 27;

    /**
    P.
    2018 年 8 月公开发布为 Android 9。针对此版本或更高版本的应用将获得这些新的行为变化。有关此版本的更多信息，请参阅 Android 9 Pie 概述。
    android.app.Service#startForeground Service.startForeground 要求应用持有权限 android.Manifest.permission#FOREGROUND_SERVICE。android.widget.LinearLayout 将始终重新测量加权子项，即使没有多余空间也是如此。
    */
    pub const P: i32 = 28;

    /**
    Q.
    2019 年 9 月公开发布为 Android 10。针对此版本或更高版本的应用将获得这些新的行为变化。有关此版本的更多信息，请参阅 Android 10 概览。
    行为变化：所有应用行为变化：针对 API 29+ 的应用
    */
    pub const Q: i32 = 29;

    /**
    R.
    2020 年 9 月公开发布，版本号为 Android 11。针对此版本或更高版本的应用将获得这些新的行为变更。有关此版本的更多信息，请参阅 Android 11 概览。
    行为变更：所有应用行为变更：针对 Android 11 的应用 Android 11 中非 SDK 接口限制的更新
    */
    pub const R: i32 = 30;

    /**
    S.
    */
    pub const S: i32 = 31;

    /**
    S V2.
    再一次冲向突破口，亲爱的朋友们，再一次。
    */
    pub const S_V2: i32 = 32;

    /**
    Tiramisu.
    */
    pub const TIRAMISU: i32 = 33;

    /**
    Upside Down Cake.
    */
    pub const UPSIDE_DOWN_CAKE: i32 = 34;
}

//noinspection SpellCheckingInspection
/**
授予对系统属性存储的访问权限。系统属性存储包含字符串键值对列表。仅将此类用于本地系统属性。
例如，在应用、分区或模块内。对于跨边界使用的系统属性，请在 *.sysprop 文件中正式定义它们并使用自动生成的方法。有关更多信息，请参阅将系统属性实现为 API。
*/
#[java_class(name = "android/os/SystemProperties")]
pub struct SystemProperties;

impl SystemProperties {
    /// Android O 删除了属性名称长度限制，但 com.amazon.kindle 7.8.1.5 在每次选择文本时都会使用反射来读取该属性名称长度限制（http://b/36095274）。
    pub const PROP_NAME_MAX: i32 = i32::MAX;

    #[doc(hidden)]
    pub const PROP_VALUE_MAX: i32 = 91;

    /**
    获取给定键的字符串值。
    `key` 查找的钥匙如果找不到键，则一个空字符串
    */
    #[java_method]
    pub fn get(key: String) -> String {}

    /**
    获取给定键的字符串值。
    返回：如果未找到键，则返回 def（如果它不为空），否则返回空字符串
    `key` 要查找的键
    `def` 如果属性未设置或为空，则返回默认值
    */
    #[java_method(overload=get)]
    pub fn get_with_default(key: String, def: String) -> String {}

    /**
    获取给定键的值，并以整数形式返回。
    返回：解析为整数的键，如果未找到或无法解析键，则返回 def
    `key` 要查找的键
    `def` 要返回的默认值
    */
    #[java_method]
    pub fn get_int(key: String, def: i32) -> i32 {}

    /**
    获取给定键的值，并以长整型返回。
    返回：解析为长整型的键，如果未找到或无法解析键，则返回 def
    `key` 要查找的键
    `def` 要返回的默认值
    */
    #[java_method]
    pub fn get_long(key: String, def: i64) -> i64 {}

    /**
    获取给定键的值，以布尔值形式返回。值“n”、“no”、“0”、“false”或“off”被视为假。值“y”、“yes”、“1”、“true”或“on”被视为真。（区分大小写）。如果键不存在或具有任何其他值，则返回默认结果。
    返回：解析为布尔值的键，如果未找到键或无法解析为布尔值，则返回 def。
    `key` 要查找的键
    `def` 要返回的默认值
    */
    #[java_method]
    pub fn get_boolean(key: String, def: bool) -> bool {}

    /**
    将给定键的值设置为 val。
    抛出：
    IllegalArgumentException – 对于非只读属性，如果 val 超过 91 个字符
    RuntimeException – 如果无法设置该属性，例如，如果它被 SELinux 阻止。libc 将记录根本原因。
    `key` 要设置的key。
    `val` 要设置的值。
    */
    #[java_method]
    pub fn set(key: String, val: Option<String>) {}

    /**
    添加一个回调，该回调将在系统属性发生任何变化时运行。
    `callback` 系统属性发生改变时应执行的 Runnable。
    */
    #[java_method]
    pub fn add_change_callback<R: Runnable>(callback: &R) {}

    /**
    删除目标回调。
    `callback` 应删除的 Runnable。
    */
    #[java_method]
    pub fn remove_change_callback<R: Runnable>(callback: &R) {}

    //noinspection SpellCheckingInspection
    /**
    通知侦听器系统属性已发生改变
    */
    #[java_method]
    pub fn report_sysprop_changed() {}

    /**
    按名称查找属性位置。
    返回：属性句柄，如果未设置属性，则返回 null
    `name` 属性的名称
    */
    #[java_method]
    pub fn find(name: String) -> Option<SystemProperties_Handle> {}
}

/// 预定位属性的句柄。提前查找属性句柄可以实现对单个属性的最佳重复查找。
#[allow(non_camel_case_types)]
#[java_class(name = "android/os/SystemProperties_Handle")]
pub struct SystemProperties_Handle;

impl SystemProperties_Handle {
    /// 返回属性的值
    #[java_method]
    pub fn get(&self) -> String {}

    /**
    返回int值或def在解析错误上
    `def` 默认值
    */
    #[java_method]
    pub fn get_int(&self, def: i32) -> i32 {}

    /**
    返回long值或def在解析错误上
    `def` 默认值
    */
    #[java_method]
    pub fn get_long(&self, def: i64) -> i64 {}

    /**
    返回boolean值或def在解析错误上
    `def` 默认值
    */
    #[java_method]
    pub fn get_boolean(&self, def: bool) -> bool {}
}

/// 测试android.os
#[cfg(feature = "test_android_os")]
pub fn test() {
    use crate::{
        android::{app::Activity, content::Context},
        java::lang::ObjectExt,
    };
    let bundle = Bundle::new();
    assert!(bundle.to_string().starts_with("Bundle"));
    let context: Context = (&Activity::fetch()).into();
    let vm: VibratorManager = context
        .get_system_service(Context::VIBRATOR_MANAGER_SERVICE.to_string())
        .unwrap()
        .cast();
    let effect = VibrationEffect::create_one_shot(500, VibrationEffect::DEFAULT_AMPLITUDE);
    vm.get_default_vibrator().vibrate_effect(&effect);
    let _ = dbg!(
        Build::PRODUCT_FOR_ATTESTATION(),
        Build::DISPLAY(),
        Build::ID(),
        Build::BOOTLOADER(),
        Build::BOARD(),
        Build::DEVICE_FOR_ATTESTATION(),
        Build::MANUFACTURER(),
        Build::MANUFACTURER_FOR_ATTESTATION(),
        Build::BRAND(),
        Build::BRAND_FOR_ATTESTATION(),
        Build::MODEL(),
        Build::MODEL_FOR_ATTESTATION(),
        Build::HARDWARE(),
        Build::SKU(),
        Build::ODM_SKU(),
        Build::IS_EMULATOR(),
        // Build::get_serial(),
        Build::SUPPORTED_ABIS(),
        Build::SUPPORTED_32_BIT_ABIS(),
        Build::SUPPORTED_64_BIT_ABIS()
    );
}
