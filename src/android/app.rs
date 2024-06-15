use droid_wrap_derive::{java_method, java_class};
use droid_wrap_utils::{android_context, global_ref};

use crate::{java::lang::CharSequence, JObjRef, JType};

#[java_class(name = "android/app/Activity")]
pub struct Activity;

impl Activity {
    #[java_method]
    pub fn finish(&self) {}

    #[java_method]
    pub fn set_title(&self, title: CharSequence) {}

    pub fn from_ctx() -> Self {
        let ctx = android_context();
        Self {
            _obj: global_ref(&ctx),
        }
    }
}
