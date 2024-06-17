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
    base_class: Option<Expr>,
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
        let based = match attrs.iter().find(|i| i.path.is_ident("extends")) {
            Some(o) => Some(o.value.clone()),
            None => None,
        };
        Ok(Self {
            class_name: cls,
            base_class: based,
        })
    }
}

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
    let attrs: Metadata = syn::parse2(Into::<TokenStream2>::into(attrs)).unwrap();
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
            impl #generics #name #generics {
                pub(crate) fn _new(this: &droid_wrap_utils::GlobalRef) -> Self {
                    Self {#added_this: this.clone(), #added_super: this.into()}
                }
            }
        }
    } else {
        quote! {
            impl #generics #name #generics {
                pub(crate) fn _new(this: &droid_wrap_utils::GlobalRef) -> Self {
                    Self {#added_this:this.clone()}
                }
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

        #impl_new

        impl<'j> JType<'j> for #name #generics {
            type Error = droid_wrap_utils::Error;
            const CLASS: &'j str = #cls;
        }

        impl<'j> JObjRef<'j> for #name #generics {
            fn java_ref(&self) -> droid_wrap_utils::GlobalRef {
                self.#added_this.clone()
            }
        }

        impl #generics PartialEq for #name #generics {
            fn eq(&self, other: &Self) -> bool {
                droid_wrap_utils::vm_attach(|env| {
                    env.call_method(
                        self.java_ref(),
                        "equals",
                        "(Ljava/lang/Object;)Z",
                        &[other.java_ref().as_obj().into()]
                    ).unwrap().z().unwrap()
                })
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
