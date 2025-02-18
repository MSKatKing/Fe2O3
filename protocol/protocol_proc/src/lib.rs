use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, DeriveInput, LitInt};

struct PacketAttributeArgs {
    id: LitInt,
}

impl Parse for PacketAttributeArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            id: input.parse()?,
        })
    }
}

#[proc_macro_attribute]
pub fn packet(attr: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as DeriveInput);
    let attr = parse_macro_input!(attr as PacketAttributeArgs);

    let struct_name = &item.ident;

    let data = match &item.data {
        syn::Data::Struct(s) => s,
        _ => panic!("expected a struct"),
    };

    let field_names = data.fields.iter().map(|f| f.ident.as_ref().unwrap()).collect::<Vec<_>>();
    let field_types = data.fields.iter().map(|f| &f.ty).collect::<Vec<_>>();

    let id = &attr.id;

    let out = quote! {
        #item

        impl crate::packets::Packet for #struct_name {
            fn serialize(self) -> Vec<u8> {
                use crate::data::PacketData;

                let mut out = Vec::new();
                #(
                    out.extend(self.#field_names.serialize());
                )*
                out
            }

            fn deserialize(queue: &mut data::queue::Queue) -> Option<Self> {
                use crate::data::PacketData;

                Some(
                    Self {
                        #(#field_names: <#field_types as PacketData>::deserialize(queue)?,)*
                    }
                )
            }

            fn id(&self) -> u8 {
                #id
            }
        }
    };

    out.into()
}