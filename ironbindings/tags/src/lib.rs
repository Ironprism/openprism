extern crate proc_macro;
use proc_macro::TokenStream;
use lazy_static::lazy_static;
use syn::{DeriveInput, parse_macro_input};
use quote::quote;

lazy_static! {
    static ref ANALYSIS: Analysis = Analysis::new();
}

struct Analysis {
    /// Types placeholders defined by the user in the Cargo.toml metadata
    type_groups: Vec<(String, Vec<String>)>,
}

impl Analysis {
    fn new() -> Self {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let manifest = std::fs::read_to_string(format!("{}/Cargo.toml", manifest_dir)).unwrap();
        let toml: toml::Value = toml::from_str(&manifest).unwrap();

        let type_groups = toml
            .get("package")
            .and_then(|package| package.get("metadata"))
            .and_then(|metadata| metadata.get("ironbindings"))
            .and_then(|ironbindings| ironbindings.as_table())
            .and_then(|ironbindings| ironbindings.get("types"))
            .and_then(|types| types.as_table())
            .map(|types| {
                types
                    .iter()
                    .map(|(group, types)| {
                        let types = types
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|t| t.as_str().unwrap().to_string())
                            .collect();
                        (group.to_string(), types)
                    })
                    .collect()
            }).unwrap_or_default();

        Self {
            type_groups: type_groups,
        }
    }
}

#[proc_macro_attribute]
pub fn ironbindings(attr: TokenStream, item: TokenStream) -> TokenStream {
    if attr.to_string() == "ignore" {
        return item;
    }
    
    dbg!(&ANALYSIS.type_groups);
    let to_parse = item.clone();
    let derive_input = parse_macro_input!(to_parse as DeriveInput);
    
    item
}