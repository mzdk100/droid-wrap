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

use droid_wrap::android::app::Activity;

//noinspection SpellCheckingInspection
#[mobile_entry_point::mobile_entry_point]
fn main() {
    #[cfg(feature = "android_app")]
    {
        droid_wrap::android::app::test();
        println!("Test android.app successfully.");
    }
    #[cfg(feature = "android_content")]
    {
        droid_wrap::android::content::test();
        println!("Test android.content successfully.");
    }
    #[cfg(feature = "android_hardware")]
    {
        droid_wrap::android::hardware::test();
        println!("Test android.hardware successfully.");
    }
    #[cfg(feature = "android_os")]
    {
        droid_wrap::android::os::test();
        println!("Test android.os successfully.");
    }
    #[cfg(feature = "android_text")]
    {
        droid_wrap::android::text::test();
        println!("Test android.text successfully.");
    }
    #[cfg(feature = "android_speech_tts")]
    {
        droid_wrap::android::speech::tts::test();
        println!("Test android.speech.tts successfully.");
    }
    #[cfg(feature = "android_view")]
    {
        droid_wrap::android::view::test();
        println!("Test android.view successfully.");
    }
    #[cfg(feature = "android_view_inputmethod")]
    {
        droid_wrap::android::view::inputmethod::test();
        println!("Test android.view.inputmethod successfully.");
    }
    #[cfg(feature = "android_widget")]
    {
        droid_wrap::android::widget::test();
        println!("Test android.widget successfully.");
    }
    #[cfg(feature = "java_io")]
    {
        droid_wrap::java::io::test();
        println!("Test java.io successfully.");
    }
    #[cfg(feature = "java_lang")]
    {
        droid_wrap::java::lang::test();
        println!("Test java.lang successfully.");
    }
    #[cfg(feature = "java_lang_reflect")]
    {
        droid_wrap::java::lang::reflect::test();
        println!("Test java.lang.reflect successfully.");
    }
    #[cfg(feature = "java_nio")]
    {
        droid_wrap::java::nio::test();
        println!("Test java.nio successfully.");
    }
    #[cfg(feature = "dalvik_system")]
    {
        droid_wrap::dalvik::system::test();
        println!("Test dalvik.system successfully.");
    }
    Activity::fetch().finish();
    println!("Test all successfully.");
}
