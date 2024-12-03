use quote::quote;
use syn::Meta;
use syn::__private::{str, TokenStream2};

pub fn list_handlers_in_module(module_path: &str) -> Vec<(bool, TokenStream2)> {
    let source_path = format!("{}.rs", module_path.replace("::", "/"));
    let file_content = std::fs::read_to_string(&source_path).expect("Failed to read module");
    let syntax = syn::parse_file(&file_content).expect("Failed to parse module file");

    syntax
        .items
        .iter()
        .filter_map(|item| {
            if let syn::Item::Fn(s) = item {
                let is_packet = s.attrs.iter().find(|attr| {
                    attr.path.is_ident("packet_handler")
                });

                let is_state_changing = s.attrs.iter().any(|attr| {
                    attr.path.is_ident("state_changing")
                });

                if let Some(attr) = is_packet {
                    if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
                        if let Some(struct_name) = meta_list.nested.first() {
                            return Some((is_state_changing, quote!(#struct_name)))
                        } else {
                            panic!("Struct name not found! (Expected #[packet_handler(...)])");
                        }
                    }
                }

                None
            } else {
                None
            }
        })
        .collect()
}