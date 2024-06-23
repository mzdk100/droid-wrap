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
    Expr, FnArg, MetaNameValue, PathArguments, PathSegment, ReturnType, Signature, Token, Type,
    TypeReference,
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

fn get_type_descriptor_token(ty: &TokenStream) -> TokenStream {
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
        quote! {#ty::get_object_sig()}
    }
}

pub(crate) fn parse_function_signature(
    sig: &Signature,
) -> (
    Option<SelfValue>,
    Vec<TokenStream>,
    TokenStream,
    String,
    TokenStream,
    TokenStream,
) {
    let mut self_: Option<SelfValue> = None;
    let mut arg_types = Vec::new();
    let mut arg_types_sig = TokenStream::new();
    let mut arg_values = Punctuated::<Expr, Token![,]>::new();
    for x in &sig.inputs {
        match x {
            FnArg::Receiver(r) => {
                self_ = Some(r.self_token.clone());
            }
            FnArg::Typed(t) => {
                let (unwrapped_ty, _) = unwrap_type(&t.ty.to_token_stream());
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

                let arg_sig = get_type_descriptor_token(&unwrapped_ty);
                arg_types.push(unwrapped_ty);
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

pub(crate) fn get_object_return_value_token(ret_type: &TokenStream) -> TokenStream {
    let (unwrapped_ty, ty) = unwrap_type(&ret_type);

    if ty.to_string().starts_with("Option") {
        quote! {
            if let Ok(obj) = env.new_global_ref(obj) {
                Some(#unwrapped_ty::_new(&obj))
            } else {
                None
            }
        }
    } else if ty.to_string().starts_with("Result") {
        quote! {
            match env.new_global_ref(obj) {
                Ok(o) => Ok(#unwrapped_ty::_new(&o)),
                Err(e) => Err(e)
            }
        }
    } else {
        quote! {
            let obj = env.new_global_ref(obj).unwrap();
            #unwrapped_ty::_new(&obj)
        }
    }
}

pub(crate) fn get_return_value_token(ret_type: &TokenStream) -> (TokenStream, TokenStream) {
    let (unwrapped_ty, ty) = unwrap_type(ret_type);
    let ret_type_sig = get_type_descriptor_token(&unwrapped_ty);

    if ret_type_sig.to_string().contains("get_object_sig") {
        let ret_value = get_object_return_value_token(&ret_type);
        return (
            quote! {
                let obj = ret.l().unwrap();
                #ret_value
            },
            quote! {#unwrapped_ty::get_object_sig()},
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
        quote! {TryInto::<#unwrapped_ty>::try_into(ret)}
    };

    let opt = if ty.to_string().starts_with("Option") {
        quote! {
            if let Ok(obj) = #opt {
                Some(obj)
            } else {
                None
            }
        }
    } else if ty.to_string().starts_with("Result") {
        opt
    } else {
        quote! {#opt.unwrap()}
    };
    (opt, ret_type_sig)
}
