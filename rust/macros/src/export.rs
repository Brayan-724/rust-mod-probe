mod parse;

use itertools::Itertools;
use quote::{format_ident, quote, quote_spanned};
use syn::ext::IdentExt;
use syn::spanned::Spanned;
use syn::{Ident, ItemImpl};

use crate::export::parse::ExportPackage;
use crate::utils::get_rename_attr;

pub fn export(
    input: proc_macro::TokenStream,
    body: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let package = syn::parse_macro_input!(input as ExportPackage);
    let body = syn::parse_macro_input!(body as ItemImpl);

    let class = match &*body.self_ty {
        syn::Type::Path(type_path) => type_path,
        _ => {
            body.self_ty
                .span()
                .unwrap()
                .error("expected path type")
                .emit();

            return proc_macro::TokenStream::new();
        }
    };

    let class_name = &class
        .path
        .segments
        .last()
        .expect("TypePath must have at least one segment")
        .ident;

    let out = body.items.into_iter().map(|item| {
        let item = match item {
            syn::ImplItem::Fn(item) => item,
            _ => {
                item.span().unwrap().error("expected function item").emit();
                return quote!();
            }
        };

        let vis = &item.vis;

        let fn_ident = &item.sig.ident;
        let method = get_rename_attr(item.span(), &item.attrs)
            .ok()
            .flatten()
            .unwrap_or_else(|| fn_ident.unraw().to_string());

        let fn_name = Ident::new(
            &generate_mangled_name(&package.str, &class_name.to_string(), &method),
            class.span(),
        );

        let qself = item.sig.receiver().map(|s| s.self_token);
        let this = qself
            .iter()
            .map(|qself| quote_spanned! {qself.span => this})
            .collect_vec();

        let this_decl = qself
            .iter()
            .map(|qself| {
                quote_spanned! {qself.span =>
                    let this = <#class as From<<#class as ::rosttasse::prelude::IntoJValue>::JniType<'_>>>::from(_class);
                }
            })
            .collect_vec();

        let fn_params = item
            .sig
            .inputs
            .iter()
            .skip(if qself.is_some() { 1 } else { 0 })
            .filter_map(|p| match p {
                syn::FnArg::Receiver(_) => None,
                syn::FnArg::Typed(pat_type) => Some(pat_type),
            })
            .collect_vec();

        let qself = qself.iter().collect_vec();
        let body = &item.block;

        let (params_decl, params_def) = fn_params
            .iter()
            .enumerate()
            .map(|(idx, param)| {
                let name = format_ident!("_arg_{idx}", span = param.pat.span());
                let ty = &param.ty;

                let ty_jni = quote_spanned! {ty.span() =>
                    <#ty as ::rosttasse::prelude::IntoJValue>::JniType
                };

                let param_decl = quote_spanned! {name.span() => #name : #ty_jni<'local>};
                let param_def =
                    quote_spanned! {ty.span() => <#ty as From<#ty_jni<'_>>>::from(#name)};

                (param_decl, param_def)
            })
            .unzip::<_, _, Vec<_>, Vec<_>>();

        let ret = &item.sig.output;
        quote_spanned! {item.span() =>
            impl #class {
                #vis fn #fn_ident<'local>(
                    #(#qself,)*
                    env: &mut ::rosttasse::prelude::JNIEnv<'local>,
                    #(#fn_params),*
                ) #ret {
                    #body
                }
            }

            #[no_mangle]
            pub extern "system" fn #fn_name<'local>(
                mut env: ::rosttasse::prelude::JNIEnv<'local>,
                _class: ::rosttasse::prelude::JNIObject<'local>,
                #(#params_decl),*
            ) -> ::rosttasse::prelude::JNIObject<'local> {
                let env = &mut env;

                #(#this_decl)*

                let res = #class::#fn_ident(#(#this,)* env, #(#params_def),*);
                ::rosttasse::prelude::IntoJValue::into_jni(res, env)
            }
        }
    });

    quote!(#(#out)*).into()
}

fn generate_mangled_name(package: &str, class: &str, method: &str) -> String {
    let package = package.replace(".", "_");
    format!("Java_{package}_{class}_{method}")
}
