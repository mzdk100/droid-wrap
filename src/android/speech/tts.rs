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

use crate::{
    android::{content::Context, os::Bundle},
    java::lang::CharSequence,
    JObjNew, JObjRef, JType,
};

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
    /// 广播操作：TextToSpeech 合成器已完成对语音队列中所有文本的处理。
    /// 请注意，当引擎完成文本数据处理时，这会通知呼叫者。此时音频播放可能尚未完成（甚至尚未开始）。如果您希望在发生这种情况时收到通知，请参阅 TextToSpeech.OnUtteranceCompletedListener。
    pub const ACTION_TTS_QUEUE_PROCESSING_COMPLETED: &'static str =
        "android.speech.tts.TTS_QUEUE_PROCESSING_COMPLETED";

    /// 表示一般操作失败。
    pub const ERROR: i32 = -1;

    /// 表示由无效请求导致的失败。
    pub const ERROR_INVALID_REQUEST: i32 = -8;

    /// 表示由网络连接问题导致的故障。
    pub const ERROR_NETWORK: i32 = -6;

    /// 表示由于网络超时导致的失败。
    pub const ERROR_NETWORK_TIMEOUT: i32 = -7;

    /// 表示由于语音数据未下载完成而导致的失败。
    pub const ERROR_NOT_INSTALLED_YET: i32 = -9;

    /// 表示与输出（音频设备或文件）相关的故障。
    pub const ERROR_OUTPUT: i32 = -5;

    /// 表示 TTS 引擎无法合成给定的输入。
    pub const ERROR_SYNTHESIS: i32 = -3;

    /// 表示该语言适用于该区域设置的语言，但不适用于该国家/地区和变体。
    pub const LANG_AVAILABLE: i32 = 0;

    /// 表示该语言适用于区域设置指定的语言和国家/地区，但不适用于变体。
    pub const LANG_COUNTRY_AVAILABLE: i32 = 1;

    /// 表示该语言完全按照区域设置指定的方式提供。
    pub const LANG_COUNTRY_VAR_AVAILABLE: i32 = 2;

    /// 表示缺少语言数据。
    pub const LANG_MISSING_DATA: i32 = -1;

    /// 表示不支持该语言。
    pub const LANG_NOT_SUPPORTED: i32 = -2;

    /// 队列模式，新条目添加到播放队列的末尾。
    pub const QUEUE_ADD: i32 = 1;

    /// 队列模式，其中整个播放队列都会被清除。这与 QUEUE_FLUSH 不同，因为所有条目都会被清除，而不仅仅是来自给定调用者的条目。
    pub const QUEUE_DESTROY: i32 = 2;

    /// 队列模式，播放队列中的所有条目（要播放的媒体和要合成的文本）都将被删除并替换为新条目。队列会根据给定的调用应用进行刷新。队列中来自其他调用者的条目不会被丢弃。
    pub const QUEUE_FLUSH: i32 = 0;

    /// 表示客户端请求停止。它仅在 API 的服务端使用，客户端不应该期望看到此结果代码。
    pub const STOPPED: i32 = -2;

    /// 表示操作成功。
    pub const SUCCESS: i32 = 0;

    /// 表示 TTS 服务失败。
    pub const ERROR_SERVICE: i32 = -4;

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

    /**
     * 释放 TextToSpeech 引擎使用的资源。例如，在 Activity 的 onDestroy() 方法中调用此方法是一种很好的做法，这样 TextToSpeech 引擎就可以完全停止。
     * */
    #[java_method]
    pub fn shutdown(&self) {}

    /**
     * 设置 TextToSpeech 引擎的语音音调。这对任何预录语音均无影响。
     * 返回：ERROR 或 SUCCESS。
     * `pitch` 语音音调。1.0 为正常音调，值越低，合成语音的音调越低，值越高，合成语音的音调越高。
     * */
    #[java_method]
    pub fn set_pitch(&self, pitch: f32) -> i32 {}

    /**
     * 设置语速。这对任何预先录制的语音没有影响。
     * 返回：ERROR 或 SUCCESS。
     * `speech_rate` 语速。1.0 是正常语速，较低的值会减慢语速（0.5 是正常语速的一半），较高的值会加快语速（2.0 是正常语速的两倍）。
     * */
    #[java_method]
    pub fn set_speech_rate(&self, speech_rate: f32) -> i32 {}

    //noinspection SpellCheckingInspection
    /**
     * 设置要使用的 TTS 引擎。
     * 返回：ERROR 或 SUCCESS。
     * `engine_package_name` 合成引擎的包名称（例如“com.svox.pico”）
     * */
    #[java_method]
    #[deprecated(
        note = "这不会在 TTS 引擎初始化时通知调用者。TextToSpeech(Context, TextToSpeech.OnInitListener, String) 可以与适当的引擎名称一起使用。此外，不能保证指定的引擎将被加载。如果未安装或禁用，则将应用用户/系统范围的默认值。"
    )]
    pub fn set_engine_by_package_name(&self, engine_package_name: String) -> i32 {}

    /**
     * 传递给 Speaking 和 SynthesizeToFile 的输入字符串的长度限制。
     * */
    #[java_method]
    pub fn get_max_speech_input_length() -> i32 {}

    /**
     * 获取默认语音合成引擎的包装名称。
     * 返回：用户选择的TTS引擎的软件包名称作为其默认值。
     * */
    #[java_method]
    pub fn get_default_engine(&self) -> String {}

    /**
     * 返回：此 TextToSpeech 实例当前使用的引擎。
     * */
    #[java_method]
    pub fn get_current_engine(&self) -> String {}

    //noinspection SpellCheckingInspection
    /**
     * 检查用户的设置是否应覆盖调用应用程序请求的设置。
     * */
    #[java_method]
    #[deprecated(note = "从 Ice creamwich 版本开始，用户设置永远不会强制覆盖应用程序的设置。")]
    pub fn are_defaults_enforced(&self) -> bool {}

    //noinspection SpellCheckingInspection
    /**
     * 使用指定的排队策略和语音参数朗读文本，文本可能以 TtsSpans 为单位。此方法是异步的，即该方法只是将请求添加到 TTS 请求队列然后返回。当此方法返回时，合成可能尚未完成（甚至尚未开始！）。为了可靠地检测合成过程中的错误，我们建议设置一个发音进度监听器（参见 setOnUtteranceProgressListener）并使用 TextToSpeech.Engine.KEY_PARAM_UTTERANCE_ID 参数。
     * 返回：排队发言操作的错误或成功。
     * `text` 要朗读的文本字符串。长度不超过 getMaxSpeechInputLength() 字符。
     * `queue_mode` 要使用的排队策略，QUEUE_ADD 或 QUEUE_FLUSH。
     * `params` 请求的参数。可以为 null。支持的参数名称：TextToSpeech.Engine.KEY_PARAM_STREAM、TextToSpeech.Engine.KEY_PARAM_VOLUME、TextToSpeech.Engine。KEY_PARAM_PAN。可以传入引擎特定参数，但参数键必须以它们所针对的引擎的名称作为前缀。例如，如果正在使用名为“com.svox.pico”的引擎，则键“com.svox.pico_foo”和“com.svox.pico:bar”将传递给该引擎。
     * `utterance_id` 此请求的唯一标识符。
     * */
    #[java_method]
    pub fn speak<CS: CharSequence>(
        &self,
        text: CS,
        queue_mode: i32,
        params: Bundle,
        utterance_id: String,
    ) -> i32 {
    }
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

//noinspection SpellCheckingInspection
#[cfg(feature = "test_android_speech_tts")]
pub fn test() {
    use crate::{
        android::app::Activity,
        java::lang::{CharSequenceExt, CharSequenceImpl},
    };
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
    assert_eq!(TextToSpeech::SUCCESS, tts.set_pitch(0.8f32));
    assert_eq!(TextToSpeech::SUCCESS, tts.set_speech_rate(0.8f32));
    // assert_eq!(TextToSpeech::SUCCESS, tts.set_engine_by_package_name("com.sgr.grtts".to_string()));
    assert!(!tts.get_default_engine().is_empty());
    assert!(!tts.get_current_engine().is_empty());
    // dbg!(tts.are_defaults_enforced());
    tts.speak(
        "你好".to_char_sequence::<CharSequenceImpl>(),
        TextToSpeech::QUEUE_ADD,
        Bundle::null(),
        "test".to_string(),
    );
    tts.shutdown();
    assert!(TextToSpeech::get_max_speech_input_length() > 0);
}
