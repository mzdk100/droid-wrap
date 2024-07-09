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
    JObjNew, JObjRef, JType,
};
use droid_wrap_derive::{java_class, java_constructor, java_method};

/**
 * 从字符串键到各种 Parcelable 值的映射。
 * 警告：请注意，Bundle 是一个惰性容器，因此它不实现 equals(Object) 或 hashCode()。
 * */
#[java_class(name = "android/os/Bundle")]
pub struct Bundle;

impl Bundle {
    /**
     * 构造一个新的空 Bundle。
     * */
    #[java_constructor]
    pub fn new() -> Self {}
}

/**
 * 从设备提供对所有振动器的访问，以及以同步方式运行它们的能力。如果您的过程退出，则您开始的任何振动都将停止。
 * */
#[java_class(name = "android/os/VibratorManager")]
pub struct VibratorManager;

impl VibratorManager {
    /**
     * 防止从框架外部进行子类化
     * */
    #[java_constructor]
    pub fn new() -> Self {}

    /**
     * 通过 ID 检索单个振动器。
     * 返回：具有给定 vibratorId 的振动器，永不为空。
     * `vibrator_id` 要检索的振动器的 ID。
     * */
    #[java_method]
    pub fn get_vibrator(&self, vibrator_id: i32) -> Vibrator {}

    /// 返回设备的默认振动器。
    #[java_method]
    pub fn get_default_vibrator(&self) -> Vibrator {}

    /// 关掉所有振动器。
    #[java_method]
    pub fn cancel(&self) {}

    /**
     * 取消特定类型的正在进行的振动。
     * `usage_filter` 要取消的振动类型，表示为 VibrationAttributes 的按位组合。使用值。
     * */
    #[java_method(overload=cancel)]
    pub fn cancel_usage_filter(&self, usage_filter: i32) {}
}

/**
 * 操作设备上的振动器的类。如果您的进程退出，您启动的任何振动都将停止。
 * */
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
     * 返回此振动器的ID。
     * 返回：一个非负整数，表示此服务控制的振动器的id，或者-1表示此服务未附加任何物理振动器。
     * */
    #[java_method]
    pub fn get_id(&self) -> i32 {}

    /**
     * 检查硬件是否有振动器。
     * 返回：如果硬件有振动器则返回 True，否则返回 false。
     * */
    #[java_method]
    pub fn has_vibrator(&self) -> bool {}

    /**
     * 检查振动器是否具有振幅控制。
     * 返回：如果硬件可以控制振动的振幅，则返回 True，否则返回 false。
     * */
    #[java_method]
    pub fn has_amplitude_control(&self) -> bool {}

    /**
     * 检查振动器是否具有独立的频率控制。
     * 返回：如果硬件可以独立于振动幅度控制振动频率，则返回 True，否则返回 false。
     * */
    #[java_method]
    pub fn has_frequency_control(&self) -> bool {}

    /**
     * 检查振动器是否支持给定 VibrationEffect 的所有组件（即振动器是否能按预期播放给定效果）。如果此方法返回 true，则 VibrationEffect 应按预期播放。 如果为 false，播放 VibrationEffect 仍可能产生振动，但振动效果可能与预期大不相同。此方法汇总功能检查方法的结果，例如 hasAmplitudeControl、areAllPrimitivesSupported(int...) 等，具体取决于 VibrationEffect 实际使用的功能。
     * 返回：如果振动器可以按预期播放给定效果，则返回 true，否则返回 false。
     * `effect` 要检查是否支持的 VibrationEffect
     * */
    #[java_method]
    pub fn are_vibration_features_supported(&self, effect: &VibrationEffect) -> bool {}

    /**
     * 使用 IExternalVibratorService 检查振动器是否可由外部服务控制。
     * 返回：如果硬件可由外部服务控制，则返回 True，否则返回 false。
     * */
    #[java_method]
    pub fn has_external_control(&self) -> bool {}
    /**
     * 获取振动器的共振频率（如果适用）。
     * 返回：振动器的共振频率，如果未知、不适用或该振动器是具有不同频率的多个物理设备的组合，则返回 NaN。
     * */
    #[java_method]
    pub fn get_resonant_frequency(&self) -> f32 {}

    /**
     * 获取振动器的 Q 因子。
     * 返回：振动器的 Q 因子，如果未知、不适用或该振动器是具有不同 Q 因子的多个物理设备的组合，则返回 NaN。
     * */
    #[java_method]
    pub fn get_q_factor(&self) -> f32 {}

    /**
     * 返回振动器使用音频触觉通道可以播放的最大振幅。这是一个正值，如果未知则为 NaN。如果返回正值 maxAmplitude，则来自音轨触觉通道的信号应在 [-maxAmplitude, maxAmplitude] 范围内。
     * 返回：一个正值，表示设备可以从音频触觉通道播放信号的最大绝对值，如果未知则为 NaN。
     * */
    #[java_method]
    pub fn get_haptic_channel_maximum_amplitude(&self) -> f32 {}

    /**
     * 在指定的时间段内持续振动。应用应处于前台才能发生振动。
     * `milliseconds` 振动的毫秒数。
     * */
    #[deprecated(note = "改用 vibrate(VibrationEffect)。")]
    #[java_method]
    pub fn vibrate(&self, milliseconds: i64) {}

    /**
     * 使用指定效果进行振动。应用程序应处于前台才能进行振动。
     * `vibe` 描述要执行的振动的振动效果。
     * */
    #[java_method(overload=vibrate)]
    pub fn vibrate_effect(&self, vibe: &VibrationEffect) {}

    /// 关掉振动器。
    #[java_method]
    pub fn cancel(&self) {}

    /**
     * 取消特定类型的正在进行的振动。
     * `usage_filter` 要取消的振动类型，表示为 VibrationAttributes 的按位组合。使用值。
     * */
    #[java_method(overload=cancel)]
    pub fn cancel_usage_filter(&self, usage_filter: i32) {}

    /**
     * 检查振动器是否正在振动。
     * 返回：如果硬件正在振动，则返回 True，否则返回 false。
     * */
    #[java_method]
    pub fn is_vibrating(&self) -> bool {}
}

/**
 * VibrationEffect 描述了振动器执行的触觉效果。这些效果可以是任意数量，从单次振动到复杂波形。
 * */
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

    pub const EFFECT_STRENGTH_LIGHT: i32 = <EffectStrengthImpl as EffectStrength>::LIGHT as i32;

    pub const EFFECT_STRENGTH_MEDIUM: i32 = <EffectStrengthImpl as EffectStrength>::MEDIUM as i32;

    pub const EFFECT_STRENGTH_STRONG: i32 = <EffectStrengthImpl as EffectStrength>::STRONG as i32;

    /**
     * 防止从框架外部进行子类化
     * */
    #[java_constructor]
    pub fn new() -> Self {}

    /**
     * 创建一次性振动。一次性振动将以指定的振幅持续振动指定的时间段，然后停止。
     * 返回：所需的效果。
     * `milliseconds` 振动的毫秒数。这必须是正数。
     * `amplitude` 振动的强度。这必须是 1 到 255 之间的值，或 DEFAULT_AMPLITUDE。
     * */
    #[java_method]
    pub fn create_one_shot(milliseconds: i64, amplitude: i32) -> Self {}

    /**
     * 创建预定义振动效果。预定义效果是一组常见的振动效果，无论它们来自哪个应用，它们都应该相同，以便为整个设备上的用户提供一致的体验。它们还可以根据设备硬件进行定制，以提供比使用通用构建块构建得更好的体验。如果存在通用模式，并且不存在特定于硬件的效果实现，则将回退到通用模式。
     * 返回：所需的效果。
     * `effect_id` 要执行的效果的 ID：EFFECT_CLICK、EFFECT_DOUBLE_CLICK、EFFECT_TICK
     * */
    #[java_method]
    pub fn create_predefined(effect_id: i32) -> Self {}

    /**
     * 获取预定义的振动效果。预定义效果是一组常见的振动效果，无论它们来自哪个应用，它们都应该相同，以便为整个设备上的用户提供一致的体验。它们还可以根据设备硬件进行定制，以提供比使用通用构建块构建得更好的体验。如果存在通用模式，并且不存在特定于硬件的效果实现，则将回退到通用模式。
     * 返回：所需的效果。
     * `effect_id` 要执行的效果的 ID：EFFECT_CLICK、EFFECT_DOUBLE_CLICK、EFFECT_TICK
     * */
    #[java_method]
    pub fn get(effect_id: i32) -> Self {}

    /**
     * 获取预定义的振动效果。预定义效果是一组常见的振动效果，无论它们来自哪个应用，它们都应该相同，以便为整个设备上的用户提供一致的体验。它们还可以根据设备硬件进行定制，以提供比使用通用构建块构建得更好的体验。您可能只想在有特定于硬件的实现时播放某些效果，因为它们可能会在不进行调整的情况下对用户造成太大的干扰。fallback 参数允许您决定是要回退到通用实现，还是仅在有经过调整的特定于硬件的实现时才播放。
     * 返回：所需的效果。
     * `effect_id` 要执行的效果的 ID：EFFECT_CLICK、EFFECT_DOUBLE_CLICK、EFFECT_TICK
     * `fallback` 如果不存在特定于硬件的实现，是否回退到通用模式。
     * */
    #[java_method(overload=get)]
    pub fn get_with_fallback(effect_id: i32, fallback: bool) -> Self {}

    #[java_method]
    pub fn describe_contents(&self) -> i32 {}

    #[java_method]
    pub fn validate(&self) {}

    /**
     * 获取振动的预计持续时间（以毫秒为单位）。对于没有定义结束的效果（例如具有非负重复索引的波形），这将返回 Long.MAX_VALUE。对于持续时间未知的效果（例如长度取决于设备和可能取决于运行时间的预烘焙效果），这将返回 -1。
     * */
    #[java_method]
    pub fn get_duration(&self) -> i64 {}

    /**
     * 检查给定的振动器是否能按预期播放此效果。有关振动器支持哪些功能以及不支持哪些功能的更多信息，请参阅Vibrator#areVibrationFeaturesSupported(VibrationEffect)。
     * */
    #[java_method]
    pub fn are_vibration_features_supported(&self, vibrator: &Vibrator) -> bool {}

    /**
     * 将默认值解析为整数振幅数字。
     * 如果振幅值已设置，则返回此值；否则，返回具有给定默认振幅的此效果的副本隐藏
     * `default_amplitude` 要应用的默认振幅，必须介于 0 和 MAX_AMPLITUDE 之间
     * */
    #[java_method]
    pub fn resolve(&self, default_amplitude: i32) -> Self {}

    /**
     * 根据给定的约束缩放振动效果强度。
     * 返回：如果没有缩放，则返回此值；否则，返回此效果的副本，并缩放振动强度
     * `scale_factor` 应用于强度的缩放因子。[0,1) 内的值将缩小强度，大于 1 的值将放大
     * */
    #[java_method]
    pub fn scale(&self, scale_factor: f32) -> Self {}

    /**
     * 将给定的效果强度应用于由 VibrationEffect.EFFECT_* 之一表示的预烘焙效果。
     * 返回：如果此效果没有变化，则返回此效果；否则，返回应用了效果强度的此效果的副本。
     * `effect_strength` 要应用的新效果强度，VibrationEffect.EFFECT_STRENGTH_* 之一。
     * */
    #[java_method]
    pub fn apply_effect_strength(&self, effect_strength: i32) -> Self {}

    /**
     * 根据给定的因子缩放给定的振动强度。
     * `intensity` 效果的相对强度，必须介于 0 和 1 之间
     * `scale_factor` 应用于强度的比例因子。[0,1) 内的值将降低强度，大于 1 的值将增加强度
     * */
    #[java_method(overload=scale)]
    pub fn scale_intensity(intensity: f32, scale_factor: f32) -> f32 {}

    #[java_method]
    pub fn effect_id_to_string(effect_id: i32) -> String {}

    #[java_method]
    pub fn effect_strength_to_string(effect_strength: i32) -> String {}
}

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
}
