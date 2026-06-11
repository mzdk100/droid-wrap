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
