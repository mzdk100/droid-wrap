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

pub use jni::{
    errors::Error,
    objects::{GlobalRef, JClass, JObject, JObjectArray, JValueGen, ReleaseMode},
    sys::{jboolean, jchar, jdouble, jfloat, jint, jlong, jshort, jsize},
    AttachGuard, JNIEnv, JavaVM, NativeMethod,
};
use log::warn;
use std::fmt::Debug;
use std::str::FromStr;
use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
    sync::{Mutex, OnceLock, RwLock},
};

// Rust 代理对象的哈希值映射到 Rust 函数
static HOOK_OBJECTS: RwLock<
    Option<
        HashMap<
            i32,
            Box<
                dyn Fn(&mut JNIEnv<'_>, &JObject<'_>, &JObjectArray<'_>) -> GlobalRef + Send + Sync,
            >,
        >,
    >,
> = RwLock::new(None);
// Rust 代理对象的哈希值映射到 Java 被代理的对象的哈希值
static HOOK_OBJECTS_OTHER: Mutex<Option<HashMap<u64, i32>>> = Mutex::new(None);

/**
 * 定义必要的trait，以便于在本地为任何数据类型实现JAVA对象所需的功能。
 * */
#[macro_export]
macro_rules! import {
    () => {
        use droid_wrap_utils::{vm_attach, GlobalRef, JObject};

        /**
         * JObjectRef trait提供从任何数据类型获取java对象的全局引用。
         * rust使用Arc管理，无须手动释放java的全局引用。
         * */
        pub trait JObjRef {
            /**
             * 获取java对象引用。
             * */
            fn java_ref(&self) -> GlobalRef;
        }

        /**
         * 用于从java对象创建本地对象。
         * */
        pub trait JObjNew {
            /**
             * 从java对象创建本地对象。
             * `this` java对象引用。
             * */
            fn _new(this: &GlobalRef) -> Self;

            /**
             * 创建空对象。
             * */
            fn null() -> Self
            where
                Self: Sized,
            {
                let null = vm_attach(|env| env.new_global_ref(JObject::null()).unwrap());
                Self::_new(&null)
            }
        }

        /**
         * 用于描述java类的信息。
         * */
        pub trait JType: JObjRef + JObjNew {
            /// 错误类型。
            type Error;
            /// java类的名称。
            const CLASS: &'static str;

            /**
             * 获取对象的签名描述。
             * */
            fn get_object_sig() -> String {
                format!("L{};", Self::CLASS)
            }
        }
    };
}

/**
 * 获取android系统的java虚拟机。
 * */
pub fn android_vm<'a>() -> JavaVM {
    let ctx = ndk_context::android_context();
    unsafe { JavaVM::from_raw(ctx.vm().cast()) }.unwrap()
}

/// 获取vm，将vm附加到当前线程，随后操作java虚拟机。
///
/// # Arguments
///
/// * `wrapper`: 一个闭包，接收可变引用的env，然后可以使用env操作当前虚拟机环境。
///
/// returns: T
/// 闭包的返回值作为vm_attach的返回值。
///
/// # Examples
///
/// ```
/// use droid_wrap_utils::vm_attach;
/// let class = vm_attach(|env| env.find_class("java/lang/String")).unwrap();
/// dbg!(class);
/// ```
pub fn vm_attach<T>(wrapper: impl Fn(&mut AttachGuard) -> T) -> T {
    let vm = android_vm();
    let mut env = vm.attach_current_thread().unwrap();
    wrapper(&mut env)
}

/**
 * 获取安卓的Context对象，这通常是NativeActivity对象的引用。
 * */
pub fn android_context<'a>() -> JObject<'a> {
    let ctx = ndk_context::android_context();
    unsafe { JObject::from_raw(ctx.context().cast()) }
}

/// 创建一个java动态代理，用于在rust层实现java接口的方法。
///
/// # Arguments
///
/// * `interfaces`: 要实现的java接口。
/// * `handler`: 代理的处理函数。
///
/// returns: GlobalRef 代理对象
///
/// # Examples
///
/// ```
/// use droid_wrap_utils::{new_proxy, vm_attach};
/// let proxy = new_proxy(&["java.lang.Runnable"], |mut env, method, args| {
///     let name = env.call_method(&method, "getName", "()Ljava/lang/String;", &[]).unwrap().l().unwrap();
///     let name = env.get_string((&name).into()).unwrap();
///     println!("Method `{}` is called with proxy.", name.to_str().unwrap());
///     env.new_global_ref(droid_wrap_utils::JObject::null()).unwrap()
/// });
/// vm_attach(|env| {
///     env.call_method(&proxy, "run", "()V", &[]).unwrap();
/// })
/// ```
//noinspection SpellCheckingInspection
pub fn new_proxy(
    interfaces: &[&str],
    handler: impl Fn(&mut JNIEnv<'_>, &JObject<'_>, &JObjectArray<'_>) -> GlobalRef
        + Send
        + Sync
        + 'static,
) -> GlobalRef {
    let class = load_rust_call_method_hook_class();
    let (hash_code, res) = vm_attach(|env| {
        let obj = env.new_object(class, "()V", &[]).unwrap();
        let faces = env
            .new_object_array(
                interfaces.len() as jsize,
                "java/lang/Class",
                &JObject::null(),
            )
            .unwrap();
        for i in 0..interfaces.len() {
            let class = env.new_string(interfaces[i]).unwrap();
            let face = env
                .call_static_method(
                    "java/lang/Class",
                    "forName",
                    "(Ljava/lang/String;)Ljava/lang/Class;",
                    &[(&class).into()],
                )
                .unwrap()
                .l()
                .unwrap();
            env.set_object_array_element(&faces, i as jsize, &face)
                .unwrap();
        }
        let hash_code = env
            .call_method(&obj, "hashCode", "()I", &[])
            .unwrap()
            .i()
            .unwrap();
        let res = env.call_static_method(
            "java/lang/reflect/Proxy",
            "newProxyInstance",
            "(Ljava/lang/ClassLoader;[Ljava/lang/Class;Ljava/lang/reflect/InvocationHandler;)Ljava/lang/Object;",
            &[
                (&JObject::null()).into(),
                (&faces).into(),
                (&obj).into()
            ]
        ).unwrap()
            .l()
            .unwrap();
        (hash_code, env.new_global_ref(&res).unwrap())
    });
    let mut lock = HOOK_OBJECTS.write().unwrap();
    if lock.is_none() {
        lock.replace(HashMap::new());
    }
    lock.as_mut().unwrap().insert(hash_code, Box::new(handler));
    drop(lock);
    let mut lock = HOOK_OBJECTS_OTHER.lock().unwrap();
    if lock.is_none() {
        lock.replace(HashMap::new());
    }
    let mut hasher = DefaultHasher::new();
    res.hash(&mut hasher);
    lock.as_mut().unwrap().insert(hasher.finish(), hash_code);
    res
}

//noinspection SpellCheckingInspection
/// 删除由new_proxy函数生成的java代理对象。
///
/// # Arguments
///
/// * `proxy`: Java 代理对象引用。
///
/// returns: ()
///
/// # Examples
///
/// ```
/// use droid_wrap_utils::{drop_proxy, new_proxy, vm_attach};
/// let proxy = new_proxy(&["java.lang.Runnable"], |mut env, method, args| {
///     let name = env.call_method(&method, "getName", "()Ljava/lang/String;", &[]).unwrap().l().unwrap();
///     let name = env.get_string((&name).into()).unwrap();
///     println!("Method `{}` is called with proxy.", name.to_str().unwrap());
///     env.new_global_ref(droid_wrap_utils::JObject::null()).unwrap()
/// });
/// vm_attach(|env| {
///     env.call_method(&proxy, "run", "()V", &[]).unwrap();
/// });
/// drop_proxy(&proxy);
/// ```
pub fn drop_proxy(proxy: &GlobalRef) {
    let mut hasher = DefaultHasher::new();
    proxy.hash(&mut hasher);
    let proxy_hash_code = hasher.finish();
    let hash_code = {
        let lock = HOOK_OBJECTS_OTHER.lock().unwrap();
        if let Some(map) = lock.as_ref() {
            let Some(code) = map.get(&proxy_hash_code) else {
                return;
            };
            let code = *code;
            drop(lock);
            let mut lock = HOOK_OBJECTS_OTHER.lock().unwrap();
            lock.as_mut().unwrap().remove_entry(&proxy_hash_code);
            code
        } else {
            return;
        }
    };
    let mut lock = HOOK_OBJECTS.write().unwrap();
    if let Some(map) = lock.as_mut() {
        map.remove_entry(&hash_code);
    }
    drop_proxy(proxy);
}

/**
 * 解析JObject类型。
 * */
pub trait ParseJObjectType<T: FromStr> {
    fn parse(&self, env: &mut JNIEnv) -> T
    where
        <T as FromStr>::Err: Debug;
}

impl<T: FromStr> ParseJObjectType<T> for JObject<'_> {
    //noinspection SpellCheckingInspection
    fn parse(&self, env: &mut JNIEnv) -> T
    where
        <T as FromStr>::Err: Debug,
    {
        let s = env
            .call_method(self, "toString", "()Ljava/lang/String;", &[])
            .unwrap()
            .l()
            .unwrap();
        let s = env.get_string((&s).into()).unwrap();
        s.to_str().unwrap().parse().unwrap()
    }
}
//noinspection SpellCheckingInspection
fn load_rust_call_method_hook_class<'a>() -> &'a GlobalRef {
    const BYTECODE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/classes.dex"));
    const LOADER_CLASS: &str = "dalvik/system/InMemoryDexClassLoader";
    static INSTANCE: OnceLock<GlobalRef> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        vm_attach(|env| {
            let byte_buffer = unsafe { env.new_direct_byte_buffer(BYTECODE.as_ptr() as *mut u8, BYTECODE.len()) }.unwrap();

            let dex_class_loader = env
                .new_object(
                    LOADER_CLASS,
                    "(Ljava/nio/ByteBuffer;Ljava/lang/ClassLoader;)V",
                    &[
                        JValueGen::Object(&JObject::from(byte_buffer)),
                        JValueGen::Object(&JObject::null()),
                    ],
                )
                .unwrap();

            let class = env.new_string("rust/CallMethodHook").unwrap();
            let class = env
                .call_method(
                    &dex_class_loader,
                    "loadClass",
                    "(Ljava/lang/String;)Ljava/lang/Class;",
                    &[(&class).into()],
                )
                .unwrap()
                .l()
                .unwrap();
            let m = NativeMethod {
                name: "invoke".into(),
                sig: "(Ljava/lang/Object;Ljava/lang/reflect/Method;[Ljava/lang/Object;)Ljava/lang/Object;".into(),
                fn_ptr: rust_callback as *mut _,
            };
            env.register_native_methods(Into::<&JClass<'_>>::into(&class), &[m]).unwrap();

            env.new_global_ref(&class).unwrap()
        })
    })
}

unsafe extern "C" fn rust_callback<'a>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
    _: JObject<'a>,
    method: JObject<'a>,
    args: JObjectArray<'a>,
) -> JObject<'a> {
    let hash_code = env
        .call_method(&this, "hashCode", "()I", &[])
        .unwrap()
        .i()
        .unwrap();
    let lock = HOOK_OBJECTS.read().unwrap();
    if let Some(map) = lock.as_ref() {
        if let Some(f) = map.get(&hash_code) {
            let ret = f(&mut env, &method, &args);
            drop(lock);
            return JObject::from_raw(ret.cast());
        } else {
            warn!("The method call has reached, but it appears that the proxy object has been dropped.");
        }
    }
    drop(lock);
    JObject::null()
}
