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

use crate::{JObjNew, JObjRef, JType, java_class};

/// AudioManager 提供对音量和铃声模式的控制。
#[java_class(name = "android/media/AudioManager")]
pub struct AudioManager;

impl AudioManager {
    /**
    广播操作：有线耳机已插入或拔出。您无法通过清单中声明的​​组件接收此信息，只能通过使用 Context.registerReceiver() 明确注册来接收。
    意图将具有以下额外值： state - 0 表示拔出，1 表示插入。 name - 耳机类型，人类可读的字符串 microphone - 如果耳机有麦克风则为 1，否则为 0
    */
    pub const ACTION_HEADSET_PLUG: &'static str = "android.intent.action.HEADSET_PLUG";
}
