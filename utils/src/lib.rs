pub use jni::{
    objects::{GlobalRef, JObject},
    sys::{jboolean, jchar, jdouble, jfloat, jint, jlong, jshort},
    AttachGuard, JavaVM,
};

pub fn get_vm() -> JavaVM {
    let ctx = ndk_context::android_context();
    unsafe { JavaVM::from_raw(ctx.vm().cast()) }.unwrap()
}

pub fn vm_attach<T>(wrapper: impl Fn(&mut AttachGuard) -> T) -> T {
    let vm = get_vm();
    let mut env = vm.attach_current_thread().unwrap();
    wrapper(&mut env)
}

pub fn android_context<'a>() -> JObject<'a> {
    let ctx = ndk_context::android_context();
    unsafe { JObject::from_raw(ctx.context().cast()) }
}

pub fn global_ref(obj: &JObject) -> GlobalRef {
    vm_attach(|env| env.new_global_ref(obj).unwrap())
}
