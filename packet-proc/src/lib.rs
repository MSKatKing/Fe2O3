mod handlers;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct, AttributeArgs, NestedMeta, Lit, Fields, Token, Ident, LitStr, DeriveInput, ItemFn};
use syn::parse::ParseStream;
use syn::punctuated::Punctuated;
use crate::handlers::list_handlers_in_module;

#[proc_macro_attribute]
pub fn outgoing(_: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn state_changing(_: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn packet_handler(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemFn);
    let attr = parse_macro_input!(attr as AttributeArgs);

    if let Some(struct_name) = attr.first() {
        let inputs = &item.sig.inputs;
        let block = &item.block;

        let state_changing = if item.attrs.iter().any(|attr| attr.path.is_ident("state_changing")) {
            quote!(#[state_changing])
        } else {
            quote!()
        };

        quote! {
            impl #struct_name {
                #state_changing
                pub(crate) fn __generated_packet_handler(#inputs) #block
            }
        }
    } else {
        quote! {
            #item
        }
    }.into()
}

#[proc_macro_derive(Serializable)]
pub fn derive_serializable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;

    let fields = match &input.data {
        syn::Data::Struct(data_struct) => &data_struct.fields,
        _ => {
            return syn::Error::new_spanned(struct_name, "Serializable can only be derived for structs.").to_compile_error().into()
        }
    };

    let field_serializations = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! {
            buffer.write(self.#field_name);
        }
    });

    let output = quote! {
        impl packet::Serializable for #struct_name {
            fn serialize(self, buffer: &mut packet::Buffer) {
                #(#field_serializations)*
            }
        }
    };

    output.into()
}

#[proc_macro_derive(Deserializable)]
pub fn derive_deserializable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;

    let fields = match &input.data {
        syn::Data::Struct(data_struct) => &data_struct.fields,
        _ => {
            return syn::Error::new_spanned(struct_name, "Deserializable can only be derived for structs.").to_compile_error().into()
        }
    };

    let field_serializations = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! {
            #field_name: buffer.read(),
        }
    });

    let output = quote! {
        impl packet::Deserializable for #struct_name {
            fn deserialize(buffer: &mut packet::Buffer) -> Self {
                Self {
                    #(#field_serializations)*
                }
            }
        }
    };

    output.into()
}

struct RegistryInput {
    mappings: Punctuated<StateModuleMapping, Token![,]>,
}

struct StateModuleMapping {
    state: Ident,
    module: LitStr,
}

impl syn::parse::Parse for RegistryInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mappings = Punctuated::parse_terminated(input)?;
        Ok(Self { mappings })
    }
}

impl syn::parse::Parse for StateModuleMapping {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let state: Ident = input.parse()?;
        input.parse::<Token![=>]>()?;
        let module: LitStr = input.parse()?;
        Ok(Self { state, module })
    }
}

fn list_structs_in_module(module_path: &str) -> Vec<Ident> {
    let source_path = format!("{}.rs", module_path.replace("::", "/"));
    let file_content = std::fs::read_to_string(&source_path).expect("Failed to read module");
    let syntax = syn::parse_file(&file_content).expect("Failed to parse module file");

    syntax
        .items
        .iter()
        .filter_map(|item| {
            if let syn::Item::Struct(s) = item {
                let has_outgoing = s.attrs.iter().any(|attr| {
                    attr.path.is_ident("outgoing")
                });

                if !has_outgoing {
                    let is_packet = s.attrs.iter().any(|attr| {
                        attr.path.is_ident("packet")
                    });

                    if is_packet {
                        Some(s.ident.clone())
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

#[proc_macro]
pub fn add_packet_fn(input: TokenStream) -> TokenStream {
    let RegistryInput { mappings } = parse_macro_input!(input as RegistryInput);

    let mut state_arms = vec![];

    let mut handler_fns = vec![];

    for mapping in mappings {
        let state = mapping.state;
        let module_path = mapping.module.value();
        let structs = list_structs_in_module(&module_path);
        let handler = list_handlers_in_module(&module_path);

        for (is_state_changing, struct_name) in handler {
            handler_fns.push((is_state_changing, quote! {
                #struct_name::__generated_packet_handler
            }))
        }

        let mut packet_arms = vec![];

        for struct_name in structs {
            packet_arms.push(quote! {
                #struct_name::ID => {
                    let p = #struct_name::from_buffer(packet.data);

                    tracing::debug!("Added packet {}", stringify!(#struct_name));

                    // bus here

                    storages.add_component(id, p);
                }
            });
        }

        state_arms.push(quote! {
            PlayerState::#state => {
                match packet.id {
                    #(#packet_arms)*
                    id => {
                        tracing::debug!("No registered packets for id 0x{id:x}");
                    }
                }
            }
        });
    }

    let output = quote! {
        pub fn add_packet(
            storages: &mut shipyard::AllStoragesViewMut,
            id: shipyard::EntityId,
            state: crate::networking::player::PlayerState,
            packet: crate::networking::packet::Packet
        ) {
            use crate::networking::player::PlayerState;
            use packet::*;

            match state {
                #(#state_arms)*
                _ => {
                    tracing::debug!("No packets registered for current state!");
                }
            }
        }
    };

    let mut state_changing_fns = Vec::new();
    let mut non_state_changing_fns = Vec::new();

    for (state_changing, func) in handler_fns {
        if state_changing {
            &mut state_changing_fns
        } else {
            &mut non_state_changing_fns
        }.push(func);
    }

    let output = if state_changing_fns.is_empty() {
        quote! {
            #output

            pub fn packet_handlers() -> shipyard::Workload {
                use shipyard::IntoWorkload;
                (
                    evaluate_unprocessed_packets,
                    #(#non_state_changing_fns),*
                ).into_sequential_workload()
            }
        }
    } else {
        quote! {
            #output

            pub fn packet_handlers() -> shipyard::Workload {
                use shipyard::IntoWorkload;
                (
                    #(#state_changing_fns),*,
                    evaluate_unprocessed_packets,
                    #(#non_state_changing_fns),*
                ).into_sequential_workload()
            }
        }
    };

    output.into()
}

#[proc_macro_attribute]
pub fn packet(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let input = parse_macro_input!(item as ItemStruct);

    let id = match args.first() {
        Some(NestedMeta::Lit(Lit::Int(id))) => match id.base10_parse::<usize>() {
            Ok(id) => id,
            Err(_) => panic!("Cannot parse id as a usize in #[packet(id)]!")
        },
        _ => panic!("Expected #[packet(id)] where id is a usize")
    };

    let struct_name = input.ident.clone();
    let struct_attrs = input.attrs.clone();

    let expanded = match input.fields {
        Fields::Named(ref fields) => {
            let field_names = fields.named.iter().map(|field| {
                let field_name = &field.ident;
                quote! {
            #field_name,
        }
            });

            let serialize_fields = fields.named.iter().map(|field| {
                let field_name = &field.ident;
                if field.attrs.iter().any(|attr| attr.path.is_ident("compressed_int")) {
                    quote! {
                        buffer.write(packet::VarInt(self.#field_name));
                    }
                } else {
                    quote! {
                        buffer.write(self.#field_name);
                    }
                }
            });

            let deserialize_fields = fields.named.iter().map(|field| {
                let field_name = &field.ident;
                if field.attrs.iter().any(|attr| attr.path.is_ident("compressed_int")) {
                    quote! {
                        let #field_name = buffer.read::<packet::VarInt>().0;
                    }
                } else {
                    quote! {
                        let #field_name = buffer.read();
                    }
                }
            });

            quote! {
            impl packet::Packet for #struct_name {
            const ID: usize = #id;

            fn into_buffer(self) -> packet::Buffer {
                let mut buffer = packet::Buffer::new();
                #(#serialize_fields)*
                buffer
            }

            fn from_buffer(mut buffer: packet::Buffer) -> Self {
                #(#deserialize_fields)*
                Self {
                    #(#field_names)*
                }
            }
            }

            #[derive(shipyard::Component)]
            #(
                #struct_attrs
            )*
            #input
            }
        },
        _ => quote! {
            impl packet::Packet for #struct_name {
            const ID: usize = #id;

            fn into_buffer(self) -> packet::Buffer {
                packet::Buffer::new()
            }

            fn from_buffer(_: packet::Buffer) -> Self {
                Self {}
            }
            }

            #[derive(shipyard::Component)]
            #(
                #struct_attrs
            )*
            #input
            }
    };

    expanded.into()
}
