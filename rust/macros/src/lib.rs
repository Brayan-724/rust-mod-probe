extern crate proc_macro;

mod class;
mod import;
mod java_class;

use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(
    JavaClass,
    attributes(package, rename, variant, instance, field, enum_of)
)]
pub fn java_class(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    let data = match &input.data {
        syn::Data::Struct(data) => java_class::main_struct(&input, data),
        syn::Data::Enum(data) => java_class::main_enum(&input, data),
        syn::Data::Union(e) => Err(syn::Error::new(
            e.union_token.span,
            "Unions are not supported yet",
        )),
    };

    data.unwrap_or_else(|e| e.into_compile_error()).into()
}

#[proc_macro_attribute]
pub fn import(
    _input: proc_macro::TokenStream,
    body: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let impl_ = syn::parse_macro_input!(body as syn::ItemImpl);

    let data = import::main(impl_);

    data.unwrap_or_else(|e| e.into_compile_error()).into()
}

#[proc_macro_attribute]
pub fn class(
    input: proc_macro::TokenStream,
    body: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as class::ClassInput);
    let body = syn::parse_macro_input!(body as class::ClassBody);

    let data = class::main(input, body);

    data.unwrap_or_else(|e| e.into_compile_error()).into()
}

#[proc_macro]
pub fn test(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = match syn::parse::<syn::LitStr>(input) {
        Ok(o) => o,
        Err(e) => return e.to_compile_error().into()
    };
    let v = std::env::var("MINE").unwrap_or_default() + &input.value();
    println!("{}", v);
    std::env::set_var("MINE", v);
    quote! {}.into()
}

