use case::CaseExt;
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{
    spanned::Spanned, DataStruct, DeriveInput, Error, Field, FieldsNamed, FieldsUnnamed, Ident,
    Visibility,
};

use super::{get_attribute, get_rename_attr, sig};

pub fn main_struct(input: &DeriveInput, data: &DataStruct) -> Result<TokenStream, Error> {
    let struct_name = &input.ident;

    let (sig_span, sig) = sig::generate(&input)?;
    let signature = quote_spanned! {sig_span =>
        impl ::probe::JSignature for #struct_name {
            const CLASS: &str = #sig;
        }
    };

    let instance_field = match &data.fields {
        syn::Fields::Named(fields) => generate_instance_field_struct(&struct_name, &fields)?,
        syn::Fields::Unnamed(fields) => generate_instance_field_tuple(&struct_name, &fields)?,
        syn::Fields::Unit => {
            return Err(Error::new(Span::call_site(), "Units are not supported"));
        }
    };

    Ok(quote! {
        #instance_field

        #signature

        impl #struct_name {
            #[allow(dead_code)]
            #[inline(always)]
            pub fn class<'local>(env: &mut ::jni::JNIEnv<'local>) -> ::probe::primitives::JClass {
                env .find_class(<#struct_name as ::probe::JSignature>::CLASS)
                    .unwrap()
                    .into()
            }
        }
    })
}

pub fn generate_instance_field_struct(
    struct_name: &Ident,
    fields: &FieldsNamed,
) -> Result<TokenStream, syn::Error> {
    let instance_field = {
        // Get just the first field with `#[instance]`. If there are more than
        // one, then return None
        let Some(instance_field) = get_instance_field(fields.named.iter()) else {
            return Err(Error::new_spanned(
                struct_name,
                "Need just one instance: #[instance] pub raw: Instance",
            ));
        };

        if !matches!(instance_field.vis, Visibility::Public(..)) {
            return Err(Error::new_spanned(
                instance_field.ident.as_ref().unwrap(),
                "Instance should be public",
            ));
        }

        if let syn::Type::Path(path) = &instance_field.ty {
            let path = &path.path;

            let has_instance = path.segments.iter().any(|i| i.ident == "Instance");

            if !has_instance {
                return Err(Error::new_spanned(
                    &instance_field.ty,
                    "Instance should be of type `probe::java::Instance`",
                ));
            }
        } else {
            return Err(Error::new_spanned(
                &instance_field.ty,
                "Instance should be of type `probe::java::Instance`",
            ));
        }

        instance_field
    };

    let instance_field_ident = instance_field.ident.as_ref().unwrap();

    let out = generate_instance_field_common(
        struct_name,
        quote_spanned! {instance_field_ident.span() => self.#instance_field_ident},
    )?;

    let other_fields = fields
        .named
        .iter()
        .filter(|f| f.ident.as_ref().is_some_and(|f| f != instance_field_ident))
        .map(|f| {
            let field_ident = f.ident.as_ref().unwrap();
            let name = get_rename_attr(field_ident.span(), &f.attrs)?
                .unwrap_or(field_ident.to_string().as_str().to_camel_lowercase());

            Ok(quote! {
                #field_ident: ::probe::class::Field::<()>::new(instance, #name)
            })
        })
        .collect::<Result<Vec<_>, Error>>()?;

    Ok(quote_spanned! { instance_field_ident.span() =>
        #out

        impl #struct_name {
            #[allow(dead_code)]
            pub fn from_instance(instance: ::probe::class::Instance) -> #struct_name {
                Self {
                    #instance_field_ident: instance,
                    #(#other_fields),*
                }
            }
        }
    })
}

fn generate_instance_field_tuple(
    struct_name: &Ident,
    fields: &FieldsUnnamed,
) -> Result<TokenStream, Error> {
    let instance_field = {
        // Get just the first field with `#[instance]`. If there are more than
        // one, then return None
        let Some(instance_field) = get_instance_field(fields.unnamed.iter()) else {
            return Err(Error::new_spanned(
                struct_name,
                "Need just one instance: #[instance] Instance",
            ));
        };

        if !matches!(instance_field.vis, Visibility::Public(..)) {
            return Err(Error::new_spanned(
                instance_field,
                "Instance should be public",
            ));
        }

        if let syn::Type::Path(path) = &instance_field.ty {
            let path = &path.path;

            let has_instance = path.segments.iter().any(|i| i.ident == "Instance");

            if !has_instance {
                return Err(Error::new_spanned(
                    &instance_field.ty,
                    "Instance should be of type `probe::java::Instance`",
                ));
            }
        } else {
            return Err(Error::new_spanned(
                &instance_field.ty,
                "Instance should be of type `probe::java::Instance`",
            ));
        }

        instance_field
    };

    let out = generate_instance_field_common(
        struct_name,
        quote_spanned! {instance_field.span() => self.0},
    )?;

    Ok(quote_spanned! { instance_field.span() =>
        #out

        impl #struct_name {
            #[allow(dead_code)]
            pub fn from_instance(instance: ::probe::class::Instance) -> #struct_name {
                Self(instance)
            }
        }
    })
}

pub fn generate_instance_field_common(
    struct_name: &Ident,
    self_ident: TokenStream,
) -> Result<TokenStream, syn::Error> {
    Ok(quote_spanned! { self_ident.span() =>
        impl From<::probe::class::Instance> for #struct_name {
            fn from(value: ::probe::class::Instance) -> Self {
                #struct_name::from_instance(value)
            }
        }

        impl Into<::probe::class::Instance> for #struct_name {
            fn into(self) -> ::probe::class::Instance {
                #self_ident
            }
        }

        impl ::core::ops::Deref for #struct_name {
            type Target = ::probe::class::Instance;

            fn deref(&self) -> &Self::Target {
                &#self_ident
            }
        }

        impl ::probe::conversion::IntoJValue for #struct_name {
            fn into_jvalue<'local>(self, env: &mut ::jni::JNIEnv<'local>) -> ::jni::objects::JValueOwned<'local> {
                #self_ident.into_jvalue(env)
            }
        }

        impl ::probe::conversion::FromJValue for #struct_name {
            fn from_jvalue<'local>(value: ::jni::objects::JValueOwned<'local>) -> Self {
                <::probe::class::Instance as ::probe::conversion::FromJValue>::from_jvalue(value).into()
            }
        }

        impl ::probe::JavaClass for #struct_name {
            fn get_raw(&self) -> ::probe::class::Instance {
                #self_ident.clone()
            }
        }

        impl #struct_name {
            #[allow(dead_code)]
            pub fn cast_unchecked<T: From<::probe::class::Instance>>(&self) -> T {
                T::from(#self_ident)
            }
        }
    })
}

fn get_instance_field<'a>(iter: impl Iterator<Item = &'a Field>) -> Option<&'a Field> {
    iter.fold(None, |prev, field| {
        let has_instance = get_attribute(&"instance", &field.attrs).is_some();

        if !has_instance {
            return prev;
        }

        match prev {
            Some(None) => prev,
            Some(Some(_)) => Some(None),
            None => Some(Some(field)),
        }
    })
    .flatten()
}
