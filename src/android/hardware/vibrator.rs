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

pub trait Effect {
    const CLICK: i32 = 0;
    const DOUBLE_CLICK: i32 = 1;
    const TICK: i32 = 2;
    const THUD: i32 = 3;
    const POP: i32 = 4;
    const HEAVY_CLICK: i32 = 5;
    const RINGTONE_1: i32 = 6;
    const RINGTONE_2: i32 = 7;
    const RINGTONE_3: i32 = 8;
    const RINGTONE_4: i32 = 9;
    const RINGTONE_5: i32 = 10;
    const RINGTONE_6: i32 = 11;
    const RINGTONE_7: i32 = 12;
    const RINGTONE_8: i32 = 13;
    const RINGTONE_9: i32 = 14;
    const RINGTONE_10: i32 = 15;
    const RINGTONE_11: i32 = 16;
    const RINGTONE_12: i32 = 17;
    const RINGTONE_13: i32 = 18;
    const RINGTONE_14: i32 = 19;
    const RINGTONE_15: i32 = 20;
    const TEXTURE_TICK: i32 = 21;
}

pub struct EffectImpl;

impl Effect for EffectImpl {}

pub trait EffectStrength {
    const LIGHT: u8 = 0;
    const MEDIUM: u8 = 1;
    const STRONG: u8 = 2;
}

pub struct EffectStrengthImpl;

impl EffectStrength for EffectStrengthImpl {}
