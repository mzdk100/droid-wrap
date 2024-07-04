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
    android::{
        app::Activity,
        content::Context,
        view::{
            ViewGroup_LayoutParams, ViewManager, WindowManagerImpl, WindowManager_LayoutParams,
        },
        widget::{LinearLayout, LinearLayout_LayoutParams, TextView},
    },
    java::lang::{CharSequenceExt, CharSequenceImpl, RunnableImpl},
};
use mobile_entry_point::mobile_entry_point;
use std::sync::Arc;

#[mobile_entry_point]
fn main() {
    let act = Activity::fetch();
    let cs = "hello".to_char_sequence::<CharSequenceImpl>();
    dbg!(&cs);
    act.set_title(&cs);
    dbg!(&act);
    let act = Arc::new(act);
    let context: Context = act.as_ref().into();
    let text_view = TextView::new(&context);
    text_view.set_text(Some(
        "你好，这是一个用Rust构建的安卓示例。".to_char_sequence::<CharSequenceImpl>(),
    ));
    let act2 = act.clone();
    act.run_on_ui_thread(
        RunnableImpl::from_fn(move || {
            let params = LinearLayout_LayoutParams::new_with_weight(
                ViewGroup_LayoutParams::MATCH_PARENT,
                ViewGroup_LayoutParams::MATCH_PARENT,
                1.0,
            );
            let layout = LinearLayout::new(&context);
            layout.set_orientation(LinearLayout::VERTICAL);
            // layout.add_view(&text_view);
            layout.set_content_description(Some("容器".to_char_sequence::<CharSequenceImpl>()));
            layout.set_layout_params(&params);

            act2.set_content_view(&layout);

            let wm: WindowManagerImpl = act2.get_window_manager();
            let params = WindowManager_LayoutParams::new();
            wm.add_view(&text_view, &params);
        })
        .as_ref(),
    );
}
