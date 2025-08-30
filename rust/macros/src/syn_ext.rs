use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Token;

pub trait ParseStreamExt {
    fn parse_separated_until<U, T, P>(
        &self,
        parser: fn(ParseStream) -> syn::Result<T>,
    ) -> syn::Result<Punctuated<T, P>>
    where
        P: Token + Parse,
        U: Token;

    fn parse_if<T>(
        &self,
        condition: fn(ParseStream) -> bool,
        parser: fn(ParseStream) -> syn::Result<T>,
    ) -> syn::Result<Option<T>>;
}

impl<'a> ParseStreamExt for ParseStream<'a> {
    fn parse_separated_until<U, T, P>(
        &self,
        parser: fn(ParseStream) -> syn::Result<T>,
    ) -> syn::Result<Punctuated<T, P>>
    where
        P: Token + Parse,
        U: Token,
    {
        let mut punctuated = Punctuated::new();

        loop {
            if self.is_empty() {
                break;
            }

            let parse = parser(self)?;
            punctuated.push_value(parse);

            if P::peek(self.cursor()) {
                let parse = P::parse(self)?;
                punctuated.push_punct(parse);
            } else if U::peek(self.cursor()) {
                break;
            } else {
                return Err(syn::Error::new(
                    self.span(),
                    format!("expected {} or {}", P::display(), U::display()),
                ));
            }
        }

        Ok(punctuated)
    }

    fn parse_if<T>(
        &self,
        condition: fn(ParseStream) -> bool,
        parser: fn(ParseStream) -> syn::Result<T>,
    ) -> syn::Result<Option<T>> {
        if condition(self) {
            Ok(Some(parser(self)?))
        } else {
            Ok(None)
        }
    }
}
