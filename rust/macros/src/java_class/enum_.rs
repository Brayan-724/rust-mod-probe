use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use syn::{spanned::Spanned, DataEnum, DeriveInput, Error, Meta, MetaList, Type};

use crate::java_class::{get_rename_attr, get_string_attr};

use super::{get_attribute, sig};

pub fn main_enum(input: &DeriveInput, data: &DataEnum) -> Result<TokenStream, Error> {
    let struct_name = &input.ident;

    let (sig_span, sig) = sig::generate(input)?;

    // `enum_of` is optional, if none then use itself as type
    let (enum_of, is_itself) = if let Some(attr) = get_attribute(&"enum_of", &input.attrs) {
        let ty = match &attr.meta {
            Meta::List(MetaList { tokens, .. }) => syn::parse2::<Type>(tokens.clone())?,
            _ => return Err(Error::new(attr.span(), "Usage: #[enum_of(Type)]")),
        };

        (ty, false)
    } else {
        let ty = syn::parse2::<Type>(input.ident.to_token_stream())?;

        (ty, true)
    };

    let signature = if is_itself {
        quote_spanned! {sig_span =>
            impl ::probe::JSignature for #struct_name {
                const CLASS: &str = #sig;
            }
        }
    } else {
        quote_spanned! {sig_span =>
            impl ::probe::JSignature for #struct_name {
                const CLASS: &str = <#enum_of as ::probe::JSignature>::CLASS;

                fn sig() -> String {
                    <#enum_of as ::probe::JSignature>::sig()
                }
            }
        }
    };

    if data.variants.is_empty() {
        return Err(Error::new(input.ident.span(), "Add some variant"));
    }

    let struct_class = quote! {
        <#struct_name as ::probe::JSignature>
    };

    let get_field = {
        let match_ = data
            .variants
            .iter()
            .map(|variant| {
                let rename = get_rename_attr(variant.span(), &variant.attrs)?;
                let variant_name = get_string_attr(variant.span(), &"variant", &variant.attrs)?;

                let name = &variant.ident;
                let name = rename
                    .map(|n| n)
                    .unwrap_or(name.to_string());

                let variant_sig = if !is_itself {
                    quote!(#struct_class::sig())
                } else if let Some(ty) = variant_name {
                    quote! {
                        format!(concat!("L{}$", #ty, ";"), #struct_class::CLASS)
                    }
                } else {
                    quote!(#struct_class::sig())
                };

                let variant_name = &variant.ident;
                Ok(quote! {
                    Self::#variant_name => (#name, #variant_sig)
                })
            })
            .collect::<Result<Vec<_>, Error>>()?;

        quote! {
            let (field, sig) = match &self {
                #(#match_),*
            };

            env
                .get_static_field(class, field, sig)
                .unwrap()
        }
    };

    Ok(quote! {
        #signature

        impl ::probe::conversion::IntoJValue for #struct_name {
            fn into_jvalue<'local>(self, env: &mut ::jni::JNIEnv<'local>) -> ::jni::objects::JValueOwned<'local> {
                self.get_raw(env)
            }
        }

        impl ::probe::conversion::FromJValue for #struct_name {
            fn from_jvalue<'local>(value: ::jni::objects::JValueOwned<'local>) -> Self {
                todo!(concat!("impl FromJValue for ", stringify!(#struct_name)))
                // <::probe::class::Instance as ::probe::conversion::FromJValue>::from_jvalue(value).into()
            }
        }

        impl #struct_name {
            #[allow(dead_code)]
            pub fn get_raw<'local>(&self, env: &mut ::jni::JNIEnv<'local>) -> ::jni::objects::JValueOwned<'local> {
                let class = env
                    .find_class(#sig)
                    .expect(concat!("Cannot get ", stringify!(#struct_name), " class"));

                #get_field
            }

            #[allow(dead_code)]
            pub fn get<'local>(&self, env: &mut ::jni::JNIEnv<'local>) -> #enum_of {
                <#enum_of as ::probe::conversion::FromJValue>::from_jvalue(
                    self.get_raw(env)
                )
            }
        }
    })
}
