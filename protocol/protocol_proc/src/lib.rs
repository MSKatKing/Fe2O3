use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Ident, LitInt, Token};
use syn::parse::ParseStream;

struct PacketAttributeArgs {
    id: LitInt,
    _comma: Token![,],
    state: Ident
}

impl Parse for PacketAttributeArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            id: input.parse()?,
            _comma: input.parse()?,
            state: input.parse()?
        })
    }
}

#[proc_macro_attribute]
pub fn packet(attr: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as DeriveInput);
    let attr = parse_macro_input!(attr as PacketAttributeArgs);
}