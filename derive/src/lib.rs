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

mod utils;

use proc_macro::TokenStream;

use heck::ToLowerCamelCase;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use syn::{
    parse2, parse_macro_input, punctuated::Punctuated, Field, FieldMutability, Fields, FieldsNamed,
    ImplItem, ItemFn, ItemImpl, ItemStruct, ItemTrait, Token, TraitItem, Type, TypeParamBound,
    Visibility,
};

use crate::utils::{
    get_object_return_value_token, get_return_value_token, parse_function_signature, ClassMetadata,
    InterfaceMetadata,
};

//noinspection SpellCheckingInspection
/// 定义java class，将此属性标记在struct上，可以自动实现操作java对象的必要功能。
///
/// # Arguments
///
/// * `attrs`: 属性输入。
/// * `input`: struct输入。
///
/// returns: TokenStream
///
/// # Examples
///
/// ```
/// use droid_wrap_derive::java_class;
/// #[java_class(name = "java/lang/System")]
/// struct System;
/// ```
#[proc_macro_attribute]
pub fn java_class(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let attrs: ClassMetadata = parse2(Into::<TokenStream2>::into(attrs)).unwrap();
    let cls = attrs.class_name;
    let based = attrs.base_class;
    let mut item = parse_macro_input!(input as ItemStruct);
    let mut add_field_this = Field {
        attrs: vec![],
        vis: Visibility::Inherited,
        mutability: FieldMutability::None,
        ident: Some(Ident::new("_this", Span::call_site())),
        colon_token: None,
        ty: Type::Verbatim(quote! {droid_wrap_utils::GlobalRef}),
    };
    let mut add_field_super = Field {
        attrs: vec![],
        vis: Visibility::Inherited,
        mutability: FieldMutability::None,
        ident: Some(Ident::new("_based", Span::call_site())),
        colon_token: None,
        ty: Type::Verbatim(based.to_token_stream()),
    };

    let (fields, added_this, added_super) = match item.fields {
        Fields::Named(mut f) => {
            f.named.push(add_field_this);
            let g = if based.is_some() {
                f.named.push(add_field_super);
                quote! {_based}
            } else {
                quote!()
            };
            (Fields::Named(f), quote! {_this}, g)
        }
        Fields::Unnamed(mut f) => {
            add_field_this.ident = None;
            add_field_super.ident = None;
            f.unnamed.push(add_field_this);
            let len = f.unnamed.len().to_string();
            let g = if based.is_some() {
                f.unnamed.push(add_field_super);
                let len = f.unnamed.len().to_string();
                quote! {#len}
            } else {
                quote!()
            };
            (Fields::Unnamed(f), quote! {#len}, g)
        }
        Fields::Unit => {
            let mut fields = Punctuated::<Field, Token![,]>::new();
            fields.push(add_field_this);
            let g = if based.is_some() {
                fields.push(add_field_super);
                quote! {_based}
            } else {
                quote!()
            };
            (
                Fields::Named(FieldsNamed {
                    brace_token: Default::default(),
                    named: fields,
                }),
                quote! {_this},
                g,
            )
        }
    };
    item.fields = fields;

    let name = item.ident.clone();
    let generics = item.generics.clone();

    let impl_new = if based.is_some() {
        quote! {
            fn _new(this: &droid_wrap_utils::GlobalRef) -> Self {
                Self {#added_this: this.clone(), #added_super: this.into()}
            }
        }
    } else {
        quote! {
            fn _new(this: &droid_wrap_utils::GlobalRef) -> Self {
                Self {#added_this:this.clone()}
            }
        }
    };

    let impl_based_deref = if based.is_some() {
        quote! {
            impl #generics std::ops::Deref for #name #generics {
                type Target = #based;

                fn deref(&self) -> &Self::Target {
                    &self.#added_super
                }
            }
        }
    } else {
        quote! {
            impl #generics std::ops::Deref for #name #generics {
                type Target = droid_wrap_utils::GlobalRef;

                fn deref(&self) -> &Self::Target {
                    &self.#added_this
                }
            }
        }
    };

    let stream = quote! {
        #item

        impl #generics JObjNew for #name #generics {
            #impl_new
        }

        impl #generics JType for #name #generics {
            type Error = droid_wrap_utils::Error;
            const CLASS: &'static str = #cls;
        }

        impl #generics JObjRef for #name #generics {
            fn java_ref(&self) -> droid_wrap_utils::GlobalRef {
                self.#added_this.clone()
            }
        }

        impl #generics PartialEq for #name #generics {
            fn eq(&self, other: &Self) -> bool {
                let r = self.java_ref();
                let t = other.java_ref();
                if r.is_null() && t.is_null() {
                    return true;
                }
                if r.is_null() {
                    return false;
                }
                droid_wrap_utils::vm_attach(|env| {
                    env.call_method(
                        r.clone(),
                        "equals",
                        "(Ljava/lang/Object;)Z",
                        &[t.as_obj().into()]
                    ).unwrap().z().unwrap()
                })
            }
    }

        impl #generics ToString for #name #generics {
            fn to_string(&self) -> String {
                let r = self.java_ref();
                if r.is_null() {
                    return "null".to_string();
                }
                droid_wrap_utils::vm_attach(|env| {
                    let s = env.call_method(r.clone(), "toString", format!("()L{};", String::CLASS).as_str(), &[]).unwrap().l().unwrap();
                    let s = env.get_string((&s).into()).unwrap();
                    s.to_str().unwrap().to_string()
                })
            }
        }

        impl #generics std::fmt::Debug for #name #generics {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.to_string())
            }
        }

        impl #generics From<&droid_wrap_utils::GlobalRef> for #name #generics {
            fn from(obj: &droid_wrap_utils::GlobalRef) -> Self {
                Self::_new(&obj)
            }
        }

        #impl_based_deref
    };
    stream.into()
}

/// 实现java类的方法，将此属性标记在fn函数上，可以自动实现调用java方法，可以自动识别静态方法（如果参数中没有“self”）。
///
/// # Arguments
///
/// * `_`: 未使用。
/// * `input`: 函数输入。
///
/// returns: TokenStream
///
/// # Examples
///
/// ```
/// use droid_wrap_derive::java_method;
/// struct System;
/// impl System {
/// #[java_method]
/// fn current_time_millis() -> i64 {}
/// }
/// ```
#[proc_macro_attribute]
pub fn java_method(_: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemFn);
    let mut attrs = TokenStream2::new();
    for i in item.attrs.iter() {
        attrs.extend(i.to_token_stream());
    }
    let name = item.sig.ident.to_string().to_lower_camel_case();
    let vis = item.vis.clone();
    let sig = item.sig.clone();

    let (self_, _, arg_types, fmt, arg_values, ret_type) = parse_function_signature(&sig);
    let (ret_value, ret_type_sig) = get_return_value_token(&ret_type);

    if self_.is_none() {
        quote! {
            #attrs
            #vis #sig {
                droid_wrap_utils::vm_attach(|env| {
                    let ret = env.call_static_method(
                        Self::CLASS,
                        #name,
                        format!(#fmt, #arg_types #ret_type_sig).as_str(),
                        &[#arg_values],
                    )
                    .unwrap();
                    #ret_value
                })
            }
        }
    } else {
        quote! {
            #attrs
            #vis #sig {
                droid_wrap_utils::vm_attach(|env| {
                    let ret = env.call_method(
                        #self_.java_ref(),
                        #name,
                        format!(#fmt, #arg_types #ret_type_sig).as_str(),
                        &[#arg_values],
                    )
                    .unwrap();
                    #ret_value
                })
            }
        }
    }
    .into()
}

/// 实现java类的构造器，将此属性标记在fn函数上，可以自动实现调用java类的构造器。
///
/// # Arguments
///
/// * `_`: 未使用。
/// * `input`: 函数输入。
///
/// returns: TokenStream
///
/// # Examples
///
/// ```
/// use droid_wrap_derive::java_constructor;
/// struct Integer;
/// impl Integer {
/// #[java_constructor]
/// fn new(value: i32) -> Self {}
/// }
/// ```
#[proc_macro_attribute]
pub fn java_constructor(_: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemFn);
    let mut attrs = TokenStream2::new();
    for i in item.attrs.iter() {
        attrs.extend(i.to_token_stream());
    }
    let vis = item.vis.clone();
    let sig = item.sig.clone();
    let block = item.block.clone();
    let (self_, _, arg_types, fmt, arg_values, ret_type) = parse_function_signature(&sig);

    if !self_.is_none() {
        panic!(
            "Incorrect constructor, please remove the '{}' in the arguments!",
            self_.to_token_stream()
        );
    }

    if !ret_type.to_string().contains("Self") {
        panic!(
            "Incorrect constructor, please modify the '{}' to 'Self', 'Option<Self>' or 'Result<Self, Self::Error>' in the return value!",
            ret_type
        );
    }

    let ret_value = get_object_return_value_token(&ret_type);

    let stream = quote! {
        #attrs
        #vis #sig {
            #block
            droid_wrap_utils::vm_attach(|env| {
                let obj = env.new_object(
                    <Self as JType>::CLASS,
                    format!(#fmt, #arg_types "V").as_str(),
                    &[#arg_values],
                )
                .unwrap();
                #ret_value
            })
        }
    };

    stream.into()
}

/// 定义java interface，将此属性标记在trait上，可以自动实现提供java对象与rust对象的互操作的功能。
///
/// # Arguments
///
/// * `attrs`: 属性。
/// * `input`: 特征输入。
///
/// returns: TokenStream
///
/// # Examples
///
/// ```
/// use droid_wrap_derive::java_interface;
/// trait Runnable {
/// fn run(&self);
/// }
/// ```
#[proc_macro_attribute]
pub fn java_interface(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let attrs: InterfaceMetadata = parse2(Into::<TokenStream2>::into(attrs)).unwrap();
    let cls = attrs.interface_name;
    let mut item = parse_macro_input!(input as ItemTrait);
    item.supertraits
        .push(TypeParamBound::Verbatim(quote! {JObjRef}));
    item.supertraits
        .push(TypeParamBound::Verbatim(quote! {JObjNew}));
    item.supertraits
        .push(TypeParamBound::Verbatim(quote! {PartialEq}));
    item.supertraits
        .push(TypeParamBound::Verbatim(quote! {std::fmt::Debug}));

    item.items.push(TraitItem::Verbatim(
        quote! {const CLASS: &'static str = #cls;},
    ));
    item.items.push(TraitItem::Verbatim(quote! {
        fn get_object_sig() -> String {
            concat!("L", #cls, ";").to_string()
        }
    }));
    let stream = quote! {
        #item
    };
    stream.into()
}

//noinspection SpellCheckingInspection
/// 实现java interface，将此属性标记在impl上，可以自动实现java接口的动态代理，从而实现java层回调rust层。
///
/// # Arguments
///
/// * `attrs`: 属性。
/// * `input`: impl输入。
///
/// returns: TokenStream
///
/// # Examples
///
/// ```
/// use std::fmt::{Debug, Formatter};
/// use droid_wrap_derive::{java_interface,java_implement};
/// #[java_interface(name = "java/lang/Runnable")]
/// trait Runnable {
/// fn run(&self);
/// }
/// struct RunnableImpl;
/// impl PartialEq for RunnableImpl {
///     fn eq(&self, other: &Self) -> bool {
///         todo!()
///     }
/// }
/// impl Debug for RunnableImpl {
///     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
///         todo!()
///     }
/// }
/// #[java_implement]
/// impl Runnable for RunnableImpl {
/// fn run(&self) {
/// println!("Called from java.");
/// }
/// }
/// ```
#[proc_macro_attribute]
pub fn java_implement(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let item: TokenStream2 = input.into();
    let attrs: TokenStream2 = attrs.into();

    let item = parse2::<ItemImpl>(item.clone()).unwrap();

    let mut methods = TokenStream2::new();
    for item in item.items.iter() {
        match item {
            ImplItem::Fn(f) => {
                let name = f.sig.ident.clone();
                let name_camel = f.sig.ident.to_string().to_lower_camel_case();
                let (self_, arg_types, _, _, _, ret_type) = parse_function_signature(&f.sig);
                if self_.is_none() {
                    continue;
                }
                let ret_token = match ret_type.to_string().as_str() {
                    "()" => quote! {Self::null().java_ref()},
                    _ => quote! {(&ret).into()},
                };
                let mut arg_tokens = TokenStream2::new();
                for i in 0..arg_types.len() {
                    let ty = &arg_types[i];
                    arg_tokens.extend(quote! {
                        droid_wrap_utils::ParseJObjectType::<#ty>::parse(&args2[#i], env),
                    });
                }
                methods.extend(quote! {
                    Ok(#name_camel) => {
                        let size = env.get_array_length(args).unwrap();
                        let mut args2 = Vec::with_capacity(size as usize);
                        for i in 0..size {
                            args2.push(env.get_object_array_element(&args, i).unwrap());
                        }
                        let ret = self_.#name(#arg_tokens);
                        #ret_token
                    },
                })
            }
            _ => {}
        }
    }
    // println!("b{}", methods);

    let name = item.self_ty.clone();
    let class_token = match item.trait_ {
        None => quote! {Self::CLASS},
        Some((_, ref p, _)) => quote! {<Self as #p>::CLASS},
    };

    let impl_new = quote! {
        fn new() -> Self {
            use std::sync::{Arc, OnceLock};
            let interface = #class_token.replace("/", ".");
            let self_: Arc<OnceLock<isize>> = OnceLock::new().into();
            let self_2 = self_.clone();
            let proxy = droid_wrap_utils::new_proxy(&[&interface], move |env, method, args| {
                let self_ = unsafe { &*((*self_2.get().unwrap()) as *const Self) };
                let name = env.call_method(&method, "getName", "()Ljava/lang/String;", &[]).unwrap().l().unwrap();
                let name = env.get_string((&name).into()).unwrap();
                match name.to_str() {
                    #methods
                    _ => Self::null().java_ref()
                }
            });
            let ret = Self::_new(&proxy);
            self_.get_or_init(|| &ret as *const Self as isize);
            ret
        }
    };

    let stream = quote! {
        #attrs
        #item

        impl #name {
            #impl_new
        }

        impl Drop for #name {
            fn drop(&mut self) {
                droid_wrap_utils::drop_proxy(&self._this);
            }
        }
    };

    stream.into()
}
