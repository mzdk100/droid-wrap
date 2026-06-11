use super::{android_vm, DroidWrapError, HOOK_OBJECTS, HOOK_OBJECTS_OTHER, Result};
use jni::{
    Env, NativeMethod, jni_str, jni_sig,
    objects::{Global, JClass, JObject, JObjectArray, JString},
    sys::jsize,
};
use log::{debug, error, warn};
use std::{
    hash::{DefaultHasher, Hash, Hasher},
    sync::{Arc, OnceLock},
};

/// Create a Java dynamic proxy that implements the given interfaces.
///
/// Returns a `Global<JObject<'static>>` proxy object.
pub fn new_proxy(interfaces: &[&str]) -> Result<Global<JObject<'static>>> {
    let class = load_rust_call_method_hook_class()?;
    with_vm_env(|env| {
        let obj = env.new_object(class, jni_sig!("()V"), &[])?;
        let faces = env.new_object_array(
            interfaces.len() as jsize,
            jni_str!("java/lang/Class"),
            &JObject::null(),
        )?;
        for (i, iface) in interfaces.iter().enumerate() {
            let class_name = env.new_string(iface)?;
            let face = env
                .call_static_method(
                    jni_str!("java/lang/Class"),
                    jni_str!("forName"),
                    jni_sig!("(Ljava/lang/String;)Ljava/lang/Class;"),
                    &[(&class_name).into()],
                )?
                .l()?;
            faces.set_element(env, i as usize, &face)?;
        }
        let hash_code = env.call_method(&obj, jni_str!("hashCode"), jni_sig!("()I"), &[])?.i()?;
        let res = env.call_static_method(
            jni_str!("java/lang/reflect/Proxy"),
            jni_str!("newProxyInstance"),
            jni_sig!("(Ljava/lang/ClassLoader;[Ljava/lang/Class;Ljava/lang/reflect/InvocationHandler;)Ljava/lang/Object;"),
            &[
                (&JObject::null()).into(),
                (&faces).into(),
                (&obj).into(),
            ],
        )?
        .l()?;
        let res = env.new_global_ref(res)?;
        let lock = HOOK_OBJECTS_OTHER.lock();
        let mut hasher = DefaultHasher::new();
        res.hash(&mut hasher);
        lock.borrow_mut().insert(hasher.finish(), hash_code);
        drop(lock);
        Ok(res)
    })
}

/// Helper: run a closure with Env by attaching the current thread.
fn with_vm_env<F, T>(f: F) -> Result<T>
where
    F: FnOnce(&mut Env<'_>) -> Result<T>,
{
    let vm = android_vm()?;
    vm.attach_current_thread(f).map_err(|e| e.into())
}

/// Bind a Rust handler function to a Java dynamic proxy.
pub fn bind_proxy_handler(
    proxy: &Global<JObject<'static>>,
    handler: impl Fn(&mut Env<'_>, &JObject<'_>, &JObjectArray<JObject<'_>>) -> Result<Global<JObject<'static>>>
        + Send
        + Sync
        + 'static,
) {
    let hash_code = get_proxy_hash_code(proxy);
    let lock = match HOOK_OBJECTS.try_lock() {
        Some(lock) => lock,
        None => {
            error!("Can't bind proxy handler.");
            return;
        }
    };
    lock.borrow_mut().insert(hash_code, Arc::new(handler));
}

/// Get the Java hash code associated with a proxy object.
pub fn get_proxy_hash_code(proxy: &Global<JObject<'static>>) -> i32 {
    let mut hasher = DefaultHasher::new();
    proxy.hash(&mut hasher);
    let proxy_hash_code = hasher.finish();
    let lock = HOOK_OBJECTS_OTHER.lock();
    if let Some(code) = lock.borrow().get(&proxy_hash_code) {
        return *code;
    }
    0
}

/// Remove the Rust handler bound to a Java dynamic proxy.
pub fn unbind_proxy_handler(proxy: &Global<JObject<'static>>) {
    let mut hasher = DefaultHasher::new();
    proxy.hash(&mut hasher);
    let proxy_hash_code = hasher.finish();
    let lock = HOOK_OBJECTS_OTHER.lock();
    if let Some(code) = lock.borrow().get(&proxy_hash_code) {
        let lock = HOOK_OBJECTS.lock();
        lock.borrow_mut().remove_entry(code);
        debug!("Proxy `{}` is dropped.", proxy_hash_code);
    }
}

/// Load the Rust CallMethodHook class from the embedded DEX bytecode.
fn load_rust_call_method_hook_class<'a>() -> Result<&'a Global<JClass<'static>>> {
    #[cfg(target_os = "android")]
    const BYTECODE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/classes.dex"));
    #[cfg(not(target_os = "android"))]
    const BYTECODE: &[u8] = &[];
    static INSTANCE: OnceLock<std::result::Result<Global<JClass<'static>>, String>> = OnceLock::new();

    INSTANCE
        .get_or_init(|| {
            with_vm_env(|env| {
                let byte_buffer = unsafe {
                    env.new_direct_byte_buffer(BYTECODE.as_ptr() as *mut u8, BYTECODE.len())
                }?;

                let dex_class_loader = env.new_object(
                    jni_str!("dalvik/system/InMemoryDexClassLoader"),
                    jni_sig!("(Ljava/nio/ByteBuffer;Ljava/lang/ClassLoader;)V"),
                    &[
                        (&JObject::from(byte_buffer)).into(),
                        (&JObject::null()).into(),
                    ],
                )?;

                let class_name = env.new_string("rust/CallMethodHook")?;
                let class = env
                    .call_method(
                        &dex_class_loader,
                        jni_str!("loadClass"),
                        jni_sig!("(Ljava/lang/String;)Ljava/lang/Class;"),
                        &[(&class_name).into()],
                    )?
                    .l()?;
                let class: JClass<'_> = env.cast_local::<JClass>(class)?;
                let m = unsafe {
                    NativeMethod::from_raw_parts(
                        jni_str!("invoke"),
                        jni_str!("(Ljava/lang/Object;Ljava/lang/reflect/Method;[Ljava/lang/Object;)Ljava/lang/Object;"),
                        rust_callback as *mut _,
                    )
                };
                unsafe { env.register_native_methods(&class, &[m])? };

                Ok(env.new_global_ref(class)?)
            })
            .map_err(|e: DroidWrapError| e.to_string())
        })
        .as_ref()
        .map_err(|e| DroidWrapError::JniOwned(e.clone()))
}

/// JNI native callback for the dynamic proxy's invoke method.
unsafe extern "system" fn rust_callback(
    raw_env: *mut jni::sys::JNIEnv,
    this: jni::sys::jobject,
    _: jni::sys::jobject,
    method: jni::sys::jobject,
    args: jni::sys::jobjectArray,
) -> jni::sys::jobject {
    let mut env_unowned = unsafe { jni::EnvUnowned::from_raw(raw_env) };

    let outcome = env_unowned.with_env(|env| {
        let this = unsafe { JObject::from_raw(env, this) };
        let method = unsafe { JObject::from_raw(env, method) };
        let args = unsafe { JObjectArray::<JObject>::from_raw(env, args) };

        fn get_hash_code(env: &mut Env<'_>, this: &JObject<'_>) -> Result<i32> {
            Ok(env.call_method(this, jni_str!("hashCode"), jni_sig!("()I"), &[])?.i()?)
        }

        fn get_name(env: &mut Env<'_>, method: &JObject<'_>) -> Result<String> {
            let name = env
                .call_method(method, jni_str!("getName"), jni_sig!("()Ljava/lang/String;"), &[])?
                .l()?;
            let name: JString<'_> = env.cast_local::<JString>(name)?;
            Ok(name.try_to_string(env)?)
        }

        let hash_code = get_hash_code(env, &this).unwrap_or_default();

        match get_name(env, &method).unwrap_or_default().as_str() {
            "toString" => {
                let s = env.new_string(format!("Proxy@{:x}", hash_code).as_str())?;
                return Ok::<_, DroidWrapError>(env.new_local_ref(&s)?.into_raw());
            }
            "equals" | "hashCode" => {
                return match env.call_method(
                    &method,
                    jni_str!("invoke"),
                    jni_sig!("(Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object;"),
                    &[(&this).into(), (&args).into()],
                ) {
                    Ok(r) => match r.l() {
                        Ok(r) => Ok(r.into_raw()),
                        Err(e) => {
                            error!("{}", e);
                            Ok(JObject::null().into_raw())
                        }
                    },
                    Err(e) => {
                        error!("{}", e);
                        Ok(JObject::null().into_raw())
                    }
                };
            }
            _ => (),
        }

        let lock = HOOK_OBJECTS.lock();
        let func = if let Some(f) = lock.borrow().get(&hash_code) {
            f.to_owned()
        } else {
            warn!(
                "Method call reached, but the proxy object appears to have been dropped."
            );
            return Ok(JObject::null().into_raw());
        };
        drop(lock);

        match func(env, &method, &args) {
            Ok(ret) => {
                let local_ref = env.new_local_ref(ret.as_ref())?;
                Ok(local_ref.into_raw())
            }
            Err(e) => {
                error!("{}", e);
                Ok(JObject::null().into_raw())
            }
        }
    });

    match outcome.into_outcome() {
        jni::Outcome::Ok(raw) => raw,
        jni::Outcome::Err(e) => {
            error!("Proxy callback error: {}", e);
            JObject::null().into_raw()
        }
        jni::Outcome::Panic(payload) => {
            error!("Proxy callback panicked");
            std::panic::resume_unwind(payload)
        }
    }
}
