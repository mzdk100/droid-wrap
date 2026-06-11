mod entry;

use proc_macro::TokenStream;

/// Android platform entry point.
///
/// Mark a `fn main()` with this attribute to automatically generate
/// the Android application entry function.
#[proc_macro_attribute]
pub fn android_main(_args: TokenStream, input: TokenStream) -> TokenStream {
    entry::android_main(input.into()).into()
}
