use rustdoc_types::{Crate, Id};
use anyhow::Result;
use std::collections::HashMap;

/// ToString implies Display, so we can just use the Display implementation
pub fn handle(_krate: &Crate, impls: &mut HashMap<String, Vec<Id>>) -> Result<Vec<syn::Item>> {
    if impls.remove("core::fmt::Display").is_none() {
        return Ok(vec![]);
    }
    
    Ok(vec![syn::Item::Fn(syn::parse_quote!(
        #[automatically_derived]
        fn __str__(&self) -> PyResult<String> {
            Ok(format!("{}", self.0))
        }
    ))])
}