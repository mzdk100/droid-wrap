/*
 * Copyright (c) 2025. The RigelA open source project team and
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

use heck::ToLowerCamelCase;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{
    Field, FieldMutability, Fields, FieldsNamed, ImplItem, ItemFn, ItemImpl, ItemStruct, ItemTrait,
    LitInt, Token, TraitItem, Type, TypeParamBound, Visibility, parse2, punctuated::Punctuated,
};

use crate::utils::{
    ClassMetadata, FieldMetadata, InterfaceMetadata, MethodMetadata, get_return_value_token,
    get_type_form, parse_function_signature,
};

//noinspection SpellCheckingInspection
pub(super) fn java_class(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let attrs: ClassMetadata = parse2(attrs).unwrap();
    let cls = attrs.class_name;
    let based = attrs.base_class;
    let mut item: ItemStruct = parse2(input).unwrap();
    let name = item.ident.clone();
    let generics = item.generics.clone();
    let mut item2 = item.clone();

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

    let (fields, added_this, added_super, added_default) = match item.fields {
        Fields::Named(mut f) => {
            let mut added_default = TokenStream::new();
            for i in f.named.iter() {
                let i = i.ident.clone();
                added_default.extend(quote! {#i: fields.#i,});
            }
            f.named.push(add_field_this);
            let added_super = if based.is_some() {
                f.named.push(add_field_super);
                quote! {_based}
            } else {
                quote!()
            };
            (Fields::Named(f), quote! {_this}, added_super, added_default)
        }
        Fields::Unnamed(mut f) => {
            let mut added_default = TokenStream::new();
            for i in 0..f.unnamed.len() {
                let i = LitInt::new(&i.to_string(), Span::call_site());
                added_default.extend(quote! {#i: fields.#i,});
            }
            add_field_this.ident = None;
            add_field_super.ident = None;
            let len = LitInt::new(&f.unnamed.len().to_string(), Span::call_site());
            let added_this = quote! {#len};
            f.unnamed.push(add_field_this);
            let added_super = if based.is_some() {
                let len = LitInt::new(&f.unnamed.len().to_string(), Span::call_site());
                f.unnamed.push(add_field_super);
                quote! {#len}
            } else {
                quote!()
            };
            (Fields::Unnamed(f), added_this, added_super, added_default)
        }
        Fields::Unit => {
            let mut fields = Punctuated::<Field, Token![,]>::new();
            fields.push(add_field_this);
            let added_super = if based.is_some() {
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
                added_super,
                quote!(),
            )
        }
    };
    item.fields = fields;

    let build_self = if based.is_some() {
        quote! {
            Self {#added_this: this.clone(), #added_super: this.into(), #added_default }
        }
    } else {
        quote! {
            Self {#added_this:this.clone(), #added_default }
        }
    };

    let (item2_token, name2) = if added_default.is_empty() {
        (quote!(), quote! {()})
    } else {
        let name2 = Ident::new((name.to_string() + "Default").as_str(), Span::call_site());
        item2.ident = name2.clone();
        (quote! {#item2}, quote! {#name2})
    };

    let impl_based_deref = if based.is_some() {
        quote! {
            impl #generics std::ops::Deref for #name #generics {
                type Target = #based;

                fn deref(&self) -> &Self::Target {
                    &self.#added_super
                }
            }

            impl #generics std::ops::DerefMut for #name #generics {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.#added_super
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

    quote! {
        #item
        #item2_token

        impl #generics JObjNew for #name #generics {
            type Fields = #name2;

            fn _new(this: &droid_wrap_utils::GlobalRef, fields: Self::Fields) -> droid_wrap_utils::Result<Self> {
                Ok(#build_self)
            }
        }

        impl #generics JType for #name #generics {
            const CLASS: &'static str = #cls;
            const OBJECT_SIG: &'static str = concat!("L", #cls, ";");
        }

        impl #generics JObjRef for #name #generics {
            fn java_ref(&self) -> droid_wrap_utils::Result<droid_wrap_utils::GlobalRef> {
                Ok(self.#added_this.clone())
            }
        }

        impl #generics PartialEq for #name #generics {
            fn eq(&self, other: &Self) -> bool {
                let call_fn = || {
                    droid_wrap_utils::java_object_equals(self.java_ref()?, other.java_ref()?)
                };
                call_fn().unwrap_or_default()
            }
    }

        impl #generics ToString for #name #generics {
            fn to_string(&self) -> String {
                let call_fn = || droid_wrap_utils::java_object_to_string(self.java_ref()?);
                call_fn().unwrap_or_default()
            }
        }

        impl #generics std::fmt::Debug for #name #generics {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.to_string())
            }
        }

        impl #generics From<&droid_wrap_utils::GlobalRef> for #name #generics {
            fn from(obj: &droid_wrap_utils::GlobalRef) -> Self {
                Self::_new(&obj, <Self as JObjNew>::Fields::default()).unwrap()
            }
        }

        impl #generics Into<droid_wrap_utils::GlobalRef> for &#name #generics {
            fn into(self) -> droid_wrap_utils::GlobalRef {
                self.#added_this.clone()
            }
        }

        #impl_based_deref
    }
}

pub(super) fn java_method(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let attrs: MethodMetadata = parse2(attrs).unwrap();
    let type_bounds = attrs.type_bounds;
    let overload = attrs.overload;
    let item: ItemFn = parse2(input).unwrap();
    let attrs = item.attrs.clone();
    let stmts = item.block.stmts.clone();
    let name = if overload.is_none() {
        item.sig.ident.to_string()
    } else {
        overload.unwrap().to_token_stream().to_string()
    }
    .to_lower_camel_case();
    let vis = item.vis.clone();
    let sig = item.sig.clone();

    let (self_, _, arg_types_sig, fmt, arg_values, ret_type) =
        parse_function_signature(&sig, &type_bounds);
    let (ret_value, ret_type_sig) = get_return_value_token(&ret_type, &sig.generics, &type_bounds);

    let class_token = if let Some(it) = type_bounds.iter().find(|i| i.0.to_string() == "Self") {
        let tt = it.1.clone();
        quote! {<Self as #tt>::CLASS}
    } else {
        quote! {Self::CLASS}
    };

    let ret_form = get_type_form(&ret_type, &None);
    let opt = if self_.is_none() {
        quote! {
            env.call_static_method(#class_token,#name,format!(#fmt, #arg_types_sig #ret_type_sig).as_str(),&[#arg_values],)
        }
    } else {
        quote! {
            env.call_method(#self_.java_ref()?,#name,format!(#fmt, #arg_types_sig #ret_type_sig).as_str(),&[#arg_values],)
        }
    };

    quote! {
        #(#attrs)*
        #vis #sig {
            #(#stmts)*
            let call_fn = || {
                let mut env = droid_wrap_utils::vm_attach()?;
                let ret = #opt?;
                Ok::<_, droid_wrap_utils::DroidWrapError>(#ret_value)
            };
            call_fn()#ret_form
        }
    }
}

pub(super) fn java_constructor(input: TokenStream) -> TokenStream {
    let item: ItemFn = parse2(input).unwrap();
    let attrs = item.attrs.clone();
    let vis = item.vis.clone();
    let sig = item.sig.clone();
    let stmts = item.block.stmts.clone();
    let (self_, _, arg_types, fmt, arg_values, ret_type) = parse_function_signature(&sig, &vec![]);

    if !self_.is_none() {
        panic!(
            "Incorrect constructor, please remove the '{}' in the arguments!",
            self_.to_token_stream()
        );
    }

    if !ret_type.to_string().contains("Self") {
        panic!(
            "Incorrect constructor, please modify the '{}' to 'Self', 'Option<Self>' or 'Result<Self>' in the return value!",
            ret_type
        );
    }

    let ret_form = get_type_form(&ret_type, &None);

    quote! {
        #(#attrs)*
        #vis #sig {
            #(#stmts)*
            let call_fn = || {
                let mut env = droid_wrap_utils::vm_attach()?;
                let obj = env.new_object(<Self as JType>::CLASS,format!(#fmt, #arg_types "V").as_str(),&[#arg_values],)?;
                Self::_new(env.new_global_ref(obj)?.as_ref(), Default::default())
            };
            call_fn()#ret_form
        }
    }
}

pub(super) fn java_interface(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let attrs: InterfaceMetadata = parse2(attrs).unwrap();
    let cls = attrs.interface_name;
    let mut item: ItemTrait = parse2(input).unwrap();
    item.supertraits
        .push(TypeParamBound::Verbatim(quote! {JObjRef}));
    item.supertraits
        .push(TypeParamBound::Verbatim(quote! {JObjNew}));
    item.supertraits
        .push(TypeParamBound::Verbatim(quote! {PartialEq}));
    item.supertraits
        .push(TypeParamBound::Verbatim(quote! {std::fmt::Debug}));

    item.items.push(TraitItem::Verbatim(quote! {
        #[doc = #cls]
        const CLASS: &'static str = #cls;
    }));
    item.items.push(TraitItem::Verbatim(quote! {
        #[doc = concat!("L", #cls, ";")]
        const OBJECT_SIG: &'static str = concat!("L", #cls, ";");
    }));
    item.items.push(TraitItem::Verbatim(quote! {
        /// 数组维度
        const DIM: u8 = 0;
    }));

    quote! {
        #item
    }
}

//noinspection SpellCheckingInspection
pub(super) fn java_implement(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse2::<ItemImpl>(input).unwrap();

    let mut methods = TokenStream::new();
    for item in item.items.iter() {
        match item {
            ImplItem::Fn(f) => {
                let name = f.sig.ident.clone();
                if name.to_string().starts_with("_") {
                    // 跳过下划线开头的函数
                    continue;
                }
                let name_camel = f.sig.ident.to_string().to_lower_camel_case();
                let (self_, arg_types, _, _, _, ret_type) =
                    parse_function_signature(&f.sig, &vec![]);
                if self_.is_none() {
                    continue;
                }

                let ret_token = match ret_type.to_string().as_str() {
                    "()" => quote! {droid_wrap_utils::null_value(env)},
                    "bool" => quote! {droid_wrap_utils::wrapper_bool_value(_ret, env)},
                    "i32" => quote! {droid_wrap_utils::wrapper_integer_value(_ret, env)},
                    "u32" => quote! {droid_wrap_utils::wrapper_integer_value(_ret as u32, env)},
                    "i64" => quote! {droid_wrap_utils::wrapper_long_value(_ret, env)},
                    "u64" => quote! {droid_wrap_utils::wrapper_long_value(_ret as u64, env)},
                    _ => quote! {_ret.java_ref()},
                };

                let mut arg_tokens = TokenStream::new();
                for i in 0..arg_types.len() {
                    let (unwrapped_ty, origin_ty) = &arg_types[i];
                    let ty_str = unwrapped_ty.to_string();
                    let ar = if [
                        "bool", "char", "i16", "u16", "i32", "u32", "i64", "u64", "f32", "f64",
                    ]
                    .contains(&ty_str.as_str())
                    {
                        quote! {
                            droid_wrap_utils::ParseJObjectType::<#unwrapped_ty>::parse(&args2[#i], env)
                        }
                    } else {
                        quote! {{
                            let r = env.new_global_ref(&args2[#i])?;
                            #unwrapped_ty::_new(&r, Default::default())
                        }}
                    };
                    let arg_form = get_type_form(&origin_ty, &None);
                    arg_tokens.extend(quote! {#ar #arg_form,});
                }
                methods.extend(quote! {
                    #name_camel => {
                        let args2 = droid_wrap_utils::to_vec(env, &args)?;
                        let _ret = self_.#name(#arg_tokens);
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
        fn new(fields: Self::Fields) -> droid_wrap_utils::Result<std::sync::Arc<Self>> {
            use std::sync::Arc;
            let interface = #class_token.replace("/", ".");
            let proxy = droid_wrap_utils::new_proxy(&[&interface])?;
            let ret = Arc::new(Self::_new(&proxy, fields)?);
            let self_ = ret.clone();
            droid_wrap_utils::bind_proxy_handler(&proxy, move |env, method, args| {
                let name = env.call_method(&method, "getName", "()Ljava/lang/String;", &[])?.l()?;
                let name = env.get_string((&name).into())?;
                match name.to_str()? {
                    #methods
                    _ => droid_wrap_utils::null_value(env)
                }
            });
            Ok(ret)
        }
    };

    quote! {
        #attrs
        #item

        impl JProxy for #name {
            #impl_new
        }

        impl Drop for #name {
            fn drop(&mut self) {
                let _ = self.release();
            }
        }
    }
}

pub(super) fn java_field(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let attrs: FieldMetadata = parse2(attrs).unwrap();
    let default_value = attrs.default_value.clone();
    let item: ItemFn = parse2(input).unwrap();
    let attrs = item.attrs.clone();
    let stmts = item.block.stmts.clone();
    let name = item.sig.ident.to_string().to_lower_camel_case();
    let vis = item.vis.clone();
    let sig = item.sig.clone();

    let (is_set, name) = if name.starts_with("get") {
        (false, name.trim_start_matches("get").to_lower_camel_case())
    } else if name.starts_with("set") {
        (true, name.trim_start_matches("set").to_lower_camel_case())
    } else {
        panic!("Field name `{}` must start with get or set.", name);
    };

    let (self_, arg_types, arg_types_sig, _, arg_values, ret_type) =
        parse_function_signature(&sig, &vec![]);
    if is_set {
        if arg_types.len() != 1 {
            panic!(
                "The number of setter arguments for the field `{}` must be one.",
                name
            )
        }
    } else {
        if !arg_types.is_empty() {
            panic!("The getter field `{}` cannot provide any arguments.", name)
        }
    }

    let (ret_value, ret_type_sig) = get_return_value_token(&ret_type, &sig.generics, &vec![]);

    let opt = if is_set {
        if self_.is_none() {
            quote! {
                Ok::<_, droid_wrap_utils::DroidWrapError>(env.set_static_field(Self::CLASS, #name, #arg_types_sig #arg_values)?)
            }
        } else {
            quote! {
                Ok::<_, droid_wrap_utils::DroidWrapError>(env.set_field(#self_.java_ref()?, #name, #arg_types_sig #arg_values)?)
            }
        }
    } else {
        if self_.is_none() {
            quote! {
                let ret = env.get_static_field(Self::CLASS, #name, #ret_type_sig)?;
                Ok::<_, droid_wrap_utils::DroidWrapError>(#ret_value)
            }
        } else {
            quote! {
                let ret = env.get_field(#self_.java_ref()?, #name, #ret_type_sig)?;
                Ok::<_, droid_wrap_utils::DroidWrapError>(#ret_value)
            }
        }
    };

    let ret_form = get_type_form(&ret_type, &default_value);

    quote! {
        #(#attrs)*
        #vis #sig {
            #(#stmts)*
            let call_fn = || {
                let mut env = droid_wrap_utils::vm_attach()?;
                #opt
            };

            call_fn()#ret_form
        }
    }
}
