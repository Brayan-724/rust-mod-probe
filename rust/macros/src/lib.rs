#![feature(proc_macro_span)]
#![feature(proc_macro_diagnostic)]

extern crate proc_macro;

mod bind;
mod export;
mod syn_ext;
mod utils;

use quote::quote;

#[proc_macro]
pub fn bind(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    bind::bind(input)
}

#[proc_macro_attribute]
pub fn export(
    input: proc_macro::TokenStream,
    body: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    export::export(input, body)
}

#[proc_macro]
pub fn test(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = match syn::parse::<syn::LitStr>(input) {
        Ok(o) => o,
        Err(e) => return e.to_compile_error().into(),
    };
    let v = std::env::var("MINE").unwrap_or_default() + &input.value();
    println!("{}", v);
    std::env::set_var("MINE", v);
    quote! {}.into()
}
