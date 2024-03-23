use rustdoc_types::{Crate, Id};
use anyhow::Result;
use std::collections::HashMap;

pub fn handle(_krate: &Crate, impls: &mut HashMap<String, Vec<Id>>) -> Result<Vec<syn::Item>> {
    if impls.remove("core::hash::Hash").is_none() {
        return Ok(vec![]);
    }
    
    Ok(vec![syn::Item::Fn(syn::parse_quote!(
        #[automatically_derived]
        fn __hash__(&self) -> PyResult<isize> {
            use core::hash::{Hash, Hasher};
            use std::collections::hash_map::DefaultHasher;
            let mut hasher = DefaultHasher::new();
            self.0.hash(&mut hasher);
            Ok(hasher.finish() as isize)
        }
    ))])
}