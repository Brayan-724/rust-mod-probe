use proc_macro2::Span;
use quote::ToTokens;
use syn::{spanned::Spanned, DeriveInput, Error, Expr, ExprLit, Lit, MetaList, MetaNameValue};

use super::{get_attribute, get_rename_attr};

pub fn generate(input: &DeriveInput) -> Result<(Span, String), syn::Error> {
    let struct_name = &input.ident;

    let Some(package) = get_attribute(&"package", &input.attrs) else {
        return Err(Error::new_spanned(
            struct_name,
            "Needs package: #[package(...)]",
        ));
    };

    let package_span = package.span();

    let package = match &package.meta {
        syn::Meta::List(MetaList { tokens, .. }) => tokens.to_token_stream().to_string(),
        syn::Meta::NameValue(MetaNameValue {
            value: Expr::Lit(ExprLit {
                lit: Lit::Str(p), ..
            }),
            ..
        }) => p.value(),
        _ => {
            return Err(Error::new(
                package_span,
                "Usage: #[package = \"com.example.mod\"]",
            ))
        }
    };

    let class_name =
        get_rename_attr(package_span, &input.attrs)?.unwrap_or(struct_name.to_string());

    let package = package.replace(".", "/") + "/" + &class_name;

    Ok((struct_name.span(), package))
}
