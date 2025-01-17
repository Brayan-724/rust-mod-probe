use case::CaseExt;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    Attribute, Error, Ident, ItemImpl, LitStr, Signature, Token, Type, Visibility,
};

use crate::java_class::get_attribute;

pub fn main(impl_: ItemImpl) -> Result<TokenStream, Error> {
    if let Some((_, trait_, _)) = impl_.trait_ {
        return Err(Error::new_spanned(
            trait_,
            "Cannot impl traits when importing from java.",
        ));
    }

    let struct_generics = &impl_.generics;
    let struct_name = &impl_.self_ty;

    let mut out = quote! {};

    let mut has_user_constructor = false;

    for item in &impl_.items {
        let item = syn::parse2::<ImportImplItem>(item.to_token_stream())?;

        let item_out = match item {
            ImportImplItem::Field(field) => {
                let vis = &field.vis;
                let ident = &field.ident;
                let ty = &field.ty;

                quote! {
                    #vis const #ident: ::probe::class::StaticField<#struct_name, #ty> = ::probe::class::StaticField::new(stringify!(#ident));
                }
            }
            ImportImplItem::Fn(fn_) => {
                let vis = &fn_.vis;
                let ident = &fn_.sig.ident;

                let fn_name = ident.to_string().as_str().to_snake();
                let extern_name = fn_
                    .sig
                    .abi
                    .as_ref()
                    .map(|abi| abi.name.as_ref().map(LitStr::value))
                    .flatten()
                    .unwrap_or_else(|| ident.to_string().as_str().to_camel_lowercase());

                let fn_name = Ident::new(&fn_name, fn_.sig.ident.span());

                let out_ty = match &fn_.sig.output {
                    syn::ReturnType::Default => quote_spanned! {fn_.sig.output.span() => ()},
                    syn::ReturnType::Type(_, ty) => quote_spanned! {ty.span() => #ty},
                };

                let ret = match &fn_.sig.output {
                    syn::ReturnType::Default => quote_spanned! {out_ty.span() =>},
                    syn::ReturnType::Type(s, ty) => {
                        if let Type::Tuple(ty_) = &**ty {
                            if ty_.elems.is_empty() {
                                quote_spanned! {out_ty.span() =>}
                            } else {
                                quote_spanned! {s.span() => <#ty as ::probe::conversion::FromJValue>::from_jvalue(_ret)}
                            }
                        } else {
                            quote_spanned! {s.span() => <#ty as ::probe::conversion::FromJValue>::from_jvalue(_ret)}
                        }
                    }
                };

                let args = &fn_.sig.inputs;

                let has_self = fn_.sig.receiver();

                let (sig_args, (decl_args, def_args)) = args
                    .into_iter()
                    .skip(if has_self.is_some() { 1 } else { 0 })
                    .filter_map(|arg| {
                        if let syn::FnArg::Typed(arg) = arg {
                            Some(arg)
                        } else {
                            None
                        }
                    })
                    .enumerate()
                    .map(|(idx, arg)| {
                        let arg_orig_ident = match &*arg.pat {
                            syn::Pat::Ident(ident) => &ident.ident,
                            _ => return Err(Error::new_spanned(&arg.pat, "Use a named parameter, patterns are not supported")),
                        };
                        let arg_ident = quote::format_ident!("_arg_{idx}", span = arg_orig_ident.span());
                        let arg_ty = &arg.ty;

                        let sig = quote_spanned! {arg.span() =>
                            #arg_orig_ident: #arg_ty
                        };

                        let decl = quote_spanned! {arg_ident.span() =>
                            let #arg_ident = ::probe::conversion::IntoJValue::into_jvalue(#arg_orig_ident, env);
                            sig_args += &(<#arg_ty as ::probe::JSignature>::sig());
                        };

                        let def = quote_spanned! {arg_ident.span() => #arg_ident.borrow()};

                        Ok((sig, (decl, def)))
                    })
                    .collect::<Result<(Vec<_>, (Vec<_>, Vec<_>)), Error>>()?;

                let out_ty_sig = quote_spanned! {fn_.sig.output.span() => <#out_ty as ::probe::JSignature>::sig()};
                let get_sig_args = quote_spanned! {fn_.sig.inputs.span() =>
                    #[allow(unused_mut)]
                    let mut sig_args = String::new();

                    #(#decl_args)*

                    let sig = ["(", &sig_args, ")", &#out_ty_sig].join("");
                };

                if get_attribute(&"constructor", &fn_.attrs).is_some() {
                    has_user_constructor = true;

                    quote_spanned! {fn_.sig.fn_token.span() =>
                        #vis fn #fn_name<'local>(env: &mut ::jni::JNIEnv<'local>, #(#sig_args),*) -> Self {
                            #get_sig_args

                            let instance: ::probe::class::Instance = env
                                .new_object(<Self as ::probe::JSignature>::CLASS, sig, &[#(#def_args),*])
                                .unwrap()
                                .into();

                            Self::from_instance(instance)
                        }
                    }
                } else if let Some(self_) = has_self {
                    let self_ = quote_spanned! {self_.span() => #self_};
                    quote_spanned! {fn_.sig.fn_token.span() =>
                        #vis fn #fn_name<'local>(&#self_, env: &mut ::jni::JNIEnv<'local>, #(#sig_args),*) -> #out_ty {
                            #get_sig_args

                            let obj: ::jni::objects::JObject = <Self as ::probe::JavaClass>::get_raw(self).into();
                            let _ret = env
                                .call_method(obj, #extern_name, sig, &[#(#def_args),*])
                                .unwrap();

                            #ret
                        }
                    }
                } else {
                    quote_spanned! {fn_.sig.fn_token.span() =>
                        #vis fn #fn_name<'local>(env: &mut ::jni::JNIEnv<'local>, #(#sig_args),*) -> #out_ty {
                            #get_sig_args

                            let _ret = env
                                .call_static_method(<Self as ::probe::JSignature>::CLASS, #extern_name, sig, &[#(#def_args),*])
                                .unwrap();

                            #ret
                        }
                    }
                }
            }
        };

        out = quote! {
            #out
            #item_out
        };
    }

    let constructor = if !has_user_constructor {
        quote! {
            pub fn new<'local>(env: &mut ::jni::JNIEnv<'local>) -> Self {
                Self::from_instance(
                    env.new_object(<Self as ::probe::JSignature>::CLASS, "()V", &[])
                        .unwrap()
                        .into(),
                )
            }
        }
    } else {
        quote!()
    };

    Ok(quote! {
        impl #struct_generics #struct_name {
            #out

            #constructor
        }
    })
}

enum ImportImplItem {
    Field(ImportImplItemField),
    Fn(ImportImplItemFn),
}

struct ImportImplItemField {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub ident: Ident,
    pub ty: Type,
}

struct ImportImplItemFn {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub sig: Signature,
}

impl Parse for ImportImplItem {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let ahead = input.fork();
        Attribute::parse_outer(&ahead)?;
        ahead.parse::<Visibility>()?;

        if ahead.peek(Token![const]) {
            input
                .parse::<ImportImplItemField>()
                .map(ImportImplItem::Field)
        } else if ahead.peek(Token![fn]) || ahead.peek(Token![extern]) {
            input.parse::<ImportImplItemFn>().map(ImportImplItem::Fn)
        } else {
            Err(Error::new(ahead.span(), "Expected `const` or `fn`"))
        }
    }
}

impl Parse for ImportImplItemField {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let attrs = Attribute::parse_outer(input)?;
        let vis = input.parse::<Visibility>()?;
        input.parse::<Token![const]>()?;
        let ident = input.parse::<Ident>()?;
        input.parse::<Token![:]>()?;
        let ty = input.parse::<Type>()?;
        input.parse::<Token![;]>()?;

        Ok(Self {
            attrs,
            vis,
            ident,
            ty,
        })
    }
}

impl Parse for ImportImplItemFn {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let attrs = Attribute::parse_outer(input)?;
        let vis = input.parse::<Visibility>()?;
        let sig = input.parse::<Signature>()?;
        input.parse::<Token![;]>()?;

        Ok(Self { attrs, vis, sig })
    }
}
