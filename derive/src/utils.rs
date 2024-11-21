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
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse2,
    punctuated::Punctuated,
    token::SelfValue,
    Expr, FnArg, Generics, MetaNameValue, PathArguments, PathSegment, ReturnType, Signature, Token,
    Type, TypeReference,
};

pub(crate) struct ClassMetadata {
    pub(crate) class_name: Expr,
    pub(crate) base_class: Option<Expr>,
}

impl Parse for ClassMetadata {
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

pub(crate) struct FieldMetadata {
    pub(crate) default_value: Option<Expr>,
}

impl Parse for FieldMetadata {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = Punctuated::<MetaNameValue, Token![,]>::parse_terminated(input)?;
        let default_value = match attrs.iter().find(|i| i.path.is_ident("default_value")) {
            None => None,
            Some(v) => Some(v.value.clone()),
        };

        Ok(Self { default_value })
    }
}

pub(crate) struct InterfaceMetadata {
    pub(crate) interface_name: Expr,
}

impl Parse for InterfaceMetadata {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = Punctuated::<MetaNameValue, Token![,]>::parse_terminated(input)?;
        let cls = attrs
            .iter()
            .find(|i| i.path.is_ident("name"))
            .unwrap()
            .value
            .clone();
        Ok(Self {
            interface_name: cls,
        })
    }
}

pub(crate) struct MethodMetadata {
    pub(crate) type_bounds: Vec<(TokenStream, TokenStream)>,
    pub(crate) overload: Option<Expr>,
}

impl Parse for MethodMetadata {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = Punctuated::<MetaNameValue, Token![,]>::parse_terminated(input)?;
        let mut type_bounds = Vec::new();
        let mut overload = None;
        for item in attrs.iter() {
            if item.path.is_ident("type_bound") {
                match item.value {
                    Expr::Tuple(ref t) => {
                        type_bounds.push((
                            t.elems.first().to_token_stream(),
                            t.elems.iter().nth(1).to_token_stream(),
                        ));
                    }
                    _ => {}
                }
            } else if item.path.is_ident("overload") {
                overload = Some(item.value.clone());
            }
        }
        Ok(Self {
            type_bounds,
            overload,
        })
    }
}

fn unwrap_type(ty: &TokenStream) -> (TokenStream, TokenStream) {
    let res = parse2::<PathSegment>(ty.clone());

    let unwrapped = match res {
        Ok(item) => match item.arguments {
            PathArguments::AngleBracketed(arg) => arg.args.first().unwrap().to_token_stream(),
            _ => item.to_token_stream(),
        },
        _ => match parse2::<Type>(ty.clone()) {
            Ok(Type::Reference(TypeReference { elem, .. })) => elem.to_token_stream(),
            _ => ty.clone(),
        },
    };
    (unwrapped, ty.to_token_stream())
}

fn get_type_descriptor_token(
    ty: &TokenStream,
    generics: &Generics,
    type_bounds: &Vec<(TokenStream, TokenStream)>,
) -> TokenStream {
    let ty_str = ty.to_string();
    if ty_str == "i8" || ty_str == "u8" {
        quote! {"B"}
    } else if ty_str == "char" {
        quote! {"C"}
    } else if ty_str == "i16" || ty_str == "u16" {
        quote! {"S"}
    } else if ty_str == "i32" || ty_str == "u32" {
        quote! {"I"}
    } else if ty_str == "i64" || ty_str == "u64" {
        quote! {"J"}
    } else if ty_str == "f32" {
        quote! {"F"}
    } else if ty_str == "f64" {
        quote! {"D"}
    } else if ty_str == "bool" {
        quote! {"Z"}
    } else if ty_str == "()" {
        quote! {"V"}
    } else {
        let ty = if let Some(gt) = generics
            .type_params()
            .find(|i| i.ident.to_string() == ty.to_string())
        {
            let gt = gt.bounds.first();
            quote! {<#ty as #gt>}
        } else if let Some(it) = type_bounds
            .iter()
            .find(|i| i.0.to_string() == ty.to_string())
        {
            let tt = it.1.clone();
            quote! {<#ty as #tt>}
        } else if !ty_str.starts_with(|c: char| c.is_alphabetic() || c == '&') {
            // 如果不是有效标识符开头的类型（也不是引用类型）则需要使用`<...>`
            quote! {<&#ty>}
        } else {
            ty.clone()
        };
        if ty_str.starts_with("&") {
            quote! {&("[".repeat(*#ty::DIM as _) + #ty::OBJECT_SIG)}
        } else {
            quote! {&("[".repeat(#ty::DIM as _) + #ty::OBJECT_SIG)}
        }
    }
}

pub(crate) fn parse_function_signature(
    sig: &Signature,
    type_bounds: &Vec<(TokenStream, TokenStream)>,
) -> (
    Option<SelfValue>,
    Vec<(TokenStream, TokenStream)>,
    TokenStream,
    String,
    TokenStream,
    TokenStream,
) {
    let mut self_ = None;
    let mut arg_types = Vec::new();
    let mut arg_types_sig = TokenStream::new();
    let mut arg_values = Punctuated::<Expr, Token![,]>::new();
    for x in &sig.inputs {
        match x {
            FnArg::Receiver(r) => {
                self_ = Some(r.self_token.clone());
            }
            FnArg::Typed(t) => {
                let (unwrapped_ty, origin_ty) = unwrap_type(&t.ty.to_token_stream());
                let ty_str = unwrapped_ty.to_string();
                let v = t.pat.clone();
                let v = if ty_str == "i8" || ty_str == "u8" {
                    quote! {(#v as droid_wrap_utils::jbyte).into()}
                } else if ty_str == "char" {
                    quote! {(#v as droid_wrap_utils::jchar).into()}
                } else if ty_str == "i16" || ty_str == "u16" {
                    quote! {(#v as droid_wrap_utils::jshort).into()}
                } else if ty_str == "i32" || ty_str == "u32" {
                    quote! {(#v as droid_wrap_utils::jint).into()}
                } else if ty_str == "i64" || ty_str == "u64" {
                    quote! {(#v as droid_wrap_utils::jlong).into()}
                } else if ty_str == "f32" {
                    quote! {(#v as droid_wrap_utils::jfloat).into()}
                } else if ty_str == "f64" {
                    quote! {(#v as droid_wrap_utils::jdouble).into()}
                } else if ty_str == "bool" {
                    quote! {(#v as droid_wrap_utils::jboolean).into()}
                } else {
                    quote! {#v.java_ref().as_obj().into()}
                };

                let arg_sig = get_type_descriptor_token(&unwrapped_ty, &sig.generics, &type_bounds);
                arg_types.push((unwrapped_ty, origin_ty));
                arg_types_sig.extend(quote!(#arg_sig,));
                arg_values.push(Expr::Verbatim(v));
            }
        }
    }

    let fmt = vec!["{}"; arg_values.len()];
    let fmt = format!("({})", fmt.join("")) + "{}";

    let ret_type = match sig.output {
        ReturnType::Default => quote! {()},
        ReturnType::Type(_, ref t) => t.to_token_stream(),
    };
    (
        self_,
        arg_types,
        arg_types_sig,
        fmt,
        arg_values.to_token_stream(),
        ret_type,
    )
}

pub(crate) fn get_object_return_value_token(ret_type: &TokenStream) -> (TokenStream, bool) {
    let (unwrapped_ty, ty) = unwrap_type(&ret_type);

    let mut is_result_type = false;
    let value = if ty.to_string().starts_with("Option") {
        quote! {
            if let Ok(obj) = env.new_global_ref(obj) {
                Option::<#unwrapped_ty>::_new(&obj, Default::default())
            } else {
                None
            }
        }
    } else if ty.to_string().starts_with("Result") {
        is_result_type = true;
        quote! {
            match env.new_global_ref(obj) {
                Ok(o) => Ok(#unwrapped_ty::_new(&o, Default::default())),
                Err(e) => Err(e)
            }
        }
    } else {
        quote! {
            let obj = env.new_global_ref(obj).unwrap();
            #unwrapped_ty::_new(&obj, Default::default())
        }
    };
    (value, is_result_type)
}

pub(crate) fn get_return_value_token(
    ret_type: &TokenStream,
    generics: &Generics,
    type_bounds: &Vec<(TokenStream, TokenStream)>,
    default_value: &Option<Expr>,
) -> (TokenStream, TokenStream, bool) {
    let (unwrapped_ty, ty) = unwrap_type(ret_type);
    let ret_type_sig = get_type_descriptor_token(&unwrapped_ty, generics, &type_bounds);
    let unwrap_value = match default_value {
        None => quote! {unwrap()},
        Some(v) => quote! {unwrap_or(#v)},
    };

    if ret_type_sig.to_string().contains("OBJECT_SIG") {
        let (ret_value, is_result_type) = get_object_return_value_token(&ret_type);
        return (
            quote! {
                let obj = ret.l().unwrap();
                #ret_value
            },
            ret_type_sig,
            is_result_type,
        );
    }

    let ty_str = unwrapped_ty.to_string();
    let opt = if ty_str == "bool" {
        // bool需要单独处理。
        quote! {ret.z()}
    } else if ty_str == "char" {
        // char需要单独处理。
        quote! {
            match ret.c() {
                Ok(c) => Ok(c as u8 as char),
                Err(e) => Err(e)
            }
        }
    } else {
        // 非bool和char
        if ty_str.starts_with("u") {
            // 无符号数字
            let ty = if ty_str.ends_with("16") {
                quote! { i16 }
            } else if ty_str.ends_with("32") {
                quote! { i32 }
            } else if ty_str.ends_with("64") {
                quote! { i64 }
            } else {
                panic!("Unsupported return value type {ty_str}.");
            };
            quote! {
                match TryInto::<#ty>::try_into(ret) {
                    Ok(v) => Ok(v as #unwrapped_ty),
                    Err(e) => Err(e)
                }
            }
        } else {
            // 带符号数字
            quote! {TryInto::<#unwrapped_ty>::try_into(ret)}
        }
    };

    let mut is_result_type = false;
    let opt = if ty.to_string().starts_with("Option") {
        quote! {
            if let Ok(obj) = #opt {
                Some(obj)
            } else {
                None
            }
        }
    } else if ty.to_string().starts_with("Result") {
        is_result_type = true;
        opt
    } else {
        quote! {#opt.#unwrap_value}
    };
    (opt, ret_type_sig, is_result_type)
}

pub(crate) fn get_result_token(is_result_type: bool, ret_value: &TokenStream) -> TokenStream {
    if is_result_type {
        quote! {
            match ret {
                Ok(ret) => {#ret_value}
                Err(e) => Err(e)
            }
        }
    } else {
        quote! {
            let ret = ret.unwrap();
            #ret_value
        }
    }
}
