use proc_macro::TokenStream;

use heck::ToLowerCamelCase;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use syn::{
    Expr,
    Field,
    FieldMutability,
    Fields,
    FieldsNamed, FnArg, ItemFn, ItemStruct, MetaNameValue, parse::{Parse, ParseStream}, parse_macro_input, punctuated::Punctuated, ReturnType,
    Token, token::SelfValue, Type, Visibility,
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

    quote! {
        #item

        impl<'j> JType<'j> for #name #generics {
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
    }
    .into()
}

#[proc_macro_attribute]
pub fn java_method(_: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemFn);
    let name = item.sig.ident.to_string().to_lower_camel_case();
    let vis = item.vis.clone();
    let sig = item.sig.clone();
    let mut self_: Option<SelfValue> = None;
    let mut arg_types = Punctuated::<Expr, Token![,]>::new();
    let mut arg_values = TokenStream2::new();
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
                    (quote! {"B",}, quote! {(#v as droid_wrap_utils::jbyte).into(),})
                } else if ty_str == "char" {
                    (quote! {"C",}, quote! {(#v as droid_wrap_utils::jchar).into(),})
                } else if ty_str == "i16" || ty_str == "u16" {
                    (quote! {"S",}, quote! {(#v as droid_wrap_utils::jshort).into(),})
                } else if ty_str == "i32" || ty_str == "u32" {
                    (quote! {"I",}, quote! {(#v as droid_wrap_utils::jint).into(),})
                } else if ty_str == "i64" || ty_str == "u64" {
                    (quote! {"J",}, quote! {(#v as droid_wrap_utils::jlong).into(),})
                } else if ty_str == "f32" {
                    (quote! {"F",}, quote! {(#v as droid_wrap_utils::jfloat).into(),})
                } else if ty_str == "f64" {
                    (quote! {"D",}, quote! {(#v as droid_wrap_utils::jdouble).into(),})
                } else if ty_str == "bool" {
                    (quote! {"Z",}, quote! {(#v as droid_wrap_utils::jboolean).into()})
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

    quote! {
        #vis #sig {
            droid_wrap_utils::vm_attach(|env| {
                let ret = env.call_method(
                    #self_.java_ref(),
                    #name,
                    format!(#fmt, #arg_types #ret_type_sig).as_str(),
                    &[#arg_values],
                )
                .unwrap();
                TryInto::<#ret_type>::try_into(ret).unwrap()
            })
        }
    }
    .into()
}
