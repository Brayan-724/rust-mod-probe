mod parse;

pub use parse::{ClassBody, ClassInput};
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};

pub fn main(input: ClassInput, body: ClassBody) -> syn::Result<TokenStream> {
    let struct_name = &body.ident;

    let signature = input.package.value();
    let signature = quote_spanned! {input.package.span() =>
        impl ::probe::JSignature for #struct_name {
            const CLASS: &str = #signature;
        }
    };

    Ok(quote! {
        #signature
    })
}
