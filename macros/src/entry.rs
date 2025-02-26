/*
 * Copyright (c) 2025. The RigelA open source project team and
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

use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemFn, parse2};

pub(super) fn android_main(input: TokenStream) -> TokenStream {
    let func: ItemFn = parse2(input).unwrap();
    let name = &func.sig.ident;

    quote! {
        fn stop_unwind<F: FnOnce() -> T, T>(f: F) -> T {
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
                Ok(t) => t,
                Err(err) => {
                    eprintln!("attempt to unwind out of `rust` with err: {:?}", err);
                    std::process::abort()
                }
            }
        }

        #[cfg_attr(target_os = "android", ndk_glue::main(
            backtrace = "on",
            ndk_glue = "ndk_glue",
        ))]
        fn _start_app() {
            #func
            let _ = dbg!(stop_unwind(#name));
        }

        #[cfg(not(target_os = "android"))]
        #func
    }
}
