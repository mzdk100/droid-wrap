pub mod android;
pub mod java;

use droid_wrap_utils::GlobalRef;

pub trait JObjRef<'a> {
    fn java_ref(&self) -> GlobalRef;
}

pub trait JType<'j>: JObjRef<'j> {
    const CLASS: &'j str;

    fn get_object_sig() -> String {
        format!("L{};", Self::CLASS)
    }
}
