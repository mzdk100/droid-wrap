mod error;
mod proxy;

pub use error::*;
pub use proxy::*;

pub use jni::{
    AttachGuard, Env, JavaVM, NativeMethod, jni_str, jni_sig,
    errors::Error as JniError,
    objects::{Global, JBooleanArray, JByteArray, JClass, JObject, JObjectArray, JString, JValue, ReleaseMode},
    refs::LoaderContext,
    strings::{JNIStr, JNIString},
    sys::{jboolean, jbyte, jchar, jdouble, jfloat, jint, jlong, jshort, jsize},
};

use parking_lot::ReentrantMutex;
use std::{cell::RefCell, collections::HashMap, sync::{Arc, LazyLock}};

// Rust proxy object hash -> Rust handler function
pub(crate) static HOOK_OBJECTS: LazyLock<
    ReentrantMutex<
        RefCell<
            HashMap<
                i32,
                Arc<
                    dyn Fn(&mut Env<'_>, &JObject<'_>, &JObjectArray<JObject<'_>>) -> Result<Global<JObject<'static>>>
                        + Send
                        + Sync,
                >,
            >,
        >,
    >,
> = LazyLock::new(|| ReentrantMutex::new(RefCell::new(HashMap::new())));

// Rust proxy object hash -> Java proxied object hash
pub(crate) static HOOK_OBJECTS_OTHER: LazyLock<ReentrantMutex<RefCell<HashMap<u64, i32>>>> =
    LazyLock::new(|| ReentrantMutex::new(RefCell::new(HashMap::new())));

/// Get the Android system JavaVM.
pub fn android_vm() -> Result<&'static JavaVM> {
    static JAVA_VM: LazyLock<std::result::Result<JavaVM, String>> = LazyLock::new(|| {
        let ctx = ndk_context::android_context();
        let vm = unsafe { JavaVM::from_raw(ctx.vm().cast()) };
        Ok(vm)
    });
    JAVA_VM
        .as_ref()
        .map_err(|e| DroidWrapError::JniOwned(e.clone()))
}

/// Attach the current thread to the JVM and run a closure with the `Env`.
pub fn with_env<F, T>(f: F) -> Result<T>
where
    F: FnOnce(&mut Env<'_>) -> Result<T>,
{
    let vm = android_vm()?;
    vm.attach_current_thread(f).map_err(|e| e.into())
}

/// Get the Android Context object (usually the NativeActivity reference).
pub fn android_context<'a, 'b>(_env: &'a Env<'b>) -> JObject<'a> {
    let ctx = ndk_context::android_context();
    unsafe { std::mem::transmute::<jni::sys::jobject, JObject<'a>>(ctx.context().cast()) }
}

/// Load a Java class using the Android app's class loader.
pub fn load_class<'a>(env: &mut Env<'a>, class_name: &str) -> Result<JClass<'a>> {
    let ctx = ndk_context::android_context();
    let activity: JObject<'a> = unsafe { std::mem::transmute::<jni::sys::jobject, JObject<'a>>(ctx.context().cast()) };
    let class_loader = env.call_method(
        activity,
        jni_str!("getClassLoader"),
        jni_sig!("()Ljava/lang/ClassLoader;"),
        &[],
    )?;
    let cls_name = env.new_string(class_name)?;
    let loaded_class = env.call_method(
        class_loader.l()?,
        jni_str!("loadClass"),
        jni_sig!("(Ljava/lang/String;)Ljava/lang/Class;"),
        &[(&cls_name).into()],
    )?;
    let class_obj = loaded_class.l()?;
    Ok(env.cast_local::<JClass>(class_obj)?)
}

/// Compare two Java objects for equality.
pub fn java_object_equals<'a>(
    env: &mut Env<'a>,
    a: &JObject<'a>,
    b: &JObject<'a>,
) -> Result<bool> {
    if a.is_null() && b.is_null() {
        return Ok(true);
    }
    if a.is_null() {
        return Ok(false);
    }
    let res = env
        .call_method(a, jni_str!("equals"), jni_sig!("(Ljava/lang/Object;)Z"), &[b.into()])?
        .z()?;
    Ok(res)
}

/// Convert a Java object to a Rust String.
pub fn java_object_to_string<'a>(env: &mut Env<'a>, obj: &JObject<'a>) -> Result<String> {
    if obj.is_null() {
        return Err(DroidWrapError::Jni(JniError::NullPtr("to_string")));
    }
    let s = env
        .call_method(obj, jni_str!("toString"), jni_sig!("()Ljava/lang/String;"), &[])?
        .l()?;
    let s = env.cast_local::<JString>(s)?;
    Ok(s.try_to_string(env)?)
}

/// Wrapper for `java.lang.Object`.
pub struct Object<'a>(JObject<'a>);

impl<'a> Object<'a> {
    /// Create a new `java.lang.Object` instance.
    pub fn new(env: &mut Env<'a>) -> Result<Self> {
        let obj = env.new_object(jni_str!("java/lang/Object"), jni_sig!("()V"), &[])?;
        Ok(Self(obj))
    }

    /// Get the underlying `JObject` reference.
    pub fn as_jobject(&self) -> &JObject<'a> { &self.0 }

    /// Convert into the underlying `JObject`.
    pub fn into_jobject(self) -> JObject<'a> { self.0 }
}

impl<'a> AsRef<JObject<'a>> for Object<'a> {
    fn as_ref(&self) -> &JObject<'a> { &self.0 }
}

impl<'a> std::ops::Deref for Object<'a> {
    type Target = JObject<'a>;
    fn deref(&self) -> &Self::Target { &self.0 }
}

/// Wrapper for `java.lang.String`.
pub struct JavaString<'a>(JString<'a>);

impl<'a> JavaString<'a> {
    /// Create a new Java `String` from a Rust `&str`.
    pub fn new(env: &mut Env<'a>, s: &str) -> Result<Self> {
        Ok(Self(env.new_string(s)?))
    }

    /// Get the underlying `JString` reference.
    pub fn as_jstring(&self) -> &JString<'a> { &self.0 }

    /// Convert into the underlying `JString`.
    pub fn into_jstring(self) -> JString<'a> { self.0 }

    /// Convert the Java String to a Rust `String`.
    pub fn to_rust_string(&self, env: &mut Env<'a>) -> Result<String> {
        Ok(self.0.try_to_string(env)?)
    }
}

impl<'a> AsRef<JObject<'a>> for JavaString<'a> {
    fn as_ref(&self) -> &JObject<'a> { self.0.as_ref() }
}

impl<'a> AsRef<JString<'a>> for JavaString<'a> {
    fn as_ref(&self) -> &JString<'a> { &self.0 }
}

impl<'a> std::ops::Deref for JavaString<'a> {
    type Target = JObject<'a>;
    fn deref(&self) -> &Self::Target { self.0.as_ref() }
}
