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

/// 振动器效果
pub trait Effect {
    #[doc(hidden)]
    const CLICK: i32 = 0;

    #[doc(hidden)]
    const DOUBLE_CLICK: i32 = 1;

    #[doc(hidden)]
    const TICK: i32 = 2;

    #[doc(hidden)]
    const THUD: i32 = 3;

    #[doc(hidden)]
    const POP: i32 = 4;

    #[doc(hidden)]
    const HEAVY_CLICK: i32 = 5;

    #[doc(hidden)]
    const RINGTONE_1: i32 = 6;

    #[doc(hidden)]
    const RINGTONE_2: i32 = 7;

    #[doc(hidden)]
    const RINGTONE_3: i32 = 8;

    #[doc(hidden)]
    const RINGTONE_4: i32 = 9;

    #[doc(hidden)]
    const RINGTONE_5: i32 = 10;

    #[doc(hidden)]
    const RINGTONE_6: i32 = 11;

    #[doc(hidden)]
    const RINGTONE_7: i32 = 12;

    #[doc(hidden)]
    const RINGTONE_8: i32 = 13;

    #[doc(hidden)]
    const RINGTONE_9: i32 = 14;

    #[doc(hidden)]
    const RINGTONE_10: i32 = 15;

    #[doc(hidden)]
    const RINGTONE_11: i32 = 16;

    #[doc(hidden)]
    const RINGTONE_12: i32 = 17;

    #[doc(hidden)]
    const RINGTONE_13: i32 = 18;

    #[doc(hidden)]
    const RINGTONE_14: i32 = 19;

    #[doc(hidden)]
    const RINGTONE_15: i32 = 20;

    #[doc(hidden)]
    const TEXTURE_TICK: i32 = 21;
}

#[doc(hidden)]
pub struct EffectImpl;

impl Effect for EffectImpl {}

#[doc(hidden)]
pub trait EffectStrength {
    #[doc(hidden)]
    const LIGHT: u8 = 0;

    #[doc(hidden)]
    const MEDIUM: u8 = 1;

    #[doc(hidden)]
    const STRONG: u8 = 2;
}

#[doc(hidden)]
pub struct EffectStrengthImpl;

impl EffectStrength for EffectStrengthImpl {}
