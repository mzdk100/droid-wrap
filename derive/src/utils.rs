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
    parse2, punctuated::Punctuated, token::SelfValue, Expr, FnArg, GenericArgument, PathArguments,
    PathSegment, ReturnType, Signature, Token, Type,
};

pub(crate) fn parse_function_signature(
    sig: &Signature,
) -> (
    Option<SelfValue>,
    TokenStream,
    String,
    TokenStream,
    TokenStream,
    TokenStream,
) {
    let mut self_: Option<SelfValue> = None;
    let mut arg_types = Punctuated::<Expr, Token![,]>::new();
    let mut arg_values = TokenStream::new();
    for x in &sig.inputs {
        match x {
            FnArg::Receiver(r) => {
                self_ = Some(r.self_token.clone());
            }
            FnArg::Typed(t) => {
                let ty = t.ty.clone();
                let ty_str = ty.to_token_stream().to_string();
                let v = t.pat.clone();
                let (ty, v) = if ty_str == "i8" || ty_str == "u8" {
                    (
                        quote! {"B",},
                        quote! {(#v as droid_wrap_utils::jbyte).into(),},
                    )
                } else if ty_str == "char" {
                    (
                        quote! {"C",},
                        quote! {(#v as droid_wrap_utils::jchar).into(),},
                    )
                } else if ty_str == "i16" || ty_str == "u16" {
                    (
                        quote! {"S",},
                        quote! {(#v as droid_wrap_utils::jshort).into(),},
                    )
                } else if ty_str == "i32" || ty_str == "u32" {
                    (
                        quote! {"I",},
                        quote! {(#v as droid_wrap_utils::jint).into(),},
                    )
                } else if ty_str == "i64" || ty_str == "u64" {
                    (
                        quote! {"J",},
                        quote! {(#v as droid_wrap_utils::jlong).into(),},
                    )
                } else if ty_str == "f32" {
                    (
                        quote! {"F",},
                        quote! {(#v as droid_wrap_utils::jfloat).into(),},
                    )
                } else if ty_str == "f64" {
                    (
                        quote! {"D",},
                        quote! {(#v as droid_wrap_utils::jdouble).into(),},
                    )
                } else if ty_str == "bool" {
                    (
                        quote! {"Z",},
                        quote! {(#v as droid_wrap_utils::jboolean).into()},
                    )
                } else {
                    (
                        quote! {#ty::get_object_sig(),},
                        quote! {#v.java_ref().as_obj().into()},
                    )
                };
                arg_types.push(Expr::Verbatim(ty));
                arg_values.extend(v);
            }
        }
    }
    let fmt = vec!["{}"; arg_types.len()];
    let arg_types = arg_types.to_token_stream();
    let fmt = format!("({})", fmt.join(",")) + "{}";

    let ret_type = match sig.output {
        ReturnType::Default => quote! {()},
        ReturnType::Type(_, ref t) => t.to_token_stream(),
    };
    let ty_str = ret_type.to_string();
    let ret_type_sig = if ty_str == "i8" || ty_str == "u8" {
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
        quote! {#ret_type::get_object_sig()}
    };
    (self_, arg_types, fmt, arg_values, ret_type_sig, ret_type)
}

pub(crate) fn get_object_return_value_token(ret_type: TokenStream) -> (TokenStream, TokenStream) {
    let res = parse2::<PathSegment>(ret_type.clone());

    match res {
        Ok(item) => {
            if let PathArguments::AngleBracketed(arg) = item.arguments {
                if let Some(GenericArgument::Type(Type::Path(arg))) = arg.args.first() {
                    if item.ident.to_string() == "Option" {
                        let ret_type = arg.path.clone();
                        return (
                            quote! {
                                if let Ok(obj) = env.new_global_ref(obj) {
                                    Some(#ret_type {_obj: obj})
                                } else {
                                    None
                                }
                            },
                            ret_type.to_token_stream(),
                        );
                    } else if item.ident.to_string() == "Result" {
                        let ret_type = arg.path.clone();
                        return (
                            quote! {
                                match env.new_global_ref(obj) {
                                    Ok(o) => Ok(#ret_type {_obj: o}),
                                    Err(e) => Err(e)
                                }
                            },
                            ret_type.to_token_stream(),
                        );
                    }
                }
            }
        }
        Err(_) => {}
    }
    (
        quote! {
            let obj = env.new_global_ref(obj).unwrap();
            #ret_type {_obj: obj}
        },
        ret_type,
    )
}

pub(crate) fn get_return_value_token(
    ret_type_sig: &TokenStream,
    ret_type: TokenStream,
) -> (TokenStream, TokenStream) {
    if ret_type_sig.to_string().contains("get_object_sig") {
        let (ret_value, ret_type) = get_object_return_value_token(ret_type);
        return (
            quote! {
                let obj = ret.l().unwrap();
                #ret_value
            },
            quote! {#ret_type::get_object_sig()},
        );
    }
    match parse2::<PathSegment>(ret_type.clone()) {
        Ok(item) => {
            if let PathArguments::AngleBracketed(arg) = item.arguments {
                if let Some(GenericArgument::Type(Type::Path(arg))) = arg.args.first() {
                    if item.ident.to_string() == "Option" {
                        let ret_type = arg.path.clone();
                        return (
                            quote! {
                                if let Ok(obj) = TryInto::<#ret_type>::try_into(ret) {
                                    Some(obj)
                                } else {
                                    None
                                }
                            },
                            quote! {#ret_type::get_object_sig()},
                        );
                    } else if item.ident.to_string() == "Result" {
                        let ret_type = arg.path.clone();
                        return (
                            quote! {
                                match TryInto::<#ret_type>::try_into(ret) {
                                    Ok(o) => Ok(o),
                                    Err(e) => Err(e)
                                }
                            },
                            quote! {#ret_type::get_object_sig()},
                        );
                    }
                }
            }
        }
        Err(_) => {}
    }
    (
        quote! {
            TryInto::<#ret_type>::try_into(ret).unwrap()
        },
        ret_type_sig.clone(),
    )
}
