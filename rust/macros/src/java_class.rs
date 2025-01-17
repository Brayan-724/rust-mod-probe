mod enum_;
mod sig;
mod struct_;

use proc_macro2::Span;
use quote::ToTokens;
use syn::{Attribute, Error, Expr, ExprLit, Lit, LitStr, Meta, MetaList, MetaNameValue};

pub use enum_::main_enum;
pub use struct_::main_struct;

pub fn get_string_attr(span: Span, attr_name: &impl AsRef<str>, attrs: &Vec<Attribute>) -> Result<Option<String>, Error> {
    let attr = get_attribute(attr_name, &attrs).map(|attr| match &attr.meta {
        Meta::List(MetaList { tokens, .. }) => {
            Ok(syn::parse2::<LitStr>(tokens.to_token_stream())?.value())
        }
        Meta::NameValue(MetaNameValue {
            value: Expr::Lit(ExprLit {
                lit: Lit::Str(p), ..
            }),
            ..
        }) => Ok(p.value()),
        _ => Err(Error::new(span, format!("Usage: #[{} = \"OtherName\"]", attr_name.as_ref()))),
    });

    if let Some(attr) = attr {
        Ok(Some(attr?))
    } else {
        Ok(None)
    }
}

pub fn get_rename_attr(span: Span, attrs: &Vec<Attribute>) -> Result<Option<String>, Error> {
    get_string_attr(span, &"rename", attrs)
}

pub fn get_attribute<'a, 'b>(
    attr_name: &'b impl AsRef<str>,
    attrs: &'a Vec<Attribute>,
) -> Option<&'a Attribute> {
    attrs.iter().find(|attr| attr.path().is_ident(attr_name))
}
