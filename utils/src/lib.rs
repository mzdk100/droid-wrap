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
    objects::{
        GlobalRef, JBooleanArray, JByteArray, JClass, JObject, JObjectArray, JString, JValueGen,
        ReleaseMode,
    },
    sys::{jboolean, jbyte, jchar, jdouble, jfloat, jint, jlong, jshort, jsize},
    AttachGuard, JNIEnv, JavaVM, NativeMethod,
};
use log::{debug, error, warn};
use parking_lot::ReentrantMutex;
use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
    str::FromStr,
    sync::{Arc, LazyLock, OnceLock},
};

// Rust 代理对象的哈希值映射到 Rust 函数
static HOOK_OBJECTS: LazyLock<
    ReentrantMutex<
        RefCell<
            HashMap<
                i32,
                Arc<
                    dyn Fn(&mut JNIEnv<'_>, &JObject<'_>, &JObjectArray<'_>) -> GlobalRef
                        + Send
                        + Sync,
                >,
            >,
        >,
    >,
> = LazyLock::new(|| ReentrantMutex::new(RefCell::new(HashMap::new())));
// Rust 代理对象的哈希值映射到 Java 被代理的对象的哈希值
static HOOK_OBJECTS_OTHER: LazyLock<ReentrantMutex<RefCell<HashMap<u64, i32>>>> =
    LazyLock::new(|| ReentrantMutex::new(RefCell::new(HashMap::new())));

/**
定义必要的trait，以便于在本地为任何数据类型实现JAVA对象所需的功能。
*/
#[macro_export]
macro_rules! import {
    () => {
        use std::{
            rc::Rc,
            sync::{Arc, Mutex},
        };
        use $crate::{
            impl_array, null_value, to_java_byte_array, to_java_object_array, unbind_proxy_handler,
            vm_attach, GlobalRef, JObject,
        };

        /**
        JObjectRef trait提供从任何数据类型获取java对象的全局引用。
        rust使用Arc管理，无须手动释放java的全局引用。
        */
        pub trait JObjRef {
            /**
            获取java对象引用。
            */
            fn java_ref(&self) -> GlobalRef;
        }

        /**
        用于从java对象创建本地对象。
        */
        pub trait JObjNew {
            /// 字段类型
            type Fields: Default;

            /**
            从java对象创建本地对象。
            `this` java对象引用。
            */
            fn _new(this: &GlobalRef, fields: Self::Fields) -> Self;

            /**
            创建空对象。
            */
            fn null() -> Self
            where
                Self: Sized,
                Self::Fields: Default,
            {
                vm_attach!(mut env);
                Self::_new(&null_value(&mut env), Default::default())
            }
        }

        /**
        用于描述java类的信息。
        */
        pub trait JType: JObjRef + JObjNew {
            /// 错误类型。
            type Error;

            /// java类的名称。
            const CLASS: &'static str;

            /// 对象的签名描述。
            const OBJECT_SIG: &'static str;

            /// 数组维度，0表示不是数组
            const DIM: u8 = 0;
        }

        /**
        用于Java动态代理的创建和删除。
        */
        pub trait JProxy: JObjNew + JObjRef {
            /**
            创建一个代理对象。
            `fields` 传递给struct的自定义字段。
            */
            fn new(fields: Self::Fields) -> std::sync::Arc<Self>;

            /**
            释放代理对象，解除绑定的handler。
            */
            fn release(&self) {
                unbind_proxy_handler(&self.java_ref());
            }
        }

        impl<T: JObjRef> JObjRef for &T {
            fn java_ref(&self) -> GlobalRef {
                self.java_ref()
            }
        }

        impl<T: JObjNew> JObjNew for &T {
            type Fields = T::Fields;

            fn _new(this: &GlobalRef, fields: Self::Fields) -> Self {
                panic!("Reference types cannot be constructed.")
            }
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
            const OBJECT_SIG: &'static str = T::OBJECT_SIG;
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
            const OBJECT_SIG: &'static str = T::OBJECT_SIG;
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
            const OBJECT_SIG: &'static str = T::OBJECT_SIG;
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

        impl_array!(u8, 1);
        impl_array!(String, 1);
    };
}

/// 实现rust数组类型与java数组的自动关联
///
/// # 示例
/// ```
/// use droid_wrap_utils::impl_array;
/// // 实现一维数组的关联
/// impl_array!(String, 1);
/// ```
#[macro_export]
macro_rules! impl_array {
    (String, $dim: expr) => {
        impl JObjNew for &[String] {
            type Fields = ();
            fn _new(this: &GlobalRef, fields: Self::Fields) -> Self {
                &[]
            }
        }

        impl JObjRef for &[String] {
            fn java_ref(&self) -> GlobalRef {
                vm_attach!(mut env);
                let arr = self
                    .iter()
                    .map(|i| env.new_string(i).unwrap())
                    .collect::<Vec<_>>();
                let sig = if Self::DIM <= 1 {
                    Self::CLASS.to_string()
                } else {
                    "[".repeat((Self::DIM - 1) as _) + Self::OBJECT_SIG
                };
                let arr = to_java_object_array(&mut env, &arr, &sig);
                env.new_global_ref(&arr).unwrap()
            }
        }

        impl JType for &[String] {
            type Error = <String as JType>::Error;
            const CLASS: &'static str = <String as JType>::CLASS;
            const OBJECT_SIG: &'static str = <String as JType>::OBJECT_SIG;
            const DIM: u8 = $dim;
        }
    };
    (u8, $dim:expr) => {
        impl JObjNew for &[u8] {
            type Fields = ();
            fn _new(this: &GlobalRef, fields: Self::Fields) -> Self {
                &[]
            }
        }

        impl JObjRef for &[u8] {
            fn java_ref(&self) -> GlobalRef {
                vm_attach!(mut env);
                let arr = self.iter().map(|i| *i as _).collect::<Vec<_>>();
                let arr = to_java_byte_array(&mut env, &arr);
                env.new_global_ref(&arr).unwrap()
            }
        }

        impl JType for &[u8] {
            type Error = <String as JType>::Error;
            const CLASS: &'static str = "B";
            const OBJECT_SIG: &'static str = "B";
            const DIM: u8 = $dim;
        }
    };
}

/**
获取android系统的java虚拟机。
*/
pub fn android_vm<'a>() -> JavaVM {
    let ctx = ndk_context::android_context();
    unsafe { JavaVM::from_raw(ctx.vm().cast()) }.unwrap()
}

/// 获取vm，将vm附加到当前线程，随后操作java虚拟机。
///
/// # Examples
///
/// ```
/// use droid_wrap_utils::vm_attach;
/// vm_attach!(_ env);
/// // or: vm_attach!(mut env);
/// let class = env.find_class("java/lang/String");
/// dbg!(class);
/// ```
#[macro_export]
macro_rules! vm_attach {
    (mut $var:ident) => {
        let vm = $crate::android_vm();
        let mut $var = vm.attach_current_thread().unwrap();
    };
    (_ $var:ident) => {
        let vm = android_vm();
        let $var = vm.attach_current_thread().unwrap()
    };
}

/**
获取安卓的Context对象，这通常是NativeActivity对象的引用。
*/
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
    vm_attach!(mut env);
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
    let res = env.new_global_ref(&res).unwrap();
    let lock = HOOK_OBJECTS_OTHER.lock();
    let mut hasher = DefaultHasher::new();
    res.hash(&mut hasher);
    lock.borrow_mut().insert(hasher.finish(), hash_code);
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
///     droid_wrap_utils::null_value(env)
/// });
/// vm_attach!(mut env);
/// env.call_method(&proxy, "run", "()V", &[]).unwrap();
/// ```
pub fn bind_proxy_handler(
    proxy: &GlobalRef,
    handler: impl Fn(&mut JNIEnv<'_>, &JObject<'_>, &JObjectArray<'_>) -> GlobalRef
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
    let lock = HOOK_OBJECTS_OTHER.lock();
    if let Some(code) = lock.borrow().get(&proxy_hash_code) {
        return *code;
    };
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
    let lock = HOOK_OBJECTS_OTHER.lock();
    if let Some(code) = lock.borrow().get(&proxy_hash_code) {
        let lock = HOOK_OBJECTS.lock();
        lock.borrow_mut().remove_entry(code);
        debug!("Proxy `{}` is dropped.", proxy_hash_code);
    };
}

/**
解析JObject类型。
*/
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
    #[cfg(target_os = "android")]
    const BYTECODE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/classes.dex"));
    #[cfg(not(target_os = "android"))]
    const BYTECODE: &[u8] = &[];
    const LOADER_CLASS: &str = "dalvik/system/InMemoryDexClassLoader";
    static INSTANCE: OnceLock<GlobalRef> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        vm_attach!(mut env);
        let byte_buffer = unsafe { env.new_direct_byte_buffer(BYTECODE.as_ptr() as *mut u8, BYTECODE.len()) }.unwrap();

        let dex_class_loader = env
            .new_object(
                LOADER_CLASS,
                "(Ljava/nio/ByteBuffer;Ljava/lang/ClassLoader;)V",
                &[
                    JValueGen::Object(&JObject::from(byte_buffer)),
                    JValueGen::Object(&JObject::null()),
                ],
            ).unwrap();

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
}

//noinspection SpellCheckingInspection
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
        .unwrap_or(0);

    let name = match env.call_method(&method, "getName", "()Ljava/lang/String;", &[]) {
        Ok(name) => name.l(),
        Err(e) => {
            error!("{}", e);
            return JObject::null();
        }
    };
    let name = match name {
        Ok(name) => name,
        Err(e) => {
            error!("{}", e);
            return JObject::null();
        }
    };
    let name = match env.get_string((&name).into()) {
        Ok(name) => name,
        Err(e) => {
            error!("{}", e);
            return JObject::null();
        }
    };
    match name.to_str() {
        Ok(name) => match name {
            "toString" => {
                return env
                    .new_string(format!("Proxy@{:x}", hash_code).as_str())
                    .unwrap()
                    .into()
            }
            "equals" | "hashCode" => {
                return env
                    .call_method(
                        &method,
                        "invoke",
                        "(Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object;",
                        &[(&this).into(), (&args).into()],
                    )
                    .unwrap()
                    .l()
                    .unwrap()
            }
            _ => (),
        },
        Err(e) => {
            error!("{}", e);
            return JObject::null();
        }
    }

    let lock = HOOK_OBJECTS.lock();
    let func = if let Some(f) = lock.borrow().get(&hash_code) {
        f.to_owned()
    } else {
        warn!(
            "The method call has reached, but it appears that the proxy object has been dropped."
        );
        return JObject::null();
    };
    drop(lock);
    let ret = func(&mut env, &method, &args);
    env.new_local_ref(ret.as_obj()).unwrap()
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
/// vm_attach!(mut env);
/// unsafe {
///     let arr=JObjectArray::from_raw(0 as _);
///     to_vec(&mut env, &arr);
/// }
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

/// 将Rust数组转换为Java对象数组
///
/// 将一个Rust数组转换为Java数组，其中每个元素都是JObject类型。
///
/// # 参数
///
/// * `env` - 一个`JNIEnv`类型的引用，用于操作Java虚拟机。
/// * `arr` - 一个Rust数组，其中每个元素都是实现了`AsRef<JObject<'a>>` trait的类型。
/// * `element_class` - Java数组元素的类名。
///
/// # 返回值
///
/// 返回一个`JObjectArray`类型的Java数组。
///
/// # 示例
///
/// ```rust
/// use droid_wrap_utils::{ JNIEnv, JObject, JObjectArray, jint, to_java_object_array, vm_attach };
///
/// vm_attach!(mut env);
/// // 假设我们有一个Rust数组
/// let rust_array = vec![env.new_string("hello").unwrap(), env.new_string("world").unwrap()];
///
/// // 将Rust数组转换为Java数组
/// let java_array = to_java_object_array(&mut env, &rust_array, "java/lang/String");
/// ```
///
pub fn to_java_object_array<'a, O: AsRef<JObject<'a>>>(
    // env：JNIEnv<'a>类型，用于操作Java虚拟机
    env: &mut JNIEnv<'a>,
    // arr：Rust数组，类型为O，O需要实现AsRef<JObject<'a>> trait
    arr: &[O],
    // element_class：Java数组元素的类名
    element_class: &str,
) -> JObjectArray<'a> {
    // 创建一个新的Java数组，长度为arr.len()，元素类型为element_class，初始值为JObject::null()
    let arr2 = env
        .new_object_array(arr.len() as _, element_class, JObject::null())
        .unwrap();
    // 遍历Rust数组，将每个元素设置为Java数组的对应位置
    for (i, j) in arr.iter().enumerate() {
        env.set_object_array_element(&arr2, i as _, j).unwrap();
    }
    // 返回Java数组
    arr2
}

//noinspection SpellCheckingInspection
/// 将Rust数组转换为Java byte数组
///
/// 将一个Rust数组转换为Java数组，其中每个元素都是jbyte类型。
///
/// # 参数
///
/// * `env` - 一个`JNIEnv`类型的引用，用于操作Java虚拟机。
/// * `arr` - 一个Rust数组。
///
/// # 返回值
///
/// 返回一个`JBooleanArray`类型的Java数组。
///
/// # 示例
///
/// ```rust
/// use droid_wrap_utils::{ JNIEnv, JBooleanArray, jint, to_java_byte_array, vm_attach };
///
/// vm_attach!(mut env);
/// // 假设我们有一个Rust数组
/// let rust_array = vec![65i8,66,67];
///
/// // 将Rust数组转换为Java数组
/// let java_array = to_java_byte_array(&mut env, &rust_array);
/// ```
///
pub fn to_java_byte_array<'a>(
    // env：JNIEnv<'a>类型，用于操作Java虚拟机
    env: &mut JNIEnv<'a>,
    // arr：Rust[u8]数组
    arr: &[jbyte],
) -> JByteArray<'a> {
    // 创建一个新的Java数组，长度为arr.len()，元素类型为element_class，初始值为JObject::null()
    let arr2 = env.new_byte_array(arr.len() as _).unwrap();
    // 遍历Rust数组，将每个元素设置为Java数组的对应位置
    env.set_byte_array_region(&arr2, 0, arr).unwrap();
    // 返回Java数组
    arr2
}

/// 获取null的全局引用值。
///
/// # Arguments
///
/// * `env`: jni环境。
///
/// returns: GlobalRef
///
/// # Examples
///
/// ```
/// use droid_wrap_utils::{null_value, vm_attach};
/// vm_attach!(mut env);
/// let null_value = null_value(&mut env);
/// ```
pub fn null_value(env: &mut JNIEnv) -> GlobalRef {
    let obj = JObject::null();
    env.new_global_ref(&obj).unwrap()
}

/// 获取boolean的包装对象的全局引用值。
///
/// # Arguments
///
/// * `value`: 数据。
/// * `env`: jni环境。
///
/// returns: GlobalRef
///
/// # Examples
///
/// ```
/// use droid_wrap_utils::{vm_attach, wrapper_bool_value};
/// vm_attach!(mut env);
/// let true_value = wrapper_bool_value(true, &mut env);
/// ```
pub fn wrapper_bool_value(value: bool, env: &mut JNIEnv) -> GlobalRef {
    let obj = env
        .new_object("java/lang/Boolean", "(Z)V", &[(value as jboolean).into()])
        .unwrap();
    env.new_global_ref(&obj).unwrap()
}

/// 获取int的包装对象的全局引用值。
///
/// # Arguments
///
/// * `value`: 数据。
/// * `env`: jni环境。
///
/// returns: GlobalRef
///
/// # Examples
///
/// ```
/// use droid_wrap_utils::{vm_attach, wrapper_integer_value};
/// vm_attach!(mut env);
/// let zero_value = wrapper_integer_value(0, &mut env);
/// ```
pub fn wrapper_integer_value(value: i32, env: &mut JNIEnv) -> GlobalRef {
    let obj = env
        .new_object("java/lang/Integer", "(I)V", &[value.into()])
        .unwrap();
    env.new_global_ref(&obj).unwrap()
}

/// 获取long的包装对象的全局引用值。
///
/// # Arguments
///
/// * `value`: 数据。
/// * `env`: jni环境。
///
/// returns: GlobalRef
///
/// # Examples
///
/// ```
/// use droid_wrap_utils::{vm_attach, wrapper_long_value};
/// vm_attach!(mut env);
/// let zero_value = wrapper_long_value(0, &mut env);
/// ```
pub fn wrapper_long_value(value: i64, env: &mut JNIEnv) -> GlobalRef {
    let obj = env
        .new_object("java/lang/Long", "(J)V", &[value.into()])
        .unwrap();
    env.new_global_ref(&obj).unwrap()
}
