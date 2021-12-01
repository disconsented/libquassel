use quote::quote;
use syn::{
    self, bracketed,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token, Expr, Result, Token,
};

#[derive(Debug)]
struct Sync {
    name: Expr,
    token: Token![,],
    brace_token: token::Bracket,
    fields: Punctuated<Expr, Token![,]>,
}

impl Parse for Sync {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Sync {
            name: input.parse()?,
            token: input.parse()?,
            brace_token: bracketed!(content in input),
            fields: content.parse_terminated(Expr::parse)?,
        })
    }
}

pub fn sync(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as Sync);
    let name = input.name;
    let values = input.fields.iter();

    let gen = quote! {
        self.send_sync(
            #name,
            vec![#(#values.into()),*],
        )
    };

    gen.into()
}
