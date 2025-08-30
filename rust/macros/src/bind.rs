#![allow(dead_code)]

mod parse;

use case::CaseExt;
use itertools::Itertools;
use quote::{quote, quote_spanned};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Ident, Type};
use syn::{Token, TypePath};

use crate::bind::parse::{
    BindDefinition, BindField, BindFieldMethod, BindFieldProperty, BindInput, BindPackage,
    BindVariant,
};
use crate::java_class::get_attribute;

pub fn bind(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let BindInput { package, defs } = syn::parse_macro_input!(input as BindInput);

    let defs = defs.into_iter().map(|def| bind_def(def, &package));

    quote!(#(#defs)*).into()
}

fn bind_def(def: BindDefinition, package: &BindPackage) -> proc_macro2::TokenStream {
    match def {
        BindDefinition::Enum {
            name,
            variants,
            enum_of,
            ..
        } => bind_def_enum(name, variants, enum_of, package),
        BindDefinition::Class { class, fields, .. } => bind_def_class(class, fields, package),
    }
}

fn bind_def_enum(
    class: Ident,
    variants: Punctuated<BindVariant, Token![,]>,
    enum_of: Option<(Token![:], TypePath)>,
    package: &BindPackage,
) -> proc_macro2::TokenStream {
    if variants.is_empty() {
        class.span().unwrap().error("Add some variant").emit();
        return quote! {};
    }

    let is_itself = !enum_of.is_some();
    let enum_of = enum_of.map_or_else(|| syn::parse_quote!(#class), |(_, ty)| ty);

    let decl_variants = variants.iter().map(|v| &v.ident).collect_vec();

    let sig = create_signature(&class, package);

    let signature = if is_itself {
        quote_spanned! {class.span() =>
            impl ::rosttasse::prelude::JSignature for #class {
                const CLASS: &str = #sig;
            }
        }
    } else {
        quote_spanned! {class.span() =>
            impl ::rosttasse::prelude::JSignature for #class {
                const CLASS: &str = <#enum_of as ::rosttasse::prelude::JSignature>::CLASS;

                fn sig() -> String {
                    <#enum_of as ::rosttasse::prelude::JSignature>::sig()
                }
            }
        }
    };

    let get_field = {
        let struct_jsig =
            quote_spanned! {class.span() => <#class as ::rosttasse::prelude::JSignature> };

        let match_ = variants
            .iter()
            .map(|variant| {
                let variant_name = variant.rename.as_ref().map(|(_, str)| str.value());

                let name = &variant.ident.to_string();

                let variant_sig = if !is_itself {
                    quote!(#struct_jsig::sig())
                } else if let Some(ty) = variant_name {
                    quote! {
                        format!(concat!("L{}$", #ty, ";"), #struct_jsig::CLASS)
                    }
                } else {
                    quote!(#struct_jsig::sig())
                };

                let variant_name = &variant.ident;
                quote! {
                    Self::#variant_name => (#name, #variant_sig)
                }
            })
            .collect_vec();

        quote! {
            let (field, sig) = match &self {
                #(#match_),*
            };

            env
                .get_static_field(class, field, sig)
                .unwrap()
        }
    };

    quote! {
        #[allow(dead_code)]
        pub enum #class {
            #(#decl_variants),*
        }

        #signature

        impl ::rosttasse::prelude::IntoJValue for #class {
            fn into_jvalue<'local>(self, env: &mut ::jni::JNIEnv<'local>) -> ::jni::objects::JValueOwned<'local> {
                self.get_raw(env)
            }
        }

        impl ::rosttasse::prelude::FromJValue for #class {
            fn from_jvalue<'local>(value: ::jni::objects::JValueOwned<'local>) -> Self {
                todo!(concat!("impl FromJValue for ", stringify!(#class)))
                // <::rosttasse::prelude::Instance as ::rosttasse::prelude::FromJValue>::from_jvalue(value).into()
            }
        }

        impl #class {
            #[allow(dead_code)]
            pub fn get_raw<'local>(&self, env: &mut ::jni::JNIEnv<'local>) -> ::jni::objects::JValueOwned<'local> {
                let class = env
                    .find_class(#sig)
                    .expect(concat!("Cannot get ", stringify!(#class), " class"));

                #get_field
            }

            #[allow(dead_code)]
            pub fn get<'local>(&self, env: &mut ::jni::JNIEnv<'local>) -> #enum_of {
                <#enum_of as ::rosttasse::prelude::FromJValue>::from_jvalue(
                    self.get_raw(env)
                )
            }
        }
    }
}

fn bind_def_class(
    class: Ident,
    fields: Vec<BindField>,
    package: &BindPackage,
) -> proc_macro2::TokenStream {
    let (props, static_props, methods) = fields.into_iter().fold(
        (vec![], vec![], vec![]),
        |(mut props, mut static_props, mut methods), field| {
            match field {
                BindField::Property(
                    field @ BindFieldProperty {
                        is_static: true, ..
                    },
                ) => static_props.push(field),
                BindField::Property(
                    field @ BindFieldProperty {
                        is_static: false, ..
                    },
                ) => props.push(field),
                BindField::Method(field) => methods.push(field),
            };

            (props, static_props, methods)
        },
    );

    let decl_props = props
        .iter()
        .map(|prop| {
            let name = &prop.ident;
            let ty = &prop.ty;
            quote_spanned! {name.span() =>
                #[allow(dead_code)]
                pub #name: ::rosttasse::prelude::Field<#ty>
            }
        })
        .collect_vec();

    let static_props = static_props
        .into_iter()
        .map(|prop| {
            let name = prop.ident;
            let ty = prop.ty;
            quote_spanned! {name.span() => pub const #name: ::rosttasse::prelude::Field<#ty> }
        })
        .collect_vec();

    let class_name = class.span();
    let methods = methods
        .into_iter()
        .map(|method| {
            let method_info = prepare_method(&method);
            if method.is_static {
                bind_fn_static(class_name, method, method_info)
            } else {
                bind_fn(method, method_info)
            }
        })
        .collect_vec();

    let sig = generate_signature(&class, package);

    let other_fields = props
        .iter()
        .map(|f| {
            let field_ident = &f.ident;
            let name = f.ident.to_string().as_str().to_camel_lowercase();

            quote! {
                #field_ident: ::rosttasse::prelude::Field::<()>::new(raw, #name)
            }
        })
        .collect::<Vec<_>>();

    let instance_impl = crate::java_class::struct_::generate_instance_field_common(
        &class,
        quote_spanned! {class.span() => self.raw},
        quote_spanned! {class.span() => Self {
            raw,
            #(#other_fields),*
        }},
    );

    quote! {
        pub struct #class {
            raw: ::rosttasse::prelude::Instance,

            #(#decl_props),*
        }

        #sig

        #instance_impl

        impl #class {
            #(#static_props)*

            #(#methods)*
        }
    }
}

fn bind_fn(method: BindFieldMethod, method_info: MethodInfo) -> proc_macro2::TokenStream {
    let name = method.name;
    let ret_ty = method.ret;
    let decl_params = method_info.decl_params;
    let def_args = method_info.def_args;
    let extern_name = method_info.extern_name;
    let get_sig_args = method_info.get_sig_args;
    let ret = method_info.ret;

    quote_spanned! {name.span() =>
        #[allow(dead_code)]
        pub fn #name<'local>(&self, #decl_params) #ret_ty {
            #get_sig_args

            let obj: ::jni::objects::JObject = <Self as ::rosttasse::prelude::JavaClass>::get_raw(self).into();
            let _ret = env
                .call_method(obj, #extern_name, sig_args, #def_args)
                .unwrap();

            #ret
        }
    }
}

fn bind_fn_static(
    class_name: proc_macro2::Span,
    method: BindFieldMethod,
    method_info: MethodInfo,
) -> proc_macro2::TokenStream {
    let name = method.name;
    let ret_ty = method.ret;
    let decl_params = method_info.decl_params;
    let def_args = method_info.def_args;
    let extern_name = method_info.extern_name;
    let get_sig_args = method_info.get_sig_args;
    let ret = method_info.ret;

    let class_sig =
        quote_spanned! {class_name => <Self as ::rosttasse::prelude::JSignature>::CLASS};

    let is_contructor = get_attribute(&"constructor", &method.attributes).is_some();

    let body = if is_contructor {
        quote! {
            let instance: ::rosttasse::prelude::Instance = env
                .new_object(#class_sig, sig_args, #def_args)
                .unwrap()
                .into();

            <Self as ::rosttasse::prelude::JavaClass>::from_raw(instance)
        }
    } else {
        quote! {
            let _ret = env
                .call_static_method(
                    #class_sig,
                    #extern_name,
                    sig_args,
                    #def_args
                )
                .unwrap();

            #ret
        }
    };

    quote_spanned! {name.span() =>
        #[allow(dead_code)]
        pub fn #name<'local>(#decl_params) #ret_ty {
            #get_sig_args

            #body
        }
    }
}

pub fn create_signature(class: &Ident, package: &BindPackage) -> String {
    package.str.replace(".", "/") + "/" + &class.to_string()
}

pub fn generate_signature(class: &Ident, package: &BindPackage) -> proc_macro2::TokenStream {
    let sig = create_signature(class, package);

    quote_spanned! {class.span() =>
        impl ::rosttasse::prelude::JSignature for #class {
            const CLASS: &str = #sig;
        }
    }
}

struct MethodInfo {
    decl_params: proc_macro2::TokenStream,
    def_args: proc_macro2::TokenStream,
    extern_name: proc_macro2::TokenStream,
    get_sig_args: proc_macro2::TokenStream,
    ret: proc_macro2::TokenStream,
}

fn prepare_method(method: &BindFieldMethod) -> MethodInfo {
    let extern_name = method.name.to_string().as_str().to_camel_lowercase();
    let extern_name = quote!(#extern_name);

    let decl_params = method.params.iter().map(|param| {
        let name = &param.0;
        let ty = &param.2;
        quote! {#name: #ty}
    });
    let decl_params = quote!(#(#decl_params,)* env: &mut ::jni::JNIEnv<'local>);

    let (decl_args, def_args) = method
        .params
        .iter()
        .enumerate()
        .map(|(idx, arg)| {
            let arg_orig_ident = &arg.0;
            let arg_ident = quote::format_ident!("_arg_{idx}", span = arg_orig_ident.span());
            let arg_ty = &arg.2;

            let decl = quote_spanned! {arg_ident.span() =>
                let #arg_ident = ::rosttasse::prelude::IntoJValue::into_jvalue(#arg_orig_ident, env);
                sig_args += &(<#arg_ty as ::rosttasse::prelude::JSignature>::sig());
            };

            let def = quote_spanned! {arg_ident.span() => #arg_ident.borrow()};

            (decl, def)
        })
        .unzip::<_, _, Vec<_>, Vec<_>>();
    let def_args = quote!(&[#(#def_args),*]);

    let out_ty = match &method.ret {
        syn::ReturnType::Default => quote_spanned! {method.ret.span() => ()},
        syn::ReturnType::Type(_, ty) => quote_spanned! {ty.span() => #ty},
    };

    let out_ty_sig =
        quote_spanned! {out_ty.span() => <#out_ty as ::rosttasse::prelude::JSignature>::sig()};
    let get_sig_args = quote! {
        #[allow(unused_mut)]
        let mut sig_args = String::from("(");

        #(#decl_args)*

        sig_args += ")";
        sig_args += &#out_ty_sig;
    };

    let ret = match &method.ret {
        syn::ReturnType::Default => quote_spanned! {out_ty.span() =>},
        syn::ReturnType::Type(s, ty) => match &**ty {
            Type::Tuple(ty_) if ty_.elems.is_empty() => quote_spanned! {out_ty.span() =>},
            _ => {
                quote_spanned! {s.span() => <#ty as ::rosttasse::prelude::FromJValue>::from_jvalue(_ret)}
            }
        },
    };

    MethodInfo {
        decl_params,
        def_args,
        extern_name,
        get_sig_args,
        ret,
    }
}
