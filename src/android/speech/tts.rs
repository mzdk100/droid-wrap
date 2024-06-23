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

use droid_wrap_derive::{
    java_class, java_constructor, java_implement, java_interface, java_method,
};

use crate::{android::content::Context, JObjNew, JObjRef, JType};

/**
 * 从文本合成语音以立即播放或创建声音文件。 TextToSpeech 实例只有在完成初始化后才可用于合成文本。实现 TextToSpeech.OnInitListener 以接收初始化完成的通知。使用完 TextToSpeech 实例后，调用 shutdown() 方法释放 TextToSpeech 引擎使用的原生资源。针对 Android 11 且使用文本转语音的应用应在其清单的查询元素中声明 TextToSpeech.Engine.INTENT_ACTION_TTS_SERVICE：
 * ```xml
 * <queries>
 *  ...
 *  <intent>
 *      <action android:name="android. intent. action. TTS_SERVICE" />
 *  </ intent>
 * </ queries>
 * ```
 * */
#[java_class(name = "android/speech/tts/TextToSpeech")]
pub struct TextToSpeech;

impl TextToSpeech {
    /**
     * TextToSpeech 类的构造函数，使用默认的 TTS 引擎。如果尚未运行，这还将初始化关联的 TextToSpeech 引擎。
     * `context` 此实例正在运行的上下文。
     * `listener` 当 TextToSpeech 引擎初始化时将调用 TextToSpeech.OnInitListener。如果发生故障，可能会在 TextToSpeech 实例完全构造之前立即调用侦听器。
     * */
    #[java_constructor]
    fn new<L: OnInitListener>(context: &Context, listener: &L) -> Self {}

    /**
     * 中断当前话语（无论是播放还是渲染到文件）并丢弃队列中的其他话语。
     * 返回：错误或成功。
     * */
    #[java_method]
    pub fn stop(&self) -> i32 {}

    /**
     * 检查 TTS 引擎是否正在讲话。请注意，一旦语音项目的音频数据被发送到音频混音器或写入文件，该语音项目即被视为完成。此时与音频硬件完成播放之间可能会有有限地滞后。如果 TTS 引擎正在讲话，则返回 true。
     * */
    #[java_method]
    pub fn is_speaking(&self) -> bool {}
}

/**
 * 调用回调接口定义，指示 TextToSpeech 引擎初始化完成。
 * */
#[java_interface(name = "android/speech/tts/TextToSpeech$OnInitListener")]
pub trait OnInitListener {
    /**
     * 调用以表示 TextToSpeech 引擎初始化完成。
     * `status` 成功或错误。
     * */
    fn on_init(&self, status: i32);
}

#[cfg(feature = "test_android_speech_tts")]
pub fn test() {
    use crate::android::app::Activity;
    let context: Context = (&Activity::fetch()).into();
    #[java_class(name = "rs/TtsListener")]
    struct OnInitListenerImpl;
    #[java_implement]
    impl OnInitListener for OnInitListenerImpl {
        fn on_init(&self, status: i32) {
            println!("Tts is initialized status: {}.", status)
        }
    }
    static INIT_LISTENER: std::sync::OnceLock<OnInitListenerImpl> = std::sync::OnceLock::new();
    let listener = INIT_LISTENER.get_or_init(|| OnInitListenerImpl::new());
    let tts = TextToSpeech::new(&context, listener);
    assert!(tts
        .to_string()
        .starts_with("android.speech.tts.TextToSpeech"));
    tts.stop();
    assert_eq!(false, tts.is_speaking());
}
