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

use droid_wrap::{
    Result,
    android::{
        app::Activity,
        view::{
            ViewGroup_LayoutParams, ViewManager, WindowManager_LayoutParams, WindowManagerImpl,
        },
        widget::{
            EditText, LinearLayout, LinearLayout_LayoutParams, TextView,
            TextView_OnEditorActionListenerImpl,
        },
    },
    java::lang::{CharSequenceExt, CharSequenceImpl, RunnableImpl}, android_main
};
use std::sync::Arc;

#[android_main]
fn main() -> Result<()> {
    let act = Activity::fetch()?;
    let cs = "hello".to_char_sequence::<CharSequenceImpl>()?;
    dbg!(&cs);
    act.set_title(&cs);
    dbg!(&act);
    let act = Arc::new(act);
    let text_view = TextView::new(act.as_ref());
    text_view.set_text(
        "你好，这是一个用Rust构建的安卓示例。"
            .to_char_sequence::<CharSequenceImpl>()
            .ok(),
    );
    let edit = EditText::new(act.as_ref());

    let editor_listener = TextView_OnEditorActionListenerImpl::from_fn(|_, _, _| true)?;
    edit.set_on_editor_action_listener(editor_listener.as_ref());
    // 请在合适的时机手动释放，因为rust无法感知java什么时候不再需要Listener。
    // editor_listener.release();

    let act2 = act.clone();
    act.run_on_ui_thread(
        RunnableImpl::from_fn(move || {
            let params = LinearLayout_LayoutParams::new_with_weight(
                ViewGroup_LayoutParams::MATCH_PARENT,
                ViewGroup_LayoutParams::MATCH_PARENT,
                1.0,
            );
            let layout = LinearLayout::new(act2.as_ref());
            layout.set_orientation(LinearLayout::VERTICAL);
            // layout.add_view(&text_view);
            layout.set_content_description(Some("容器".to_char_sequence::<CharSequenceImpl>()?));
            layout.set_layout_params(&params);

            act2.set_content_view(&layout);

            let wm: WindowManagerImpl = act2.get_window_manager();
            let params = WindowManager_LayoutParams::new();
            let _ = wm.add_view(&text_view, &params);
            wm.remove_view(&text_view);
            let _ = wm.add_view(&edit, &params);
            let runnable = RunnableImpl::from_fn(|| {
                println!("post delayed");
                Ok(())
            })?;
            edit.post_delayed(runnable.as_ref(), 100);
            // 请在合适的时机手动释放，因为rust无法感知java什么时候不再需要Runnable。
            // runnable.release();
            Ok(())
        })?
        .as_ref(),
    );

    Ok(())
}
