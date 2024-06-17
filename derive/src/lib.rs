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
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Expr, Field, FieldMutability, Fields, FieldsNamed, ItemFn, ItemStruct, MetaNameValue, Token,
    Type, Visibility,
};

use crate::utils::{
    get_object_return_value_token, get_return_value_token, parse_function_signature,
};

struct Metadata {
    class_name: Expr,
}

impl Parse for Metadata {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = Punctuated::<MetaNameValue, Token![,]>::parse_terminated(input)?;
        let cls = attrs
            .iter()
            .find(|i| i.path.is_ident("name"))
            .unwrap()
            .value
            .clone();
        Ok(Self { class_name: cls })
    }
}

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
    let attrs: Metadata = syn::parse2(Into::<TokenStream2>::into(attrs)).unwrap();
    let cls = attrs.class_name;
    let mut item = parse_macro_input!(input as ItemStruct);
    let mut add_field = Field {
        attrs: vec![],
        vis: Visibility::Inherited,
        mutability: FieldMutability::None,
        ident: Some(Ident::new("_obj", Span::call_site())),
        colon_token: None,
        ty: Type::Verbatim(quote! {droid_wrap_utils::GlobalRef}),
    };
    let (fields, added) = match item.fields {
        Fields::Named(mut f) => {
            f.named.push(add_field);
            (Fields::Named(f), quote! {self._obj})
        }
        Fields::Unnamed(mut f) => {
            add_field.ident = None;
            f.unnamed.push(add_field);
            let len = f.unnamed.len().to_string();
            (Fields::Unnamed(f), quote! {self.#len})
        }
        Fields::Unit => {
            let mut fields = Punctuated::<Field, Token![,]>::new();
            fields.push(add_field);
            (
                Fields::Named(FieldsNamed {
                    brace_token: Default::default(),
                    named: fields,
                }),
                quote! {self._obj},
            )
        }
    };
    item.fields = fields;
    let name = item.ident.clone();
    let generics = item.generics.clone();

    let stream = quote! {
        #item

        impl<'j> JType<'j> for #name #generics {
            type Error = droid_wrap_utils::Error;
            const CLASS: &'j str = #cls;
        }

        impl<'j> JObjRef<'j> for #name #generics {
            fn java_ref(&self) -> droid_wrap_utils::GlobalRef {
                #added.clone()
            }
        }

        impl #generics ToString for #name #generics {
            fn to_string(&self) -> String {
                droid_wrap_utils::vm_attach(|env| {
                    let s = env.call_method(self.java_ref(), "toString", format!("()L{};", String::CLASS).as_str(), &[]).unwrap().l().unwrap();
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

        impl #generics std::ops::Deref for #name #generics {
            type Target = droid_wrap_utils::GlobalRef;

            fn deref(&self) -> &Self::Target {
                &#added
            }
        }
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
    let (self_, arg_types, fmt, arg_values, ret_type_sig, ret_type) =
        parse_function_signature(&sig);

    let (ret_value, ret_type_sig) = get_return_value_token(&ret_type_sig, ret_type);

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
    let (self_, arg_types, fmt, arg_values, _, ret_type) = parse_function_signature(&sig);

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

    let (ret_value, _) = get_object_return_value_token(ret_type);

    let stream = quote! {
        #attrs
        #vis #sig {
            droid_wrap_utils::vm_attach(|env| {
                let obj = env.new_object(
                    Self::CLASS,
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
