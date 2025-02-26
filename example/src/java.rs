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
    android::app::Activity,
    java::lang::{Float, Integer, System},
    android_main,
};

#[android_main]
fn main() -> Result<()> {
    Activity::fetch()?.finish();
    let num = Integer::value_of(6)?;
    let _ = dbg!(num);
    let num = Float::value_of(6f32)?;
    let _ = dbg!(num);
    dbg!(System::current_time_millis());
    System::gc();
    System::exit(0)?;

    Ok(())
}
