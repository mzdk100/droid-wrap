use crate::{JObjRef, JType};
use droid_wrap_derive::java_class;
use droid_wrap_utils::{global_ref, jint, vm_attach, GlobalRef};

impl<'a> JObjRef<'a> for String {
    fn java_ref(&self) -> GlobalRef {
        vm_attach(|env| {
            let s = env.new_string(&self).unwrap();
            global_ref(s.as_ref())
        })
    }
}

impl<'a> JType<'a> for String {
    const CLASS: &'a str = "java/lang/String";
}

impl<'a> JObjRef<'a> for &'static str {
    fn java_ref(&self) -> GlobalRef {
        vm_attach(|env| {
            let s = env.new_string(*self).unwrap();
            global_ref(s.as_ref())
        })
    }
}

impl<'a> JType<'a> for &'static str {
    const CLASS: &'a str = "java/lang/String";
}

#[java_class(name = "java/lang/Integer")]
pub struct Integer;

impl Integer {
    pub fn new(value: i32) -> Self {
        let obj = vm_attach(|env| {
            let i = env
                .new_object(Self::CLASS, "(I)V", &[(value as jint).into()])
                .unwrap();
            global_ref(&i)
        });
        Self { _obj: obj }
    }
}

#[java_class(name = "java/lang/CharSequence")]
pub struct CharSequence;

impl From<&str> for CharSequence {
    fn from(value: &str) -> Self {
        let obj = vm_attach(|env| {
            let cs = env.new_string(value).unwrap();
            global_ref(&cs)
        });
        Self { _obj: obj }
    }
}
