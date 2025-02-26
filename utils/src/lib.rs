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

mod error;

pub use error::*;

pub use jni::{
    AttachGuard, JNIEnv, JavaVM, NativeMethod,
    errors::Error as JniError,
    objects::{
        GlobalRef, JBooleanArray, JByteArray, JClass, JObject, JObjectArray, JString, JValueGen,
        ReleaseMode,
    },
    sys::{jboolean, jbyte, jchar, jdouble, jfloat, jint, jlong, jshort, jsize},
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
                    dyn Fn(&mut JNIEnv<'_>, &JObject<'_>, &JObjectArray<'_>) -> Result<GlobalRef>
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
导出所有的pub条目。
这还将定义必要的trait，以便于在本地为任何数据类型实现JAVA对象所需的功能。
*/
#[macro_export]
macro_rules! import {
    () => {
        use std::{
            rc::Rc,
            sync::{Arc, Mutex},
        };
        pub use $crate::Result;
        use $crate::{
            GlobalRef, JObject, impl_array, null_value, to_java_byte_array, to_java_object_array,
            unbind_proxy_handler, vm_attach,
        };

        /**
        JObjectRef trait提供从任何数据类型获取java对象的全局引用。
        rust使用Arc管理，无须手动释放java的全局引用。
        */
        pub trait JObjRef {
            /**
            获取java对象引用。
            */
            fn java_ref(&self) -> Result<GlobalRef>;
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
            fn _new(this: &GlobalRef, fields: Self::Fields) -> Result<Self>
            where
                Self: Sized;

            /**
            创建空对象。
            */
            fn null() -> Result<Self>
            where
                Self: Sized,
                Self::Fields: Default,
            {
                let mut env = vm_attach()?;
                Self::_new(null_value(&mut env)?.as_ref(), Default::default())
            }
        }

        /**
        用于描述java类的信息。
        */
        pub trait JType: JObjRef + JObjNew {
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
            fn new(fields: Self::Fields) -> Result<Arc<Self>>;

            /**
            释放代理对象，解除绑定的handler。
            */
            fn release(&self) -> () {
                if let Ok(ref r) = self.java_ref() {
                    unbind_proxy_handler(r)
                }
            }
        }

        impl<T: JObjRef> JObjRef for &T {
            fn java_ref(&self) -> Result<GlobalRef> {
                self.java_ref()
            }
        }

        impl<T: JObjNew> JObjNew for &T {
            type Fields = T::Fields;

            fn _new(this: &GlobalRef, fields: Self::Fields) -> Result<Self> {
                panic!("Reference types cannot be constructed.")
            }
        }

        impl<T: JObjRef + JObjNew> JObjRef for Option<T>
        where
            <T as JObjNew>::Fields: Default,
        {
            fn java_ref(&self) -> Result<GlobalRef> {
                match self {
                    None => T::null()?.java_ref(),
                    Some(v) => v.java_ref(),
                }
            }
        }

        impl<T: JObjRef + JObjNew> JObjNew for Option<T> {
            type Fields = T::Fields;

            fn _new(this: &GlobalRef, fields: Self::Fields) -> Result<Self> {
                Ok(match this.is_null() {
                    true => None,
                    false => T::_new(this, fields).ok(),
                })
            }
        }

        impl<T: JType> JType for Arc<T> {
            const CLASS: &'static str = T::CLASS;
            const OBJECT_SIG: &'static str = T::OBJECT_SIG;
        }

        impl<T: JObjRef> JObjRef for Arc<T> {
            fn java_ref(&self) -> Result<GlobalRef> {
                self.as_ref().java_ref()
            }
        }

        impl<T: JObjNew> JObjNew for Arc<T> {
            type Fields = T::Fields;

            fn _new(this: &GlobalRef, fields: Self::Fields) -> Result<Self> {
                Ok(T::_new(this, fields)?.into())
            }
        }

        impl<T: JType> JType for Rc<T> {
            const CLASS: &'static str = T::CLASS;
            const OBJECT_SIG: &'static str = T::OBJECT_SIG;
        }

        impl<T: JObjRef> JObjRef for Rc<T> {
            fn java_ref(&self) -> Result<GlobalRef> {
                self.as_ref().java_ref()
            }
        }

        impl<T: JObjNew> JObjNew for Rc<T> {
            type Fields = T::Fields;

            fn _new(this: &GlobalRef, fields: Self::Fields) -> Result<Self> {
                Ok(T::_new(this, fields)?.into())
            }
        }

        impl<T: JType> JType for Mutex<T> {
            const CLASS: &'static str = T::CLASS;
            const OBJECT_SIG: &'static str = T::OBJECT_SIG;
        }

        impl<T: JObjNew> JObjNew for Mutex<T> {
            type Fields = T::Fields;

            fn _new(this: &GlobalRef, fields: Self::Fields) -> Result<Self> {
                Ok(T::_new(this, fields)?.into())
            }
        }

        impl<T: JObjRef> JObjRef for Mutex<T> {
            fn java_ref(&self) -> Result<GlobalRef> {
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

            fn _new(this: &GlobalRef, fields: Self::Fields) -> Result<Self> {
                Ok(&[])
            }
        }

        impl JObjRef for &[String] {
            fn java_ref(&self) -> Result<GlobalRef> {
                let mut env = vm_attach()?;
                let arr = self
                    .iter()
                    .filter_map(|i| env.new_string(i).ok())
                    .collect::<Vec<_>>();
                let sig = if Self::DIM <= 1 {
                    Self::CLASS.to_string()
                } else {
                    "[".repeat((Self::DIM - 1) as _) + Self::OBJECT_SIG
                };
                let arr = to_java_object_array(&mut env, &arr, &sig)?;
                Ok(env.new_global_ref(&arr)?)
            }
        }

        impl JType for &[String] {
            const CLASS: &'static str = <String as JType>::CLASS;
            const OBJECT_SIG: &'static str = <String as JType>::OBJECT_SIG;
            const DIM: u8 = $dim;
        }
    };

    (u8, $dim:expr) => {
        impl JObjNew for &[u8] {
            type Fields = ();

            fn _new(this: &GlobalRef, fields: Self::Fields) -> Result<Self> {
                Ok(&[])
            }
        }

        impl JObjRef for &[u8] {
            fn java_ref(&self) -> Result<GlobalRef> {
                let mut env = vm_attach()?;
                let arr = self.iter().map(|i| *i as _).collect::<Vec<_>>();
                let arr = to_java_byte_array(&mut env, &arr)?;
                Ok(env.new_global_ref(&arr)?)
            }
        }

        impl JType for &[u8] {
            const CLASS: &'static str = "B";
            const OBJECT_SIG: &'static str = "B";
            const DIM: u8 = $dim;
        }
    };
}

/**
获取android系统的java虚拟机。
*/
pub fn android_vm<'a>() -> Result<&'static JavaVM> {
    static JAVA_VM: LazyLock<Result<JavaVM>> = LazyLock::new(|| {
        let ctx = ndk_context::android_context();
        let vm = unsafe { JavaVM::from_raw(ctx.vm().cast()) }?;
        Ok(vm)
    });
    JAVA_VM.as_ref().map_err(|e| e.to_owned())
}

/// 获取vm，将vm附加到当前线程，随后操作java虚拟机。
///
/// # 示例
///
/// ```
/// use droid_wrap_utils::{vm_attach, Result};
/// fn main() -> Result<()> {
/// let mut env = vm_attach()?;
/// let class = env.find_class("java/lang/String");
/// dbg!(class)
/// }
/// ```
#[inline(always)]
pub fn vm_attach<'a>() -> Result<AttachGuard<'a>> {
    let vm = android_vm()?;
    vm.attach_current_thread().map_err(|e| e.into())
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
/// returns: Result<GlobalRef> 代理对象
///
/// # 示例
///
/// ```
/// use droid_wrap_utils::new_proxy;
/// let proxy = new_proxy(&["java.lang.Runnable"]);
/// ```
//noinspection SpellCheckingInspection
pub fn new_proxy(interfaces: &[&str]) -> Result<GlobalRef> {
    let class = load_rust_call_method_hook_class()?;
    let mut env = vm_attach()?;
    let obj = env.new_object(class, "()V", &[])?;
    let faces = env.new_object_array(
        interfaces.len() as jsize,
        "java/lang/Class",
        &JObject::null(),
    )?;
    for i in 0..interfaces.len() {
        let class = env.new_string(interfaces[i])?;
        let face = env
            .call_static_method(
                "java/lang/Class",
                "forName",
                "(Ljava/lang/String;)Ljava/lang/Class;",
                &[(&class).into()],
            )?
            .l()?;
        env.set_object_array_element(&faces, i as jsize, &face)?;
    }
    let hash_code = env.call_method(&obj, "hashCode", "()I", &[])?.i()?;
    let res = env.call_static_method(
        "java/lang/reflect/Proxy",
        "newProxyInstance",
        "(Ljava/lang/ClassLoader;[Ljava/lang/Class;Ljava/lang/reflect/InvocationHandler;)Ljava/lang/Object;",
        &[
            (&JObject::null()).into(),
            (&faces).into(),
            (&obj).into()
        ]
    )?
        .l()?;
    let res = env.new_global_ref(&res)?;
    let lock = HOOK_OBJECTS_OTHER.lock();
    let mut hasher = DefaultHasher::new();
    res.hash(&mut hasher);
    lock.borrow_mut().insert(hasher.finish(), hash_code);
    drop(lock);
    Ok(res)
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
/// # 示例
///
/// ```
/// use droid_wrap_utils::{bind_proxy_handler, new_proxy, vm_attach};
/// let proxy = new_proxy(&["java.lang.Runnable"]).unwrap();
/// bind_proxy_handler(&proxy, |mut env, method, args| {
///     let name = env.call_method(&method, "getName", "()Ljava/lang/String;", &[])?.l()?;
///     let name = env.get_string((&name).into())?;
///     println!("Method `{}` is called with proxy.", name.to_str()?);
///     droid_wrap_utils::null_value(env)
/// });
/// let mut env = vm_attach().unwrap();
/// env.call_method(&proxy, "run", "()V", &[]).unwrap();
/// ```
pub fn bind_proxy_handler(
    proxy: &GlobalRef,
    handler: impl Fn(&mut JNIEnv<'_>, &JObject<'_>, &JObjectArray<'_>) -> Result<GlobalRef>
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
/// # 示例
///
/// ```
/// use droid_wrap_utils::{new_proxy, get_proxy_hash_code};
/// let proxy = new_proxy(&["java.lang.Runnable"]).unwrap();
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
/// # 示例
///
/// ```
/// use droid_wrap_utils::{unbind_proxy_handler, new_proxy};
/// let proxy = new_proxy(&["java.lang.Runnable"]).unwrap();
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
    fn parse(&self, env: &mut JNIEnv) -> Result<T>
    where
        <T as FromStr>::Err: Debug;
}

impl<T: FromStr> ParseJObjectType<T> for JObject<'_> {
    //noinspection SpellCheckingInspection
    fn parse(&self, env: &mut JNIEnv) -> Result<T>
    where
        <T as FromStr>::Err: Debug,
    {
        let s = env
            .call_method(self, "toString", "()Ljava/lang/String;", &[])?
            .l()?;
        let s = env.get_string((&s).into())?;
        let s = s.to_str()?;
        Ok(s.parse()
            .map_err(|_| DroidWrapError::FromStr(format!("Invalid value: {}", s)))?)
    }
}

//noinspection SpellCheckingInspection
fn load_rust_call_method_hook_class<'a>() -> Result<&'a GlobalRef> {
    #[cfg(target_os = "android")]
    const BYTECODE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/classes.dex"));
    #[cfg(not(target_os = "android"))]
    const BYTECODE: &[u8] = &[];
    const LOADER_CLASS: &str = "dalvik/system/InMemoryDexClassLoader";
    static INSTANCE: OnceLock<Result<GlobalRef>> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        let mut env = vm_attach()?;
        let byte_buffer = unsafe { env.new_direct_byte_buffer(BYTECODE.as_ptr() as *mut u8, BYTECODE.len()) }?;

        let dex_class_loader = env
            .new_object(
                LOADER_CLASS,
                "(Ljava/nio/ByteBuffer;Ljava/lang/ClassLoader;)V",
                &[
                    JValueGen::Object(&JObject::from(byte_buffer)),
                    JValueGen::Object(&JObject::null()),
                ],
            )?;

        let class = env.new_string("rust/CallMethodHook")?;
        let class = env
            .call_method(
                &dex_class_loader,
                "loadClass",
                "(Ljava/lang/String;)Ljava/lang/Class;",
                &[(&class).into()],
            )?
            .l()?;
        let m = NativeMethod {
            name: "invoke".into(),
            sig: "(Ljava/lang/Object;Ljava/lang/reflect/Method;[Ljava/lang/Object;)Ljava/lang/Object;".into(),
            fn_ptr: rust_callback as *mut _,
        };
        env.register_native_methods(Into::<&JClass<'_>>::into(&class), &[m])?;

        Ok(env.new_global_ref(&class)?)
    }).as_ref().map_err(|e| e.clone())
}

//noinspection SpellCheckingInspection
unsafe extern "C" fn rust_callback<'a>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
    _: JObject<'a>,
    method: JObject<'a>,
    args: JObjectArray<'a>,
) -> JObject<'a> {
    fn get_hash_code<'a>(env: &mut JNIEnv<'a>, this: &JObject<'a>) -> Result<i32> {
        Ok::<_, DroidWrapError>(env.call_method(this, "hashCode", "()I", &[])?.i()?)
    }

    fn get_name<'a>(env: &mut JNIEnv<'a>, method: &JObject<'a>) -> Result<String> {
        let name = env
            .call_method(method, "getName", "()Ljava/lang/String;", &[])?
            .l()?;
        Ok::<_, DroidWrapError>(env.get_string((&name).into())?.to_str()?.to_string())
    }

    let hash_code = get_hash_code(&mut env, &this).unwrap_or_default();

    match get_name(&mut env, &method).unwrap_or_default().as_str() {
        "toString" => {
            return match env.new_string(format!("Proxy@{:x}", hash_code).as_str()) {
                Ok(r) => r.into(),
                _ => JObject::null(),
            };
        }
        "equals" | "hashCode" => {
            return match env.call_method(
                &method,
                "invoke",
                "(Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object;",
                &[(&this).into(), (&args).into()],
            ) {
                Ok(r) => match r.l() {
                    Ok(r) => r,
                    Err(e) => {
                        error!("{}", e);
                        return JObject::null();
                    }
                },
                Err(e) => {
                    error!("{}", e);
                    return JObject::null();
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
            "The method call has reached, but it appears that the proxy object has been dropped."
        );
        return JObject::null();
    };
    drop(lock);

    match func(&mut env, &method, &args) {
        Ok(ret) => match env.new_local_ref(ret.as_obj()) {
            Ok(r) => return r,
            Err(e) => {
                error!("{}", e);
                JObject::null()
            }
        },
        Err(e) => {
            error!("{}", e);
            JObject::null()
        }
    }
}

/// 把java对象数组转换成Vec
///
/// # 参数
///
/// * `env`: java环境
/// * `arr`: java数组
///
/// # 返回值
///
/// 返回: Result<Vec<JObject, Global>>
///
/// # 示例
///
/// ```
/// use jni::objects::JObjectArray;
/// use droid_wrap_utils::{to_vec, vm_attach};
/// let mut env = vm_attach().unwrap();
/// unsafe {
///     let arr=JObjectArray::from_raw(0 as _);
///     to_vec(&mut env, &arr).unwrap();
/// }
/// ```
pub fn to_vec<'a>(env: &mut JNIEnv<'a>, arr: &JObjectArray) -> Result<Vec<JObject<'a>>> {
    let size = env.get_array_length(arr)?;
    let mut arr2 = Vec::with_capacity(size as usize);
    for i in 0..size {
        arr2.push(env.get_object_array_element(arr, i)?);
    }

    Ok(arr2)
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
/// 返回: Result<JObjectArray>
///
/// # 示例
///
/// ```rust
/// use droid_wrap_utils::{ JNIEnv, JObject, JObjectArray, jint, to_java_object_array, vm_attach };
///
/// let mut env = vm_attach().unwrap();
/// // 假设我们有一个Rust数组
/// let rust_array = vec![env.new_string("hello").unwrap(), env.new_string("world").unwrap()];
///
/// // 将Rust数组转换为Java数组
/// let java_array = to_java_object_array(&mut env, &rust_array, "java/lang/String").unwrap();
/// ```
///
pub fn to_java_object_array<'a, O: AsRef<JObject<'a>>>(
    // env：JNIEnv<'a>类型，用于操作Java虚拟机
    env: &mut JNIEnv<'a>,
    // arr：Rust数组，类型为O，O需要实现AsRef<JObject<'a>> trait
    arr: &[O],
    // element_class：Java数组元素的类名
    element_class: &str,
) -> Result<JObjectArray<'a>> {
    // 创建一个新的Java数组，长度为arr.len()，元素类型为element_class，初始值为JObject::null()
    let arr2 = env.new_object_array(arr.len() as _, element_class, JObject::null())?;
    // 遍历Rust数组，将每个元素设置为Java数组的对应位置
    for (i, j) in arr.iter().enumerate() {
        env.set_object_array_element(&arr2, i as _, j)?;
    }

    // 返回Java数组
    Ok(arr2)
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
/// let mut env = vm_attach().unwrap();
/// // 假设我们有一个Rust数组
/// let rust_array = vec![65i8,66,67];
///
/// // 将Rust数组转换为Java数组
/// let java_array = to_java_byte_array(&mut env, &rust_array).unwrap();
/// ```
///
pub fn to_java_byte_array<'a>(
    // env：JNIEnv<'a>类型，用于操作Java虚拟机
    env: &mut JNIEnv<'a>,
    // arr：Rust[u8]数组
    arr: &[jbyte],
) -> Result<JByteArray<'a>> {
    // 创建一个新的Java数组，长度为arr.len()，元素类型为element_class，初始值为JObject::null()
    let arr2 = env.new_byte_array(arr.len() as _)?;
    // 遍历Rust数组，将每个元素设置为Java数组的对应位置
    env.set_byte_array_region(&arr2, 0, arr)?;
    // 返回Java数组
    Ok(arr2)
}

/// 获取null的全局引用值。
///
/// # 参数
///
/// * `env`: jni环境。
///
/// # 返回值
///
/// 返回: Result<GlobalRef>
///
/// # 示例
///
/// ```
/// use droid_wrap_utils::{null_value, vm_attach};
/// let mut env = vm_attach().unwrap();
/// let null_value = null_value(&mut env);
/// ```
pub fn null_value(env: &mut JNIEnv) -> Result<GlobalRef> {
    let obj = JObject::null();
    Ok(env.new_global_ref(&obj)?)
}

/// 获取boolean的包装对象的全局引用值。
///
/// # 参数
///
/// * `value`: 数据。
/// * `env`: jni环境。
///
/// # 返回值
///
/// 返回: Result<GlobalRef>
///
/// # 示例
///
/// ```
/// use droid_wrap_utils::{vm_attach, wrapper_bool_value};
/// let mut env = vm_attach().unwrap();
/// let true_value = wrapper_bool_value(true, &mut env).unwrap();
/// ```
pub fn wrapper_bool_value(value: bool, env: &mut JNIEnv) -> Result<GlobalRef> {
    let obj = env.new_object("java/lang/Boolean", "(Z)V", &[(value as jboolean).into()])?;
    Ok(env.new_global_ref(&obj)?)
}

/// 获取int的包装对象的全局引用值。
///
/// # 参数
///
/// * `value`: 数据。
/// * `env`: jni环境。
///
/// # 返回值
///
/// 返回: Result<GlobalRef>
///
/// # 示例
///
/// ```
/// use droid_wrap_utils::{vm_attach, wrapper_integer_value};
/// let mut env = vm_attach().unwrap();
/// let zero_value = wrapper_integer_value(0, &mut env).unwrap();
/// ```
pub fn wrapper_integer_value(value: i32, env: &mut JNIEnv) -> Result<GlobalRef> {
    let obj = env.new_object("java/lang/Integer", "(I)V", &[value.into()])?;
    Ok(env.new_global_ref(&obj)?)
}

/// 获取long的包装对象的全局引用值。
///
/// # 参数
///
/// * `value`: 数据。
/// * `env`: jni环境。
///
/// # 返回值
///
/// 返回: Result<GlobalRef>
///
/// # 示例
///
/// ```
/// use droid_wrap_utils::{vm_attach, wrapper_long_value};
/// let mut env = vm_attach().unwrap();
/// let zero_value = wrapper_long_value(0, &mut env).unwrap();
/// ```
pub fn wrapper_long_value(value: i64, env: &mut JNIEnv) -> Result<GlobalRef> {
    let obj = env.new_object("java/lang/Long", "(J)V", &[value.into()])?;
    Ok(env.new_global_ref(&obj)?)
}

//noinspection SpellCheckingInspection
/// 比较两个Java对象是否相等。
///
/// # 参数
///
/// * `a`: 第一个对象。
/// * `b`: 第二个对象。
///
/// # 返回值
///
/// 返回: Result<bool>
///
/// # 示例
///
/// ```
/// use droid_wrap_utils::{vm_attach, java_object_equals};
/// let mut env = vm_attach().unwrap();
/// let obj1 = env.new_object("java/lang/Object", "()V", &[]).unwrap();
/// let obj2 = env.new_object("java/lang/Object", "()V", &[]).unwrap();
/// let equal = java_object_equals(obj1, obj2).unwrap();
/// ```
pub fn java_object_equals<'a, O: AsRef<JObject<'a>>>(a: O, b: O) -> Result<bool> {
    let a = a.as_ref();
    let b = b.as_ref();
    if a.is_null() && a.is_null() {
        return Ok(true);
    }
    if a.is_null() {
        return Ok(false);
    }
    let mut env = vm_attach()?;
    let res = env
        .call_method(a, "equals", "(Ljava/lang/Object;)Z", &[b.into()])?
        .z()?;

    Ok(res)
}

//noinspection SpellCheckingInspection
/// 将Java对象转换为字符串。
///
/// # 参数
///
/// * `obj`: Java对象。
///
/// # 返回值
///
/// 返回: Result<String>
///
/// # 示例
///
/// ```
/// use droid_wrap_utils::{vm_attach, java_object_to_string};
/// let mut env = vm_attach().unwrap();
/// let obj = env.new_object("java/lang/Object", "()V", &[]).unwrap();
/// let s = java_object_to_string(obj).unwrap();
/// ```
pub fn java_object_to_string<'a, O: AsRef<JObject<'a>>>(obj: O) -> Result<String> {
    let obj = obj.as_ref();
    if obj.is_null() {
        return Err(DroidWrapError::Jni(JniError::NullPtr("to_string")));
    }
    let mut env = vm_attach()?;
    let s = env
        .call_method(obj, "toString", "()Ljava/lang/String;", &[])?
        .l()?;
    let s = env.get_string((&s).into())?;

    Ok(s.to_str()?.to_string())
}
