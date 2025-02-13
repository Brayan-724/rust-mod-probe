use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token, FieldsNamed, Ident, LitStr, Token, Type, Visibility,
};

mod kw {
    use syn::custom_keyword;

    custom_keyword!(extends);
    pub type Extends = extends;

    custom_keyword!(implements);
    pub type Implements = implements;
}

pub use kw::{Extends, Implements};

pub struct ClassInput {
    pub package: LitStr,
    pub extends: Option<(Extends, Type)>,
    pub implements_token: Option<Implements>,
    pub implements: Punctuated<Type, Token![,]>,
}

impl Parse for ClassInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let package = input.parse()?;
        let extends = if let Ok(extends) = input.parse() {
            let ty = input.parse()?;
            Some((extends, ty))
        } else {
            None
        };

        let implements_token = input.parse().ok();
        let implements = if implements_token.is_some() {
            input.parse_terminated(Type::parse, Token![,])?
        } else {
            Punctuated::new()
        };

        Ok(Self {
            package,
            extends,
            implements_token,
            implements,
        })
    }
}

pub struct ClassBody {
    pub vis: Visibility,
    pub struct_token: Token![struct],
    pub ident: Ident,
    pub fields: Option<FieldsNamed>,
}

impl Parse for ClassBody {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let vis = input.parse()?;
        let struct_token = input.parse()?;
        let ident = input.parse()?;
        let fields = if input.peek(token::Brace) {
            Some(input.parse()?)
        } else {
            input.parse::<Token![;]>()?;
            None
        };

        Ok(Self {
            vis,
            struct_token,
            ident,
            fields,
        })
    }
}
