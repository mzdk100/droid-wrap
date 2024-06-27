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
use log::{debug, warn};
use std::{
    collections::HashMap,
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
    str::FromStr,
    sync::{
        mpsc::{channel, Receiver, Sender},
        OnceLock, RwLock,
    },
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
static HOOK_OBJECTS_OTHER: RwLock<Option<HashMap<u64, i32>>> = RwLock::new(None);
static HOOK_DROP_CHANNEL: OnceLock<(Sender<u64>, Recv)> = OnceLock::new();

struct Recv(Receiver<u64>);
unsafe impl Sync for Recv {}
unsafe impl Send for Recv {}
impl std::ops::Deref for Recv {
    type Target = Receiver<u64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/**
 * 定义必要的trait，以便于在本地为任何数据类型实现JAVA对象所需的功能。
 * */
#[macro_export]
macro_rules! import {
    () => {
        use droid_wrap_utils::{vm_attach, GlobalRef, JObject};
        use std::{
            rc::Rc,
            sync::{Arc, Mutex},
        };

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
            /// 字段类型
            type Fields;

            /**
             * 从java对象创建本地对象。
             * `this` java对象引用。
             * */
            fn _new(this: &GlobalRef, fields: Self::Fields) -> Self;

            /**
             * 创建空对象。
             * */
            fn null() -> Self
            where
                Self: Sized,
                Self::Fields: Default,
            {
                let null = vm_attach(|env| env.new_global_ref(JObject::null()).unwrap());
                Self::_new(&null, Default::default())
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

        /**
         * 用于Java动态代理的创建。
         * */
        pub trait JProxy: JObjNew {
            fn new(fields: Self::Fields) -> std::sync::Arc<Self>;
        }

        impl<T: JObjRef + JObjNew> JObjRef for Option<T>
        where
            <T as JObjNew>::Fields: Default,
        {
            fn java_ref(&self) -> GlobalRef {
                match self {
                    None => T::null().java_ref(),
                    Some(v) => v.java_ref(),
                }
            }
        }

        impl<T: JObjRef + JObjNew> JObjNew for Option<T> {
            type Fields = T::Fields;

            fn _new(this: &GlobalRef, fields: Self::Fields) -> Self {
                match this.is_null() {
                    true => None,
                    false => Some(T::_new(this, fields)),
                }
            }
        }

        impl<T: JType> JType for Arc<T> {
            const CLASS: &'static str = T::CLASS;
            type Error = T::Error;
            fn get_object_sig() -> String {
                T::get_object_sig()
            }
        }

        impl<T: JObjRef> JObjRef for Arc<T> {
            fn java_ref(&self) -> GlobalRef {
                self.as_ref().java_ref()
            }
        }

        impl<T: JObjNew> JObjNew for Arc<T> {
            type Fields = T::Fields;

            fn _new(this: &GlobalRef, fields: Self::Fields) -> Self {
                T::_new(this, fields).into()
            }
        }

        impl<T: JType> JType for Rc<T> {
            const CLASS: &'static str = T::CLASS;
            type Error = T::Error;
            fn get_object_sig() -> String {
                T::get_object_sig()
            }
        }

        impl<T: JObjRef> JObjRef for Rc<T> {
            fn java_ref(&self) -> GlobalRef {
                self.as_ref().java_ref()
            }
        }

        impl<T: JObjNew> JObjNew for Rc<T> {
            type Fields = T::Fields;

            fn _new(this: &GlobalRef, fields: Self::Fields) -> Self {
                T::_new(this, fields).into()
            }
        }

        impl<T: JType> JType for Mutex<T> {
            const CLASS: &'static str = T::CLASS;
            type Error = T::Error;
            fn get_object_sig() -> String {
                T::get_object_sig()
            }
        }

        impl<T: JObjNew> JObjNew for Mutex<T> {
            type Fields = T::Fields;

            fn _new(this: &GlobalRef, fields: Self::Fields) -> Self {
                T::_new(this, fields).into()
            }
        }

        impl<T: JObjRef> JObjRef for Mutex<T> {
            fn java_ref(&self) -> GlobalRef {
                self.java_ref()
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
/// use droid_wrap_utils::new_proxy;
/// let proxy = new_proxy(&["java.lang.Runnable"]);
/// ```
//noinspection SpellCheckingInspection
pub fn new_proxy(interfaces: &[&str]) -> GlobalRef {
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
    let mut lock = HOOK_OBJECTS_OTHER.write().unwrap();
    if lock.is_none() {
        lock.replace(HashMap::new());
    }
    let mut hasher = DefaultHasher::new();
    res.hash(&mut hasher);
    lock.as_mut().unwrap().insert(hasher.finish(), hash_code);
    drop(lock);
    res
}

//noinspection SpellCheckingInspection
/// java动态代理绑定rust函数。
///
/// # Arguments
///
/// * `proxy`: 代理对象。
/// * `handler`: 一个处理函数。
///
/// returns: ()
///
/// # Examples
///
/// ```
/// use droid_wrap_utils::{bind_proxy_handler, new_proxy, vm_attach};
/// let proxy = new_proxy(&["java.lang.Runnable"]);
/// bind_proxy_handler(&proxy, |mut env, method, args| {
///     let name = env.call_method(&method, "getName", "()Ljava/lang/String;", &[]).unwrap().l().unwrap();
///     let name = env.get_string((&name).into()).unwrap();
///     println!("Method `{}` is called with proxy.", name.to_str().unwrap());
///     env.new_global_ref(droid_wrap_utils::JObject::null()).unwrap()
/// });
/// vm_attach(|env| {
///     env.call_method(&proxy, "run", "()V", &[]).unwrap();
/// })
/// ```
pub fn bind_proxy_handler(
    proxy: &GlobalRef,
    handler: impl Fn(&mut JNIEnv<'_>, &JObject<'_>, &JObjectArray<'_>) -> GlobalRef
        + Send
        + Sync
        + 'static,
) {
    let hash_code = get_proxy_hash_code(proxy);
    let mut lock = HOOK_OBJECTS.try_write().unwrap();
    if lock.is_none() {
        lock.replace(HashMap::new());
    }
    lock.as_mut().unwrap().insert(hash_code, Box::new(handler));
    let (_, rx) = HOOK_DROP_CHANNEL.get_or_init(|| {
        let (tx, rx) = channel();
        (tx, Recv(rx))
    });
    loop {
        match rx.try_recv() {
            Ok(proxy_hash_code) => {
                let lock2 = HOOK_OBJECTS_OTHER.read().unwrap();
                if let Some(map) = lock2.as_ref() {
                    let Some(code) = map.get(&proxy_hash_code) else {
                        continue;
                    };
                    let code = *code;
                    drop(lock2);
                    if let Some(map) = lock.as_mut() {
                        map.remove_entry(&code);
                        debug!("Proxy `{}` is dropped.", proxy_hash_code);
                    }
                }
            }
            Err(..) => {
                break;
            }
        }
    }
}

/// 获取java代理对象的哈希值。
///
/// # Arguments
///
/// * `proxy`: 代理对象。
///
/// returns: i32
///
/// # Examples
///
/// ```
/// use droid_wrap_utils::{new_proxy, get_proxy_hash_code};
/// let proxy = new_proxy(&["java.lang.Runnable"]);
/// let hash_code = get_proxy_hash_code(&proxy);
/// ```
pub fn get_proxy_hash_code(proxy: &GlobalRef) -> i32 {
    let mut hasher = DefaultHasher::new();
    proxy.hash(&mut hasher);
    let proxy_hash_code = hasher.finish();
    let lock = HOOK_OBJECTS_OTHER.read().unwrap();
    if let Some(map) = lock.as_ref() {
        let Some(code) = map.get(&proxy_hash_code) else {
            return 0;
        };
        return *code;
    }
    0
}

//noinspection SpellCheckingInspection
/// 删除java动态代理绑定的rust函数。
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
/// use droid_wrap_utils::{unbind_proxy_handler, new_proxy};
/// let proxy = new_proxy(&["java.lang.Runnable"]);
/// unbind_proxy_handler(&proxy);
/// ```
pub fn unbind_proxy_handler(proxy: &GlobalRef) {
    let mut hasher = DefaultHasher::new();
    proxy.hash(&mut hasher);
    let proxy_hash_code = hasher.finish();
    if let Some((tx, _)) = HOOK_DROP_CHANNEL.get() {
        let _ = tx.send(proxy_hash_code);
    }
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

/// 把java对象数组转换成Vec
///
/// # Arguments
///
/// * `env`: java环境
/// * `arr`: java数组
///
/// returns: Vec<JObject, Global>
///
/// # Examples
///
/// ```
/// use jni::objects::JObjectArray;
/// use droid_wrap_utils::{to_vec, vm_attach};
/// unsafe { vm_attach(|env| {
/// let arr=JObjectArray::from_raw(0 as _);
/// to_vec(env, &arr);
/// }) }
/// ```
pub fn to_vec<'a>(env: &mut JNIEnv<'a>, arr: &JObjectArray) -> Vec<JObject<'a>> {
    let Ok(size) = env.get_array_length(arr) else {
        return vec![];
    };
    let mut arr2 = Vec::with_capacity(size as usize);
    for i in 0..size {
        arr2.push(env.get_object_array_element(arr, i).unwrap());
    }
    arr2
}
